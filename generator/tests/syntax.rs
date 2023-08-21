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

/// Use a script to format generated codes if changed.
///
/// ```shell
/// rustfmt generator/tests/syntax-generated.txt
/// ```
#[test]
fn generated_rules() {
    let path_generated = "tests/syntax.generated.txt";
    let path_expected = if cfg!(feature = "grammar-extras") {
        "tests/syntax.extras.expected.txt"
    } else {
        "tests/syntax.expected.txt"
    };
    let actual = derive_typed_parser(
        quote! {
            #[grammar = "tests/syntax.pest"]
            #[emit_rule_reference]
            #[emit_tagged_node_reference]
            #[no_warnings]
            struct Parser;
        },
        false,
    );
    let actual = actual.to_string();
    std::fs::write(path_generated, &actual).unwrap();
    let output = std::process::Command::new("rustfmt")
        .arg(path_generated)
        .arg("--config")
        .arg("use_small_heuristics=Max,max_width=1000")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "STDOUT:\n{}\nSTDERR:\n{}",
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    );

    if std::fs::read(path_generated).unwrap() != std::fs::read(path_expected).unwrap() {
        panic!("Generated codes have changed.")
    }
}
