### Lifetime

Structs have fields that are contains references borrowed from the input, so each of them has a lifetime argument `'i`.

Sometimes, you may encounter a lifetime error. Do not panic, just consider them seriously.

```rust
use pest_typed_derive::TypedParser;
use pest_typed::{error::Error, ParsableTypedNode as _, RuleStruct as _};

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
        b1.span().as_str()
    } else if let Some(b2) = a.b2() {
        b2.span().as_str()
    } else {
        panic!("All branches failed in succeeded matching");
    };
    Ok(res)
}

fn main() -> Result<(), Error<Rule>> {
    let res = parse("abbb")?;
    println!("{}", res);
    Ok(())
}
```
