// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Adapted from [generator.rs](./generator.rs) (commit ac0aed3eecf435fd93ba575a39704aaa88a375b7).

use super::docs::{consume, DocComment};
use super::generator::{generate_enum, generate_include};
use super::helper::{collect_data, get_string, GrammarSource};
use super::types::result_type;
use crate::config::Config;
use crate::graph::{generate_typed_pair_from_rule, pest, pest_typed};
use crate::helper::get_bool;
use pest_meta::optimizer::OptimizedRule;
use pest_meta::parser::{consume_rules, parse, rename_meta_rule, Rule};
use pest_meta::{optimizer::optimize, unwrap_or_report};
use proc_macro2::TokenStream;
use quote::quote;
use std::path::PathBuf;
use syn::DeriveInput;
use syn::{self, Generics, Ident};

/// Processes the derive/proc macro input and generates the corresponding typed parser and nodes
/// based on the parsed grammar. It will generate an explicit "include_str" statement.
///
#[doc = include_str!("../Usage.md")]
pub fn derive_typed_parser(input: TokenStream, include_grammar: bool) -> TokenStream {
    let ast: DeriveInput = syn::parse2(input).unwrap();
    let (name, generics, contents, config) = parse_typed_derive(ast);

    let (data, paths) = collect_data(contents);

    let pairs = match parse(Rule::grammar_rules, &data) {
        Ok(pairs) => pairs,
        Err(error) => panic!("error parsing \n{}", error.renamed_rules(rename_meta_rule)),
    };

    let doc_comment = consume(pairs.clone());
    let ast = unwrap_or_report(consume_rules(pairs));
    let optimized = optimize(ast);

    generate_typed(
        name,
        &generics,
        paths,
        optimized,
        &doc_comment,
        include_grammar,
        config,
    )
}

fn parse_typed_derive(ast: DeriveInput) -> (Ident, Generics, Vec<GrammarSource>, Config) {
    let name = ast.ident;
    let generics = ast.generics;

    let mut grammar_sources = vec![];
    let mut config = Config::default();
    for attr in ast.attrs.iter() {
        let path = attr.meta.path();
        if path.is_ident("grammar") {
            grammar_sources.push(GrammarSource::File(get_string(attr)));
        } else if path.is_ident("grammar_inline") {
            grammar_sources.push(GrammarSource::Inline(get_string(attr)));
        } else if path.is_ident("emit_rule_reference") {
            config.emit_rule_reference = get_bool(attr);
        } else if path.is_ident("emit_tagged_node_reference") {
            config.emit_tagged_node_reference = get_bool(attr);
        } else if path.is_ident("do_not_emit_span") {
            config.do_not_emit_span = get_bool(attr);
        } else if path.is_ident("truncate_accesser_at_node_tag") {
            if cfg!(not(feature = "grammar-extras")) && !config.no_warnigs {
                eprintln!("Specify `truncate_accesser_at_node_tag` does not take effect when `grammar-extras` is not enabled.");
            }
            config.truncate_accesser_at_node_tag = get_bool(attr);
        } else if path.is_ident("simulate_pair_api") {
            config.simulate_pair_api = get_bool(attr);
        } else if path.is_ident("no_warnings") {
            config.no_warnigs = get_bool(attr);
        }
    }

    if grammar_sources.is_empty() {
        panic!("A grammar file needs to be provided with the #[grammar = \"PATH\"] or #[grammar_inline = \"GRAMMAR CONTENTS\"] attribute.");
    }

    (name, generics, grammar_sources, config)
}

/// Generate codes for Parser.
fn generate_typed(
    name: Ident,
    generics: &Generics,
    paths: Vec<PathBuf>,
    rules: Vec<OptimizedRule>,
    doc_comment: &DocComment,
    include_grammar: bool,
    config: Config,
) -> TokenStream {
    let include_fix = if include_grammar {
        generate_include(&name, paths)
    } else {
        quote!()
    };
    let rule_enum = generate_enum(&rules, doc_comment);
    let pairs = generate_typed_pair_from_rule(&rules, config);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let result = result_type();

    let pest_typed = pest_typed();
    let pest = pest();

    let parser_impl = quote! {
        #[allow(clippy::all)]
        impl #impl_generics #pest_typed::TypedParser<Rule> for #name #ty_generics #where_clause {
            fn parse<'i, T: #pest_typed::ParsableTypedNode<'i, Rule>>(
                input: &'i str
            ) -> #result<
                T,
                #pest::error::Error<Rule>
            > {
                T::parse(input)
            }
        }
    };

    let res = quote! {
        #include_fix
        #rule_enum
        #pairs
        #parser_impl
    };
    res
}
