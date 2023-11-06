use pest_typed::{ParsableTypedNode as _, Span, Storage as _};
use pest_typed_derive::{match_choices, TypedParser};

#[derive(TypedParser)]
#[grammar_inline = r#"
a =  { (b | c) ~ d }
b = _{ "b" }
c = @{ "c" }
d = ${ "d" }
"#]
#[emit_rule_reference]
struct Parser;

fn parse(input: &'static str) -> Result<(), pest_typed::error::Error<Rule>> {
    let a = pairs::a::try_parse(input)?;
    // With accesser API.
    // Call `b()` to get reference to `b`.
    // Call `c()` to get reference to `c`.
    // Call `d()` to get reference to `d`.
    if let Some(b) = a.b() {
        // `b` is a silent rule, it only contains inner expressions.
        // And as its content is wrapped in a `Box`,
        // its size will always be the size of a Box.
        assert_eq!(std::mem::size_of_val(b), std::mem::size_of::<Box<usize>>());
    } else if let Some(c) = a.c() {
        assert_eq!(c.span.as_str(), "c");
    }
    let d = a.d();
    assert_eq!(d.span.as_str(), "d");

    // With structural API.
    use generics::Choice2;
    // Call `get_matched` to destruct the sequence.
    let (b_or_c, d) = a.get_matched();
    match b_or_c {
        Choice2::_0(b) => assert_eq!(std::mem::size_of_val(b.content.as_ref()), 0),
        Choice2::_1(c) => assert_eq!(std::mem::size_of_val(c), std::mem::size_of::<Span>()),
    }
    // Or match_choices from `pest_typed_derive`.
    // Note that if module `generics` is not in current scope,
    // you should import `generics` from somewhere.
    // This may be very easy to use, but may have a worse experience with IDE.
    match_choices!(b_or_c {
        b => println!("{b:?}"),
        c => println!("{c:?}"),
    });
    assert_eq!(d.content.get_content(), "d");

    Ok(())
}

fn main() -> Result<(), pest_typed::error::Error<Rule>> {
    parse("bd")?;
    parse("cd")?;
    Ok(())
}
