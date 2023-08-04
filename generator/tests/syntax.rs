// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use pest_typed_generator::derive_typed_parser;
use quote::quote;

/// This test is mainly for compatibility.
#[test]
fn compatibility() {
    let stream = derive_typed_parser(
        quote! {
            #[grammar = "tests/syntax.pest"]
            #[emit_rule_reference]
            struct Parser;
        },
        false,
    );
    // std::fs::write("tests/syntax.txt", format!("{}", stream)).unwrap();
    let string = String::from_utf8(std::fs::read("tests/syntax.txt").unwrap()).unwrap();
    assert_eq!(stream.to_string(), string);
}
