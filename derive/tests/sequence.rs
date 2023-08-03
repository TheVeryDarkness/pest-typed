// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use pest_typed_derive::TypedParser;

#[derive(TypedParser)]
#[grammar_inline = r#"
s1  = { "a" }
s2  = { "a" ~ "b" }
s3  = { "a" ~ "b" ~ "c" }
s4  = { "a" ~ "b" ~ "c" ~ "d" }
s5  = { "a" ~ "b" ~ "c" ~ "d" ~ "e" }
s6  = { "a" ~ "b" ~ "c" ~ "d" ~ "e" ~ "f" }
s7  = { "a" ~ "b" ~ "c" ~ "d" ~ "e" ~ "f" ~ "g" }
s8  = { "a" ~ "b" ~ "c" ~ "d" ~ "e" ~ "f" ~ "g" ~ "h" }
s9  = { "a" ~ "b" ~ "c" ~ "d" ~ "e" ~ "f" ~ "g" ~ "h" ~ "i" }
s10 = { "a" ~ "b" ~ "c" ~ "d" ~ "e" ~ "f" ~ "g" ~ "h" ~ "i" ~ "j" }
s11 = { "a" ~ "b" ~ "c" ~ "d" ~ "e" ~ "f" ~ "g" ~ "h" ~ "i" ~ "j" ~ "k" }
s12 = { "a" ~ "b" ~ "c" ~ "d" ~ "e" ~ "f" ~ "g" ~ "h" ~ "i" ~ "j" ~ "k" ~ "l" }
"#]
struct Parser;

macro_rules! test {
    ($name:ident, $input:literal) => {
        mod $name {
            use super::{pairs, Rule};
            use pest_typed::{error::Error, ParsableTypedNode};

            #[test]
            fn matched() -> Result<(), Error<Rule>> {
                pairs::$name::parse($input)?;
                Ok(())
            }
            #[test]
            #[should_panic]
            fn unmatched() {
                pairs::$name::parse(concat!("_", $input)).unwrap();
            }
            #[test]
            #[should_panic]
            fn incomplete() {
                pairs::$name::parse(concat!($input, "_")).unwrap();
            }
        }
    };
}

test!(s1, "a");
test!(s2, "ab");
test!(s3, "abc");
test!(s4, "abcd");
test!(s5, "abcde");
test!(s6, "abcdef");
test!(s7, "abcdefg");
test!(s8, "abcdefgh");
test!(s9, "abcdefghi");
test!(s10, "abcdefghij");
test!(s11, "abcdefghijk");
test!(s12, "abcdefghijkl");
