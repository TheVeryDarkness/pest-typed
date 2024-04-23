use anyhow::Error;
use pest_typed::ParsableTypedNode;
use pest_typed_derive::TypedParser;
use std::iter::once;

#[derive(TypedParser)]
#[grammar = "examples/csv.pest"]
#[emit_rule_reference]
struct Csv;

fn main() -> Result<(), Error> {
    let file = pairs::file::try_parse(
        r#"1,2,3
a,b,c
A,B,C
"#,
    )?;
    let (first, following) = file.row();
    let rows = once(first).chain(following);
    let cells: Vec<_> = rows
        .map(|row| {
            let (first, following) = row.item();
            let columns = once(first).chain(following);
            let columns = columns.map(|item: &pairs::item<'_>| {
                item.if_then(|escaped| {
                    let (_, content, _) = escaped.as_ref();
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
