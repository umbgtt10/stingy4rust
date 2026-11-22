// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::parse_file;
use xtask::boundary::unsafe_use_collector::UnsafeUseCollector;

fn collected(source: &str) -> Vec<String> {
    let syntax = parse_file(source).expect("valid Rust");
    UnsafeUseCollector::collect(&syntax)
}

#[test]
fn collect_a_file_with_no_unsafe_finds_nothing() {
    // Arrange & Act
    let found = collected("fn f(values: &[u8]) -> usize { values.len() }");

    // Assert
    assert_eq!(found, Vec::<String>::new());
}

#[test]
fn collect_an_unsafe_attribute_names_it() {
    // Arrange & Act
    let found = collected("#[unsafe(no_mangle)]\npub extern \"C\" fn f() {}");

    // Assert
    assert_eq!(found, vec![String::from("an unsafe attribute")]);
}

#[test]
fn collect_an_unsafe_block_inside_a_closure_names_it() {
    // Arrange & Act
    let found = collected("fn f(p: *const u8) -> u8 { let read = || unsafe { *p }; read() }");

    // Assert
    assert_eq!(found, vec![String::from("an unsafe block")]);
}

#[test]
fn collect_an_unsafe_block_names_it() {
    // Arrange & Act
    let found = collected("fn f(p: *const u8) -> u8 { unsafe { *p } }");

    // Assert
    assert_eq!(found, vec![String::from("an unsafe block")]);
}

#[test]
fn collect_an_unsafe_extern_block_names_it() {
    // Arrange & Act
    let found = collected("unsafe extern \"C\" { fn abs(input: i32) -> i32; }");

    // Assert
    assert_eq!(found, vec![String::from("an unsafe extern block")]);
}

#[test]
fn collect_an_unsafe_fn_names_it() {
    // Arrange & Act
    let found = collected("unsafe fn f() {}");

    // Assert
    assert_eq!(found, vec![String::from("an unsafe fn")]);
}

#[test]
fn collect_an_unsafe_impl_names_it() {
    // Arrange & Act
    let found = collected("struct S;\nunsafe impl Send for S {}");

    // Assert
    assert_eq!(found, vec![String::from("an unsafe impl")]);
}

#[test]
fn collect_an_unsafe_method_names_it_as_an_unsafe_fn() {
    // Arrange & Act
    let found = collected("struct S;\nimpl S { unsafe fn f(&self) {} }");

    // Assert
    assert_eq!(found, vec![String::from("an unsafe fn")]);
}

#[test]
fn collect_an_unsafe_trait_names_it() {
    // Arrange & Act
    let found = collected("unsafe trait Marker {}");

    // Assert
    assert_eq!(found, vec![String::from("an unsafe trait")]);
}

#[test]
fn collect_every_kind_names_each_once_in_order() {
    // Arrange & Act
    let found = collected(
        "struct S;\nunsafe impl Send for S {}\nunsafe impl Sync for S {}\nunsafe fn f(p: *const u8) -> u8 { unsafe { *p } }\nfn g(p: *const u8) -> u8 { unsafe { *p } }",
    );

    // Assert
    assert_eq!(
        found,
        vec![
            String::from("an unsafe block"),
            String::from("an unsafe fn"),
            String::from("an unsafe impl"),
        ]
    );
}

#[test]
fn collect_the_word_unsafe_in_a_string_or_a_comment_finds_nothing() {
    // Arrange & Act
    let found = collected("// unsafe { }\nconst WORD: &str = \"unsafe { *p }\";");

    // Assert
    assert_eq!(found, Vec::<String>::new());
}
