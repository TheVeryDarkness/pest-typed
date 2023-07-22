## Extra examples

### Auto-skipped rules

When a rule is not atomic, inner contents that match `COMMENT` or `WHITESPACE` will be skipped automatically.

### Generation of inner content

There are three cases related to code generation:

- Emit inner nodes and a span (normal rule, non-atomic rule and compound atomic rule in **pest**).
- Emit a whole span (atomic rule in **pest**).
- Emit nothing (silent rule in **pest**). Just parse and drop.

### Node tags

An example using node tags.

```rust
use pest_typed_derive::TypedParser;
use pest_typed::{ParsableTypedNode as _, TypedParser as _, error::Error};

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
