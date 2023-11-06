// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use pest_typed::{ParsableTypedNode, Storage};
use pest_typed_derive::{match_choices, TypedParser};

#[derive(TypedParser)]
#[grammar_inline = r#"
c1  = { ("a"){1} }
c2  = { ("a" | "b"){2} }
c3  = { ("a" | "b" | "c"){3} }
c4  = { ("a" | "b" | "c" | "d"){4} }
c5  = { ("a" | "b" | "c" | "d" | "e"){5} }
c6  = { ("a" | "b" | "c" | "d" | "e" | "f"){6} }
c7  = { ("a" | "b" | "c" | "d" | "e" | "f" | "g"){7} }
c8  = { ("a" | "b" | "c" | "d" | "e" | "f" | "g" | "h"){8} }
c9  = { ("a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i"){9} }
c10 = { ("a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j"){10} }
c11 = { ("a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k"){11} }
c12 = { ("a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l"){12} }
"#]
struct Parser;

macro_rules! matching {
    ($res:expr, $input:literal) => {};
    ($res:expr, $($input:literal)*) => {
        match_choices!{
            $res.get_matched().0 {
                $(
                    s => assert_eq!(s.get_content(), $input)
                )*
            }
        }
    };
}
macro_rules! test {
    ($name:ident, $($input:literal)*) => {
        mod $name {
            #[allow(unused_imports)]
            use super::{pairs, Rule, generics, match_choices};
            #[allow(unused_imports)]
            use pest_typed::{
                error::Error,
                iterators::{Pair, Pairs},
                ParsableTypedNode, Storage,
            };
            const INPUT : &'static str = concat!($($input,)*);
            #[test]
            fn success() -> Result<(), Error<Rule>> {
                let res = pairs::$name::try_parse(INPUT)?;
                let span = res.span;
                assert_eq!(span, res.iter_pairs().next().unwrap().span());
                assert_eq!(span, res.clone().into_iter_pairs().next().unwrap().span());
                assert!(res.inner().next().is_none());
                assert!(res.clone().into_inner().next().is_none());
                assert_eq!(res, res.clone());

                matching!(res, $($input)*);

                Ok(())
            }
            #[test]
            fn failed() {
                let mut buf = String::from(INPUT);
                buf.pop();
                pairs::$name::try_parse(buf.as_str()).unwrap_err();
            }
        }
    };
}
test!(c1, "a");
test!(c2, "a""b");
test!(c3, "a""b""c");
test!(c4, "a""b""c""d");
test!(c5, "a""b""c""d""e");
test!(c6, "a""b""c""d""e""f");
test!(c7, "a""b""c""d""e""f""g");
test!(c8, "a""b""c""d""e""f""g""h");
test!(c9, "a""b""c""d""e""f""g""h""i");
test!(c10, "a""b""c""d""e""f""g""h""i""j");
test!(c11, "a""b""c""d""e""f""g""h""i""j""k");
test!(c12, "a""b""c""d""e""f""g""h""i""j""k""l");

#[test]
fn choices() {
    let c4 = pairs::c4::try_parse("abcd").unwrap();
    let (_0, _1, _2, _3) = c4.as_ref();
    macro_rules! t {
        ($branch:ident) => {
            $branch
                .if_then(|_0| assert_eq!(_0.get_content(), "a"))
                .else_if(|_1| assert_eq!(_1.get_content(), "b"))
                .else_if(|_2| assert_eq!(_2.get_content(), "c"))
                .else_then(|_3| assert_eq!(_3.get_content(), "d"));
            $branch
                .reference()
                .else_if(|_0| assert_eq!(_0.get_content(), "a"))
                .else_if(|_1| assert_eq!(_1.get_content(), "b"))
                .else_if(|_2| assert_eq!(_2.get_content(), "c"))
                .else_then(|_3| assert_eq!(_3.get_content(), "d"));
            $branch
                .clone()
                .consume()
                .else_if(|_0| assert_eq!(_0.get_content(), "a"))
                .else_if(|_1| assert_eq!(_1.get_content(), "b"))
                .else_if(|_2| assert_eq!(_2.get_content(), "c"))
                .else_then(|_3| assert_eq!(_3.get_content(), "d"));
            $branch
                .clone()
                .consume_if_then(|_0| assert_eq!(_0.get_content(), "a"))
                .else_if(|_1| assert_eq!(_1.get_content(), "b"))
                .else_if(|_2| assert_eq!(_2.get_content(), "c"))
                .else_then(|_3| assert_eq!(_3.get_content(), "d"));
            $branch.$branch().unwrap();
            if "_0" == stringify!($branch) {
                assert!($branch._0().is_some());
            } else {
                assert!($branch._0().is_none());
            }
        };
    }
    t!(_0);
    t!(_1);
    t!(_2);
    t!(_3);
}
