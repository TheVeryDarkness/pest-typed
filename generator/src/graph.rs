// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use super::types::{option_type, result_type, vec_type};
use pest::unicode::unicode_property_names;
use pest_meta::optimizer::OptimizedExpr;
use pest_meta::{ast::RuleType, optimizer::OptimizedRule};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::btree_map;
pub use std::collections::BTreeMap as Map;
use std::collections::BTreeSet;

pub fn pest() -> TokenStream {
    quote! {::pest_typed}
}

pub fn pest_typed() -> TokenStream {
    quote! {::pest_typed}
}

fn pest_unicode() -> TokenStream {
    quote! {::pest_typed::unicode}
}

fn ident(s: &str) -> Ident {
    format_ident!("r#{}", s)
}

fn rule_wrappers() -> TokenStream {
    quote! {
        super::rule_wrappers
    }
}

fn constant_wrappers() -> TokenStream {
    quote! {
        constant_wrappers
    }
}

fn unicode_mod() -> TokenStream {
    quote! {
        unicode
    }
}

fn ignore() -> TokenStream {
    let pest_typed = pest_typed();
    quote! {
        #pest_typed::predefined_node::Ign::<
            'i,
            super::Rule,
            COMMENT::<'i>,
            WHITESPACE::<'i>,
        >
    }
}

#[derive(Clone)]
enum Accessed {
    Vec,
    Option,
}

