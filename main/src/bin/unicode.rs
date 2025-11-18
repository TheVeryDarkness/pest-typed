use pest::unicode::unicode_property_names;
use std::{fs::File, io::Write, path::PathBuf};

fn main() {
    let mut output = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    output.push("src/predefined_node/unicode.rs");
    let mut out = File::create(output).unwrap();
    out.write_all(
        r#"//! Wrapped types for unicode property. See [pest::unicode] for details.

use crate::{
    iterators::{Pairs, Token},
    tracker::Tracker,
    Cursor, RuleType, Span, Stack, TypedNode,
};
use core::fmt;

macro_rules! unicode {
    ($property_ident:ident) => {
        #[allow(non_camel_case_types)]
        #[doc = concat!("Auto generated. Unicode property ", stringify!($property_ident))]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Clone, Hash, PartialEq, Eq)]
        pub struct $property_ident {
            /// Matched character.
            ///
            /// Do not trust this field as it may be assigned to after creation.
            pub content: char,
        }
        impl From<char> for $property_ident {
            fn from(content: char) -> Self {
                Self { content }
            }
        }
        impl<C: Cursor, R: RuleType> TypedNode<C, R> for $property_ident {
            #[inline]
            fn try_parse_partial_with(
                mut input: C,
                _stack: &mut Stack<Span<C::String>>,
                _tracker: &mut Tracker<C::String, R>,
            ) -> Option<(C, Self)> {
                match super::match_char_by(&mut input, pest::unicode::$property_ident) {
                    Some(content) => Some((input, Self::from(content))),
                    None => None,
                }
            }
            #[inline]
            fn try_check_partial_with(
                mut input: C,
                _stack: &mut Stack<Span<C::String>>,
                _tracker: &mut Tracker<C::String, R>,
            ) -> Option<C> {
                match super::match_char_by(&mut input, pest::unicode::$property_ident) {
                    Some(_) => Some(input),
                    None => None,
                }
            }
        }
        impl fmt::Debug for $property_ident {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($property_ident))
                    .field("content", &self.content)
                    .finish()
            }
        }
        impl<S, R: RuleType> Pairs<S, R> for $property_ident {
            fn for_self_or_each_child(&self, _f: &mut impl FnMut(Token<S, R>)) {}
        }
    };
}
"#
        .as_bytes(),
    )
    .unwrap();
    for unicode in unicode_property_names() {
        writeln!(out, "unicode!({});", unicode).unwrap()
    }
}
