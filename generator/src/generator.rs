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

use super::docs::DocComment;
use crate::graph::{pest_typed, Generate};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::path::PathBuf;
use syn::{self, Ident};

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
pub(crate) fn generate_enum<R: Generate>(rules: &[R], doc_comment: &DocComment) -> TokenStream {
    let rule_names = rules.iter().map(|rule| format_ident!("r#{}", rule.name()));
    let rule_name_strings = rules.iter().map(|rule| rule.name());

    let rules = rules.iter().map(|rule| {
        let rule_name = format_ident!("r#{}", rule.name());

        match doc_comment.line_docs.get(rule.name()) {
            Some(doc) => quote! {
                #[doc = #doc]
                #rule_name
            },
            None => quote! {
                #rule_name
            },
        }
    });

    let pest_typed = pest_typed();

    let grammar_doc = &doc_comment.grammar_doc;
    quote! {
        #[doc = #grammar_doc]
        #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum Rule {
            EOI,
            #( #rules, )*
        }
        impl #pest_typed::RuleType for Rule {
            fn name(&self) -> &'static ::core::primitive::str {
                match self {
                    #(
                        Self::#rule_names => #rule_name_strings,
                    )*
                    Self::EOI => "EOI",
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest_meta::{
        ast::RuleType,
        optimizer::{OptimizedExpr, OptimizedRule},
    };
    use std::collections::HashMap;

    #[test]
    fn rule_enum_simple() {
        let rules = vec![OptimizedRule {
            name: "f".to_owned(),
            ty: RuleType::Normal,
            expr: OptimizedExpr::Ident("g".to_owned()),
        }];

        let mut line_docs = HashMap::new();
        line_docs.insert("f".to_owned(), "This is rule comment".to_owned());

        let doc_comment = &DocComment {
            grammar_doc: "Rule doc\nhello".to_owned(),
            line_docs,
        };

        assert_eq!(
            generate_enum(&rules, doc_comment,).to_string(),
            quote! {
                #[doc = "Rule doc\nhello"]
                #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
                #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
                pub enum Rule {
                    EOI,
                    #[doc = "This is rule comment"]
                    r#f,
                }
                impl ::pest_typed::RuleType for Rule {
                    fn name(&self) -> &'static ::core::primitive::str {
                        match self {
                            Self::r#f => "f",
                            Self::EOI => "EOI",
                        }
                    }
                }
            }
            .to_string()
        );
    }
}
