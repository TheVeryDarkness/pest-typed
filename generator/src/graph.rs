// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::config::Config;
use crate::docs::DocComment;
use crate::types::{box_type, option_type, vec_type};
use pest::unicode::unicode_property_names;
use pest_meta::{
    ast::RuleType,
    optimizer::{OptimizedExpr, OptimizedRule},
};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::collections::{btree_map, BTreeMap, BTreeSet};
use syn::Index;

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

fn generics() -> TokenStream {
    quote! {
        generics
    }
}

fn rules_mod() -> TokenStream {
    quote! {rules}
}

fn rules_impl_mod() -> TokenStream {
    quote! {rules_impl}
}

fn pairs_mod() -> TokenStream {
    quote! {pairs}
}

fn ignore(root: &TokenStream) -> TokenStream {
    let generics = generics();
    quote! {
        #root::#generics::Skipped::<'i>
    }
}

#[derive(Clone)]
enum Edge {
    // Type remained.
    Content,
    ContentI(usize),
    // Type wrapped by Option.
    ChoiceI(usize),
    Optional,
    // Type wrapped by Vec.
    Contents,
}
#[derive(Clone)]
enum Node<'g> {
    /// - Type: `&#ident`
    /// - Path: `.content.deref()`
    Rule(&'g str, bool, bool),
    /// - Type: `&#ident`
    /// - Path: `.content`
    #[cfg(feature = "grammar-extras")]
    Tag(&'g str, &'g str, Vec<TokenStream>),
    // Type remained.
    /// - Type: `#inner`
    /// - Path: `.content`
    Content(Box<Self>),
    /// - Type: `#inner`
    /// - Path: `.content.#index.matched`
    SequenceI(usize, Box<Self>),
    // Type wrapped by Option.
    /// - Type: `#opt::<#inner>`
    /// - Path: `._#index().and_then(|res| Some(#inner)) #flat`
    ChoiceI(usize, bool, Box<Self>),
    /// - Type: `#opt::<#inner>`
    /// - Path: `.as_ref().and_then(|res| Some(#inner)) #flat`
    Optional(bool, Box<Self>),
    // Type wrapped by Vec.
    /// - Type: `#vec::<#inner>`
    /// - Path: `.content.iter().map(|res| {let res = res.matched; #inner}).collect::<#vec<_>>()`
    Contents(Box<Self>),
    // Type wrapped by tuple.
    /// - Type: `(#(#inner),*)`
    /// - Path: `(#(#inner),*)`
    Tuple(Vec<Self>),
}

