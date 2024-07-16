// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::{collections::HashSet, ops::DerefMut};

use pest_typed::{predefined_node::RepMin, ParsableTypedNode};
use pest_typed_derive::TypedParser;

#[allow(dead_code)]
#[derive(TypedParser)]
#[grammar_inline = r#"
main = { PUSH(^"foo")* ~ " " ~ PEEK[1..] }
"#]
struct Parser;

#[test]
fn tree() {
    let mut main = HashSet::new();
    let mut strings = HashSet::new();
    for input in ["foo ", "foOFoo Foo", "fOoFOoFoO FOoFoO"] {
        let s = rules::main::try_parse(input).unwrap();
        main.insert(s.clone());
        for string in s.get_matched().0.iter_matched() {
            strings.insert(string.content.clone());
        }
        for string in s.get_matched().0.iter_all() {
            strings.insert(string.matched.content.clone());
        }
        for string in s.get_matched().0.clone().into_iter_all() {
            strings.insert(string.matched.content.clone());
        }
    }
    assert_eq!(main.len(), 3);
    assert_eq!(strings.len(), 6);

    let mut x = rules::main::try_parse("FOO ").unwrap();
    x.deref_mut().0.matched.content.clear();
    assert_eq!(x.get_matched().0, &RepMin::default());

    let error = rules::main::try_parse(" ").unwrap_err();
    assert_eq!(
        format!("{error}"),
        " --> 1:2
  |
1 |  
  |  ^---
  |
  =  ^---
    Unknown error (no rule tracked), by main.
    Peek slice 1.. out of bound. (By main)"
    );
}
