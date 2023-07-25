### Emitted Fields of Rule Structs

A rule structs is a struct that corresponds to a rule.

```rust
use pest_typed_derive::TypedParser;
use pest_typed::{ParsableTypedNode as _, Storage as _, TypedParser as _, error::Error};

#[derive(TypedParser)]
#[grammar_inline = r#"a = { (b | c) ~ d } b = _{ "b" } c = @{ "c" } d = ${ "d" }"#]
#[emit_rule_reference]
struct Parser;

fn parse(input: &'static str) -> Result<(), pest_typed::error::Error<Rule>> {
    let a = pairs::a::parse(input)?;
    if let Some(b) = a.b() {
        // `b` is a silent rule, it contains nothing.
        assert_eq!(std::mem::size_of_val(b), 0);
    } else if let Some(c) = a.c() {
        assert_eq!(c.span.as_str(), "c");
    }
    let d = a.d();
    assert_eq!(d.span.as_str(), "d");
    assert_eq!(d.content.get_content(), "d");
    Ok(())
}

fn main() -> Result<(), pest_typed::error::Error<Rule>> {
    parse("bd")?;
    parse("cd")?;
    Ok(())
}
```
