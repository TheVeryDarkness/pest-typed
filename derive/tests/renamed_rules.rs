// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Regression test for [issue #4](https://github.com/TheVeryDarkness/pest-typed/issues/4):
//! rule renaming via [`pest_typed::error::Error::renamed_rules`] should take effect.

use pest_typed::ParsableTypedNode;
use pest_typed_derive::TypedParser;

#[allow(dead_code)]
#[derive(TypedParser)]
#[grammar_inline = r#"
command = { SOI ~ word ~ EOI }
word = @{ ASCII_ALPHA+ }
"#]
#[emit_rule_reference]
struct Parser;

#[test]
fn renamed_rules() {
    let error = rules::command::try_parse("hello!").unwrap_err();
    let unrenamed = format!("{error}");
    let called = std::cell::Cell::new(false);
    let error = error.renamed_rules(|rule| {
        called.set(true);
        match *rule {
            Rule::command => "command line".to_owned(),
            Rule::word => "a word".to_owned(),
            Rule::EOI => "end of input".to_owned(),
        }
    });
    assert!(called.get(), "`renamed_rules` should invoke the closure");
    assert_eq!(
        unrenamed,
        " --> 1:6
  |
1 | hello!
  |      ^---
  |
  = expected EOI"
    );
    assert_eq!(
        format!("{error}"),
        " --> 1:6
  |
1 | hello!
  |      ^---
  |
  = expected end of input"
    );
}
