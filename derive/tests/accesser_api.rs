extern crate alloc;
use pest_typed::{error::Error, ParsableTypedNode};
use pest_typed_derive::TypedParser;
use std::iter::once;

#[derive(TypedParser)]
#[grammar = "tests/csv.pest"]
#[emit_rule_reference]
struct CSV;

#[test]
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
                    let (_, escaped) = escaped.next();
                    let (content, _) = escaped.next();
                    content.span.as_str()
                })
                .else_then(|unescaped| unescaped.span.as_str())
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
