// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Config {
    pub emit_rule_reference: bool,
    pub emit_tagged_node_reference: bool,
    pub do_not_emit_span: bool,
    pub pest_optimizer: bool,
    pub truncate_getter_at_node_tag: bool,
    pub simulate_pair_api: bool,
    pub box_only_if_needed: bool,
    pub no_warnings: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            emit_rule_reference: false,
            emit_tagged_node_reference: false,
            do_not_emit_span: false,
            pest_optimizer: true,
            truncate_getter_at_node_tag: true,
            simulate_pair_api: false,
            box_only_if_needed: false,
            no_warnings: false,
        }
    }
}
