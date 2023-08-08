// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use pest_typed::{error::Error, ParsableTypedNode, RuleStruct};
use pest_typed_derive::TypedParser;
use std::iter::once;

#[derive(TypedParser)]
#[grammar = "tests/csv.pest"]
#[emit_rule_reference]
struct CSV;

fn main() -> Result<(), Error<Rule>> {
    let file = pairs::file::parse(
        r#"1,2,3
a,b,c
A,B,C
"#,
    )?;
    let (first, following) = file.row();
    let rows = once(first).chain(following.into_iter());
    let cells: Vec<_> = rows
        .map(|row| {
            let (first, following) = row.item();
            let columns = once(first).chain(following.into_iter());
            let columns = columns.map(|item: &pairs::item<'_>| {
                item.if_then(|escaped| {
                    let (_, content, _) = escaped.as_ref();
                    content.span().as_str()
                })
                .else_then(|unescaped| unescaped.span().as_str())
            });
            let columns: Vec<_> = columns.collect();
            columns
        })
        .collect();
    assert_eq!(
        format!("{:?}", cells),
        r#"[["1", "2", "3"], ["a", "b", "c"], ["A", "B", "C"]]"#
    );
    Ok(())
}
