// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs::create_dir_all;
use std::fs::read_to_string;
use std::fs::remove_file;
use std::fs::write;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;

use crate::ports::file_store::FileStore;
use crate::ports::store_failure::StoreFailure;

// Files on disk: one not there reads as none and removes as done, and a
// write makes the directories above it first.
pub struct DiskFileStore;

impl DiskFileStore {
    // What a file operation's error at `path` becomes.
    fn failed(path: &str) -> impl FnOnce(Error) -> StoreFailure + '_ {
        move |error| StoreFailure {
            path: String::from(path),
            reason: error.to_string(),
        }
    }
}

impl FileStore for DiskFileStore {
    fn read(&self, path: &str) -> Result<Option<String>, StoreFailure> {
        match read_to_string(path) {
            Ok(text) => Ok(Some(text)),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
            Err(error) => Err(Self::failed(path)(error)),
        }
    }

    fn remove(&self, path: &str) -> Result<(), StoreFailure> {
        match remove_file(path) {
            Err(error) if error.kind() != ErrorKind::NotFound => Err(Self::failed(path)(error)),
            _ => Ok(()),
        }
    }

    fn write(&self, path: &str, text: &str) -> Result<(), StoreFailure> {
        if let Some(parent) = Path::new(path).parent()
            && !parent.as_os_str().is_empty()
        {
            create_dir_all(parent).map_err(Self::failed(path))?;
        }
        write(path, text).map_err(Self::failed(path))
    }
}
