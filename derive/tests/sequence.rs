// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use pest_typed::{ParsableTypedNode, Storage};
use pest_typed_derive::TypedParser;
use std::ops::Deref;

#[allow(dead_code)]
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
    ($name:ident, $input:literal, $($fields:tt)*) => {
        mod $name {
            use super::{pairs, Deref};
            use pest_typed::{ ParsableTypedNode, ConstantStorage};
            use anyhow::Error;

            #[test]
            fn matched() -> Result<(), Error> {
                let res = pairs::$name::try_parse($input)?;
                assert_eq!(res, res.clone());
                assert_eq!(res.content, res.content.clone());
                assert_eq!(&res.deref().0, res.get_all().0);
                assert_eq!(res.get_matched().0, &res.get_all().0.matched);
                let ( $($fields, )* ) = res.get_matched();
                assert_eq!([$($fields.get_constant(), )*].concat(), $input);
                let ( $($fields, )* ) = res.deref().clone().into_matched();
                assert_eq!([$($fields.get_constant(), )*].concat(), $input);
                Ok(())
            }
            #[test]
            fn unmatched() {
                pairs::$name::try_parse(concat!("_", $input)).unwrap_err();
            }
            #[test]
            fn incomplete() {
                pairs::$name::try_parse(concat!($input, "_")).unwrap_err();
            }
        }
    };
}

test!(s2, "ab", e0 e1);
test!(s3, "abc", e0 e1 e2);
test!(s4, "abcd", e0 e1 e2 e3);
test!(s5, "abcde", e0 e1 e2 e3 e4);
test!(s6, "abcdef", e0 e1 e2 e3 e4 e5);
test!(s7, "abcdefg", e0 e1 e2 e3 e4 e5 e6);
test!(s8, "abcdefgh", e0 e1 e2 e3 e4 e5 e6 e7);
test!(s9, "abcdefghi", e0 e1 e2 e3 e4 e5 e6 e7 e8);
test!(s10, "abcdefghij", e0 e1 e2 e3 e4 e5 e6 e7 e8 e9);
test!(s11, "abcdefghijk", e0 e1 e2 e3 e4 e5 e6 e7 e8 e9 e10);
test!(s12, "abcdefghijkl", e0 e1 e2 e3 e4 e5 e6 e7 e8 e9 e10 e11);

#[test]
fn as_ref() {
    let s4 = pairs::s4::try_parse("abcd").unwrap();
    let (a, b, c, d) = s4.get_matched();
    assert_eq!(a.get_content(), "a");
    assert_eq!(b.get_content(), "b");
    assert_eq!(c.get_content(), "c");
    assert_eq!(d.get_content(), "d");

    assert_eq!(&s4.deref().clone(), s4.deref());
    assert_eq!(
        format!("{:?}", s4.as_ref()),
        format!("({:?}, {:?}, {:?}, {:?})", a, b, c, d)
    );
}
