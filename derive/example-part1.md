Note: to use **pest_typed_derive** as a dependency, **pest_typed** is also needed.

## Basic example

Here is a basic example on how to access and process those rules in a rule:

```rust
extern crate alloc;
use alloc::vec::Vec;
use core::{iter, result::Result};
use pest_typed_derive::TypedParser;
use pest_typed::{ParsableTypedNode as _, TypedParser as _, error::Error};

/// See https://datatracker.ietf.org/doc/html/rfc4180.html for CSV's format.
#[derive(TypedParser)]
#[grammar = "../tests/csv.pest"]
#[emit_rule_reference]
struct Parser;

fn main() -> Result<(), Error<Rule>> {
    let input = "name,age\nTom,10\nJerry,20";
    let file = pairs::file::parse(input)?;
    let (first_row, following_rows) = file.row();
    let rows = iter::once(first_row).chain(following_rows.into_iter());
    let columns = rows.map(
        |row| {
            let (first_column, following_columns) = row.item();
            let columns = iter::once(first_column).chain(following_columns.into_iter());
            let line = columns.map(|column| column.span.as_str()).collect::<Vec<_>>().join(",");
            line
        }
    );
    let columns = columns.collect::<Vec<_>>().join("\n");
    assert_eq!(columns, input);
    Ok(())
}
```
