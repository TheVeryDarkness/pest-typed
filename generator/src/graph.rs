// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::config::Config;

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
enum Edge {
    // Type remained.
    First,
    Second,
    Content,
    // Type wrapped by Option.
    GetFirst,
    GetSecond,
    OptionalContent,
    // Type wrapped by Option.
    Contents,
}
#[derive(Clone)]
enum Node {
    /// - Type: `&#ident`
    /// - Path: `.content.deref()`
    Leaf(Ident),
    // Type remained.
    /// - Type: `#inner`
    /// - Path: `.first`
    First(Box<Node>),
    /// - Type: `#inner`
    /// - Path: `.second`
    Second(Box<Node>),
    /// - Type: `#inner`
    /// - Path: `.content`
    Content(Box<Node>),
    // Type wrapped by Option.
    /// - Type: `#opt::<#inner>`
    /// - Path: `.get_first().as_ref().and_then(|e|Some(e #inner)) #flat`
    GetFirst(bool, Box<Node>),
    /// - Type: `#opt::<#inner>`
    /// - Path: `.get_second().as_ref().and_then(|e|Some(e #inner)) #flat`
    GetSecond(bool, Box<Node>),
    /// - Type: `#opt::<#inner>`
    /// - Path: `.content.as_ref().and_then(|e|Some(e #inner)) #flat`
    OptionalContent(bool, Box<Node>),
    // Type wrapped by Vec.
    /// - Type: `#vec::<#inner>`
    /// - Path: `.content.iter().map(|e|e #inner).collect::<#vec<_>>()`
    Contents(Box<Node>),
    // Type wrapped by tuple.
    /// - Type: `(#(#inner),*)`
    /// - Path: `(#(#inner),*)`
    Tuple(Vec<Node>),
}

