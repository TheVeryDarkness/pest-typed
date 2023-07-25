### Emitted Fields and Functions of Nodes

We can handle complexer problems with lower-level API.

|            Node Type            |                                     Fields                                     |                                 Functions                                 |
| :-----------------------------: | :----------------------------------------------------------------------------: | :-----------------------------------------------------------------------: |
|         Non-silent rule         |     Matched `content`, which can be used to access nodes mentioned below.      |                     See [Accesser API](#accesser-api)                     |
|  Exact string (case-sensitive)  |                                                                                | `const fn get_content(&self)`, which requires trait `pest_typed::Storage` |
| Exact string (case-insensitive) |                        Matched `content` (an `&'i str`)                        |                                                                           |
|            Sequence             | `first` and `second` (for multi-elements sequence, this is like a linked list) |             `next(&self)`, which returns `(&first, &second)`              |
|             Choices             |                        Two variant `First` and `Second`                        |                             `if_then(&self)`                              |
|            Optional             |                   Matched ` content` wrapped in a [`Option`]                   |

#### Sequence

One can use `next(&self)` of the `Seq` to access the first element of it.

```rust
use pest_typed_derive::TypedParser;
use pest_typed::{error::Error, ParsableTypedNode as _, Storage as _};

#[derive(TypedParser)]
#[grammar_inline = r#"
a = { "a" ~ (b1 | b2) ~ ^"c" }
b1 = { "bbb" }
b2 = { "cc" }
b3 = { "d" }
"#]
#[emit_rule_reference]
struct Parser;

fn parse(input: &str) -> Result<(), pest_typed::error::Error<Rule>> {
    let a = pairs::a::parse(input);
    let (str_a, a) = a.content.next();
    assert_eq!(str_a.get_content(), "a");
    let (var_b, c) = a.next();
    var_b
        .if_then(|b1| println!("b1: {}", b1))
        .else_if(|b2| println!("b2: {}", b2))
        .else(|b3| println!("b3: {}", b3));
    println!("c: {}", c);
    println!();
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
```
