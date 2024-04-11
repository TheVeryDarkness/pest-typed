use anyhow::Error;
use pest_typed::{ParsableTypedNode as _, Storage as _};
use pest_typed_derive::{match_choices, TypedParser};

#[derive(TypedParser)]
#[grammar_inline = r#"
a  = { "a" ~ (b1 | b2 | b3) ~ ^"c" }
b1 = { "bbb" }
b2 = { "cc" }
b3 = { "d" }
"#]
#[emit_rule_reference]
struct Parser;

fn parse(input: &str) -> Result<(), Error> {
    let a = pairs::a::try_parse(input)?;
    let (str_a, var_b, c) = a.as_ref();
    assert_eq!(str_a.get_content(), "a");
    match_choices!(var_b {
        b1 => assert_eq!(b1.get_content(), "bbb"),
        b2 => assert_eq!(b2.get_content(), "cc"),
        b3 => assert_eq!(b3.get_content(), "d"),
    });
    // Or equivalently codes below. Sometimes you may need to call `.deref()`.
    use generics::Choice3;
    match var_b {
        Choice3::_0(b1) => assert_eq!(b1.get_content(), "bbb"),
        Choice3::_1(b2) => assert_eq!(b2.get_content(), "cc"),
        Choice3::_2(b3) => assert_eq!(b3.get_content(), "d"),
    };
    // Or codes below. Note that rust compiler won't be aware
    // that only exactly one of those closures you pass will be called,
    // so sometimes compiler will prevent you from using this.
    // This method is no longer recommended and may be deprecated in the future.
    // However, at current this is the only way that you can place a type innotation after the identifier.
    var_b
        .if_then(|b1: &pairs::b1<'_>| assert_eq!(b1.get_content(), "bbb"))
        .else_if(|b2: &pairs::b2<'_>| assert_eq!(b2.get_content(), "cc"))
        .else_then(|b3: &pairs::b3<'_>| assert_eq!(b3.get_content(), "d"));

    assert_eq!(c.get_content(), "c");
    assert!(c.content == "C" || c.content == "c");
    Ok(())
}

fn main() -> Result<(), Error> {
    parse("abbbc")?;
    parse("abbbC")?;
    parse("accc")?;
    parse("accC")?;
    parse("adc")?;
    parse("adC")?;
    Ok(())
}