impl Node {
    pub fn root(id: Ident) -> Self {
        Self::Leaf(id)
    }
    fn flattenable(&self) -> bool {
        match self {
            Node::Leaf(_) => false,
            Node::First(inner) | Node::Second(inner) | Node::Content(inner) => inner.flattenable(),
            Node::GetFirst(false, _)
            | Node::GetSecond(false, _)
            | Node::OptionalContent(false, _) => true,
            Node::GetFirst(true, inner)
            | Node::GetSecond(true, inner)
            | Node::OptionalContent(true, inner) => inner.flattenable(),
            Node::Contents(_) | Node::Tuple(_) => false,
        }
    }
    pub fn wrap(self, edge: Edge) -> Self {
        match edge {
            Edge::First => Self::First(Box::new(self)),
            Edge::Second => Self::Second(Box::new(self)),
            Edge::Content => Self::Content(Box::new(self)),
            Edge::GetFirst => Self::GetFirst(self.flattenable(), Box::new(self)),
            Edge::GetSecond => Self::GetSecond(self.flattenable(), Box::new(self)),
            Edge::OptionalContent => Self::OptionalContent(self.flattenable(), Box::new(self)),
            Edge::Contents => Self::Contents(Box::new(self)),
        }
    }
    pub fn merge(self, other: Self) -> Self {
        match self {
            Node::Tuple(vec) => match other {
                Node::Tuple(mut v) => {
                    let mut vec = vec;
                    vec.append(&mut v);
                    Node::Tuple(vec)
                }
                _ => {
                    let mut vec = vec;
                    vec.push(other);
                    Node::Tuple(vec)
                }
            },
            _ => match other {
                Node::Tuple(mut v) => {
                    let mut vec = vec![self];
                    vec.append(&mut v);
                    Node::Tuple(vec)
                }
                _ => Node::Tuple(vec![self, other]),
            },
        }
    }
    pub fn expand(&self) -> (TokenStream, TokenStream) {
        let flat = |flatten: &bool| {
            if *flatten {
                quote! {.flatten()}
            } else {
                quote! {}
            }
        };
        let opt = |flatten: &bool, inner: TokenStream| {
            let opt = option_type();
            if *flatten {
                quote! {#inner}
            } else {
                quote! {#opt::<#inner>}
            }
        };
        let vec = vec_type();
        match self {
            Node::Leaf(id) => (quote! {{let res = res.content.deref(); res}}, quote! {&#id}),
            Node::First(inner) => {
                let (pa, ty) = inner.expand();
                (quote! {{let res = &res.first; #pa}}, quote! {#ty})
            }
            Node::Second(inner) => {
                let (pa, ty) = inner.expand();
                (quote! {{let res = &res.second; #pa}}, quote! {#ty})
            }
            Node::Content(inner) => {
                let (pa, ty) = inner.expand();
                (quote! {{let res = &res.content; #pa}}, quote! {#ty})
            }
            Node::OptionalContent(flatten, inner) => {
                let (pa, ty) = inner.expand();
                let flat = flat(flatten);
                (
                    quote! {{let res = res.content.as_ref().and_then(|res| Some(#pa)) #flat; res}},
                    opt(flatten, ty),
                )
            }
            Node::GetFirst(flatten, inner) => {
                let (pa, ty) = inner.expand();
                let flat = flat(flatten);
                (
                    quote! {{let res = res.get_first().as_ref().and_then(|res| Some(#pa)) #flat; res}},
                    opt(flatten, ty),
                )
            }
            Node::GetSecond(flatten, inner) => {
                let (pa, ty) = inner.expand();
                let flat = flat(flatten);
                (
                    quote! {{let res = res.get_second().as_ref().and_then(|res| Some(#pa)) #flat; res}},
                    opt(flatten, ty),
                )
            }
            Node::Contents(inner) => {
                let (pa, ty) = inner.expand();
                (
                    quote! {{let res = res.content.iter().map(|res| #pa).collect::<#vec<_>>(); res}},
                    quote! {#vec::<#ty>},
                )
            }
            Node::Tuple(tuple) => {
                let (pa, ty): (Vec<_>, Vec<_>) = tuple.iter().map(|e| e.expand()).unzip();
                (quote! {{let res = (#(#pa),*); res}}, quote! {(#(#ty),*)})
            }
        }
    }
}

struct Accesser {
    /// name -> (path, type)
    accessers: Map<String, Node>,
}
impl Accesser {
    pub fn new() -> Self {
        Self {
            accessers: Map::new(),
        }
    }
    pub fn from_item(name: String, id: Ident) -> Self {
        let mut res = Map::new();
        res.insert(name, Node::root(id));
        Self { accessers: res }
    }
    pub fn content(self) -> Self {
        self.prepend(Edge::Content)
    }
    pub fn contents(self) -> Self {
        self.prepend(Edge::Contents)
    }
    pub fn optional_content(self) -> Self {
        self.prepend(Edge::OptionalContent)
    }
    pub fn optional_first(self) -> Self {
        self.prepend(Edge::GetFirst)
    }
    pub fn optional_second(self) -> Self {
        self.prepend(Edge::GetSecond)
    }
    pub fn first(self) -> Self {
        self.prepend(Edge::First)
    }
    pub fn second(self) -> Self {
        self.prepend(Edge::Second)
    }
    #[inline]
    fn prepend(mut self, edge: Edge) -> Self {
        for (_, node) in self.accessers.iter_mut() {
            // TODO: Ellide clone here.
            *node = node.clone().wrap(edge.clone());
        }
        self
    }
    pub fn join(mut self, other: Accesser) -> Accesser {
        other.accessers.into_iter().for_each(|(name, tree)| {
            let entry = self.accessers.entry(name.clone());
            match entry {
                btree_map::Entry::Vacant(entry) => {
                    entry.insert(tree);
                }
                btree_map::Entry::Occupied(mut entry) => {
                    // TODO: Ellide clone here.
                    let t = entry.get_mut();
                    *t = t.clone().merge(tree);
                }
            }
        });
        self
    }
    pub fn collect(&self) -> TokenStream {
        let accessers = self.accessers.iter().map(|(name, node)| {
            let id = ident(name.as_str());
            let (paths, types) = node.expand();
            let src = quote! {
                #[allow(non_snake_case)]
                pub fn #id(&self) -> #types {
                    let res = &self.content;
                    #paths
                }
            };
            // We may generate source codes to help debugging here.
            let doc = format! {"A helper function to access [`{}`].", name};
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

#[derive(Clone, Copy)]
enum Emission {
    Nothing,
    Span,
    InnerToken,
}

fn rule(
    name: &str,
    id: &Ident,
    type_name: &TokenStream,
    rule_name: &Ident,
    doc: &String,
    accessers: &Accesser,
    inner_spaces: Option<bool>,
    emission: Emission,
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
    let _bool = _bool();
    let str = _str();
    let tracker = tracker();
    let (atomicity, atomicity_doc) = match inner_spaces {
        Some(false) => (quote! {true}, "Atomic rule."),
        Some(true) => (quote! {false}, "Non-atomic rule."),
        None => (quote! {ATOMIC}, "Normal rule."),
    };
    let accessers = match emission {
        Emission::InnerToken => accessers.collect(),
        Emission::Silent | Emission::Span => quote! {},
    };
    let stmt_ign = if let Some(false) = inner_spaces {
        quote! {}
    } else {
        quote! {
            let (input, _) = #ignore::parse_with::<false, #rule_wrappers::EOI>(input, &mut stack);
        }
    };
    let (fields, parse_impl, debug_impl) = match emission {
        Emission::Nothing => (
            quote! {
                _phantom: ::core::marker::PhantomData<&'i #type_name>,
            },
            quote! {
                let (input, _) = #type_name::try_parse_with::<#atomicity, #rule_wrappers::#rule_name>(input, stack)?;
                Ok((input, Self { _phantom: ::core::marker::PhantomData }))
            },
            quote! {
                f.debug_struct(#name)
                    .finish()
            },
        ),
        Emission::Span => (
            quote! {
                #[doc = "Matched content."]
                pub content: #type_name,
                pub span: #span<'i>,
            },
            quote! {
                let start = input.clone();
                let (input, content) = #type_name::try_parse_with::<#atomicity, #rule_wrappers::#rule_name>(input, stack)?;
                let span = start.span(&input);
                Ok((input, Self { content, span }))
            },
            quote! {
                f.debug_struct(#name)
                    .field("content", &self.content)
                    .field("span", &self.span)
                    .finish()
            },
        ),
        Emission::InnerToken => (
            quote! {
                #[doc = "Matched content."]
                pub content: #type_name,
            },
            quote! {
                let (input, content) = #type_name::try_parse_with::<#atomicity, #rule_wrappers::#rule_name>(input, stack)?;
                Ok((input, Self { content }))
            },
            quote! {
                f.debug_struct(#name)
                    .field("content", &self.content)
                    .finish()
            },
        ),
    };
    quote! {
        #[doc = #doc]
        #[doc = #atomicity_doc]
        #[allow(non_camel_case_types)]
        pub struct #id<'i> {
            #fields
        }
        impl<'i> #id<'i> {
            #accessers
        }
        impl<'i> #pest_typed::RuleWrapper<super::Rule> for #id<'i> {
            const RULE: super::Rule = super::Rule::#rule_name;
        }
        impl<'i> #pest_typed::TypeWrapper for #id<'i> {
            type Inner = #type_name;
        }
        impl<'i> #pest_typed::TypedNode<'i, super::Rule> for #id<'i> {
            #[inline]
            fn try_parse_with<const ATOMIC: #_bool, _Rule: #pest_typed::RuleWrapper<super::Rule>>(
                input: #position<'i>,
                stack: &mut #stack<#span<'i>>,
            ) -> #result<(#position<'i>, Self), #tracker<'i, super::Rule>> {
                #parse_impl
            }
        }
        impl<'i> #pest_typed::ParsableTypedNode<'i, super::Rule> for #id<'i> {
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
        impl<'i> ::core::fmt::Debug for #id<'i> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                #debug_impl
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

/// Returns (type name, accesser).
fn process_single_alias(
    map: &mut Output,
    expr: &OptimizedExpr,
    rule_name: &str,
    candidate_name: String,
    type_name: TokenStream,
    accessers: Accesser,
    inner_spaces: Option<bool>,
    emission: Emission,
    explicit: bool,
) -> (TokenStream, Accesser) {
    let rule_name = ident(rule_name);
    let name = ident(&candidate_name);
    if explicit {
        let doc = format!("Corresponds to expression: `{}`.", expr);
        let def = rule(
            candidate_name.as_str(),
            &name,
            &type_name,
            &rule_name,
            &doc,
            &accessers,
            inner_spaces,
            emission,
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
    emission: Emission,
    config: Config,
) -> (TokenStream, Accesser) {
    let ignore = ignore();
    let pest_typed = pest_typed();
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
                emission,
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
                emission,
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
            emission,
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
                emission,
                config,
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
                emission,
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
                emission,
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
                    #pest_typed::predefined_node::CharRange::<'i, super::Rule, #start, #end>
                },
                Accesser::new(),
                inner_spaces,
                emission,
                explicit,
            )
        }
        OptimizedExpr::Ident(id) => {
            let inner = ident(id);
            let accessers = if config.emit_rule_reference {
                Accesser::from_item(id.clone(), inner.clone())
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
                emission,
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
                emission,
                config,
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
                emission,
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
                emission,
                config,
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
                emission,
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
                emission,
                config,
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
                emission,
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
                emission,
                config,
            );
            let (second, acc_second) = generate_graph_node(
                rhs,
                rule_name,
                format! {"{}_1", candidate_name},
                map,
                false,
                inner_spaces,
                emission,
                config,
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
                emission,
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
                emission,
                config,
            );
            let acc_first = acc_first.optional_first();
            let (second, acc_second) = generate_graph_node(
                rhs,
                rule_name,
                format! {"{}_1", candidate_name},
                map,
                false,
                inner_spaces,
                emission,
                config,
            );
            let acc_second = acc_second.optional_second();
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
                emission,
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
                emission,
                config,
            );
            let accessers = accessers.optional_content();
            process_single_alias(
                map,
                expr,
                rule_name,
                candidate_name,
                quote! {#pest_typed::predefined_node::Opt::<'i, super::Rule, #inner_name>},
                accessers,
                inner_spaces,
                emission,
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
                emission,
                config,
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
                emission,
                explicit,
            )
        }
        #[cfg(feature = "grammar-extras")]
        OptimizedExpr::RepOnce(inner) => {
            let (inner_name, accessers) = generate_graph_node(
                inner,
                rule_name,
                format!("{}_ro", candidate_name),
                map,
                false,
                inner_spaces,
                emission,
                config,
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
                        #inner_name,
                        #pest_typed::predefined_node::Rep::<
                            'i,
                            super::Rule,
                            #inner_name,
                            #ignore,
                        >,
                        #ignore
                    >
                },
                accessers.contents(),
                inner_spaces,
                emission,
                explicit,
            )
        }
        #[cfg(feature = "grammar-extras")]
        OptimizedExpr::NodeTag(inner_expr, tag) => {
            let (inner, accesser) = generate_graph_node(
                inner_expr,
                rule_name,
                format!("{}", candidate_name),
                map,
                explicit,
                inner_spaces,
                emission,
                config,
            );
            let id = ident(tag.as_str());
            let accesser = if config.emit_tagged_node_reference {
                let new_accesser = Accesser::from_item(tag.clone(), id);
                if config.truncate_accesser_at_node_tag {
                    let accessers = accesser.collect();
                    map.insert(quote! {
                        impl<'i> #inner {
                            #accessers
                        }
                    });
                    new_accesser
                } else {
                    new_accesser.join(accesser)
                }
            } else {
                accesser
            };
            process_single_alias(
                map,
                inner_expr,
                rule_name,
                candidate_name,
                inner,
                accesser,
                inner_spaces,
                emission,
                explicit,
            )
        }
    }
}

fn generate_graph(rules: &[OptimizedRule], config: Config) -> Output {
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
                    Emission::InnerToken,
                    config,
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
                    Emission::Nothing,
                    config,
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
                    Emission::InnerToken,
                    config,
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
                    Emission::InnerToken,
                    config,
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
                    Emission::Span,
                    config,
                );
            }
        }
    }
    res
}

pub(crate) fn generate_typed_pair_from_rule(
    rules: &[OptimizedRule],
    config: Config,
) -> TokenStream {
    let pest_typed = pest_typed();
    let graph = generate_graph(rules, config);
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
