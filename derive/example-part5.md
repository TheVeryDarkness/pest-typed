### Emitted Fields and Functions of Nodes

We can handle complexer problems with lower-level API (also named **Structual API**).

But note that the structure of a **Rule Struct** depends on the optimizer in **pest**, so it may change in the future.

Maybe we can use [`pest_meta::ast::Expr`](https://docs.rs/pest_meta/latest/pest_meta/ast/enum.Expr.html) by default in the future.

|            Node Type            |                                      Fields                                      |                                              Functions                                               |
| :-----------------------------: | :------------------------------------------------------------------------------: | :--------------------------------------------------------------------------------------------------: |
|         Non-silent rule         | Matched `content`, which can be used to access match expression; matched `span`. |                                  See [Accesser API](#accesser-api)                                   |
|  Exact string (case-sensitive)  |                                                                                  | Original string to match, `const fn get_content(&self)`, which requires trait `pest_typed::Storage`. |
| Exact string (case-insensitive) |                        Matched `content` (an `&'i str`).                         | Original string to match, `const fn get_content(&self)`, which requires trait `pest_typed::Storage`. |
|      Sequence `T, Res...`       |           `first` and `second` (not recommended to be used directly).            |                           `next(&self)`, which returns `(&first, &second)`                           |
|       Choices `T, Res...`       |     Two variant `First` and `Second` (not recommended to be used directly).      |                                           `if_then(&self)`                                           |
|            Optional             |                    Matched `content` wrapped in a [`Option`].                    |
|        Repetition of `T`        |                         Matched `content` (an `Vec<T>`).                         |

For multi-elements sequence and multi-branches choices, its underlying implementation is like a list in functional programing. Those fields or variants are not so easy to read and use, and it's recommended to use function API.

#### Sequence

One can use `next(&self)` of the `Seq` to access the first element and remained part of it.

Using `next`, one can iterate those elements one by one in order.

#### Choices

We provide several functions that simulate control structure like `if` (`if_then(f)`), `else-if` (`else_if(f)`) and `else` (`else_then(f)`).

Each of those functions accept a function `f` as argument, if and only if the branch is the actual case, `f` is called.

The structure must start with `if_then(f)`. And `else_if` is only available when there are at least two cases that haven't been handled, so if it's the last case, use `else_then(f)` instead.

Except that `else_then(f)` returns the final result, `if_then(f)` and `else_if(f)` will return a temporary helper object.

Using these functions, one can handle those cases one by one in order.

#### Example

```rust
use core::ops::Deref;
use pest_typed_derive::TypedParser;
use pest_typed::{error::Error, ParsableTypedNode as _, Storage as _};

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
    let (str_a, a) = a.content.next();
    assert_eq!(str_a.get_content(), "a");
    let (var_b, c) = a.next();
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
```
