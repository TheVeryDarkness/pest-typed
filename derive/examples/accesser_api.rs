extern crate alloc;
use alloc::vec::Vec;
use core::{iter, result::Result};
use pest_typed::{error::Error, ParsableTypedNode as _};
use pest_typed_derive::TypedParser;

/// See https://datatracker.ietf.org/doc/html/rfc4180.html for CSV's format.
#[derive(TypedParser)]
#[grammar = "examples/csv.pest"]
#[emit_rule_reference]
struct Parser;

fn main() -> Result<(), Error<Rule>> {
    // Prepare input. Output syntax will depend on this.
    let input = "name,age\nTom,10\nJerry,20";
    // Parser output. We may need some extra operations to make it into a table.
    let file = pairs::file::try_parse(input)?;
    // Separate file by row. As pest don't have separator yet, this may be very common.
    // As `file` has 2 references to `row` in total,
    // return value type will also be a tuple with 2 elements.
    // Hide other rules, we'll get `row ~ (_ ~ row)* ~ _?`.
    // Therefore the return value type will be `(&row, Vec<&row>)`.
    let (first_row, following_rows) = file.row();
    // Join rows as a single iterator.
    let rows = iter::once(first_row).chain(following_rows.into_iter());
    // Sheet.
    let table = rows
        .map(|row| {
            // Separate each row by column.
            // Hide other rules, we'll get `item ~ (_ ~ item)*`.
            // Therefore the return value type will be `(&item, Vec<&item>)`.
            let (first_item, following_items) = row.item();
            // Join columns as a single iterator.
            let items = iter::once(first_item).chain(following_items.into_iter());
            // Extract string from each cell.
            let row = items.map(|cell| cell.span.as_str()).collect::<Vec<_>>();
            row
        })
        .collect::<Vec<_>>();
    // Recover input from sheet.
    let recovered = table
        .into_iter()
        .map(|row| row.join(","))
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(recovered, input);
    Ok(())
}
