use pest_typed::{ParsableTypedNode as _, Storage as _};
use pest_typed_derive::TypedParser;

#[derive(TypedParser)]
#[grammar_inline = r#"
a  = { "a" ~ (b1 | b2 | b3) ~ ^"c" }
b1 = { "bbb" }
b2 = { "cc" }
b3 = { "d" }
"#]
#[emit_rule_reference]
struct Parser;

fn parse(input: &str) -> Result<(), pest_typed::error::Error<Rule>> {
    let a = pairs::a::parse(input)?;
    let (str_a, var_b, c) = a.as_ref();
    assert_eq!(str_a.get_content(), "a");
    var_b
        .if_then(|b1| assert_eq!(b1.get_content(), "bbb"))
        .else_if(|b2| assert_eq!(b2.get_content(), "cc"))
        .else_then(|b3| assert_eq!(b3.get_content(), "d"));
    assert_eq!(c.get_content(), "c");
    assert!(c.content == "C" || c.content == "c");
    Ok(())
}

fn main() -> Result<(), pest_typed::error::Error<Rule>> {
    parse("abbbc")?;
    parse("abbbC")?;
    parse("accc")?;
    parse("accC")?;
    parse("adc")?;
    parse("adC")?;
    Ok(())
}
