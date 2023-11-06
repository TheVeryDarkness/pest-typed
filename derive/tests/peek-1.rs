// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use pest_typed::ParsableTypedNode;
use pest_typed_derive::TypedParser;

#[derive(TypedParser)]
#[grammar_inline = r#"
main = { PUSH("foo") ~ PUSH("Foo") ~ PUSH("FOO") ~ PEEK[1..] ~ PEEK[1..2] }
"#]
struct Parser;

#[test]
fn tree() {
    rules::main::try_parse("fooFooFOOFooFOOFoo").unwrap();
    let a = rules::main::try_parse("fooFooFOOFOOFooFoo").unwrap_err();
    assert_eq!(
        format!("{a}"),
        " --> 1:1
  |
1 | fooFooFOOFOOFooFoo
  | ^---
  |
  = ^---
    Expected [main]."
    );
    let b = rules::main::try_parse("foo Foo FOO Foo FOO Foo").unwrap_err();
    assert_eq!(
        format!("{b}"),
        " --> 1:1
  |
1 | foo Foo FOO Foo FOO Foo
  | ^---
  |
  = ^---
    Expected [main]."
    );
}
