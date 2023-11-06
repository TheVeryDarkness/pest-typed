use pest_typed::ParsableTypedNode as _;
use pest_typed_derive::TypedParser;

#[derive(TypedParser)]
#[grammar_inline = r#"a = { "a" ~ #b = (b1 ~ #c = (b2 ~ b3)) } b1 = { "b" } b2 = { "bb" } b3 = { "bbb" }"#]
#[emit_rule_reference]
#[emit_tagged_node_reference]
struct Parser;

fn main() -> Result<(), pest_typed::error::Error<Rule>> {
    let a = pairs::a::try_parse("abbbbbb")?;
    #[cfg(feature = "grammar-extras")]
    {
        // With node tags, one can access inner nodes more precisely without defining many rules.
        // This maybe especially useful when you have some references the same rule.
        let b = a.b();
        assert_eq!(b.span.as_str(), "bbbbbb");
        let b1 = b.b1();
        assert_eq!(b1.span.as_str(), "b");
        let c = b.c();
        assert_eq!(c.span.as_str(), "bbbbb");
        let b2 = c.b2();
        assert_eq!(b2.span.as_str(), "bb");
        let b3 = c.b3();
        assert_eq!(b3.span.as_str(), "bbb");
    }
    #[cfg(not(feature = "grammar-extras"))]
    {
        let b1 = a.b1();
        assert_eq!(b1.span.as_str(), "b");
        let b2 = a.b2();
        assert_eq!(b2.span.as_str(), "bb");
        let b3 = a.b3();
        assert_eq!(b3.span.as_str(), "bbb");
    }
    Ok(())
}
