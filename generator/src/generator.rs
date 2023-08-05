// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Copied from **pest/generator/src/generator.rs** (commit ac0aed3eecf435fd93ba575a39704aaa88a375b7)
//! and modified.

use std::path::PathBuf;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{self, Ident};

use pest_meta::optimizer::*;

use super::docs::DocComment;

/// Generate Rust `include_str!` for grammar files, then Cargo will watch changes in grammars.
pub(crate) fn generate_include(name: &Ident, paths: Vec<PathBuf>) -> TokenStream {
    let const_name = format_ident!("_PEST_GRAMMAR_{}", name);
    // Need to make this relative to the current directory since the path to the file
    // is derived from the CARGO_MANIFEST_DIR environment variable
    let current_dir = std::env::current_dir().expect("Unable to get current directory");

    let include_tokens = paths.iter().map(|path| {
        let path = path.to_str().expect("non-Unicode path");

        let relative_path = current_dir
            .join(path)
            .to_str()
            .expect("path contains invalid unicode")
            .to_string();

        quote! {
            include_str!(#relative_path)
        }
    });

    let len = include_tokens.len();
    quote! {
        #[allow(non_upper_case_globals)]
        const #const_name: [&'static ::core::primitive::str; #len] = [
            #(#include_tokens),*
        ];
    }
}
pub(crate) fn generate_enum(
    rules: &[OptimizedRule],
    doc_comment: &DocComment,
) -> TokenStream {
    let rules = rules.iter().map(|rule| {
        let rule_name = format_ident!("r#{}", rule.name);

        match doc_comment.line_docs.get(&rule.name) {
            Some(doc) => quote! {
                #[doc = #doc]
                #rule_name
            },
            None => quote! {
                #rule_name
            },
        }
    });

    let grammar_doc = &doc_comment.grammar_doc;
    quote! {
        #[doc = #grammar_doc]
        #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum Rule {
            EOI,
            #( #rules, )*
        }
    }
}
