# Metering a project's transport and its storage

stingy4rust cannot know a project's ports: what a message is, how it is
encoded, what a store writes. It gives two meters, and the project writes one
thin decorator over each port that calls them. Because every adapter behind a
port keeps one contract, one decorator a port meters every adapter: the
in-memory one a profile runs on, and the network or the disk in production.

## The meters

A profile's body is handed a `Run`; the meters come from it, and each is
`Copy` and `'static`, so it can sit inside a project's own boxed adapter:

| meter | from | calls |
|---|---|---|
| `WireMeter` | `run.wire()` | `sent(kind, bytes)` -- one message of a kind, and its encoded bytes |
| `StorageMeter` | `run.storage("journal")`, `run.storage_numbered("log", 3)` | `written(bytes)` -- one write; `transaction()` -- one sync or commit; `at_rest(bytes)` -- what the store holds, encoded; `on_disk(bytes)` -- its files' size, where it has a disk |

`at_rest` and `on_disk` keep the last value given: call them at the end of the
profile. A store that never reports `on_disk` has no such figure. While an
operation is open -- `let _command = run.operation("command");` -- every
message sent is charged to it as well as to its kind, so the report gives the
messages and bytes an operation costs. Operations do not nest: opening one
while another is open charges the wire to the newer one until it ends.

A kind, a store or an operation is named by a `&'static str` with no `.` in
it: the name is part of a figure's path, `wire.append.bytes`. Every name is a
budgeted figure, so keep them few and stable -- a message's kind, never its
content.

## A transport

The project's port, as small as it may be:

```rust
pub trait Transport {
    fn send(&mut self, to: u32, message: Message);
}

pub enum Message {
    Ping,
    Append(Vec<u8>),
}

impl Message {
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Ping => "ping",
            Self::Append(_) => "append",
        }
    }

    // The bytes the wire carries, as the project encodes them.
    pub fn encoded_len(&self) -> usize {
        match self {
            Self::Ping => 1,
            Self::Append(entries) => 1 + 4 + entries.len(),
        }
    }
}
```

and its decorator, which counts each message and hands it on unchanged:

```rust
use stingy4rust::run::wire_meter::WireMeter;

pub struct MeteredTransport<T> {
    inner: T,
    meter: WireMeter,
}

impl<T> MeteredTransport<T> {
    pub const fn new(inner: T, meter: WireMeter) -> Self {
        Self { inner, meter }
    }
}

impl<T: Transport> Transport for MeteredTransport<T> {
    fn send(&mut self, to: u32, message: Message) {
        self.meter.sent(message.kind(), message.encoded_len() as u64);
        self.inner.send(to, message);
    }
}
```

## A store

```rust
pub trait Storage {
    fn append(&mut self, record: &[u8]);
    fn sync(&mut self);
    fn held_bytes(&self) -> u64;
}
```

```rust
use stingy4rust::run::storage_meter::StorageMeter;

pub struct MeteredStorage<S> {
    inner: S,
    meter: StorageMeter,
}

impl<S> MeteredStorage<S> {
    pub const fn new(inner: S, meter: StorageMeter) -> Self {
        Self { inner, meter }
    }
}

impl<S: Storage> MeteredStorage<S> {
    // Called once, at the end of the profile.
    pub fn report_at_rest(&self) {
        self.meter.at_rest(self.inner.held_bytes());
    }
}

impl<S: Storage> Storage for MeteredStorage<S> {
    fn append(&mut self, record: &[u8]) {
        self.meter.written(record.len() as u64);
        self.inner.append(record);
    }

    fn sync(&mut self) {
        self.meter.transaction();
        self.inner.sync();
    }

    fn held_bytes(&self) -> u64 {
        self.inner.held_bytes()
    }
}
```

A store with files reports their size the same way, through `on_disk`, read
from the files themselves. What it holds at rest and what it takes on disk
differ by the store's own amplification -- pages, journals -- which is the
store's, not the code's; both are kept.

## In a profile

The decorators wrap the adapters a profile runs on, inside the scopes and
operations the profile opens:

```rust
fn steady_load(run: &Run) {
    let transport = MeteredTransport::new(InMemoryTransport::new(), run.wire());
    let storage = MeteredStorage::new(InMemoryStorage::new(), run.storage_numbered("log", 1));
    let mut node = Node::new(transport, storage, run.seed());
    for _ in 0..100 {
        let _command = run.operation("command");
        let _step = run.scope_numbered("node", 1);
        node.submit_and_advance();
    }
    node.storage().report_at_rest();
}
```

The decorators belong to the profile target -- the package's `stingy` test,
or a feature only it turns on -- never to the code that ships.
