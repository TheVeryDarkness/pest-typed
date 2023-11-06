use pest_typed::{error::Error, ParsableTypedNode as _};
use pest_typed_derive::TypedParser;

#[derive(TypedParser)]
#[grammar_inline = r#"
a  = { "a" ~ (b1 | b2) }
b1 = { "bbb" }
b2 = { "cc" }
"#]
#[emit_rule_reference]
struct Parser;

fn parse<'i>(input: &'i str) -> Result<&'i str, Error<Rule>> {
    let a = pairs::a::parse(input)?;
    let res = if let Some(b1) = a.b1() {
        b1.span.as_str()
    } else if let Some(b2) = a.b2() {
        b2.span.as_str()
    } else {
        unreachable!("All branches failed in succeeded matching");
    };
    Ok(res)
}

fn main() -> Result<(), Error<Rule>> {
    let res = parse("abbb")?;
    println!("{}", res);
    Ok(())
}
