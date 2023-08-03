// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use pest_typed::{error::Error, iterators::PairTree, ParsableTypedNode};
use pest_typed_derive::TypedParser;
use std::fmt::Write;

#[derive(TypedParser)]
#[grammar = "tests/csv.pest"]
#[emit_rule_reference]
struct CSV;

fn main() -> Result<(), Error<Rule>> {
    let res = pairs::file::parse("a,b,c\nd,e,f")?;
    let mut buf = String::new();
    res.iterate_pre_order(|pair, depth| {
        write!(&mut buf, "{}", "  ".repeat(depth)).unwrap();
        writeln!(&mut buf, "{:?} {:?}", pair.rule(), pair.span().as_str()).unwrap();
    });
    assert_eq!(
        buf,
        r#"file "a,b,c\nd,e,f"
  row "a,b,c"
    item "a"
      non_escaped_item "a"
        legal_ascii "a"
    comma ","
    item "b"
      non_escaped_item "b"
        legal_ascii "b"
    comma ","
    item "c"
      non_escaped_item "c"
        legal_ascii "c"
  row "d,e,f"
    item "d"
      non_escaped_item "d"
        legal_ascii "d"
    comma ","
    item "e"
      non_escaped_item "e"
        legal_ascii "e"
    comma ","
    item "f"
      non_escaped_item "f"
        legal_ascii "f"
"#
    );
    Ok(())
}
