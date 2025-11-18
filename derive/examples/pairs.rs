use anyhow::Result;
use indoc::indoc;
use pest_typed::{iterators::PairTree, TypedParser};
use pest_typed_derive::TypedParser;

#[derive(TypedParser)]
#[grammar = "examples/csv.pest"]
struct Csv;

fn main() -> Result<()> {
    let res = Csv::try_parse::<&str, pairs::file<&str>>("a,b,c\nd,e,f")?;
    let mut buf = String::new();
    res.write_tree_to(&mut buf)?;
    assert_eq!(
        buf,
        indoc! {r#"
            file "a,b,c\nd,e,f"
                row "a,b,c"
                    item "a"
                        non_escaped_item "a"
                            legal_ascii "a"
                    comma ","
                    item "b"
                        non_escaped_item "b"
                            legal_ascii "b"
                    comma ","
                    item "c"
                        non_escaped_item "c"
                            legal_ascii "c"
                row "d,e,f"
                    item "d"
                        non_escaped_item "d"
                            legal_ascii "d"
                    comma ","
                    item "e"
                        non_escaped_item "e"
                            legal_ascii "e"
                    comma ","
                    item "f"
                        non_escaped_item "f"
                            legal_ascii "f"
        "#}
    );
    Ok(())
}
