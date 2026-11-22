// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::visit::{
    Visit, visit_attribute, visit_expr_unsafe, visit_item_foreign_mod, visit_item_impl,
    visit_item_trait, visit_signature,
};
use syn::{Attribute, ExprUnsafe, File, ItemForeignMod, ItemImpl, ItemTrait, Signature};

// Every kind of `unsafe` one parsed file holds -- a block, a fn, an impl, a
// trait, an extern block, an attribute -- each named once, in order. Read from
// the syntax tree, so the word in a string or a comment is not a use; a macro's
// arguments are tokens syn never parses, so a use inside `assert!(...)` is not
// seen, and the compiler's `unsafe_code` lint stays the first line.
pub struct UnsafeUseCollector {
    found: Vec<String>,
}

impl UnsafeUseCollector {
    #[must_use]
    pub fn collect(file: &File) -> Vec<String> {
        let mut collector = Self { found: Vec::new() };
        collector.visit_file(file);
        collector.found.sort();
        collector.found.dedup();
        collector.found
    }

    fn record(&mut self, present: bool, kind: &str) {
        if present {
            self.found.push(format!("an unsafe {kind}"));
        }
    }
}

impl<'ast> Visit<'ast> for UnsafeUseCollector {
    fn visit_attribute(&mut self, node: &'ast Attribute) {
        self.record(node.path().is_ident("unsafe"), "attribute");
        visit_attribute(self, node);
    }

    fn visit_expr_unsafe(&mut self, node: &'ast ExprUnsafe) {
        self.record(true, "block");
        visit_expr_unsafe(self, node);
    }

    fn visit_item_foreign_mod(&mut self, node: &'ast ItemForeignMod) {
        self.record(node.unsafety.is_some(), "extern block");
        visit_item_foreign_mod(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        self.record(node.unsafety.is_some(), "impl");
        visit_item_impl(self, node);
    }

    fn visit_item_trait(&mut self, node: &'ast ItemTrait) {
        self.record(node.unsafety.is_some(), "trait");
        visit_item_trait(self, node);
    }

    fn visit_signature(&mut self, node: &'ast Signature) {
        self.record(node.unsafety.is_some(), "fn");
        visit_signature(self, node);
    }
}
