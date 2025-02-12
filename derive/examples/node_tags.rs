use anyhow::Error;
use pest_typed::TypedParser;
use pest_typed_derive::TypedParser;

#[derive(TypedParser)]
#[grammar_inline = r#"
a  = { "a" ~ #b = (b1 | b2) }
b1 = { "bbb" }
b2 = { "cc" }
item = { "x" }
c  = { #a = item ~ ("," ~ #a = item)* }
"#]
#[emit_rule_reference]
#[emit_tagged_node_reference]
struct Parser;

fn main() -> Result<(), Error> {
    let a = Parser::try_parse::<pairs::a>("abbb")?;
    // Tags enabled.
    #[cfg(feature = "grammar-extras")]
    {
        // Access tag `b` with `b()`.
        let _b = a.b();
        // Tag `b` also has getter functions.

        // if let Some(b1) = b.b1() {
        //     assert_eq!(b1.span.as_str(), "bbb");
        // } else if let Some(b2) = b.b2() {
        //     assert_eq!(b2.span.as_str(), "cc");
        // }
    }
    // Tags disabled.
    #[cfg(not(feature = "grammar-extras"))]
    {
        // Tag `b` is transparent.
        if let Some(b1) = a.b1() {
            assert_eq!(b1.span.as_str(), "bbb");
        } else if let Some(b2) = a.b2() {
            assert_eq!(b2.span.as_str(), "cc");
        }
    }
    Ok(())
}
