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
use std::io::Write;

fn generate(name: &'static str) {
    let path_generated = format!("tests/{name}-generated.rs");
    let path_expected = if cfg!(feature = "grammar-extras") {
        format!("tests/{name}-expected-extras.rs")
    } else {
        format!("tests/{name}-expected.rs")
    };
    let feature = if cfg!(feature = "grammar-extras") {
        "#![cfg(feature = \"grammar-extras\")]\n"
    } else {
        "#![cfg(not(feature = \"grammar-extras\"))]\n"
    };
    let grammar = format!("tests/{name}.pest");
    let actual = derive_typed_parser(
        quote! {
            #[grammar = #grammar]
            #[emit_rule_reference]
            #[emit_tagged_node_reference]
            #[no_warnings]
            struct Parser;
        },
        false,
        false,
    );
    let actual = actual.to_string();
    let mut f = std::fs::File::create(&path_generated).unwrap();
    writeln!(f, "{}", feature).unwrap();
    writeln!(f, "{}", "#![allow(unused_parens)]").unwrap();
    writeln!(f, "{}", actual).unwrap();
    drop(f);
    let output = std::process::Command::new("cargo")
        .arg("fmt")
        .arg("--all")
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

#[test]
fn syntax() {
    generate("syntax")
}

#[test]
fn grammar() {
    generate("grammar")
}
