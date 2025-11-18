use super::{
    generics, ident, pest_typed, rules_mod,
    traits::{process_single_alias, Generate},
    Emission, Getter, Implicit, Output, RuleConfig,
};
use crate::{config::Config, docs::DocComment, types::option_type};
use pest_meta::ast::{Expr, Rule, RuleType};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeSet;

impl Generate for Rule {
    type Expr = Expr;

    fn name(&self) -> &str {
        &self.name
    }

    fn ty(&self) -> RuleType {
        self.ty
    }

    /// Returns type name.
    #[allow(private_interfaces)]
    fn generate_graph_node<'g>(
        expr: &'g Expr,
        rule_config: &RuleConfig<'g>,
        // From node name to type definition and implementation
        map: &mut Output<'g>,
        explicit: bool,
        emission: Emission,
        config: Config,
        root: &TokenStream,
    ) -> (TokenStream, Getter<'g>) {
        let generics = generics();
        let skip = match rule_config.atomicity {
            Some(true) => quote! {0},
            Some(false) => quote! {1},
            None => quote! {INHERITED},
        };
        macro_rules! walk {
            ($root:expr, $var:ident) => {{
                let mut current = $root;
                let mut nodes = Vec::<&Expr>::new();
                while let Expr::$var(lhs, rhs) = current {
                    current = rhs;
                    nodes.push(&lhs);
                }
                nodes.push(current);
                nodes
            }};
        }
        // Still some compile-time information not taken.
        match expr {
            Expr::Str(content) => {
                let wrapper = map.insert_string_wrapper(content.as_str());
                process_single_alias(
                    map,
                    rule_config,
                    quote! {
                        #root::#generics::Str::<#root::#wrapper>
                    },
                    Getter::new(),
                    root,
                    emission,
                    explicit,
                )
            }
            Expr::Insens(content) => {
                let wrapper = map.insert_string_wrapper(content.as_str());
                process_single_alias(
                    map,
                    rule_config,
                    quote! {
                        #root::#generics::Insens::<S, #root::#wrapper>
                    },
                    Getter::new(),
                    root,
                    emission,
                    explicit,
                )
            }
            Expr::PeekSlice(start, end) => process_single_alias(
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
                Getter::new(),
                root,
                emission,
                explicit,
            ),
            Expr::Push(expr) => {
                let (inner, getter) = Self::generate_graph_node(
                    expr,
                    rule_config,
                    map,
                    false,
                    emission,
                    config,
                    root,
                );
                process_single_alias(
                    map,
                    rule_config,
                    quote! {
                        #root::#generics::Push::<#inner>
                    },
                    getter.content(),
                    root,
                    emission,
                    explicit,
                )
            }
            Expr::Skip(strings) => {
                let wrapper = map.insert_string_array_wrapper(strings);
                process_single_alias(
                    map,
                    rule_config,
                    quote! {
                        #root::#generics::Skip::<S, #root::#wrapper>
                    },
                    Getter::new(),
                    root,
                    emission,
                    explicit,
                )
            }
            Expr::Range(start, end) => {
                let start = start.chars().next().unwrap();
                let end = end.chars().next().unwrap();
                process_single_alias(
                    map,
                    rule_config,
                    quote! {
                        #root::#generics::CharRange::<#start, #end>
                    },
                    Getter::new(),
                    root,
                    emission,
                    explicit,
                )
            }
            Expr::Ident(id) => {
                let inner = ident(id);
                let rules = rules_mod();
                let has_generic_s = rule_config.defined.contains(id.as_str())
                    || !rule_config.builtins_without_lifetime.contains(id.as_str());
                let has_skip = rule_config.defined.contains(id.as_str());
                let generics = match (has_generic_s, has_skip) {
                    (true, true) => quote! {::<S, #skip>},
                    (true, false) => quote! {::<S>},
                    (false, true) => quote! {::<#skip>},
                    (false, false) => quote! {},
                };
                let getters = if config.emit_rule_reference {
                    Getter::from_rule(id, id.as_str(), has_generic_s, has_skip)
                } else {
                    Getter::new()
                };
                let type_name = quote! {#root::#rules::#inner #generics};
                process_single_alias(
                    map,
                    rule_config,
                    type_name,
                    getters,
                    root,
                    emission,
                    explicit,
                )
            }
            Expr::PosPred(expr) => {
                let (inner, getters) = Self::generate_graph_node(
                    expr,
                    rule_config,
                    map,
                    false,
                    emission,
                    config,
                    root,
                );
                process_single_alias(
                    map,
                    rule_config,
                    quote! {
                        #root::#generics::Positive::<#inner>
                    },
                    getters.content(),
                    root,
                    emission,
                    explicit,
                )
            }
            Expr::NegPred(expr) => {
                // Impossible to access inner tokens.
                let (inner, _) = Self::generate_graph_node(
                    expr,
                    rule_config,
                    map,
                    false,
                    emission,
                    config,
                    root,
                );
                process_single_alias(
                    map,
                    rule_config,
                    quote! {
                        #root::#generics::Negative::<#inner>
                    },
                    Getter::new(),
                    root,
                    emission,
                    explicit,
                )
            }
            Expr::Seq(_, _) => {
                let vec = walk!(expr, Seq);
                let mut types = Vec::<TokenStream>::with_capacity(vec.len());
                let mut getter = Getter::new();
                for (i, expr) in vec.into_iter().enumerate() {
                    let (child, acc) = Self::generate_graph_node(
                        expr,
                        rule_config,
                        map,
                        false,
                        emission,
                        config,
                        root,
                    );
                    types.push(child);
                    getter = getter.join(acc.content_i(i));
                }
                let seq = format_ident!("Seq{}", types.len());
                map.record_seq(types.len());

                let pest_typed = pest_typed();
                let args = types.iter().map(
                |t| quote! {(#pest_typed::predefined_node::Skipped<#t, #root::generics::Skipped<S>, #skip>)},
            );
                process_single_alias(
                    map,
                    rule_config,
                    quote! { #root::#generics::#seq::<#(#args, )*> },
                    getter,
                    root,
                    emission,
                    explicit,
                )
            }
            Expr::Choice(_, _) => {
                let vec = walk!(expr, Choice);
                let mut types = Vec::<TokenStream>::with_capacity(vec.len());
                let mut getter = Getter::new();
                for (i, expr) in vec.into_iter().enumerate() {
                    let (child, acc) = Self::generate_graph_node(
                        expr,
                        rule_config,
                        map,
                        false,
                        emission,
                        config,
                        root,
                    );
                    types.push(child);
                    getter = getter.join(acc.choice(i));
                }
                let choice = format_ident!("Choice{}", types.len());
                map.record_choice(types.len());
                process_single_alias(
                    map,
                    rule_config,
                    quote! { #root::#generics::#choice::<#(#types, )*> },
                    getter,
                    root,
                    emission,
                    explicit,
                )
            }
            Expr::Opt(inner) => {
                let (inner_name, getters) = Self::generate_graph_node(
                    inner,
                    rule_config,
                    map,
                    false,
                    emission,
                    config,
                    root,
                );
                let getters = getters.optional();
                let option = option_type();
                process_single_alias(
                    map,
                    rule_config,
                    quote! {#option::<#inner_name>},
                    getters,
                    root,
                    emission,
                    explicit,
                )
            }
            Expr::Rep(inner) => {
                let (inner_name, getters) = Self::generate_graph_node(
                    inner,
                    rule_config,
                    map,
                    false,
                    emission,
                    config,
                    root,
                );
                process_single_alias(
                    map,
                    rule_config,
                    quote! { #root::#generics::Rep::<S, #skip, #inner_name> },
                    getters.contents(),
                    root,
                    emission,
                    explicit,
                )
            }
            Expr::RepExact(inner, times) => {
                let (inner_name, getters) = Self::generate_graph_node(
                    inner,
                    rule_config,
                    map,
                    false,
                    emission,
                    config,
                    root,
                );
                process_single_alias(
                    map,
                    rule_config,
                    quote! { #root::#generics::RepExact::<S, #skip, #inner_name, #times> },
                    getters.contents(),
                    root,
                    emission,
                    explicit,
                )
            }
            Expr::RepMin(inner, min) => {
                let (inner_name, getters) = Self::generate_graph_node(
                    inner,
                    rule_config,
                    map,
                    false,
                    emission,
                    config,
                    root,
                );
                process_single_alias(
                    map,
                    rule_config,
                    quote! { #root::#generics::RepMin::<S, #skip, #inner_name, #min> },
                    getters.contents(),
                    root,
                    emission,
                    explicit,
                )
            }
            Expr::RepMax(inner, max) => {
                let (inner_name, getters) = Self::generate_graph_node(
                    inner,
                    rule_config,
                    map,
                    false,
                    emission,
                    config,
                    root,
                );
                process_single_alias(
                    map,
                    rule_config,
                    quote! { #root::#generics::RepMax::<S, #skip, #inner_name, #max> },
                    getters.contents(),
                    root,
                    emission,
                    explicit,
                )
            }
            Expr::RepMinMax(inner, min, max) => {
                let (inner_name, getters) = Self::generate_graph_node(
                    inner,
                    rule_config,
                    map,
                    false,
                    emission,
                    config,
                    root,
                );
                process_single_alias(
                    map,
                    rule_config,
                    quote! { #root::#generics::RepMax::<S, #skip, #inner_name, #min, #max> },
                    getters.contents(),
                    root,
                    emission,
                    explicit,
                )
            }
            Expr::RepOnce(inner) => {
                let (inner_name, getters) = Self::generate_graph_node(
                    inner,
                    rule_config,
                    map,
                    false,
                    emission,
                    config,
                    root,
                );
                process_single_alias(
                    map,
                    rule_config,
                    quote! { #root::#generics::RepOnce::<S, #skip, #inner_name> },
                    getters.contents(),
                    root,
                    emission,
                    explicit,
                )
            }
            #[cfg(feature = "grammar-extras")]
            Expr::PushLiteral(literal) => {
                let wrapper = map.insert_string_wrapper(literal.as_str());
                process_single_alias(
                    map,
                    rule_config,
                    quote! {
                        #root::#generics::PushLiteral::<#root::#wrapper>
                    },
                    Getter::new(),
                    root,
                    emission,
                    explicit,
                )
            }
            #[cfg(feature = "grammar-extras")]
            Expr::NodeTag(inner_expr, tag) => {
                if config.emit_tagged_node_reference {
                    let tag_id = ident(tag.as_str());
                    let (inner, getter) = Self::generate_graph_node(
                        inner_expr,
                        rule_config,
                        map,
                        explicit,
                        emission,
                        config,
                        root,
                    );
                    map.insert_tag(&rule_config.rule_id, &tag_id, inner.clone(), getter.clone());
                    let new_getter =
                        Getter::from_tag(rule_config.rule_name, tag.as_str(), inner.clone());
                    if config.truncate_getter_at_node_tag {
                        (inner, new_getter)
                    } else {
                        (inner, new_getter.join(getter))
                    }
                } else {
                    let (inner, getter) = Self::generate_graph_node(
                        inner_expr,
                        rule_config,
                        map,
                        explicit,
                        emission,
                        config,
                        root,
                    );
                    process_single_alias(map, rule_config, inner, getter, root, emission, false)
                }
            }
        }
    }

    #[allow(private_interfaces)]
    fn generate_graph<'g: 'f, 'f>(
        rules: &'g [Self],
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
            Self::generate_graph_node(
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

    #[allow(private_interfaces)]
    fn collect_used_rule<'s>(rule: &'s Self, implicit: Implicit, res: &mut BTreeSet<&'s str>) {
        //
        if rule.ty() == RuleType::Normal {
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
                Expr::Str(_) | Expr::Insens(_) | Expr::Range(_, _) => (),
                Expr::Ident(rule_name) => {
                    res.insert(rule_name.as_str());
                }
                Expr::PeekSlice(_, _) => (),
                Expr::PosPred(expr) | Expr::NegPred(expr) => exprs.push(expr),
                Expr::Seq(lhs, rhs) | Expr::Choice(lhs, rhs) => {
                    exprs.push(lhs);
                    exprs.push(rhs);
                }
                Expr::Opt(expr)
                | Expr::Rep(expr)
                | Expr::RepExact(expr, _)
                | Expr::RepMin(expr, _)
                | Expr::RepMax(expr, _)
                | Expr::RepMinMax(expr, _, _)
                | Expr::RepOnce(expr) => exprs.push(expr),
                Expr::Skip(_) => (),
                Expr::Push(expr) => exprs.push(expr),
                #[cfg(feature = "grammar-extras")]
                Expr::PushLiteral(_) => (),
                #[cfg(feature = "grammar-extras")]
                Expr::NodeTag(expr, _) => exprs.push(expr),
            }
        }
    }
}
