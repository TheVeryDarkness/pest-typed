// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Copied from **pest/generator/src/lib.rs** (commit ac0aed3eecf435fd93ba575a39704aaa88a375b7)
//! and modified.

use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use proc_macro2::TokenStream;
use syn::{Attribute, DeriveInput, Expr, ExprLit, Generics, Ident, Lit, Meta};

use pest_meta::parser::{self, rename_meta_rule, Rule};
use pest_meta::{optimizer, unwrap_or_report, validator};

pub(crate) fn collect_data(contents: Vec<GrammarSource>) -> (String, Vec<PathBuf>) {
    let mut data = String::new();
    let mut paths = vec![];

    for content in contents {
        let (_data, _path) = match content {
            GrammarSource::File(ref path) => {
                let root = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());

                // Check whether we can find a file at the path relative to the CARGO_MANIFEST_DIR
                // first.
                //
                // If we cannot find the expected file over there, fallback to the
                // `CARGO_MANIFEST_DIR/src`, which is the old default and kept for convenience
                // reasons.
                // TODO: This could be refactored once `std::path::absolute()` get's stabilized.
                // https://doc.rust-lang.org/std/path/fn.absolute.html
                let path = if Path::new(&root).join(path).exists() {
                    Path::new(&root).join(path)
                } else {
                    Path::new(&root).join("src/").join(path)
                };

                let file_name = match path.file_name() {
                    Some(file_name) => file_name,
                    None => panic!("grammar attribute should point to a file"),
                };

                let data = match read_file(&path) {
                    Ok(data) => data,
                    Err(error) => panic!("error opening {:?}: {}", file_name, error),
                };
                (data, Some(path.clone()))
            }
            GrammarSource::Inline(content) => (content, None),
        };

        data.push_str(&_data);
        if let Some(path) = _path {
            paths.push(path);
        }
    }

    (data, paths)
}

#[derive(Debug, PartialEq)]
pub(crate) enum GrammarSource {
    File(String),
    Inline(String),
}

pub(crate) fn get_attribute(attr: &Attribute) -> GrammarSource {
    match &attr.meta {
        Meta::NameValue(name_value) => match &name_value.value {
            Expr::Lit(ExprLit {
                lit: Lit::Str(string),
                ..
            }) => {
                if name_value.path.is_ident("grammar") {
                    GrammarSource::File(string.value())
                } else {
                    GrammarSource::Inline(string.value())
                }
            }
            _ => panic!("grammar attribute must be a string"),
        },
        _ => panic!("grammar attribute must be of the form `grammar = \"...\"`"),
    }
}

fn read_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path.as_ref())?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}