impl<'g> Node<'g> {
    fn from_rule(value: &'g str, has_lifetime: bool, has_skip: bool) -> Self {
        Self::Rule(value, has_lifetime, has_skip)
    }
    #[cfg(feature = "grammar-extras")]
    fn from_tag(rule_name: &'g str, tag_name: &'g str, tokens: Vec<TokenStream>) -> Self {
        Self::Tag(rule_name, tag_name, tokens)
    }
    fn flattenable(&self) -> bool {
        match self {
            Node::Rule(_, _, _) => false,
            #[cfg(feature = "grammar-extras")]
            Node::Tag(_, _, _) => false,
            Node::Content(inner) | Node::SequenceI(_, inner) => inner.flattenable(),
            Node::ChoiceI(_, false, _) | Node::Optional(false, _) => true,
            Node::ChoiceI(_, true, inner) | Node::Optional(true, inner) => inner.flattenable(),
            Node::Contents(_) | Node::Tuple(_) => false,
        }
    }
    pub fn wrap(self, edge: Edge) -> Self {
        match edge {
            Edge::Content => Self::Content(Box::new(self)),
            Edge::ContentI(i) => Self::SequenceI(i, Box::new(self)),
            Edge::ChoiceI(i) => Self::ChoiceI(i, self.flattenable(), Box::new(self)),
            Edge::Optional => Self::Optional(self.flattenable(), Box::new(self)),
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
    pub fn expand(
        &self,
        root: &TokenStream,
        config: &RuleConfig<'g>,
    ) -> (TokenStream, TokenStream) {
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
        let rules_mod = rules_mod();
        match self {
            Node::Rule(t, has_life_time, has_skip) => {
                let skip = match config.atomicity {
                    Some(true) => quote! {0},
                    Some(false) => quote! {1},
                    None => quote! {INHERITED},
                };
                let generics = match (has_life_time, has_skip) {
                    (true, true) => quote! {::<'i, #skip>},
                    (true, false) => quote! {::<'i>},
                    (false, true) => quote! {::<#skip>},
                    (false, false) => quote! {},
                };
                let t = ident(t);
                (quote! {res}, quote! {&'s #root::#rules_mod::#t #generics})
            }
            #[cfg(feature = "grammar-extras")]
            Node::Tag(_rule, _tag, inner) => {
                // let tag_id = format_ident!("r#{tag}");
                // let rule_id = format_ident!("r#{rule}");
                (quote! {res}, quote! {(#(&#inner),*)})
            }
            Node::Content(inner) => {
                let (pa, ty) = inner.expand(root, config);
                (quote! {{let res = &res.content; #pa}}, quote! {#ty})
            }
            Node::SequenceI(i, inner) => {
                let (pa, ty) = inner.expand(root, config);
                let i = Index::from(*i);
                (
                    quote! {{let res = &res.content.#i.matched; #pa}},
                    quote! {#ty},
                )
            }
            Node::Optional(flatten, inner) => {
                let (pa, ty) = inner.expand(root, config);
                let flat = flat(flatten);
                (
                    quote! {{let res = res.as_ref().map(|res| #pa) #flat; res}},
                    opt(flatten, ty),
                )
            }
            Node::ChoiceI(index, flatten, inner) => {
                let (pa, ty) = inner.expand(root, config);
                let func = format_ident!("_{}", index);
                let flat = flat(flatten);
                (
                    quote! {{let res = res.#func().map(|res| #pa) #flat; res}},
                    opt(flatten, ty),
                )
            }
            Node::Contents(inner) => {
                let (pa, ty) = inner.expand(root, config);
                (
                    quote! {{let res = res.content.iter().map(|res| { let res = &res.matched; #pa }).collect::<#vec<_>>(); res}},
                    quote! {#vec::<#ty>},
                )
            }
            Node::Tuple(tuple) => {
                let (pa, ty): (Vec<_>, Vec<_>) =
                    tuple.iter().map(|e| e.expand(root, config)).unzip();
                (quote! {{let res = (#(#pa),*); res}}, quote! {(#(#ty),*)})
            }
        }
    }
}

/// `'g` stands for the lifetime of rules.
#[derive(Clone)]
struct Accesser<'g> {
    /// name -> (path, type)
    accessers: BTreeMap<&'g str, Node<'g>>,
}
impl<'g> Default for Accesser<'g> {
    fn default() -> Self {
        Self::new()
    }
}
impl<'g> Accesser<'g> {
    pub fn new() -> Self {
        Self {
            accessers: BTreeMap::new(),
        }
    }
    pub fn from_rule(name: &'g str, id: &'g str, has_life_time: bool, has_skip: bool) -> Self {
        let res = BTreeMap::from([(name, Node::from_rule(id, has_life_time, has_skip))]);
        Self { accessers: res }
    }
    #[cfg(feature = "grammar-extras")]
    pub fn from_tag(rule: &'g str, name: &'g str, tokens: TokenStream) -> Self {
        let res = BTreeMap::from([(name, Node::from_tag(rule, name, vec![tokens]))]);
        Self { accessers: res }
    }
    pub fn content(self) -> Self {
        self.prepend(Edge::Content)
    }
    pub fn content_i(self, i: usize) -> Self {
        self.prepend(Edge::ContentI(i))
    }
    pub fn contents(self) -> Self {
        self.prepend(Edge::Contents)
    }
    pub fn optional(self) -> Self {
        self.prepend(Edge::Optional)
    }
    pub fn choice(self, i: usize) -> Self {
        self.prepend(Edge::ChoiceI(i))
    }
    #[inline]
    fn prepend(mut self, edge: Edge) -> Self {
        for (_, node) in self.accessers.iter_mut() {
            // TODO: Ellide clone here.
            *node = node.clone().wrap(edge.clone());
        }
        self
    }
    /// Join two accesser forest in the same level.
    pub fn join_mut(&mut self, other: Accesser<'g>) {
        other.accessers.into_iter().for_each(|(name, tree)| {
            let entry = self.accessers.entry(name);
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
    }
    /// Join two accesser forest in the same level.
    pub fn join(mut self, other: Accesser<'g>) -> Self {
        self.join_mut(other);
        self
    }
    pub fn collect(&self, root: &TokenStream, config: &RuleConfig<'g>) -> TokenStream {
        let accessers = self.accessers.iter().map(|(name, node)| {
            let id = ident(name);
            let (paths, types) = node.expand(root, config);
            let content = if config.boxed {
                quote! {&*self.content}
            } else {
                quote! {&self.content}
            };
            let src = quote! {
                #[allow(non_snake_case)]
                pub fn #id<'s>(&'s self) -> #types {
                    let res = #content;
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
fn _usize() -> TokenStream {
    quote! {::core::primitive::usize}
}
fn _char() -> TokenStream {
    quote! {::core::primitive::char}
}
fn _i32() -> TokenStream {
    quote! {::core::primitive::i32}
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
    /// Current rule will not contain a span.
    /// Current rule will not be visible in some APIs.
    Expression,
    /// Current rule will only contain a span.
    /// Inner structures will not be emitted.
    Span,
    /// Normal rule.
    Both,
}
impl ToTokens for Emission {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Expression => tokens.append(format_ident!("Expression")),
            Self::Span => tokens.append(format_ident!("Span")),
            Self::Both => tokens.append(format_ident!("Both")),
        }
    }
}

#[derive(Clone)]
struct RuleConfig<'g> {
    pub atomicity: Option<bool>,
    pub rule_id: Ident,
    pub boxed: bool,
    #[allow(dead_code)]
    pub rule_name: &'g str,
    pub rule_desc: String,
    pub rule_doc: Option<&'g str>,
    pub defined: &'g BTreeSet<&'g str>,
    pub builtins_without_lifetime: &'g BTreeSet<&'g str>,
}
impl<'g> RuleConfig<'g> {
    fn get_doc<'s>(&'s self) -> impl Iterator<Item = &'s str>
    where
        'g: 's,
    {
        use core::iter::once;
        once(self.rule_desc.as_str())
            .chain(once(""))
            .chain(self.rule_doc.iter().cloned())
    }
}

fn rule<'g>(
    rule_config: &RuleConfig<'g>,
    type_name: TokenStream,
    accessers: &Accesser<'g>,
    emission: Emission,
) -> TokenStream {
    let root = quote! {super::super};
    let _bool = _bool();
    let accessers = match emission {
        Emission::Both | Emission::Expression => accessers.collect(&root, rule_config),
        Emission::Span => quote! {},
    };
    #[allow(clippy::needless_lifetimes)]
    fn create<'g>(
        rule_config: &RuleConfig<'g>,
        accesser_impl: TokenStream,
        inner_type: TokenStream,
        emission: Emission,
    ) -> TokenStream {
        let root = quote! {super::super};
        let pest_typed = pest_typed();
        let name = &rule_config.rule_id;
        let atomicity = match rule_config.atomicity {
            Some(true) => quote! {true},
            Some(false) => quote! {false},
            None => quote! {INHERITED},
        };
        let docs = rule_config.get_doc();
        let ignore = ignore(&root);
        let boxed = rule_config.boxed;
        let usize = _usize();
        quote! {
            #pest_typed::rule!(#name, #(#docs)*, #root::Rule, #root::Rule::#name, #inner_type, #ignore, #atomicity, #emission, #boxed);
            impl<'i, const INHERITED: #usize> #name<'i, INHERITED> {
                #accesser_impl
            }
        }
    }
    create(rule_config, accessers, type_name, emission)
}

struct Output<'g> {
    content: Vec<TokenStream>,
    wrappers: Vec<TokenStream>,
    wrapper_counter: usize,
    rule_configs: BTreeMap<Ident, RuleConfig<'g>>,
    /// Rule Name -> (Tag Name, ([Type], Accesser)).
    #[cfg(feature = "grammar-extras")]
    tagged_nodes: BTreeMap<Ident, BTreeMap<Ident, (Vec<TokenStream>, Accesser<'g>)>>,
    sequences: BTreeSet<usize>,
    choices: BTreeSet<usize>,
}
impl<'g> Output<'g> {
    fn new() -> Self {
        Self {
            content: Vec::new(),
            wrappers: Vec::new(),
            wrapper_counter: 0,
            rule_configs: BTreeMap::new(),
            #[cfg(feature = "grammar-extras")]
            tagged_nodes: BTreeMap::new(),
            sequences: BTreeSet::new(),
            choices: BTreeSet::new(),
        }
    }
    /// Record usage of Seq* generics.
    fn record_seq(&mut self, index: usize) {
        self.sequences.insert(index);
    }
    /// Record usage of Choices* generics.
    fn record_choice(&mut self, index: usize) {
        self.choices.insert(index);
    }
    /// Used sequences.
    fn seq(&self) -> &BTreeSet<usize> {
        &self.sequences
    }
    /// Used choices.
    fn choices(&self) -> &BTreeSet<usize> {
        &self.choices
    }
    /// Insert rule struct to rule module.
    fn insert(&mut self, tokens: TokenStream, config: RuleConfig<'g>) {
        self.content.push(tokens);
        let prev = self.rule_configs.insert(config.rule_id.clone(), config);
        assert!(prev.is_none());
    }
    /// Insert built-in rule structs.
    fn insert_builtin(&mut self, tokens: TokenStream) {
        self.content.push(tokens);
    }
    /// Insert tag struct to tag module.
    /// Return the module path relative to module root.
    #[cfg(feature = "grammar-extras")]
    fn insert_tag(
        &mut self,
        rule_name: &Ident,
        tag_name: &Ident,
        inner: TokenStream,
        accesser: Accesser<'g>,
    ) -> () {
        let entry = self.tagged_nodes.entry(rule_name.clone()).or_default();
        let entry = entry.entry(tag_name.clone()).or_default();
        entry.0.push(inner);
        entry.1.join_mut(accesser);
    }
    /// Insert a string wrapper to corresponding module.
    /// Return the module path relative to module root.
    fn insert_string_wrapper(&mut self, string: &str) -> TokenStream {
        let s = ident(&format!("w_{}", self.wrapper_counter));
        self.wrapper_counter += 1;
        let doc = format!("A wrapper for `{:?}`.", string);
        let str = _str();
        let wrapper_mod = constant_wrappers();
        let pest_typed = pest_typed();
        let wrapper = quote! {
            #[doc = #doc]
            #[allow(non_camel_case_types)]
            #[derive(Clone, Hash, PartialEq, Eq)]
            pub struct #s;
            impl #pest_typed::StringWrapper for #s {
                const CONTENT: &'static #str = #string;
            }
        };
        self.wrappers.push(wrapper);
        quote! {#wrapper_mod::#s}
    }
    /// Insert a string array wrapper to corresponding module.
    /// Return the module path relative to module root.
    fn insert_string_array_wrapper(&mut self, strings: &[String]) -> TokenStream {
        let s = ident(&format!("w_{}", self.wrapper_counter));
        self.wrapper_counter += 1;
        let doc = format!("A wrapper for `{:?}`.", strings);
        let str = _str();
        let wrapper_mod = constant_wrappers();
        let pest_typed = pest_typed();
        let wrapper = quote! {
            #[doc = #doc]
            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq)]
            pub struct #s;
            impl #pest_typed::StringArrayWrapper for #s {
                const CONTENT: &'static [&'static #str] = &[ #(#strings),* ];
            }
        };
        self.wrappers.push(wrapper);
        quote! {#wrapper_mod::#s}
    }
    /// (nodes, wrappers)
    fn collect(&self) -> TokenStream {
        let content = &self.content;
        let wrappers = &self.wrappers;
        let wrapper_mod = constant_wrappers();
        let rules = rules_mod();
        #[cfg(feature = "grammar-extras")]
        let tags = self.tagged_nodes.iter().flat_map(|(rule_name, tags)| {
            let usize = _usize();
            // let root = quote! {super::super};
            // let config = self.rule_configs.get(rule_name).unwrap();
            #[allow(unused_variables)]
            let tags = tags.iter().map(|(tag_name, (types, accesser))| {
                let comment = format!("Tag {} referenced by {}.", tag_name, rule_name);
                // let accesser = accesser.collect(&root, config);
                quote! {
                    #[doc = #comment]
                    #[allow(non_camel_case_types)]
                    pub type #tag_name<'i, 's, const INHERITED: #usize> = ( #(&'s #types),* );
                }
                /*
                quote! {
                    #[doc = #comment]
                    pub struct #tag_name<'s, 'i, const INHERITED: #usize>{
                        /// Tag contents.
                        pub content: ( #(&'s #types, )* )
                    }
                    impl<'s, 'i, const INHERITED: #usize> #tag_name<'s, 'i, INHERITED> {
                        #accesser
                    }
                }
                */
            });
            let doc = format!("Tags inside rule [super::super::rules::{}].", rule_name);
            quote! {
                #[doc = #doc]
                #[allow(non_snake_case)]
                pub mod #rule_name {
                    #(#tags)*
                }
            }
        });
        #[cfg(feature = "grammar-extras")]
        let mod_tags = quote! {
            #[doc = "Generated structs for tags."]
            pub mod tags {
                #(#tags)*
            }
        };
        #[cfg(not(feature = "grammar-extras"))]
        let mod_tags = quote! {};
        let rules_impl = rules_impl_mod();
        quote! {
            mod #wrapper_mod {
                #(#wrappers)*
            }
            #mod_tags
            #[doc = "Definitions of statically typed nodes generated by pest-generator."]
            pub mod #rules_impl {
                #[doc = "Definitions of statically typed nodes generated by pest-generator."]
                pub mod #rules {
                    #(#content)*
                }
            }
            pub use #rules_impl::#rules as #rules;
        }
    }
}

/// Returns (type name, accesser).
fn process_single_alias<'g>(
    map: &mut Output<'g>,
    rule_config: &RuleConfig<'g>,
    type_name: TokenStream,
    accessers: Accesser<'g>,
    root: &TokenStream,
    emission: Emission,
    explicit: bool,
) -> (TokenStream, Accesser<'g>) {
    if explicit {
        let rule_id = &rule_config.rule_id;
        let def = rule(rule_config, type_name, &accessers, emission);
        map.insert(def, rule_config.clone());
        let rules = rules_mod();
        (quote! {#root::#rules::#rule_id::<'i>}, accessers)
    } else {
        (type_name, accessers)
    }
}

/// Returns type name.
fn generate_graph_node<'g>(
    expr: &'g OptimizedExpr,
    rule_config: &RuleConfig<'g>,
    // From node name to type definition and implementation
    map: &mut Output<'g>,
    explicit: bool,
    emission: Emission,
    config: Config,
    root: &TokenStream,
) -> (TokenStream, Accesser<'g>) {
    let generics = generics();
    let skip = match rule_config.atomicity {
        Some(true) => quote! {0},
        Some(false) => quote! {1},
        None => quote! {INHERITED},
    };
    macro_rules! walk {
        ($root:expr, $var:ident) => {{
            let mut current = $root;
            let mut nodes = Vec::<&OptimizedExpr>::new();
            while let OptimizedExpr::$var(lhs, rhs) = current {
                current = rhs;
                nodes.push(&lhs);
            }
            nodes.push(current);
            nodes
        }};
    }
    // Still some compile-time information not taken.
    match expr {
        OptimizedExpr::Str(content) => {
            let wrapper = map.insert_string_wrapper(content.as_str());
            process_single_alias(
                map,
                rule_config,
                quote! {
                    #root::#generics::Str::<#root::#wrapper>
                },
                Accesser::new(),
                root,
                emission,
                explicit,
            )
        }
        OptimizedExpr::Insens(content) => {
            let wrapper = map.insert_string_wrapper(content.as_str());
            process_single_alias(
                map,
                rule_config,
                quote! {
                    #root::#generics::Insens::<'i, #root::#wrapper>
                },
                Accesser::new(),
                root,
                emission,
                explicit,
            )
        }
        OptimizedExpr::PeekSlice(start, end) => process_single_alias(
            map,
            rule_config,
            match end {
                Some(end) => quote! {
                    #root::#generics::PeekSlice2::<#start, #end>
                },
                None => quote! {
                    #root::#generics::PeekSlice1::<#start>
                },
            },
            Accesser::new(),
            root,
            emission,
            explicit,
        ),
        OptimizedExpr::Push(expr) => {
            let (inner, accesser) =
                generate_graph_node(expr, rule_config, map, false, emission, config, root);
            process_single_alias(
                map,
                rule_config,
                quote! {
                    #root::#generics::Push::<#inner>
                },
                accesser.content(),
                root,
                emission,
                explicit,
            )
        }
        OptimizedExpr::Skip(strings) => {
            let wrapper = map.insert_string_array_wrapper(strings);
            process_single_alias(
                map,
                rule_config,
                quote! {
                    #root::#generics::Skip::<#root::#wrapper>
                },
                Accesser::new(),
                root,
                emission,
                explicit,
            )
        }
        OptimizedExpr::Range(start, end) => {
            let start = start.chars().next().unwrap();
            let end = end.chars().next().unwrap();
            process_single_alias(
                map,
                rule_config,
                quote! {
                    #root::#generics::CharRange::<#start, #end>
                },
                Accesser::new(),
                root,
                emission,
                explicit,
            )
        }
        OptimizedExpr::Ident(id) => {
            let inner = ident(id);
            let rules = rules_mod();
            let has_life_time = rule_config.defined.contains(id.as_str())
                || !rule_config.builtins_without_lifetime.contains(id.as_str());
            let has_skip = rule_config.defined.contains(id.as_str());
            let generics = match (has_life_time, has_skip) {
                (true, true) => quote! {::<'i, #skip>},
                (true, false) => quote! {::<'i>},
                (false, true) => quote! {::<#skip>},
                (false, false) => quote! {},
            };
            let accessers = if config.emit_rule_reference {
                Accesser::from_rule(id, id.as_str(), has_life_time, has_skip)
            } else {
                Accesser::new()
            };
            let type_name = quote! {#root::#rules::#inner #generics};
            process_single_alias(
                map,
                rule_config,
                type_name,
                accessers,
                root,
                emission,
                explicit,
            )
        }
        OptimizedExpr::PosPred(expr) => {
            let (inner, accessers) =
                generate_graph_node(expr, rule_config, map, false, emission, config, root);
            process_single_alias(
                map,
                rule_config,
                quote! {
                    #root::#generics::Positive::<#inner>
                },
                accessers.content(),
                root,
                emission,
                explicit,
            )
        }
        OptimizedExpr::NegPred(expr) => {
            // Impossible to access inner tokens.
            let (inner, _) =
                generate_graph_node(expr, rule_config, map, false, emission, config, root);
            process_single_alias(
                map,
                rule_config,
                quote! {
                    #root::#generics::Negative::<#inner>
                },
                Accesser::new(),
                root,
                emission,
                explicit,
            )
        }
        OptimizedExpr::RestoreOnErr(inner) => {
            generate_graph_node(inner, rule_config, map, explicit, emission, config, root)
        }
        OptimizedExpr::Seq(_, _) => {
            let vec = walk!(expr, Seq);
            let mut types = Vec::<TokenStream>::with_capacity(vec.len());
            let mut accesser = Accesser::new();
            for (i, expr) in vec.into_iter().enumerate() {
                let (child, acc) =
                    generate_graph_node(expr, rule_config, map, false, emission, config, root);
                types.push(child);
                accesser = accesser.join(acc.content_i(i));
            }
            let seq = format_ident!("Seq{}", types.len());
            map.record_seq(types.len());

            let pest_typed = pest_typed();
            let args = types.iter().map(
                |t| quote! {(#pest_typed::predefined_node::Skipped<#t, #root::generics::Skipped<'i>, #skip>)},
            );
            process_single_alias(
                map,
                rule_config,
                quote! { #root::#generics::#seq::<#(#args, )*> },
                accesser,
                root,
                emission,
                explicit,
            )
        }
        OptimizedExpr::Choice(_, _) => {
            let vec = walk!(expr, Choice);
            let mut types = Vec::<TokenStream>::with_capacity(vec.len());
            let mut accesser = Accesser::new();
            for (i, expr) in vec.into_iter().enumerate() {
                let (child, acc) =
                    generate_graph_node(expr, rule_config, map, false, emission, config, root);
                types.push(child);
                accesser = accesser.join(acc.choice(i));
            }
            let choice = format_ident!("Choice{}", types.len());
            map.record_choice(types.len());
            process_single_alias(
                map,
                rule_config,
                quote! { #root::#generics::#choice::<#(#types, )*> },
                accesser,
                root,
                emission,
                explicit,
            )
        }
        OptimizedExpr::Opt(inner) => {
            let (inner_name, accessers) =
                generate_graph_node(inner, rule_config, map, false, emission, config, root);
            let accessers = accessers.optional();
            let option = option_type();
            process_single_alias(
                map,
                rule_config,
                quote! {#option::<#inner_name>},
                accessers,
                root,
                emission,
                explicit,
            )
        }
        OptimizedExpr::Rep(inner) => {
            let (inner_name, accessers) =
                generate_graph_node(inner, rule_config, map, false, emission, config, root);
            process_single_alias(
                map,
                rule_config,
                quote! { #root::#generics::Rep::<'i, #skip, #inner_name> },
                accessers.contents(),
                root,
                emission,
                explicit,
            )
        }
        #[cfg(feature = "grammar-extras")]
        OptimizedExpr::RepOnce(inner) => {
            let (inner_name, accessers) =
                generate_graph_node(inner, rule_config, map, false, emission, config, root);
            process_single_alias(
                map,
                rule_config,
                quote! { #root::#generics::RepOnce::<'i, #skip, #inner_name> },
                accessers.contents(),
                root,
                emission,
                explicit,
            )
        }
        #[cfg(feature = "grammar-extras")]
        OptimizedExpr::NodeTag(inner_expr, tag) => {
            if config.emit_tagged_node_reference {
                let tag_id = ident(tag.as_str());
                let (inner, accesser) = generate_graph_node(
                    inner_expr,
                    rule_config,
                    map,
                    explicit,
                    emission,
                    config,
                    root,
                );
                map.insert_tag(
                    &rule_config.rule_id,
                    &tag_id,
                    inner.clone(),
                    accesser.clone(),
                );
                let new_accesser =
                    Accesser::from_tag(rule_config.rule_name, tag.as_str(), inner.clone());
                if config.truncate_accesser_at_node_tag {
                    (inner, new_accesser)
                } else {
                    (inner, new_accesser.join(accesser))
                }
            } else {
                let (inner, accesser) = generate_graph_node(
                    inner_expr,
                    rule_config,
                    map,
                    explicit,
                    emission,
                    config,
                    root,
                );
                process_single_alias(map, rule_config, inner, accesser, root, emission, false)
            }
        }
    }
}

fn generate_graph<'g: 'f, 'f>(
    rules: &'g [OptimizedRule],
    defined: &'g BTreeSet<&'g str>,
    not_boxed: &'f BTreeSet<&'g str>,
    builtins_without_lifetime: &'g BTreeSet<&'g str>,
    config: Config,
    doc: &'g DocComment,
) -> Output<'g> {
    let mut res = Output::new();
    for rule in rules.iter() {
        let rule_name = rule.name.as_str();
        let (atomicity, emission) = match rule.ty {
            RuleType::Normal => (None, Emission::Both),
            RuleType::Silent => (None, Emission::Expression),
            RuleType::NonAtomic => (Some(false), Emission::Both),
            RuleType::CompoundAtomic => (Some(true), Emission::Both),
            RuleType::Atomic => (Some(true), Emission::Span),
        };
        let atomicity_doc = match atomicity {
            Some(true) => "Atomic rule.",
            Some(false) => "Non-atomic rule.",
            None => "Normal rule.",
        };
        let rule_desc = format!(
            "Corresponds to expression: `{}`. {}",
            rule.expr, atomicity_doc
        );
        let boxed = !config.box_only_if_needed || !not_boxed.contains(rule_name);
        let rule_doc = doc.line_docs.get(rule_name).map(|s| s.as_str());
        let rule_config = RuleConfig {
            atomicity,
            boxed,
            rule_id: ident(rule_name),
            rule_name,
            rule_desc,
            rule_doc,
            defined,
            builtins_without_lifetime,
        };
        generate_graph_node(
            &rule.expr,
            &rule_config,
            &mut res,
            true,
            emission,
            config,
            &quote! {super::super},
        );
    }
    res
}

/// Whether skipped rules has been defined.
#[derive(Clone, Copy)]
struct Implicit {
    whitespace: bool,
    comment: bool,
}

impl<'s> From<&'s [OptimizedRule]> for Implicit {
    fn from(value: &'s [OptimizedRule]) -> Self {
        let whitespace = value.iter().any(|rule| rule.name == "WHITESPACE");
        let comment = value.iter().any(|rule| rule.name == "COMMENT");
        Self {
            whitespace,
            comment,
        }
    }
}

fn collect_used_rule<'s>(rule: &'s OptimizedRule, implicit: Implicit, res: &mut BTreeSet<&'s str>) {
    //
    if rule.ty == RuleType::Normal {
        if implicit.comment {
            res.insert("COMMENT");
        }
        if implicit.whitespace {
            res.insert("WHITESPACE");
        }
    }
    let mut exprs = vec![&rule.expr];
    while let Some(expr) = exprs.pop() {
        match expr {
            OptimizedExpr::Str(_) | OptimizedExpr::Insens(_) | OptimizedExpr::Range(_, _) => (),
            OptimizedExpr::Ident(rule_name) => {
                res.insert(rule_name.as_str());
            }
            OptimizedExpr::PeekSlice(_, _) => (),
            OptimizedExpr::PosPred(expr) | OptimizedExpr::NegPred(expr) => exprs.push(expr),
            OptimizedExpr::Seq(lhs, rhs) | OptimizedExpr::Choice(lhs, rhs) => {
                exprs.push(lhs);
                exprs.push(rhs);
            }
            OptimizedExpr::Opt(expr) | OptimizedExpr::Rep(expr) => exprs.push(expr),
            #[cfg(feature = "grammar-extras")]
            OptimizedExpr::RepOnce(expr) => exprs.push(expr),
            OptimizedExpr::Skip(_) => (),
            OptimizedExpr::Push(expr) | OptimizedExpr::RestoreOnErr(expr) => exprs.push(expr),
            #[cfg(feature = "grammar-extras")]
            OptimizedExpr::NodeTag(expr, _) => exprs.push(expr),
        }
    }
}

fn collect_used_rules<'s>(rules: &'s [OptimizedRule], implicit: Implicit) -> BTreeSet<&'s str> {
    let mut res = BTreeSet::<&'s str>::new();
    for rule in rules {
        collect_used_rule(rule, implicit, &mut res);
    }
    res
}

fn collect_reachability<'g>(
    rules: &'g [OptimizedRule],
    implicit: Implicit,
) -> BTreeMap<&'g str, BTreeSet<&'g str>> {
    let mut res: BTreeMap<&'g str, BTreeSet<&'g str>> = BTreeMap::new();
    for rule in rules {
        let entry = res.entry(rule.name.as_str()).or_default();
        collect_used_rule(rule, implicit, entry);
    }
    for _ in 0..rules.len() {
        for rule in rules {
            if let Some(cur) = res.remove(rule.name.as_str()) {
                let mut new = cur.clone();
                for referenced in cur {
                    if let Some(iter) = res.get(referenced) {
                        new.extend(iter);
                    }
                }
                if !new.contains(rule.name.as_str()) {
                    res.insert(rule.name.as_str(), new);
                }
            }
        }
    }
    res
}

pub(crate) fn generate_typed_pair_from_rule(
    rules: &[OptimizedRule],
    doc: &DocComment,
    config: Config,
) -> TokenStream {
    let pest_typed = pest_typed();

    let defined_rules: BTreeSet<&str> = rules.iter().map(|rule| rule.name.as_str()).collect();

    let implicit = Implicit::from(rules);

    let referenced_rules = collect_used_rules(rules, implicit);

    let (builtin, mut builtins_without_lifetime) =
        generate_builtin(&defined_rules, &referenced_rules);

    let unicode_rule = generate_unicode(
        &defined_rules,
        &referenced_rules,
        &mut builtins_without_lifetime,
    );

    let not_boxed = collect_reachability(rules, implicit)
        .keys()
        .cloned()
        .collect();

    let mut graph = generate_graph(
        rules,
        &defined_rules,
        &not_boxed,
        &builtins_without_lifetime,
        config,
        doc,
    );

    graph.insert_builtin(quote! {#builtin});

    let mods = graph.collect();
    let unicode = unicode_mod();
    let generics = {
        let root = quote! {super};
        let rules_mod = rules_mod();
        let _i32 = _i32();
        let usize = _usize();
        let fill = |set: &BTreeSet<usize>,
                    target: &mut Vec<TokenStream>,
                    prefix: &str,
                    mac: &Ident,
                    module: &Ident,
                    mod_prefix: Option<&'static str>,
                    seq: bool| {
            for item in set {
                let generics_i = format_ident!("{}{}", prefix, item);
                let (types, field): (Vec<_>, Vec<_>) = (0..*item)
                    .map(|i| {
                        let field = if seq {
                            let i = Index::from(i);
                            quote! {#i}
                        } else {
                            let i = format_ident!("_{}", i);
                            quote! {#i}
                        };
                        (format_ident!("T{}", i), field)
                    })
                    .unzip();
                // `pest_typed` and `TypedNode` is already imported, so can be referred directly.
                if *item >= 12 {
                    let mod_i = if let Some(mod_prefix) = mod_prefix {
                        let mod_name = format_ident!("{}{}", mod_prefix, item);
                        quote! {#mod_name, }
                    } else {
                        quote! {}
                    };
                    target.push(quote! {
                        pest_typed::#mac!(#generics_i, pest_typed, #mod_i #item, #(#types, #field, )*);
                    });
                } else {
                    target.push(quote! {
                        pub use pest_typed::#module::#generics_i;
                    })
                }
            }
        };
        let mut seq = vec![];
        let mut chs = vec![];
        fill(
            graph.seq(),
            &mut seq,
            "Seq",
            &format_ident!("seq"),
            &format_ident!("sequence"),
            None,
            true,
        );
        fill(
            graph.choices(),
            &mut chs,
            "Choice",
            &format_ident!("choices"),
            &format_ident!("choices"),
            Some("choice"),
            false,
        );

        let has_white_space = defined_rules.contains("WHITESPACE");
        let has_comment = defined_rules.contains("COMMENT");
        let skip = match (has_white_space, has_comment) {
            (true, true) => quote! {
                predefined_node::AtomicRep<
                    #pest_typed::choices::Choice2<
                        #root::#rules_mod::WHITESPACE<'i, 0>,
                        #root::#rules_mod::COMMENT<'i, 0>,
                    >,
                >
            },
            (true, false) => quote! {
                predefined_node::AtomicRep<
                    #root::#rules_mod::WHITESPACE<'i, 0>,
                >
            },
            (false, true) => quote! {
                predefined_node::AtomicRep<
                    #root::#rules_mod::COMMENT<'i, 0>,
                >
            },
            (false, false) => quote! {
                predefined_node::Empty<'i>
            },
        };

        quote! {
            #[doc = "Used generics."]
            pub mod generics {
                use #pest_typed::predefined_node;
                /// Skipped content.
                pub type Skipped<'i> = #skip;
                pub use predefined_node::{Str, Insens, PeekSlice1, PeekSlice2, Push, Skip, CharRange, Positive, Negative};
                #(#seq)*
                #(#chs)*
                /// Repeat arbitrary times.
                pub type Rep<'i, const SKIP: #usize, T> = predefined_node::Rep<T, Skipped<'i>, SKIP>;
                /// Repeat at least once.
                pub type RepOnce<'i, const SKIP: #usize, T> = predefined_node::RepOnce<T, Skipped<'i>, SKIP>;
            }
        }
    };
    let pairs = {
        let rules_mod = rules_mod();
        let pairs_mod = pairs_mod();
        let doc = format! {"Re-export some types from {} to simplify the usage.", rules_mod};
        quote! {
            #[doc = #doc]
            pub use #rules_mod as #pairs_mod;
        }
    };
    let res = quote! {
        #[doc = "Unicode rules."]
        pub mod #unicode {
            #unicode_rule
        }
        #mods
        #generics
        #pairs
    };
    res
}

fn generate_unicode(
    rule_names: &BTreeSet<&str>,
    referenced: &BTreeSet<&str>,
    without_lifetime: &mut BTreeSet<&'static str>,
) -> TokenStream {
    let mut results = vec![];
    let pest_typed = pest_typed();
    let pest_unicode = pest_unicode();
    let option = option_type();
    let position = position();
    let stack = stack();
    let span = _span();
    let char = _char();
    let tracker = tracker();
    let root = quote! {super};
    let box_ = box_type();

    for property in unicode_property_names() {
        let property_ident: Ident = syn::parse_str(property).unwrap();
        // insert manually for #property substitution

        let doc = format!("Auto generated. Unicode property {}.", property);

        if !rule_names.contains(property) && referenced.contains(property) {
            without_lifetime.insert(property);
            results.push(quote! {
                #[allow(non_camel_case_types)]
                #[doc = #doc]
                #[derive(Clone, Hash, PartialEq, Eq)]
                pub struct #property_ident {
                    pub content: #char,
                }
                impl ::core::convert::From<#char> for #property_ident {
                    fn from(content: #char) -> Self {
                        Self {
                            content,
                        }
                    }
                }
                impl<'i> #pest_typed::TypedNode<'i, #root::Rule> for #property_ident {
                    #[inline]
                    fn try_parse_with(
                        mut input: #position<'i>,
                        _stack: &mut #stack<#span<'i>>,
                        _tracker: &mut #tracker<'i, #root::Rule>,
                    ) -> #option<(#position<'i>, Self)> {
                        match #pest_typed::predefined_node::match_char_by(&mut input, #pest_unicode::#property_ident) {
                            Some(content) => {
                                Some((input, Self::from(content)))
                            }
                            None => {
                                None
                            }
                        }
                    }
                }
                impl ::core::fmt::Debug for #property_ident {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct(#property)
                            .field("content", &self.content)
                            .finish()
                    }
                }
                impl<'i: 'n, 'n> #pest_typed::iterators::Pairs<'i, 'n, #root::Rule> for #property_ident {
                    type Iter = ::core::iter::Empty<&'n dyn #pest_typed::iterators::Pair<'i, 'n, #root::Rule>>;
                    type IntoIter = ::core::iter::Empty<#box_<dyn #pest_typed::iterators::Pair<'i, 'n, #root::Rule> + 'n>>;

                    fn iter_pairs(&'n self) -> Self::Iter {
                        ::core::iter::empty()
                    }
                    fn into_iter_pairs(self) -> Self::IntoIter {
                        ::core::iter::empty()
                    }
                }
            });
        }
    }
    quote! {
        #(#results)*
    }
}

fn generate_builtin(
    defined: &BTreeSet<&str>,
    referenced: &BTreeSet<&str>,
) -> (TokenStream, BTreeSet<&'static str>) {
    let root = quote! {super::super};
    let pest_typed = pest_typed();
    let unicode = unicode_mod();
    let mut results = vec![quote! {
        #[allow(unused_imports)]
        use #root::#unicode::*;
    }];
    let mut builtins_without_lifetime = BTreeSet::new();
    macro_rules! insert_builtin {
        ($name:expr, $def:path) => {
            if !defined.contains($name) && referenced.contains($name) {
                let id = ident($name);
                builtins_without_lifetime.insert($name);
                results.push(quote! {
                    #[allow(non_camel_case_types)]
                    pub use #pest_typed::predefined_node::$def as #id;
                });
            }
        };
    }
    macro_rules! insert_builtin_with_lifetime {
        ($name:expr, $def:path) => {
            if !defined.contains($name) && referenced.contains($name) {
                let id = ident($name);
                results.push(quote! {
                    #[allow(non_camel_case_types)]
                    pub use #pest_typed::predefined_node::$def as #id;
                });
            }
        };
    }

    results.push(quote! {
        #pest_typed::rule_eoi!(EOI, #root::Rule);
    });

    insert_builtin!("ANY", ANY);
    insert_builtin!("SOI", SOI);
    insert_builtin_with_lifetime!("PEEK", PEEK);
    insert_builtin_with_lifetime!("PEEK_ALL", PEEK_ALL);
    insert_builtin_with_lifetime!("POP", POP);
    insert_builtin_with_lifetime!("POP_ALL", POP_ALL);
    insert_builtin!("DROP", DROP);

    insert_builtin!("ASCII_DIGIT", ASCII_DIGIT);
    insert_builtin!("ASCII_NONZERO_DIGIT", ASCII_NONZERO_DIGIT);
    insert_builtin!("ASCII_BIN_DIGIT", ASCII_BIN_DIGIT);
    insert_builtin!("ASCII_OCT_DIGIT", ASCII_OCT_DIGIT);
    insert_builtin!("ASCII_HEX_DIGIT", ASCII_HEX_DIGIT);
    insert_builtin!("ASCII_ALPHA_LOWER", ASCII_ALPHA_LOWER);
    insert_builtin!("ASCII_ALPHA_UPPER", ASCII_ALPHA_UPPER);
    insert_builtin!("ASCII_ALPHA", ASCII_ALPHA);
    insert_builtin!("ASCII_ALPHANUMERIC", ASCII_ALPHANUMERIC);
    insert_builtin!("ASCII", ASCII);
    insert_builtin!("NEWLINE", NEWLINE);

    insert_builtin_with_lifetime!("WHITESPACE", AlwaysFail);
    insert_builtin_with_lifetime!("COMMENT", AlwaysFail);

    (quote! { #(#results)*}, builtins_without_lifetime)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::docs::consume;
    use lazy_static::lazy_static;
    use pest_meta::{
        parse_and_optimize,
        parser::{parse, Rule},
    };
    use std::string::String;

    lazy_static! {
        static ref SYNTAX: String =
            String::from_utf8(std::fs::read("tests/syntax.pest").unwrap()).unwrap();
        static ref PARSE_RESULT: (Vec<&'static str>, Vec<OptimizedRule>) =
            parse_and_optimize(&SYNTAX).unwrap();
        static ref DOC_COMMENT: DocComment = consume(parse(Rule::grammar_rules, &SYNTAX).unwrap());
    }

    #[test]
    fn inlined_used_rules() {
        let (_, rules) = parse_and_optimize(r#"x = { a ~ b } a = { "a" } b = { ^"b" }"#).unwrap();
        let implicit = Implicit::from(rules.as_slice());
        let used = collect_used_rules(&rules, implicit);
        assert_eq!(used, BTreeSet::from(["a", "b"]));
    }
    #[test]
    /// Check collected used rules in a complex grammar.
    ///
    /// PEEK and PUSH are translated to [`OptimizedExpr::PeekSlice`] and [`OptimizedExpr::Push`].
    fn used_rules() {
        let rules = &PARSE_RESULT.1;
        let implicit = Implicit::from(rules.as_slice());
        let used = collect_used_rules(&rules, implicit);
        let expected = include!("../tests/syntax.used.rules.txt");
        let expected = BTreeSet::from(expected);
        assert_eq!(used, expected);
    }
    #[test]
    /// Check we can actually break the cycles.
    fn inter_reference() {
        let (_, rules) =
            parse_and_optimize(r#"a = { "a" ~ b* } b = { "b" ~ c? } c = { a+ }"#).unwrap();
        let implicit = Implicit::from(rules.as_slice());
        let used = collect_used_rules(&rules, implicit);
        assert_eq!(used, BTreeSet::from(["a", "b", "c"]));
        let graph = collect_reachability(&rules, implicit);
        assert_eq!(graph, BTreeMap::from([("b", BTreeSet::from(["a", "c"]))]));
    }
}
