use anyhow::Error;
use pest_typed::TypedParser;
use pest_typed_derive::TypedParser;

#[derive(TypedParser)]
#[grammar_inline = r#"
a  = { "a" ~ (b1 | b2) }
b1 = { "bbb" }
b2 = { "cc" }
"#]
#[emit_rule_reference]
struct Parser;

fn parse(input: &'_ str) -> Result<&'_ str, Error> {
    let a = Parser::try_parse::<pairs::a>(input)?;
    let res = if let Some(b1) = a.b1() {
        b1.span.as_str()
    } else if let Some(b2) = a.b2() {
        b2.span.as_str()
    } else {
        unreachable!("All branches failed in succeeded matching");
    };
    Ok(res)
}

fn main() -> Result<(), Error> {
    let res = parse("abbb")?;
    println!("{}", res);
    Ok(())
}
