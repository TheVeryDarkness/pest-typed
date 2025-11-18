use super::{rule, rules_mod, Emission, Getter, Implicit, Output, RuleConfig};
use crate::{config::Config, docs::DocComment};
use pest_meta::ast::RuleType;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeSet;

/// Returns (type name, getter).
pub(super) fn process_single_alias<'g>(
    map: &mut Output<'g>,
    rule_config: &RuleConfig<'g>,
    type_name: TokenStream,
    getters: Getter<'g>,
    root: &TokenStream,
    emission: Emission,
    explicit: bool,
) -> (TokenStream, Getter<'g>) {
    if explicit {
        let rule_id = &rule_config.rule_id;
        let def = rule(rule_config, type_name, &getters, emission);
        map.insert(def, rule_config.clone());
        let rules = rules_mod();
        (quote! {#root::#rules::#rule_id::<S>}, getters)
    } else {
        (type_name, getters)
    }
}

pub(crate) trait Generate: Sized {
    type Expr;

    /// Returns rule name.
    fn name(&self) -> &str;

    fn ty(&self) -> RuleType;

    /// Returns (type name, getter).
    #[allow(private_interfaces)]
    fn generate_graph_node<'g>(
        expr: &'g Self::Expr,
        rule_config: &RuleConfig<'g>,
        // From node name to type definition and implementation
        map: &mut Output<'g>,
        explicit: bool,
        emission: Emission,
        config: Config,
        root: &TokenStream,
    ) -> (TokenStream, Getter<'g>);

    #[allow(private_interfaces)]
    fn generate_graph<'g: 'f, 'f>(
        rules: &'g [Self],
        defined: &'g BTreeSet<&'g str>,
        not_boxed: &'f BTreeSet<&'g str>,
        builtins_without_lifetime: &'g BTreeSet<&'g str>,
        config: Config,
        doc: &'g DocComment,
    ) -> Output<'g>;

    #[allow(private_interfaces)]
    fn collect_used_rule<'s>(rule: &'s Self, implicit: Implicit, res: &mut BTreeSet<&'s str>);
}
