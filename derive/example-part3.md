### Emitted Fields of Tag Structs

An example using node tags.

```rust
use pest_typed_derive::TypedParser;
use pest_typed::ParsableTypedNode as _;

#[derive(TypedParser)]
#[grammar_inline = r#"a = { "a" ~ #b = (b1 | b2) } b1 = { "bbb" } b2 = { "cc" } "#]
#[emit_rule_reference]
#[emit_tagged_node_reference]
struct Parser;

fn main() -> Result<(), pest_typed::error::Error<Rule>> {
    let a = pairs::a::parse("abbb")?;
    let b = a.b();
    if let Some(b1) = b.b1() {
        assert_eq!(b1.span.as_str(), "bbb");
    } else if let Some(b2) = b.b2() {
        assert_eq!(b2.span.as_str(), "cc");
    }
    Ok(())
}
```

An example using nested node tags.

```rust
use pest_typed_derive::TypedParser;
use pest_typed::ParsableTypedNode as _;

#[derive(TypedParser)]
#[grammar_inline = r#"a = { "a" ~ #b = (b1 ~ #c = (b2 ~ b3)) } b1 = { "b" } b2 = { "bb" } b3 = { "bbb" }"#]
#[emit_rule_reference]
#[emit_tagged_node_reference]
struct Parser;

fn main() -> Result<(), pest_typed::error::Error<Rule>> {
    let a = pairs::a::parse("abbbbbb")?;
    let b = a.b();
    assert_eq!(b.span.as_str(), "bbbbbb");
    let b1 = b.b1();
    assert_eq!(b1.span.as_str(), "b");
    let c = b.c();
    assert_eq!(c.span.as_str(), "bbbbb");
    let b2 = c.b2();
    assert_eq!(c.span.as_str(), "bb");
    let b3 = c.b3();
    assert_eq!(c.span.as_str(), "bbb");
    Ok(())
}
```
