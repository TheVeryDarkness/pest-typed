// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[derive(Clone, Copy, Debug)]
pub(crate) struct Config {
    pub emit_rule_reference: bool,
    pub emit_tagged_node_reference: bool,
    pub do_not_emit_span: bool,
    pub truncate_accesser_at_node_tag: bool,
    pub simulate_pair_api: bool,
    pub no_warnigs: bool,
}
impl Config {
    #[allow(dead_code)]
    /// Enable as more as possible features.
    pub fn all() -> Self {
        Self {
            emit_rule_reference: true,
            emit_tagged_node_reference: true,
            do_not_emit_span: false,
            truncate_accesser_at_node_tag: true,
            simulate_pair_api: true,
            no_warnigs: false,
        }
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            emit_rule_reference: false,
            emit_tagged_node_reference: false,
            do_not_emit_span: false,
            truncate_accesser_at_node_tag: true,
            simulate_pair_api: false,
            no_warnigs: false,
        }
    }
}
