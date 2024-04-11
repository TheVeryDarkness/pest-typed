use anyhow::Error;
use pest_typed::{ParsableTypedNode as _, RuleStruct, Span, Storage};
use pest_typed_derive::{match_choices, TypedParser};

#[derive(TypedParser)]
#[grammar_inline = r#"
a =  { (b | c) ~ d }
b = _{ "b" ~ b? }
c = @{ "c" }
d = ${ "d" }
"#]
#[emit_rule_reference]
struct Parser;

fn parse(input: &'static str) -> Result<(), Error> {
    let a = pairs::a::try_parse(input)?;
    // With accesser API.
    // Call `b()` to get reference to `b`.
    // Call `c()` to get reference to `c`.
    // Call `d()` to get reference to `d`.
    if let Some(b) = a.b() {
        // `b` is a silent rule, it only contains inner expressions.
        // Its content may be wrapped in a Box when it's one of the nodes that is in a cycle with minimal length.
        // Then its size will always be the size of a Box.
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
        Choice2::_0(b) => assert_eq!(
            std::mem::size_of_val(b.ref_inner()),
            std::mem::size_of::<Box<rules::b>>()
        ),
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

fn main() -> Result<(), Error> {
    parse("bd")?;
    parse("cd")?;
    Ok(())
}