struct Accesser {
    /// name -> [(path, type, type)]
    accessers: Map<String, Vec<(TokenStream, Vec<Accessed>, Ident)>>,
}
impl Accesser {
    pub fn new() -> Self {
        Self {
            accessers: Map::new(),
        }
    }
    pub fn from_item(name: String, path: TokenStream, id: Ident) -> Self {
        let mut res = Map::new();
        res.insert(name, vec![(path, vec![], id)]);
        Self { accessers: res }
    }
    pub fn content(self) -> Self {
        self.prepend(|path| quote! {.content #path}, None)
    }
    pub fn contents(self) -> Self {
        let vec = vec_type();
        self.prepend(
            |inner| quote! {.content.iter().map(|e|e #inner).collect::<#vec<_>>()},
            Some(Accessed::Vec),
        )
    }
    pub fn option(self, prefix: TokenStream) -> Self {
        self.prepend(
            |inner| quote! {#prefix.as_ref().and_then(|e|Some(e #inner))},
            Some(Accessed::Option),
        )
    }
    pub fn first(self) -> Self {
        self.prepend(|inner| quote! {.first #inner}, None)
    }
    pub fn second(self) -> Self {
        self.prepend(|inner| quote! {.second #inner}, None)
    }
    fn prepend(
        mut self,
        fn_path: impl Fn(TokenStream) -> TokenStream,
        nesting: Option<Accessed>,
    ) -> Self {
        for (_, vec) in self.accessers.iter_mut() {
            for (path, accessed, _) in vec.iter_mut() {
                *path = fn_path(path.clone());
                if let Some(nesting) = &nesting {
                    if let Accessed::Option = nesting {
                        match accessed.last() {
                            Some(Accessed::Option) => {
                                *path = quote! {#path.flatten()};
                                continue;
                            }
                            _ => (),
                        }
                    }
                    accessed.push(nesting.clone());
                }
            }
        }
        self
    }
    pub fn join(mut self, other: Accesser) -> Accesser {
        other.accessers.into_iter().for_each(|(name, vec)| {
            match self.accessers.entry(name.clone()) {
                btree_map::Entry::Vacant(entry) => {
                    entry.insert(vec);
                }
                btree_map::Entry::Occupied(mut entry) => {
                    let v = entry.get_mut();
                    v.extend(vec.into_iter());
                }
            }
        });
        self
    }
    fn expand(accessed: &Vec<Accessed>, inner: &Ident) -> TokenStream {
        let vec = vec_type();
        let option = option_type();
        let mut res = quote! {&#inner};
        for accessed in accessed.iter() {
            match accessed {
                Accessed::Vec => res = quote! {#vec::<#res>},
                Accessed::Option => res = quote! {#option::<#res>},
            }
        }
        res
    }
    pub fn collect(&self) -> TokenStream {
        let accessers = self.accessers.iter().map(|(name, vec)| {
            let (paths, types): (Vec<_>, Vec<_>) = vec
                .iter()
                .map(|(path, accessed, inner)| (path, Self::expand(accessed, inner)))
                .unzip();
            let id = ident(name.as_str());
            let src = if vec.len() == 1 {
                quote! {
                    #[allow(non_snake_case)]
                    pub fn #id(&self) -> #(#types)* {
                        #( self.content #paths )*
                    }
                }
            } else {
                quote! {
                    #[allow(non_snake_case)]
                    pub fn #id(&self) -> ( #(#types),* ) {
                        ( #( self.content #paths ),* )
                    }
                }
            };
            // We may generate source codes to help debugging here.
            let doc = format! {"A helper function to access [`struct@{}`].", name};
            quote! {
                #[doc = #doc]
                #src
            }
        });
        quote! {
            #(#accessers)*
        }
    }
}

fn position() -> TokenStream {
    let pest = pest();
    quote! {#pest::Position}
}
fn _bool() -> TokenStream {
    quote! {::core::primitive::bool}
}
fn _char() -> TokenStream {
    quote! {::core::primitive::char}
}
fn _str() -> TokenStream {
    quote! {::core::primitive::str}
}
fn stack() -> TokenStream {
    let pest = pest();
    quote! {#pest::Stack}
}
fn tracker() -> TokenStream {
    let pest = pest();
    quote! {#pest::tracker::Tracker}
}
fn _span() -> TokenStream {
    let pest = pest();
    quote! {#pest::Span}
}

fn rule(
    name: &Ident,
    type_name: &TokenStream,
    rule_name: &Ident,
    doc: &String,
    accessers: &Accesser,
    inner_spaces: Option<bool>,
) -> TokenStream {
    let pest_typed = pest_typed();
    let rule_wrappers = rule_wrappers();
    let result = result_type();
    let position = position();
    let stack = stack();
    let span = _span();
    let pest = pest();
    let error = quote! {#pest::error::Error};
    let ignore = ignore();
    let accessers = accessers.collect();
    let _bool = _bool();
    let str = _str();
    let tracker = tracker();
    let (atomicity, atomicity_doc) = match inner_spaces {
        Some(false) => (quote! {true}, "Atomic rule."),
        Some(true) => (quote! {false}, "Non-atomic rule."),
        None => (quote! {ATOMIC}, "Normal rule."),
    };
    let stmt_ign = if let Some(false) = inner_spaces {
        quote! {}
    } else {
        quote! {
            let (input, _) = #ignore::parse_with::<false, #rule_wrappers::EOI>(input, &mut stack);
        }
    };
    quote! {
        #[doc = #doc]
        #[doc = #atomicity_doc]
        #[allow(non_camel_case_types)]
        #[derive(Debug)]
        pub struct #name<'i> {
            #[doc = "Matched content."]
            pub content: #type_name,
        }
        impl<'i> #name<'i> {
            #accessers
        }
        impl<'i> #pest_typed::RuleWrapper<super::Rule> for #name<'i> {
            const RULE: super::Rule = super::Rule::#rule_name;
        }
        impl<'i> #pest_typed::TypeWrapper for #name<'i> {
            type Inner = #type_name;
        }
        impl<'i> ::core::convert::From<#type_name> for #name<'i> {
            fn from(content: #type_name) -> Self {
                Self { content }
            }
        }
        impl<'i> #pest_typed::TypedNode<'i, super::Rule> for #name<'i> {
            #[inline]
            fn try_parse_with<const ATOMIC: #_bool, _Rule: #pest_typed::RuleWrapper<super::Rule>>(
                input: #position<'i>,
                stack: &mut #stack<#span<'i>>,
            ) -> #result<(#position<'i>, Self), #tracker<'i, super::Rule>> {
                let (input, content) = #type_name::try_parse_with::<#atomicity, #rule_wrappers::#rule_name>(input, stack)?;
                Ok((input, Self::from(content)))
            }
        }
        impl<'i> #pest_typed::ParsableTypedNode<'i, super::Rule> for #name<'i> {
            #[inline]
            fn parse(input: &'i #str) -> #result<Self, #error<super::Rule>> {
                let mut stack = #stack::new();
                let (input, res) =
                    match Self::try_parse_with::<false, #rule_wrappers::#rule_name>(#position::from_start(input), &mut stack) {
                        Ok((input, res)) => (input, res),
                        Err(e) => return Err(e.collect()),
                    };
                #stmt_ign
                let (_, _) = match EOI::try_parse_with::<false, #rule_wrappers::EOI>(input, &mut stack) {
                    Ok((input, res)) => (input, res),
                    Err(e) => return Err(e.collect()),
                };
                Ok(res)
            }

            #[inline]
            fn parse_partial(input: &'i #str) -> #result<(#position<'i>, Self), #error<super::Rule>> {
                let mut stack = #stack::new();
                match Self::try_parse_with::<false, #rule_wrappers::#rule_name>(#position::from_start(input), &mut stack) {
                    Ok((input, res)) => Ok((input, res)),
                    Err(e) => return Err(e.collect()),
                }
            }
        }
    }
}

struct Output {
    content: Vec<TokenStream>,
    wrappers: Vec<TokenStream>,
}
impl Output {
    fn new() -> Self {
        Self {
            content: Vec::new(),
            wrappers: Vec::new(),
        }
    }
    fn insert(&mut self, tokens: TokenStream) {
        self.content.push(tokens);
    }
    /// Insert to wrapper module.
    /// Return the module path.
    fn insert_wrapper(&mut self, tokens: TokenStream) -> TokenStream {
        self.wrappers.push(tokens);
        let wrappers = constant_wrappers();
        quote! { super::#wrappers }
    }
    /// (nodes, wrappers)
    fn collect(&self) -> (TokenStream, TokenStream) {
        let content = &self.content;
        let wrappers = &self.wrappers;
        (
            quote! {
                #(#content)*
            },
            quote! {
                #(#wrappers)*
            },
        )
    }
}

/// Returns type name.
fn process_single_alias(
    map: &mut Output,
    expr: &OptimizedExpr,
    rule_name: &str,
    candidate_name: String,
    type_name: TokenStream,
    accessers: Accesser,
    inner_spaces: Option<bool>,
    explicit: bool,
) -> (TokenStream, Accesser) {
    let rule_name = ident(rule_name);
    let name = ident(&candidate_name);
    if explicit {
        let doc = format!("Corresponds to expression: `{}`.", expr);
        let def = rule(
            &name,
            &type_name,
            &rule_name,
            &doc,
            &accessers,
            inner_spaces,
        );
        map.insert(def);
        (quote! {#name::<'i>}, accessers)
    } else {
        (type_name, accessers)
    }
}

/// Returns type name.
fn generate_graph_node(
    expr: &OptimizedExpr,
    rule_name: &str,
    candidate_name: String,
    // From node name to type definition and implementation
    map: &mut Output,
    explicit: bool,
    inner_spaces: Option<bool>,
    inner_tokens: bool,
    silent: bool,
    emit_rule_reference: bool,
    emit_tagged_node_reference: bool,
) -> (TokenStream, Accesser) {
    let ignore = ignore();
    let pest_typed = pest_typed();
    fn for_option(accessers: Accesser, _inner: &OptimizedExpr, prefix: TokenStream) -> Accesser {
        accessers.option(prefix)
    }
    // Still some compile-time information not taken.
    match expr {
        OptimizedExpr::Str(content) => {
            let wrapper = format_ident!("r#{}", candidate_name);
            let doc = format!("A wrapper for `{:?}`.", content);
            let str = _str();
            let module = map.insert_wrapper(quote! {
                #[doc = #doc]
                #[allow(non_camel_case_types)]
                pub struct #wrapper();
                impl #pest_typed::StringWrapper for #wrapper {
                    const CONTENT: &'static #str = #content;
                }
            });
            process_single_alias(
                map,
                expr,
                rule_name,
                candidate_name,
                quote! {
                    #pest_typed::predefined_node::Str::<'i, super::Rule, #module::#wrapper>
                },
                Accesser::new(),
                inner_spaces,
                explicit,
            )
        }
        OptimizedExpr::Insens(content) => {
            let wrapper = format_ident!("r#{}", candidate_name);
            let doc = format!("A wrapper for `{:?}`.", content);
            let str = _str();
            let module = map.insert_wrapper(quote! {
                #[doc = #doc]
                #[allow(non_camel_case_types)]
                pub struct #wrapper();
                impl #pest_typed::StringWrapper for #wrapper {
                    const CONTENT: &'static #str = #content;
                }
            });
            process_single_alias(
                map,
                expr,
                rule_name,
                candidate_name,
                quote! {
                    #pest_typed::predefined_node::Insens::<'i, super::Rule, #module::#wrapper>
                },
                Accesser::new(),
                inner_spaces,
                explicit,
            )
        }
        OptimizedExpr::PeekSlice(start, end) => process_single_alias(
            map,
            expr,
            rule_name,
            candidate_name,
            match end {
                Some(end) => quote! {
                    #pest_typed::predefined_node::PeekSlice2::<'i, super::Rule, #start, #end>
                },
                None => quote! {
                    #pest_typed::predefined_node::PeekSlice1::<'i, super::Rule, #start>
                },
            },
            Accesser::new(),
            inner_spaces,
            explicit,
        ),
        OptimizedExpr::Push(expr) => {
            let (inner, accesser) = generate_graph_node(
                expr,
                rule_name,
                format! {"{}_p", candidate_name},
                map,
                false,
                inner_spaces,
                inner_tokens,
                silent,
                emit_rule_reference,
                emit_tagged_node_reference,
            );
            process_single_alias(
                map,
                expr,
                rule_name,
                candidate_name,
                quote! {
                    #pest_typed::predefined_node::Push::<'i, super::Rule, #inner>
                },
                accesser.content(),
                inner_spaces,
                explicit,
            )
        }
        OptimizedExpr::Skip(strings) => {
            let wrapper = format_ident!("r#{}", candidate_name);
            let doc = format!("A wrapper for `{:?}`.", strings);
            let str = _str();
            let module = map.insert_wrapper(quote! {
                #[doc = #doc]
                #[allow(non_camel_case_types)]
                pub struct #wrapper();
                impl #pest_typed::StringArrayWrapper for #wrapper {
                    const CONTENT: &'static[&'static #str] = &[ #(#strings),* ];
                }
            });
            process_single_alias(
                map,
                expr,
                rule_name,
                candidate_name,
                quote! {
                    #pest_typed::predefined_node::Skip::<'i, super::Rule, #module::#wrapper>
                },
                Accesser::new(),
                inner_spaces,
                explicit,
            )
        }
        OptimizedExpr::Range(start, end) => {
            let start = start.chars().next().unwrap();
            let end = end.chars().next().unwrap();
            process_single_alias(
                map,
                expr,
                rule_name,
                candidate_name,
                quote! {
                    #pest_typed::predefined_node::Range::<'i, super::Rule, #start, #end>
                },
                Accesser::new(),
                inner_spaces,
                explicit,
            )
        }
        OptimizedExpr::Ident(id) => {
            let inner = ident(id);
            let accessers = if emit_rule_reference {
                Accesser::from_item(id.clone(), quote! {.content.deref()}, inner.clone())
            } else {
                Accesser::new()
            };
            process_single_alias(
                map,
                expr,
                rule_name,
                candidate_name,
                quote! {#pest_typed::predefined_node::Box::<'i, super::Rule, #inner::<'i>>},
                accessers,
                inner_spaces,
                explicit,
            )
        }
        OptimizedExpr::PosPred(expr) => {
            let (inner, accessers) = generate_graph_node(
                expr,
                rule_name,
                format! {"{}_P", candidate_name},
                map,
                false,
                inner_spaces,
                inner_tokens,
                silent,
                emit_rule_reference,
                emit_tagged_node_reference,
            );
            process_single_alias(
                map,
                expr,
                rule_name,
                candidate_name,
                quote! {
                    #pest_typed::predefined_node::Positive::<'i, super::Rule, #inner>
                },
                accessers.content(),
                inner_spaces,
                explicit,
            )
        }
        OptimizedExpr::NegPred(expr) => {
            // Impossible to access inner tokens.
            let (inner, _) = generate_graph_node(
                expr,
                rule_name,
                format! {"{}_N", candidate_name},
                map,
                false,
                inner_spaces,
                inner_tokens,
                silent,
                emit_rule_reference,
                emit_tagged_node_reference,
            );
            process_single_alias(
                map,
                expr,
                rule_name,
                candidate_name,
                quote! {
                    #pest_typed::predefined_node::Negative::<'i, super::Rule, #inner>
                },
                Accesser::new(),
                inner_spaces,
                explicit,
            )
        }
        OptimizedExpr::RestoreOnErr(expr) => {
            let (inner, accessers) = generate_graph_node(
                expr,
                rule_name,
                format! {"{}_E", candidate_name},
                map,
                false,
                inner_spaces,
                inner_tokens,
                silent,
                emit_rule_reference,
                emit_tagged_node_reference,
            );
            let accessers = accessers.content();
            process_single_alias(
                map,
                expr,
                rule_name,
                candidate_name,
                quote! {
                    #pest_typed::predefined_node::Restorable::<'i, super::Rule, #inner>
                },
                accessers,
                inner_spaces,
                explicit,
            )
        }
        OptimizedExpr::Seq(lhs, rhs) => {
            let (first, acc_first) = generate_graph_node(
                lhs,
                rule_name,
                format! {"{}_0", candidate_name},
                map,
                false,
                inner_spaces,
                inner_tokens,
                silent,
                emit_rule_reference,
                emit_tagged_node_reference,
            );
            let (second, acc_second) = generate_graph_node(
                rhs,
                rule_name,
                format! {"{}_1", candidate_name},
                map,
                false,
                inner_spaces,
                inner_tokens,
                silent,
                emit_rule_reference,
                emit_tagged_node_reference,
            );
            process_single_alias(
                map,
                expr,
                rule_name,
                candidate_name,
                quote! {
                    #pest_typed::predefined_node::Seq::<
                        'i,
                        super::Rule,
                        #first,
                        #second,
                        #ignore
                    >
                },
                acc_first.first().join(acc_second.second()),
                inner_spaces,
                explicit,
            )
        }
        OptimizedExpr::Choice(lhs, rhs) => {
            let (first, acc_first) = generate_graph_node(
                lhs,
                rule_name,
                format! {"{}_0", candidate_name},
                map,
                false,
                inner_spaces,
                inner_tokens,
                silent,
                emit_rule_reference,
                emit_tagged_node_reference,
            );
            let acc_first = for_option(acc_first, lhs, quote! {.get_first()});
            let (second, acc_second) = generate_graph_node(
                rhs,
                rule_name,
                format! {"{}_1", candidate_name},
                map,
                false,
                inner_spaces,
                inner_tokens,
                silent,
                emit_rule_reference,
                emit_tagged_node_reference,
            );
            let acc_second = for_option(acc_second, rhs, quote! {.get_second()});
            process_single_alias(
                map,
                expr,
                rule_name,
                candidate_name,
                quote! {
                    #pest_typed::predefined_node::Choice::<
                        'i,
                        super::Rule,
                        #first,
                        #second,
                    >
                },
                acc_first.join(acc_second),
                inner_spaces,
                explicit,
            )
        }
        OptimizedExpr::Opt(inner) => {
            let (inner_name, accessers) = generate_graph_node(
                inner,
                rule_name,
                format!("{}_o", candidate_name),
                map,
                false,
                inner_spaces,
                inner_tokens,
                silent,
                emit_rule_reference,
                emit_tagged_node_reference,
            );
            let accessers = for_option(accessers, inner, quote! {.content});
            process_single_alias(
                map,
                expr,
                rule_name,
                candidate_name,
                quote! {#pest_typed::predefined_node::Opt::<'i, super::Rule, #inner_name>},
                accessers,
                inner_spaces,
                explicit,
            )
        }
        OptimizedExpr::Rep(inner) => {
            let (inner_name, accessers) = generate_graph_node(
                inner,
                rule_name,
                format!("{}_r", candidate_name),
                map,
                false,
                inner_spaces,
                inner_tokens,
                silent,
                emit_rule_reference,
                emit_tagged_node_reference,
            );
            process_single_alias(
                map,
                expr,
                rule_name,
                candidate_name,
                quote! {
                    #pest_typed::predefined_node::Rep::<
                        'i,
                        super::Rule,
                        #inner_name,
                        #ignore,
                    >
                },
                accessers.contents(),
                inner_spaces,
                explicit,
            )
        }
        #[cfg(feature = "grammar-extras")]
        OptimizedExpr::NodeTag(inner_expr, _tag) => {
            generate_graph_node(
                inner_expr,
                rule_name,
                format!("{}_0", candidate_name),
                map,
                explicit,
                inner_spaces,
                inner_tokens,
                silent,
                emit_rule_reference,
                emit_tagged_node_reference,
            );
            todo!()
        }
    }
}

fn generate_graph(
    rules: &[OptimizedRule],
    emit_rule_reference: bool,
    emit_tagged_node_reference: bool,
) -> Output {
    // println!("{:#?}", rules);
    let mut res = Output::new();
    for rule in rules.iter() {
        let rule_name = rule.name.as_str();
        let candidate_name = rule.name.clone();
        match rule.ty {
            RuleType::Normal => {
                generate_graph_node(
                    &rule.expr,
                    rule_name,
                    candidate_name,
                    &mut res,
                    true,
                    None,
                    true,
                    false,
                    emit_rule_reference,
                    emit_tagged_node_reference,
                );
            }
            RuleType::Silent => {
                generate_graph_node(
                    &rule.expr,
                    rule_name,
                    candidate_name,
                    &mut res,
                    true,
                    None,
                    true,
                    true,
                    emit_rule_reference,
                    emit_tagged_node_reference,
                );
            }
            RuleType::NonAtomic => {
                generate_graph_node(
                    &rule.expr,
                    rule_name,
                    candidate_name,
                    &mut res,
                    true,
                    Some(true),
                    true,
                    true,
                    emit_rule_reference,
                    emit_tagged_node_reference,
                );
            }
            RuleType::CompoundAtomic => {
                generate_graph_node(
                    &rule.expr,
                    rule_name,
                    candidate_name,
                    &mut res,
                    true,
                    Some(false),
                    true,
                    false,
                    emit_rule_reference,
                    emit_tagged_node_reference,
                );
            }
            RuleType::Atomic => {
                generate_graph_node(
                    &rule.expr,
                    rule_name,
                    candidate_name,
                    &mut res,
                    true,
                    Some(false),
                    false,
                    false,
                    emit_rule_reference,
                    emit_tagged_node_reference,
                );
            }
        }
    }
    res
}

pub fn generate_typed_pair_from_rule(
    rules: &[OptimizedRule],
    emit_rule_reference: bool,
    emit_tagged_node_reference: bool,
) -> TokenStream {
    // let names: Vec<_> = rules.iter().map(|rule| &rule.name).collect();
    // eprintln!("{:#?}", names);
    let pest_typed = pest_typed();
    let graph = generate_graph(rules, emit_rule_reference, emit_tagged_node_reference);
    let as_wrapper = |name: &Ident| {
        quote! {
            #[allow(non_camel_case_types)]
            pub struct #name;
            impl #pest_typed::RuleWrapper<super::Rule> for #name {
                const RULE: super::Rule = super::Rule::#name;
            }
        }
    };
    let rule_wrappers = rules.iter().map(|rule| {
        let name = ident(rule.name.as_str());
        as_wrapper(&name)
    });
    let eoi = as_wrapper(&ident("EOI"));
    let (pairs, wrappers) = graph.collect();
    let rule_names: BTreeSet<&str> = rules.iter().map(|rule| rule.name.as_str()).collect();
    let builtin = generate_builtin(&rule_names);
    // let names = rules.iter().map(|rule| format_ident!("r#{}", rule.name));
    let unicode = unicode_mod();
    let unicode_rule = generate_unicode(&rule_names);
    let wrapper_mod = constant_wrappers();
    let res = quote! {
        #[doc(hidden)]
        mod rule_wrappers {
            #(#rule_wrappers)*
            #eoi
        }
        mod #unicode {
            #unicode_rule
        }
        mod #wrapper_mod {
            #wrappers
        }
        #[doc = "Definitions of statically typed nodes generated by pest-generator."]
        pub mod pairs {
            use #pest_typed::NeverFailedTypedNode as _;
            #builtin

            #pairs
        }
    };
    // println!("{}", res);
    res
}

fn generate_unicode(rule_names: &BTreeSet<&str>) -> TokenStream {
    let mut results = vec![];
    let pest_typed = pest_typed();
    let pest_unicode = pest_unicode();
    let bool = _bool();
    let result = result_type();
    let position = position();
    let stack = stack();
    let span = _span();
    let char = _char();
    let tracker = tracker();

    for property in unicode_property_names() {
        let property_ident: Ident = syn::parse_str(property).unwrap();
        // insert manually for #property substitution

        let doc = format!("Auto generated. Unicode property {}.", property);

        if !rule_names.contains(property) {
            results.push(quote! {
                #[allow(non_camel_case_types)]
                #[doc = #doc]
                pub struct #property_ident<'i> {
                    pub content: #char,
                    _phantom: ::core::marker::PhantomData<&'i #char>
                }
                impl<'i> ::core::convert::From<#char> for #property_ident<'i> {
                    fn from(content: #char) -> Self {
                        Self {
                            content,
                            _phantom: ::core::marker::PhantomData
                        }
                    }
                }
                impl<'i> #pest_typed::TypedNode<'i, super::Rule> for #property_ident<'i> {
                    #[inline]
                    fn try_parse_with<const ATOMIC: #bool, _Rule: #pest_typed::RuleWrapper<super::Rule>>(
                        mut input: #position<'i>,
                        _stack: &mut #stack<#span<'i>>,
                    ) -> #result<(#position<'i>, Self), #tracker<'i, super::Rule>> {
                        match #pest_typed::predefined_node::match_char_by(&mut input, #pest_unicode::#property_ident) {
                            Some(content) => {
                                Ok((input, Self::from(content)))
                            }
                            None => Err(#tracker::new(input))
                        }
                    }
                }
                impl<'i> ::core::fmt::Debug for #property_ident<'i> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct(#property)
                            .field("content", &self.content)
                            .finish()
                    }
                }
            });
        }
    }
    quote! {
        #(#results)*
    }
}

fn generate_builtin(rule_names: &BTreeSet<&str>) -> TokenStream {
    let pest_typed = pest_typed();
    let unicode = unicode_mod();
    let mut results = vec![quote! {
        use #pest_typed::TypedNode as _;
        use ::core::ops::Deref as _;
        use super::#unicode::*;
    }];
    macro_rules! insert_builtin {
        ($name:expr, $def:path) => {
            if !rule_names.contains($name) {
                let id = ident($name);
                results.push(quote! {
                    type #id<'i> = #pest_typed::predefined_node::$def;
                });
            }
        };
    }
    insert_builtin!("ANY", ANY::<'i>);
    insert_builtin!("SOI", SOI::<'i>);
    insert_builtin!("EOI", EOI::<'i>);
    insert_builtin!("PEEK", PEEK::<'i>);
    insert_builtin!("PEEK_ALL", PEEK_ALL::<'i>);
    insert_builtin!("POP", POP::<'i>);
    insert_builtin!("POP_ALL", POP_ALL::<'i>);
    insert_builtin!("DROP", DROP::<'i>);
    insert_builtin!("ASCII_DIGIT", ASCII_DIGIT::<'i, super::Rule>);
    insert_builtin!(
        "ASCII_NONZERO_DIGIT",
        ASCII_NONZERO_DIGIT::<'i, super::Rule>
    );
    insert_builtin!("ASCII_BIN_DIGIT", ASCII_BIN_DIGIT::<'i, super::Rule>);
    insert_builtin!("ASCII_OCT_DIGIT", ASCII_OCT_DIGIT::<'i, super::Rule>);
    insert_builtin!("ASCII_HEX_DIGIT", ASCII_HEX_DIGIT::<'i, super::Rule>);
    insert_builtin!("ASCII_ALPHA_LOWER", ASCII_ALPHA_LOWER::<'i, super::Rule>);
    insert_builtin!("ASCII_ALPHA_UPPER", ASCII_ALPHA_UPPER::<'i, super::Rule>);
    insert_builtin!("ASCII_ALPHA", ASCII_ALPHA::<'i, super::Rule>);
    insert_builtin!("ASCII_ALPHANUMERIC", ASCII_ALPHANUMERIC::<'i, super::Rule>);
    insert_builtin!("ASCII", ASCII::<'i, super::Rule>);
    insert_builtin!("NEWLINE", NEWLINE::<'i>);

    insert_builtin!("WHITESPACE", AlwaysFail::<'i>);
    insert_builtin!("COMMENT", AlwaysFail::<'i>);

    quote! {
        #(#results)*
    }
}
