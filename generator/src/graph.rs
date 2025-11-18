// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::{
    config::Config,
    docs::DocComment,
    types::{option_type, vec_type},
};
use pest::unicode::unicode_property_names;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::collections::{btree_map, BTreeMap, BTreeSet};
use syn::Index;
pub(crate) use traits::Generate;

mod optimized_rule;
mod rule;
mod traits;

pub(crate) fn pest_typed() -> TokenStream {
    quote! {::pest_typed}
}

fn pest_unicode() -> TokenStream {
    quote! {::pest_typed::predefined_node::unicode}
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
        #root::#generics::Skipped::<S>
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
    const fn from_rule(value: &'g str, has_lifetime: bool, has_skip: bool) -> Self {
        Self::Rule(value, has_lifetime, has_skip)
    }
    #[cfg(feature = "grammar-extras")]
    const fn from_tag(rule_name: &'g str, tag_name: &'g str, tokens: Vec<TokenStream>) -> Self {
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
    fn wrap(self, edge: Edge) -> Self {
        match edge {
            Edge::Content => Self::Content(Box::new(self)),
            Edge::ContentI(i) => Self::SequenceI(i, Box::new(self)),
            Edge::ChoiceI(i) => Self::ChoiceI(i, self.flattenable(), Box::new(self)),
            Edge::Optional => Self::Optional(self.flattenable(), Box::new(self)),
            Edge::Contents => Self::Contents(Box::new(self)),
        }
    }
    fn merge(self, other: Self) -> Self {
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
    fn expand(&self, root: &TokenStream, config: &RuleConfig<'g>) -> (TokenStream, TokenStream) {
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
                    (true, true) => quote! {::<S, #skip>},
                    (true, false) => quote! {::<S>},
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
                (quote! {res}, quote! {(#(&'s #inner),*)})
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
struct Getter<'g> {
    /// name -> (path, type)
    getters: BTreeMap<&'g str, Node<'g>>,
}
impl Default for Getter<'_> {
    fn default() -> Self {
        Self::new()
    }
}
impl<'g> Getter<'g> {
    const fn new() -> Self {
        Self {
            getters: BTreeMap::new(),
        }
    }
    fn from_rule(name: &'g str, id: &'g str, has_life_time: bool, has_skip: bool) -> Self {
        let res = BTreeMap::from([(name, Node::from_rule(id, has_life_time, has_skip))]);
        Self { getters: res }
    }
    #[cfg(feature = "grammar-extras")]
    fn from_tag(rule: &'g str, name: &'g str, tokens: TokenStream) -> Self {
        let res = BTreeMap::from([(name, Node::from_tag(rule, name, vec![tokens]))]);
        Self { getters: res }
    }
    fn content(self) -> Self {
        self.prepend(Edge::Content)
    }
    fn content_i(self, i: usize) -> Self {
        self.prepend(Edge::ContentI(i))
    }
    fn contents(self) -> Self {
        self.prepend(Edge::Contents)
    }
    fn optional(self) -> Self {
        self.prepend(Edge::Optional)
    }
    fn choice(self, i: usize) -> Self {
        self.prepend(Edge::ChoiceI(i))
    }
    #[inline]
    fn prepend(mut self, edge: Edge) -> Self {
        for (_, node) in self.getters.iter_mut() {
            // TODO: Ellide clone here.
            *node = node.clone().wrap(edge.clone());
        }
        self
    }
    /// Join two getter forest in the same level.
    fn join_mut(&mut self, other: Self) {
        other.getters.into_iter().for_each(|(name, tree)| {
            let entry = self.getters.entry(name);
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
    /// Join two getter forest in the same level.
    fn join(mut self, other: Self) -> Self {
        self.join_mut(other);
        self
    }
    fn collect(&self, root: &TokenStream, config: &RuleConfig<'g>) -> TokenStream {
        let getters = self.getters.iter().map(|(name, node)| {
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
            let doc = format!("A helper function to access [`{}`].", name);
            quote! {
                #[doc = #doc]
                #src
            }
        });
        quote! {
            #(#getters)*
        }
    }
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
    getters: &Getter<'g>,
    emission: Emission,
) -> TokenStream {
    let root = quote! {super::super};
    let _bool = _bool();
    let getters = match emission {
        Emission::Both | Emission::Expression => getters.collect(&root, rule_config),
        Emission::Span => quote! {},
    };
    #[allow(clippy::needless_lifetimes)]
    fn create<'g>(
        rule_config: &RuleConfig<'g>,
        getter_impl: TokenStream,
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
            #pest_typed::rule!(pub #name, #(#docs)*, #root::Rule, #root::Rule::#name, #inner_type, #ignore, #atomicity, #emission, #boxed);
            impl<S, const INHERITED: #usize> #name<S, INHERITED> {
                #getter_impl
            }
        }
    }
    create(rule_config, getters, type_name, emission)
}

struct Output<'g> {
    content: Vec<TokenStream>,
    wrappers: Vec<TokenStream>,
    wrapper_counter: usize,
    rule_configs: BTreeMap<Ident, RuleConfig<'g>>,
    /// Rule Name -> (Tag Name, ([Type], Getter)).
    #[cfg(feature = "grammar-extras")]
    tagged_nodes: BTreeMap<Ident, BTreeMap<Ident, (Vec<TokenStream>, Getter<'g>)>>,
    sequences: BTreeSet<usize>,
    choices: BTreeSet<usize>,
}
impl<'g> Output<'g> {
    const fn new() -> Self {
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
    const fn seq(&self) -> &BTreeSet<usize> {
        &self.sequences
    }
    /// Used choices.
    const fn choices(&self) -> &BTreeSet<usize> {
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
        getter: Getter<'g>,
    ) {
        let entry = self.tagged_nodes.entry(rule_name.clone()).or_default();
        let entry = entry.entry(tag_name.clone()).or_default();
        entry.0.push(inner);
        entry.1.join_mut(getter);
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
            let tags = tags.iter().map(|(tag_name, (types, getter))| {
                let comment = format!("Tag {} referenced by {}.", tag_name, rule_name);
                // let getter = getter.collect(&root, config);
                quote! {
                    #[doc = #comment]
                    #[allow(non_camel_case_types)]
                    pub type #tag_name<'s, S, const INHERITED: #usize> = ( #(&'s #types),* );
                }
                /*
                quote! {
                    #[doc = #comment]
                    pub struct #tag_name<'s, S, const INHERITED: #usize>{
                        /// Tag contents.
                        pub content: ( #(&'s #types, )* )
                    }
                    impl<'s, S, const INHERITED: #usize> #tag_name<'s, S, INHERITED> {
                        #getter
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

/// Whether skipped rules has been defined.
#[derive(Clone, Copy)]
struct Implicit {
    whitespace: bool,
    comment: bool,
}

impl<'s, R: Generate> From<&'s [R]> for Implicit {
    fn from(value: &'s [R]) -> Self {
        let whitespace = value.iter().any(|rule| rule.name() == "WHITESPACE");
        let comment = value.iter().any(|rule| rule.name() == "COMMENT");
        Self {
            whitespace,
            comment,
        }
    }
}

fn collect_used_rules<'s, R: Generate>(rules: &'s [R], implicit: Implicit) -> BTreeSet<&'s str> {
    let mut res = BTreeSet::<&'s str>::new();
    for rule in rules {
        R::collect_used_rule(rule, implicit, &mut res);
    }
    res
}

fn collect_reachability<'g, R: Generate>(
    rules: &'g [R],
    implicit: Implicit,
) -> BTreeMap<&'g str, BTreeSet<&'g str>> {
    let mut res: BTreeMap<&'g str, BTreeSet<&'g str>> = BTreeMap::new();
    for rule in rules {
        let entry = res.entry(rule.name()).or_default();
        R::collect_used_rule(rule, implicit, entry);
    }
    for _ in 0..rules.len() {
        let mut updated = false;
        for rule in rules {
            if let Some(cur) = res.remove(rule.name()) {
                let old_len = cur.len();
                let mut new = cur.clone();
                for referenced in cur {
                    if let Some(iter) = res.get(referenced) {
                        new.extend(iter);
                    }
                }
                if new.len() > old_len {
                    updated = true;
                }
                if !new.contains(rule.name()) {
                    res.insert(rule.name(), new);
                }
            }
        }
        if !updated {
            break;
        }
    }
    res
}

pub(crate) fn generate_typed_pair_from_rule<R: Generate>(
    rules: &[R],
    doc: &DocComment,
    config: Config,
) -> TokenStream {
    let pest_typed = pest_typed();

    let defined_rules: BTreeSet<&str> = rules.iter().map(|rule| rule.name()).collect();

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

    let mut graph = R::generate_graph(
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
                        pest_typed::#mac!(#generics_i, #mod_i #item, #(#types, #field, )*);
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
                predefined_node::AtomicRepeat<
                    #pest_typed::choices::Choice2<
                        #root::#rules_mod::WHITESPACE<S, 0>,
                        #root::#rules_mod::COMMENT<S, 0>,
                    >,
                >
            },
            (true, false) => quote! {
                predefined_node::AtomicRepeat<
                    #root::#rules_mod::WHITESPACE<S, 0>,
                >
            },
            (false, true) => quote! {
                predefined_node::AtomicRepeat<
                    #root::#rules_mod::COMMENT<S, 0>,
                >
            },
            (false, false) => quote! {
                predefined_node::Empty<S>
            },
        };

        quote! {
            #[doc = "Used generics."]
            pub mod generics {
                use #pest_typed::predefined_node;
                /// Skipped content.
                pub type Skipped<S> = #skip;
                pub use predefined_node::{Str, Insens, PeekSlice1, PeekSlice2, Push, PushLiteral, Skip, CharRange, Positive, Negative};
                #(#seq)*
                #(#chs)*
                /// Repeat arbitrary times.
                pub type Rep<S, const SKIP: #usize, T> = predefined_node::Rep<T, Skipped<S>, SKIP>;
                /// Repeat at least once.
                pub type RepOnce<S, const SKIP: #usize, T> = predefined_node::RepOnce<T, Skipped<S>, SKIP>;
            }
        }
    };
    let pairs = {
        let rules_mod = rules_mod();
        let pairs_mod = pairs_mod();
        let doc = format!(
            "Re-export some types from {} to simplify the usage.",
            rules_mod
        );
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
    let pest_unicode = pest_unicode();

    let mut used_unicode = BTreeSet::new();

    for property in unicode_property_names() {
        let property_ident: Ident = syn::parse_str(property).unwrap();
        // insert manually for #property substitution

        if !rule_names.contains(property) && referenced.contains(property) {
            without_lifetime.insert(property);
            used_unicode.insert(property_ident);
        }
    }
    if !used_unicode.is_empty() {
        results.push(quote! {
            #[allow(unused_imports)]
            pub use #pest_unicode::{#(#used_unicode),*};
        });
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
                    pub use #pest_typed::predefined_node::$def as #id;
                });
            }
        };
    }

    results.push(quote! {
        #pest_typed::rule_eoi!(pub EOI, #root::Rule);
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
        optimizer::OptimizedRule,
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
