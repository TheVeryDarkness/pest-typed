#![cfg(not(feature = "grammar-extras"))]
#![allow(unused_parens)]
#[doc = ""]
#[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    r#string,
    r#insensitive,
    r#range,
    r#ident,
    r#pos_pred,
    r#neg_pred,
    r#double_neg_pred,
    r#sequence,
    r#sequence_compound,
    r#sequence_atomic,
    r#sequence_non_atomic,
    r#sequence_atomic_compound,
    r#sequence_nested,
    r#sequence_compound_nested,
    r#node_tag,
    r#choice,
    r#choice_prefix,
    r#optional,
    r#repeat,
    r#repeat_atomic,
    r#repeat_once,
    r#repeat_once_atomic,
    r#repeat_min_max,
    r#repeat_min_max_atomic,
    r#repeat_exact,
    r#repeat_min,
    r#repeat_min_atomic,
    r#repeat_max,
    r#repeat_max_atomic,
    r#soi_at_start,
    r#repeat_mutate_stack,
    r#repeat_mutate_stack_pop_all,
    r#will_fail,
    r#stack_resume_after_fail,
    r#peek_,
    r#peek_all,
    r#peek_slice_23,
    r#pop_,
    r#pop_all,
    r#pop_fail,
    r#checkpoint_restore,
    r#ascii_digits,
    r#ascii_nonzero_digits,
    r#ascii_bin_digits,
    r#ascii_oct_digits,
    r#ascii_hex_digits,
    r#ascii_alpha_lowers,
    r#ascii_alpha_uppers,
    r#ascii_alphas,
    r#ascii_alphanumerics,
    r#asciis,
    r#newline,
    r#unicode,
    r#SYMBOL,
    r#han,
    r#hangul,
    r#hiragana,
    r#arabic,
    r#emoji,
    r#WHITESPACE,
    r#COMMENT,
}
#[doc = "Unicode rules."]
pub mod unicode {
    #[allow(non_camel_case_types)]
    #[doc = "Auto generated. Unicode property EMOJI."]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct EMOJI {
        pub content: ::core::primitive::char,
    }
    impl ::core::convert::From<::core::primitive::char> for EMOJI {
        fn from(content: ::core::primitive::char) -> Self {
            Self { content }
        }
    }
    impl<'i> ::pest_typed::TypedNode<'i, super::Rule> for EMOJI {
        #[inline]
        fn try_parse_with(
            mut input: ::pest_typed::Position<'i>,
            _stack: &mut ::pest_typed::Stack<::pest_typed::Span<'i>>,
            _tracker: &mut ::pest_typed::tracker::Tracker<'i, super::Rule>,
        ) -> ::pest_typed::re_exported::Result<(::pest_typed::Position<'i>, Self), ()> {
            match ::pest_typed::predefined_node::match_char_by(
                &mut input,
                ::pest_typed::unicode::EMOJI,
            ) {
                Some(content) => Ok((input, Self::from(content))),
                None => Err(()),
            }
        }
    }
    impl ::core::fmt::Debug for EMOJI {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("EMOJI")
                .field("content", &self.content)
                .finish()
        }
    }
    impl<'i: 'n, 'n> ::pest_typed::iterators::Pairs<'i, 'n, super::Rule> for EMOJI {
        type Iter = ::core::iter::Empty<&'n dyn::pest_typed::iterators::Pair<'i, 'n, super::Rule>>;
        type IntoIter = ::core::iter::Empty<
            ::pest_typed::re_exported::Box<
                dyn::pest_typed::iterators::Pair<'i, 'n, super::Rule> + 'n,
            >,
        >;
        fn iter_pairs(&'n self) -> Self::Iter {
            ::core::iter::empty()
        }
        fn into_iter_pairs(self) -> Self::IntoIter {
            ::core::iter::empty()
        }
    }
    #[allow(non_camel_case_types)]
    #[doc = "Auto generated. Unicode property XID_CONTINUE."]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct XID_CONTINUE {
        pub content: ::core::primitive::char,
    }
    impl ::core::convert::From<::core::primitive::char> for XID_CONTINUE {
        fn from(content: ::core::primitive::char) -> Self {
            Self { content }
        }
    }
    impl<'i> ::pest_typed::TypedNode<'i, super::Rule> for XID_CONTINUE {
        #[inline]
        fn try_parse_with(
            mut input: ::pest_typed::Position<'i>,
            _stack: &mut ::pest_typed::Stack<::pest_typed::Span<'i>>,
            _tracker: &mut ::pest_typed::tracker::Tracker<'i, super::Rule>,
        ) -> ::pest_typed::re_exported::Result<(::pest_typed::Position<'i>, Self), ()> {
            match ::pest_typed::predefined_node::match_char_by(
                &mut input,
                ::pest_typed::unicode::XID_CONTINUE,
            ) {
                Some(content) => Ok((input, Self::from(content))),
                None => Err(()),
            }
        }
    }
    impl ::core::fmt::Debug for XID_CONTINUE {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("XID_CONTINUE")
                .field("content", &self.content)
                .finish()
        }
    }
    impl<'i: 'n, 'n> ::pest_typed::iterators::Pairs<'i, 'n, super::Rule> for XID_CONTINUE {
        type Iter = ::core::iter::Empty<&'n dyn::pest_typed::iterators::Pair<'i, 'n, super::Rule>>;
        type IntoIter = ::core::iter::Empty<
            ::pest_typed::re_exported::Box<
                dyn::pest_typed::iterators::Pair<'i, 'n, super::Rule> + 'n,
            >,
        >;
        fn iter_pairs(&'n self) -> Self::Iter {
            ::core::iter::empty()
        }
        fn into_iter_pairs(self) -> Self::IntoIter {
            ::core::iter::empty()
        }
    }
    #[allow(non_camel_case_types)]
    #[doc = "Auto generated. Unicode property XID_START."]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct XID_START {
        pub content: ::core::primitive::char,
    }
    impl ::core::convert::From<::core::primitive::char> for XID_START {
        fn from(content: ::core::primitive::char) -> Self {
            Self { content }
        }
    }
    impl<'i> ::pest_typed::TypedNode<'i, super::Rule> for XID_START {
        #[inline]
        fn try_parse_with(
            mut input: ::pest_typed::Position<'i>,
            _stack: &mut ::pest_typed::Stack<::pest_typed::Span<'i>>,
            _tracker: &mut ::pest_typed::tracker::Tracker<'i, super::Rule>,
        ) -> ::pest_typed::re_exported::Result<(::pest_typed::Position<'i>, Self), ()> {
            match ::pest_typed::predefined_node::match_char_by(
                &mut input,
                ::pest_typed::unicode::XID_START,
            ) {
                Some(content) => Ok((input, Self::from(content))),
                None => Err(()),
            }
        }
    }
    impl ::core::fmt::Debug for XID_START {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("XID_START")
                .field("content", &self.content)
                .finish()
        }
    }
    impl<'i: 'n, 'n> ::pest_typed::iterators::Pairs<'i, 'n, super::Rule> for XID_START {
        type Iter = ::core::iter::Empty<&'n dyn::pest_typed::iterators::Pair<'i, 'n, super::Rule>>;
        type IntoIter = ::core::iter::Empty<
            ::pest_typed::re_exported::Box<
                dyn::pest_typed::iterators::Pair<'i, 'n, super::Rule> + 'n,
            >,
        >;
        fn iter_pairs(&'n self) -> Self::Iter {
            ::core::iter::empty()
        }
        fn into_iter_pairs(self) -> Self::IntoIter {
            ::core::iter::empty()
        }
    }
    #[allow(non_camel_case_types)]
    #[doc = "Auto generated. Unicode property ARABIC."]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct ARABIC {
        pub content: ::core::primitive::char,
    }
    impl ::core::convert::From<::core::primitive::char> for ARABIC {
        fn from(content: ::core::primitive::char) -> Self {
            Self { content }
        }
    }
    impl<'i> ::pest_typed::TypedNode<'i, super::Rule> for ARABIC {
        #[inline]
        fn try_parse_with(
            mut input: ::pest_typed::Position<'i>,
            _stack: &mut ::pest_typed::Stack<::pest_typed::Span<'i>>,
            _tracker: &mut ::pest_typed::tracker::Tracker<'i, super::Rule>,
        ) -> ::pest_typed::re_exported::Result<(::pest_typed::Position<'i>, Self), ()> {
            match ::pest_typed::predefined_node::match_char_by(
                &mut input,
                ::pest_typed::unicode::ARABIC,
            ) {
                Some(content) => Ok((input, Self::from(content))),
                None => Err(()),
            }
        }
    }
    impl ::core::fmt::Debug for ARABIC {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("ARABIC")
                .field("content", &self.content)
                .finish()
        }
    }
    impl<'i: 'n, 'n> ::pest_typed::iterators::Pairs<'i, 'n, super::Rule> for ARABIC {
        type Iter = ::core::iter::Empty<&'n dyn::pest_typed::iterators::Pair<'i, 'n, super::Rule>>;
        type IntoIter = ::core::iter::Empty<
            ::pest_typed::re_exported::Box<
                dyn::pest_typed::iterators::Pair<'i, 'n, super::Rule> + 'n,
            >,
        >;
        fn iter_pairs(&'n self) -> Self::Iter {
            ::core::iter::empty()
        }
        fn into_iter_pairs(self) -> Self::IntoIter {
            ::core::iter::empty()
        }
    }
    #[allow(non_camel_case_types)]
    #[doc = "Auto generated. Unicode property HAN."]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct HAN {
        pub content: ::core::primitive::char,
    }
    impl ::core::convert::From<::core::primitive::char> for HAN {
        fn from(content: ::core::primitive::char) -> Self {
            Self { content }
        }
    }
    impl<'i> ::pest_typed::TypedNode<'i, super::Rule> for HAN {
        #[inline]
        fn try_parse_with(
            mut input: ::pest_typed::Position<'i>,
            _stack: &mut ::pest_typed::Stack<::pest_typed::Span<'i>>,
            _tracker: &mut ::pest_typed::tracker::Tracker<'i, super::Rule>,
        ) -> ::pest_typed::re_exported::Result<(::pest_typed::Position<'i>, Self), ()> {
            match ::pest_typed::predefined_node::match_char_by(
                &mut input,
                ::pest_typed::unicode::HAN,
            ) {
                Some(content) => Ok((input, Self::from(content))),
                None => Err(()),
            }
        }
    }
    impl ::core::fmt::Debug for HAN {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("HAN")
                .field("content", &self.content)
                .finish()
        }
    }
    impl<'i: 'n, 'n> ::pest_typed::iterators::Pairs<'i, 'n, super::Rule> for HAN {
        type Iter = ::core::iter::Empty<&'n dyn::pest_typed::iterators::Pair<'i, 'n, super::Rule>>;
        type IntoIter = ::core::iter::Empty<
            ::pest_typed::re_exported::Box<
                dyn::pest_typed::iterators::Pair<'i, 'n, super::Rule> + 'n,
            >,
        >;
        fn iter_pairs(&'n self) -> Self::Iter {
            ::core::iter::empty()
        }
        fn into_iter_pairs(self) -> Self::IntoIter {
            ::core::iter::empty()
        }
    }
    #[allow(non_camel_case_types)]
    #[doc = "Auto generated. Unicode property HANGUL."]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct HANGUL {
        pub content: ::core::primitive::char,
    }
    impl ::core::convert::From<::core::primitive::char> for HANGUL {
        fn from(content: ::core::primitive::char) -> Self {
            Self { content }
        }
    }
    impl<'i> ::pest_typed::TypedNode<'i, super::Rule> for HANGUL {
        #[inline]
        fn try_parse_with(
            mut input: ::pest_typed::Position<'i>,
            _stack: &mut ::pest_typed::Stack<::pest_typed::Span<'i>>,
            _tracker: &mut ::pest_typed::tracker::Tracker<'i, super::Rule>,
        ) -> ::pest_typed::re_exported::Result<(::pest_typed::Position<'i>, Self), ()> {
            match ::pest_typed::predefined_node::match_char_by(
                &mut input,
                ::pest_typed::unicode::HANGUL,
            ) {
                Some(content) => Ok((input, Self::from(content))),
                None => Err(()),
            }
        }
    }
    impl ::core::fmt::Debug for HANGUL {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("HANGUL")
                .field("content", &self.content)
                .finish()
        }
    }
    impl<'i: 'n, 'n> ::pest_typed::iterators::Pairs<'i, 'n, super::Rule> for HANGUL {
        type Iter = ::core::iter::Empty<&'n dyn::pest_typed::iterators::Pair<'i, 'n, super::Rule>>;
        type IntoIter = ::core::iter::Empty<
            ::pest_typed::re_exported::Box<
                dyn::pest_typed::iterators::Pair<'i, 'n, super::Rule> + 'n,
            >,
        >;
        fn iter_pairs(&'n self) -> Self::Iter {
            ::core::iter::empty()
        }
        fn into_iter_pairs(self) -> Self::IntoIter {
            ::core::iter::empty()
        }
    }
    #[allow(non_camel_case_types)]
    #[doc = "Auto generated. Unicode property HIRAGANA."]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct HIRAGANA {
        pub content: ::core::primitive::char,
    }
    impl ::core::convert::From<::core::primitive::char> for HIRAGANA {
        fn from(content: ::core::primitive::char) -> Self {
            Self { content }
        }
    }
    impl<'i> ::pest_typed::TypedNode<'i, super::Rule> for HIRAGANA {
        #[inline]
        fn try_parse_with(
            mut input: ::pest_typed::Position<'i>,
            _stack: &mut ::pest_typed::Stack<::pest_typed::Span<'i>>,
            _tracker: &mut ::pest_typed::tracker::Tracker<'i, super::Rule>,
        ) -> ::pest_typed::re_exported::Result<(::pest_typed::Position<'i>, Self), ()> {
            match ::pest_typed::predefined_node::match_char_by(
                &mut input,
                ::pest_typed::unicode::HIRAGANA,
            ) {
                Some(content) => Ok((input, Self::from(content))),
                None => Err(()),
            }
        }
    }
    impl ::core::fmt::Debug for HIRAGANA {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("HIRAGANA")
                .field("content", &self.content)
                .finish()
        }
    }
    impl<'i: 'n, 'n> ::pest_typed::iterators::Pairs<'i, 'n, super::Rule> for HIRAGANA {
        type Iter = ::core::iter::Empty<&'n dyn::pest_typed::iterators::Pair<'i, 'n, super::Rule>>;
        type IntoIter = ::core::iter::Empty<
            ::pest_typed::re_exported::Box<
                dyn::pest_typed::iterators::Pair<'i, 'n, super::Rule> + 'n,
            >,
        >;
        fn iter_pairs(&'n self) -> Self::Iter {
            ::core::iter::empty()
        }
        fn into_iter_pairs(self) -> Self::IntoIter {
            ::core::iter::empty()
        }
    }
}
mod constant_wrappers {
    #[doc = "A wrapper for `\"abc\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_0;
    impl ::pest_typed::StringWrapper for r#w_0 {
        const CONTENT: &'static ::core::primitive::str = "abc";
    }
    #[doc = "A wrapper for `\"abc\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_1;
    impl ::pest_typed::StringWrapper for r#w_1 {
        const CONTENT: &'static ::core::primitive::str = "abc";
    }
    #[doc = "A wrapper for `\",\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_2;
    impl ::pest_typed::StringWrapper for r#w_2 {
        const CONTENT: &'static ::core::primitive::str = ",";
    }
    #[doc = "A wrapper for `\",\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_3;
    impl ::pest_typed::StringWrapper for r#w_3 {
        const CONTENT: &'static ::core::primitive::str = ",";
    }
    #[doc = "A wrapper for `\"FAIL\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_4;
    impl ::pest_typed::StringWrapper for r#w_4 {
        const CONTENT: &'static ::core::primitive::str = "FAIL";
    }
    #[doc = "A wrapper for `\"\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_5;
    impl ::pest_typed::StringWrapper for r#w_5 {
        const CONTENT: &'static ::core::primitive::str = "";
    }
    #[doc = "A wrapper for `\"a\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_6;
    impl ::pest_typed::StringWrapper for r#w_6 {
        const CONTENT: &'static ::core::primitive::str = "a";
    }
    #[doc = "A wrapper for `\"b\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_7;
    impl ::pest_typed::StringWrapper for r#w_7 {
        const CONTENT: &'static ::core::primitive::str = "b";
    }
    #[doc = "A wrapper for `\"b\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_8;
    impl ::pest_typed::StringWrapper for r#w_8 {
        const CONTENT: &'static ::core::primitive::str = "b";
    }
    #[doc = "A wrapper for `\"a\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_9;
    impl ::pest_typed::StringWrapper for r#w_9 {
        const CONTENT: &'static ::core::primitive::str = "a";
    }
    #[doc = "A wrapper for `\"shadows builtin\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_10;
    impl ::pest_typed::StringWrapper for r#w_10 {
        const CONTENT: &'static ::core::primitive::str = "shadows builtin";
    }
    #[doc = "A wrapper for `\" \"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_11;
    impl ::pest_typed::StringWrapper for r#w_11 {
        const CONTENT: &'static ::core::primitive::str = " ";
    }
    #[doc = "A wrapper for `\"$\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_12;
    impl ::pest_typed::StringWrapper for r#w_12 {
        const CONTENT: &'static ::core::primitive::str = "$";
    }
    #[doc = "A wrapper for `\"$\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_13;
    impl ::pest_typed::StringWrapper for r#w_13 {
        const CONTENT: &'static ::core::primitive::str = "$";
    }
}
#[doc = "Definitions of statically typed nodes generated by pest-generator."]
pub mod rules_impl {
    #[doc = "Definitions of statically typed nodes generated by pest-generator."]
    pub mod rules {
        :: pest_typed :: rule ! (r#string , "Corresponds to expression: `\"abc\"`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#string , super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_0 > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#string<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#insensitive , "Corresponds to expression: `^\"abc\"`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#insensitive , super :: super :: generics :: Insens :: < 'i , super :: super :: constant_wrappers :: r#w_1 > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#insensitive<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#range , "Corresponds to expression: `('0'..'9')`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#range , super :: super :: generics :: CharRange :: < '0' , '9' > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#range<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#ident , "Corresponds to expression: `string`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ident , super :: super :: rules :: r#string :: < 'i , INHERITED > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#ident<'i, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(&'s self) -> &'s super::super::rules::r#string<'i, INHERITED> {
                let res = &self.content;
                res
            }
        }
        :: pest_typed :: rule ! (r#pos_pred , "Corresponds to expression: `&string`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#pos_pred , super :: super :: generics :: Positive :: < super :: super :: rules :: r#string :: < 'i , INHERITED > > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#pos_pred<'i, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(&'s self) -> &'s super::super::rules::r#string<'i, INHERITED> {
                let res = &self.content;
                {
                    let res = &res.content;
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#neg_pred , "Corresponds to expression: `!string`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#neg_pred , super :: super :: generics :: Negative :: < super :: super :: rules :: r#string :: < 'i , INHERITED > > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#neg_pred<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#double_neg_pred , "Corresponds to expression: `!!string`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#double_neg_pred , super :: super :: generics :: Negative :: < super :: super :: generics :: Negative :: < super :: super :: rules :: r#string :: < 'i , INHERITED > > > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#double_neg_pred<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#sequence , "Corresponds to expression: `(string ~ string)`. Non-atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#sequence , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , 1 > , super :: super :: generics :: Skipped < 'i > , 1 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , 1 > , super :: super :: generics :: Skipped < 'i > , 1 >) , > , super :: super :: generics :: Skipped :: < 'i > , false , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#sequence<'i, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#string<'i, 1>,
                &'s super::super::rules::r#string<'i, 1>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            res
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#sequence_compound , "Corresponds to expression: `(string ~ string)`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#sequence_compound , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , 0 > , super :: super :: generics :: Skipped < 'i > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , 0 > , super :: super :: generics :: Skipped < 'i > , 0 >) , > , super :: super :: generics :: Skipped :: < 'i > , true , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#sequence_compound<'i, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#string<'i, 0>,
                &'s super::super::rules::r#string<'i, 0>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            res
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#sequence_atomic , "Corresponds to expression: `(string ~ string)`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#sequence_atomic , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , 0 > , super :: super :: generics :: Skipped < 'i > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , 0 > , super :: super :: generics :: Skipped < 'i > , 0 >) , > , super :: super :: generics :: Skipped :: < 'i > , true , Span , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#sequence_atomic<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#sequence_non_atomic , "Corresponds to expression: `sequence`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#sequence_non_atomic , super :: super :: rules :: r#sequence :: < 'i , 0 > , super :: super :: generics :: Skipped :: < 'i > , true , Span , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#sequence_non_atomic<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#sequence_atomic_compound , "Corresponds to expression: `sequence_compound`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#sequence_atomic_compound , super :: super :: rules :: r#sequence_compound :: < 'i , 0 > , super :: super :: generics :: Skipped :: < 'i > , true , Span , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#sequence_atomic_compound<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#sequence_nested , "Corresponds to expression: `(string ~ string)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#sequence_nested , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#sequence_nested<'i, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#string<'i, INHERITED>,
                &'s super::super::rules::r#string<'i, INHERITED>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            res
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#sequence_compound_nested , "Corresponds to expression: `sequence_nested`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#sequence_compound_nested , super :: super :: rules :: r#sequence_nested :: < 'i , 0 > , super :: super :: generics :: Skipped :: < 'i > , true , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#sequence_compound_nested<'i, INHERITED> {
            #[doc = "A helper function to access [`sequence_nested`]."]
            #[allow(non_snake_case)]
            pub fn r#sequence_nested<'s>(
                &'s self,
            ) -> &'s super::super::rules::r#sequence_nested<'i, 0> {
                let res = &self.content;
                res
            }
        }
        :: pest_typed :: rule ! (r#node_tag , "Corresponds to expression: `string`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#node_tag , super :: super :: rules :: r#string :: < 'i , INHERITED > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#node_tag<'i, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(&'s self) -> &'s super::super::rules::r#string<'i, INHERITED> {
                let res = &self.content;
                res
            }
        }
        :: pest_typed :: rule ! (r#choice , "Corresponds to expression: `(string | range)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#choice , super :: super :: generics :: Choice2 :: < super :: super :: rules :: r#string :: < 'i , INHERITED > , super :: super :: rules :: r#range :: < 'i , INHERITED > , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#choice<'i, INHERITED> {
            #[doc = "A helper function to access [`range`]."]
            #[allow(non_snake_case)]
            pub fn r#range<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#range<'i, INHERITED>>
            {
                let res = &self.content;
                {
                    let res = res._1().map(|res| res);
                    res
                }
            }
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#string<'i, INHERITED>>
            {
                let res = &self.content;
                {
                    let res = res._0().map(|res| res);
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#choice_prefix , "Corresponds to expression: `(string | range)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#choice_prefix , super :: super :: generics :: Choice2 :: < super :: super :: rules :: r#string :: < 'i , INHERITED > , super :: super :: rules :: r#range :: < 'i , INHERITED > , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#choice_prefix<'i, INHERITED> {
            #[doc = "A helper function to access [`range`]."]
            #[allow(non_snake_case)]
            pub fn r#range<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#range<'i, INHERITED>>
            {
                let res = &self.content;
                {
                    let res = res._1().map(|res| res);
                    res
                }
            }
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#string<'i, INHERITED>>
            {
                let res = &self.content;
                {
                    let res = res._0().map(|res| res);
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#optional , "Corresponds to expression: `string?`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#optional , :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#string :: < 'i , INHERITED > > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#optional<'i, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#string<'i, INHERITED>>
            {
                let res = &self.content;
                {
                    let res = res.as_ref().map(|res| res);
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#repeat , "Corresponds to expression: `string*`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat , super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#string :: < 'i , INHERITED > > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#repeat<'i, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#string<'i, INHERITED>>
            {
                let res = &self.content;
                {
                    let res = res
                        .content
                        .iter()
                        .map(|res| {
                            let res = &res.matched;
                            res
                        })
                        .collect::<::pest_typed::re_exported::Vec<_>>();
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#repeat_atomic , "Corresponds to expression: `string*`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_atomic , super :: super :: generics :: Rep :: < 'i , 0 , super :: super :: rules :: r#string :: < 'i , 0 > > , super :: super :: generics :: Skipped :: < 'i > , true , Span , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#repeat_atomic<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#repeat_once , "Corresponds to expression: `(string ~ string*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_once , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#string :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#repeat_once<'i, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#string<'i, INHERITED>,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#string<'i, INHERITED>>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = res
                                    .content
                                    .iter()
                                    .map(|res| {
                                        let res = &res.matched;
                                        res
                                    })
                                    .collect::<::pest_typed::re_exported::Vec<_>>();
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#repeat_once_atomic , "Corresponds to expression: `(string ~ string*)`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_once_atomic , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , 0 > , super :: super :: generics :: Skipped < 'i > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , 0 , super :: super :: rules :: r#string :: < 'i , 0 > > , super :: super :: generics :: Skipped < 'i > , 0 >) , > , super :: super :: generics :: Skipped :: < 'i > , true , Span , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#repeat_once_atomic<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#repeat_min_max , "Corresponds to expression: `(string ~ string ~ string?)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_min_max , super :: super :: generics :: Seq3 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#string :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#repeat_min_max<'i, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#string<'i, INHERITED>,
                &'s super::super::rules::r#string<'i, INHERITED>,
                ::pest_typed::re_exported::Option<&'s super::super::rules::r#string<'i, INHERITED>>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            res
                        },
                        {
                            let res = &res.content.2.matched;
                            {
                                let res = res.as_ref().map(|res| res);
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#repeat_min_max_atomic , "Corresponds to expression: `(string ~ string ~ string?)`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_min_max_atomic , super :: super :: generics :: Seq3 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , 0 > , super :: super :: generics :: Skipped < 'i > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , 0 > , super :: super :: generics :: Skipped < 'i > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#string :: < 'i , 0 > > , super :: super :: generics :: Skipped < 'i > , 0 >) , > , super :: super :: generics :: Skipped :: < 'i > , true , Span , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#repeat_min_max_atomic<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#repeat_exact , "Corresponds to expression: `(string ~ string)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_exact , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#repeat_exact<'i, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#string<'i, INHERITED>,
                &'s super::super::rules::r#string<'i, INHERITED>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            res
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#repeat_min , "Corresponds to expression: `(string ~ string ~ string*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_min , super :: super :: generics :: Seq3 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#string :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#repeat_min<'i, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#string<'i, INHERITED>,
                &'s super::super::rules::r#string<'i, INHERITED>,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#string<'i, INHERITED>>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            res
                        },
                        {
                            let res = &res.content.2.matched;
                            {
                                let res = res
                                    .content
                                    .iter()
                                    .map(|res| {
                                        let res = &res.matched;
                                        res
                                    })
                                    .collect::<::pest_typed::re_exported::Vec<_>>();
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#repeat_min_atomic , "Corresponds to expression: `(string ~ string ~ string*)`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_min_atomic , super :: super :: generics :: Seq3 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , 0 > , super :: super :: generics :: Skipped < 'i > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , 0 > , super :: super :: generics :: Skipped < 'i > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , 0 , super :: super :: rules :: r#string :: < 'i , 0 > > , super :: super :: generics :: Skipped < 'i > , 0 >) , > , super :: super :: generics :: Skipped :: < 'i > , true , Span , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#repeat_min_atomic<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#repeat_max , "Corresponds to expression: `(string? ~ string?)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_max , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#string :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#string :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#repeat_max<'i, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> (
                ::pest_typed::re_exported::Option<&'s super::super::rules::r#string<'i, INHERITED>>,
                ::pest_typed::re_exported::Option<&'s super::super::rules::r#string<'i, INHERITED>>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            {
                                let res = res.as_ref().map(|res| res);
                                res
                            }
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = res.as_ref().map(|res| res);
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#repeat_max_atomic , "Corresponds to expression: `(string? ~ string?)`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_max_atomic , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#string :: < 'i , 0 > > , super :: super :: generics :: Skipped < 'i > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#string :: < 'i , 0 > > , super :: super :: generics :: Skipped < 'i > , 0 >) , > , super :: super :: generics :: Skipped :: < 'i > , true , Span , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#repeat_max_atomic<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#soi_at_start , "Corresponds to expression: `(SOI ~ string)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#soi_at_start , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#SOI , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#soi_at_start<'i, INHERITED> {
            #[doc = "A helper function to access [`SOI`]."]
            #[allow(non_snake_case)]
            pub fn r#SOI<'s>(&'s self) -> &'s super::super::rules::r#SOI {
                let res = &self.content;
                {
                    let res = &res.content.0.matched;
                    res
                }
            }
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(&'s self) -> &'s super::super::rules::r#string<'i, INHERITED> {
                let res = &self.content;
                {
                    let res = &res.content.1.matched;
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#repeat_mutate_stack , "Corresponds to expression: `((PUSH(('a'..'c')) ~ \",\")* ~ POP ~ POP ~ POP)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_mutate_stack , super :: super :: generics :: Seq4 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: generics :: CharRange :: < 'a' , 'c' > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_2 > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP :: < 'i > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP :: < 'i > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP :: < 'i > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#repeat_mutate_stack<'i, INHERITED> {
            #[doc = "A helper function to access [`POP`]."]
            #[allow(non_snake_case)]
            pub fn r#POP<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#POP<'i>,
                &'s super::super::rules::r#POP<'i>,
                &'s super::super::rules::r#POP<'i>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.1.matched;
                            res
                        },
                        {
                            let res = &res.content.2.matched;
                            res
                        },
                        {
                            let res = &res.content.3.matched;
                            res
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#repeat_mutate_stack_pop_all , "Corresponds to expression: `((PUSH(('a'..'c')) ~ \",\")* ~ POP_ALL)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_mutate_stack_pop_all , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: generics :: CharRange :: < 'a' , 'c' > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_3 > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP_ALL :: < 'i > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#repeat_mutate_stack_pop_all<'i, INHERITED> {
            #[doc = "A helper function to access [`POP_ALL`]."]
            #[allow(non_snake_case)]
            pub fn r#POP_ALL<'s>(&'s self) -> &'s super::super::rules::r#POP_ALL<'i> {
                let res = &self.content;
                {
                    let res = &res.content.1.matched;
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#will_fail , "Corresponds to expression: `(repeat_mutate_stack_pop_all ~ \"FAIL\")`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#will_fail , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#repeat_mutate_stack_pop_all :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_4 > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#will_fail<'i, INHERITED> {
            #[doc = "A helper function to access [`repeat_mutate_stack_pop_all`]."]
            #[allow(non_snake_case)]
            pub fn r#repeat_mutate_stack_pop_all<'s>(
                &'s self,
            ) -> &'s super::super::rules::r#repeat_mutate_stack_pop_all<'i, INHERITED> {
                let res = &self.content;
                {
                    let res = &res.content.0.matched;
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#stack_resume_after_fail , "Corresponds to expression: `(will_fail | repeat_mutate_stack_pop_all)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#stack_resume_after_fail , super :: super :: generics :: Choice2 :: < super :: super :: rules :: r#will_fail :: < 'i , INHERITED > , super :: super :: rules :: r#repeat_mutate_stack_pop_all :: < 'i , INHERITED > , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#stack_resume_after_fail<'i, INHERITED> {
            #[doc = "A helper function to access [`repeat_mutate_stack_pop_all`]."]
            #[allow(non_snake_case)]
            pub fn r#repeat_mutate_stack_pop_all<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<
                &'s super::super::rules::r#repeat_mutate_stack_pop_all<'i, INHERITED>,
            > {
                let res = &self.content;
                {
                    let res = res._1().map(|res| res);
                    res
                }
            }
            #[doc = "A helper function to access [`will_fail`]."]
            #[allow(non_snake_case)]
            pub fn r#will_fail<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<
                &'s super::super::rules::r#will_fail<'i, INHERITED>,
            > {
                let res = &self.content;
                {
                    let res = res._0().map(|res| res);
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#peek_ , "Corresponds to expression: `(PUSH(range) ~ PUSH(range) ~ PEEK ~ PEEK)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#peek_ , super :: super :: generics :: Seq4 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#PEEK :: < 'i > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#PEEK :: < 'i > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#peek_<'i, INHERITED> {
            #[doc = "A helper function to access [`PEEK`]."]
            #[allow(non_snake_case)]
            pub fn r#PEEK<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#PEEK<'i>,
                &'s super::super::rules::r#PEEK<'i>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.2.matched;
                            res
                        },
                        {
                            let res = &res.content.3.matched;
                            res
                        },
                    );
                    res
                }
            }
            #[doc = "A helper function to access [`range`]."]
            #[allow(non_snake_case)]
            pub fn r#range<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#range<'i, INHERITED>,
                &'s super::super::rules::r#range<'i, INHERITED>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            {
                                let res = &res.content;
                                res
                            }
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = &res.content;
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#peek_all , "Corresponds to expression: `(PUSH(range) ~ PUSH(range) ~ PEEK_ALL)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#peek_all , super :: super :: generics :: Seq3 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#PEEK_ALL :: < 'i > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#peek_all<'i, INHERITED> {
            #[doc = "A helper function to access [`PEEK_ALL`]."]
            #[allow(non_snake_case)]
            pub fn r#PEEK_ALL<'s>(&'s self) -> &'s super::super::rules::r#PEEK_ALL<'i> {
                let res = &self.content;
                {
                    let res = &res.content.2.matched;
                    res
                }
            }
            #[doc = "A helper function to access [`range`]."]
            #[allow(non_snake_case)]
            pub fn r#range<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#range<'i, INHERITED>,
                &'s super::super::rules::r#range<'i, INHERITED>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            {
                                let res = &res.content;
                                res
                            }
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = &res.content;
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#peek_slice_23 , "Corresponds to expression: `(PUSH(range) ~ PUSH(range) ~ PUSH(range) ~ PUSH(range) ~ PUSH(range) ~ PEEK[1..-2])`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#peek_slice_23 , super :: super :: generics :: Seq6 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: PeekSlice2 :: < 1i32 , - 2i32 > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#peek_slice_23<'i, INHERITED> {
            #[doc = "A helper function to access [`range`]."]
            #[allow(non_snake_case)]
            pub fn r#range<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#range<'i, INHERITED>,
                &'s super::super::rules::r#range<'i, INHERITED>,
                &'s super::super::rules::r#range<'i, INHERITED>,
                &'s super::super::rules::r#range<'i, INHERITED>,
                &'s super::super::rules::r#range<'i, INHERITED>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            {
                                let res = &res.content;
                                res
                            }
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = &res.content;
                                res
                            }
                        },
                        {
                            let res = &res.content.2.matched;
                            {
                                let res = &res.content;
                                res
                            }
                        },
                        {
                            let res = &res.content.3.matched;
                            {
                                let res = &res.content;
                                res
                            }
                        },
                        {
                            let res = &res.content.4.matched;
                            {
                                let res = &res.content;
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#pop_ , "Corresponds to expression: `(PUSH(range) ~ PUSH(range) ~ POP ~ POP)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#pop_ , super :: super :: generics :: Seq4 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP :: < 'i > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP :: < 'i > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#pop_<'i, INHERITED> {
            #[doc = "A helper function to access [`POP`]."]
            #[allow(non_snake_case)]
            pub fn r#POP<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#POP<'i>,
                &'s super::super::rules::r#POP<'i>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.2.matched;
                            res
                        },
                        {
                            let res = &res.content.3.matched;
                            res
                        },
                    );
                    res
                }
            }
            #[doc = "A helper function to access [`range`]."]
            #[allow(non_snake_case)]
            pub fn r#range<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#range<'i, INHERITED>,
                &'s super::super::rules::r#range<'i, INHERITED>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            {
                                let res = &res.content;
                                res
                            }
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = &res.content;
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#pop_all , "Corresponds to expression: `(PUSH(range) ~ PUSH(range) ~ POP_ALL)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#pop_all , super :: super :: generics :: Seq3 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP_ALL :: < 'i > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#pop_all<'i, INHERITED> {
            #[doc = "A helper function to access [`POP_ALL`]."]
            #[allow(non_snake_case)]
            pub fn r#POP_ALL<'s>(&'s self) -> &'s super::super::rules::r#POP_ALL<'i> {
                let res = &self.content;
                {
                    let res = &res.content.2.matched;
                    res
                }
            }
            #[doc = "A helper function to access [`range`]."]
            #[allow(non_snake_case)]
            pub fn r#range<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#range<'i, INHERITED>,
                &'s super::super::rules::r#range<'i, INHERITED>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            {
                                let res = &res.content;
                                res
                            }
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = &res.content;
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#pop_fail , "Corresponds to expression: `(PUSH(range) ~ !POP ~ range ~ POP)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#pop_fail , super :: super :: generics :: Seq4 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Negative :: < super :: super :: rules :: r#POP :: < 'i > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#range :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP :: < 'i > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#pop_fail<'i, INHERITED> {
            #[doc = "A helper function to access [`POP`]."]
            #[allow(non_snake_case)]
            pub fn r#POP<'s>(&'s self) -> &'s super::super::rules::r#POP<'i> {
                let res = &self.content;
                {
                    let res = &res.content.3.matched;
                    res
                }
            }
            #[doc = "A helper function to access [`range`]."]
            #[allow(non_snake_case)]
            pub fn r#range<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#range<'i, INHERITED>,
                &'s super::super::rules::r#range<'i, INHERITED>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            {
                                let res = &res.content;
                                res
                            }
                        },
                        {
                            let res = &res.content.2.matched;
                            res
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#checkpoint_restore , "Corresponds to expression: `(PUSH(\"\") ~ ((PUSH(\"a\") ~ \"b\" ~ POP) | (DROP ~ \"b\") | (POP ~ \"a\")) ~ EOI)`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#checkpoint_restore , super :: super :: generics :: Seq3 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_5 > > , super :: super :: generics :: Skipped < 'i > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Choice3 :: < super :: super :: generics :: Seq3 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_6 > > , super :: super :: generics :: Skipped < 'i > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_7 > , super :: super :: generics :: Skipped < 'i > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP :: < 'i > , super :: super :: generics :: Skipped < 'i > , 0 >) , > , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#DROP , super :: super :: generics :: Skipped < 'i > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_8 > , super :: super :: generics :: Skipped < 'i > , 0 >) , > , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP :: < 'i > , super :: super :: generics :: Skipped < 'i > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_9 > , super :: super :: generics :: Skipped < 'i > , 0 >) , > , > , super :: super :: generics :: Skipped < 'i > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#EOI :: < 'i > , super :: super :: generics :: Skipped < 'i > , 0 >) , > , super :: super :: generics :: Skipped :: < 'i > , true , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#checkpoint_restore<'i, INHERITED> {
            #[doc = "A helper function to access [`DROP`]."]
            #[allow(non_snake_case)]
            pub fn r#DROP<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#DROP> {
                let res = &self.content;
                {
                    let res = &res.content.1.matched;
                    {
                        let res = res._1().map(|res| {
                            let res = &res.content.0.matched;
                            res
                        });
                        res
                    }
                }
            }
            #[doc = "A helper function to access [`EOI`]."]
            #[allow(non_snake_case)]
            pub fn r#EOI<'s>(&'s self) -> &'s super::super::rules::r#EOI<'i> {
                let res = &self.content;
                {
                    let res = &res.content.2.matched;
                    res
                }
            }
            #[doc = "A helper function to access [`POP`]."]
            #[allow(non_snake_case)]
            pub fn r#POP<'s>(
                &'s self,
            ) -> (
                ::pest_typed::re_exported::Option<&'s super::super::rules::r#POP<'i>>,
                ::pest_typed::re_exported::Option<&'s super::super::rules::r#POP<'i>>,
            ) {
                let res = &self.content;
                {
                    let res = &res.content.1.matched;
                    {
                        let res = (
                            {
                                let res = res._0().map(|res| {
                                    let res = &res.content.2.matched;
                                    res
                                });
                                res
                            },
                            {
                                let res = res._2().map(|res| {
                                    let res = &res.content.0.matched;
                                    res
                                });
                                res
                            },
                        );
                        res
                    }
                }
            }
        }
        :: pest_typed :: rule ! (r#ascii_digits , "Corresponds to expression: `(ASCII_DIGIT ~ ASCII_DIGIT*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ascii_digits , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#ASCII_DIGIT , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#ASCII_DIGIT > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#ascii_digits<'i, INHERITED> {
            #[doc = "A helper function to access [`ASCII_DIGIT`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII_DIGIT<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#ASCII_DIGIT,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII_DIGIT>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = res
                                    .content
                                    .iter()
                                    .map(|res| {
                                        let res = &res.matched;
                                        res
                                    })
                                    .collect::<::pest_typed::re_exported::Vec<_>>();
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#ascii_nonzero_digits , "Corresponds to expression: `(ASCII_NONZERO_DIGIT ~ ASCII_NONZERO_DIGIT*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ascii_nonzero_digits , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#ASCII_NONZERO_DIGIT , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#ASCII_NONZERO_DIGIT > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#ascii_nonzero_digits<'i, INHERITED> {
            #[doc = "A helper function to access [`ASCII_NONZERO_DIGIT`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII_NONZERO_DIGIT<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#ASCII_NONZERO_DIGIT,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII_NONZERO_DIGIT>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = res
                                    .content
                                    .iter()
                                    .map(|res| {
                                        let res = &res.matched;
                                        res
                                    })
                                    .collect::<::pest_typed::re_exported::Vec<_>>();
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#ascii_bin_digits , "Corresponds to expression: `(ASCII_BIN_DIGIT ~ ASCII_BIN_DIGIT*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ascii_bin_digits , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#ASCII_BIN_DIGIT , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#ASCII_BIN_DIGIT > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#ascii_bin_digits<'i, INHERITED> {
            #[doc = "A helper function to access [`ASCII_BIN_DIGIT`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII_BIN_DIGIT<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#ASCII_BIN_DIGIT,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII_BIN_DIGIT>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = res
                                    .content
                                    .iter()
                                    .map(|res| {
                                        let res = &res.matched;
                                        res
                                    })
                                    .collect::<::pest_typed::re_exported::Vec<_>>();
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#ascii_oct_digits , "Corresponds to expression: `(ASCII_OCT_DIGIT ~ ASCII_OCT_DIGIT*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ascii_oct_digits , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#ASCII_OCT_DIGIT , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#ASCII_OCT_DIGIT > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#ascii_oct_digits<'i, INHERITED> {
            #[doc = "A helper function to access [`ASCII_OCT_DIGIT`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII_OCT_DIGIT<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#ASCII_OCT_DIGIT,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII_OCT_DIGIT>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = res
                                    .content
                                    .iter()
                                    .map(|res| {
                                        let res = &res.matched;
                                        res
                                    })
                                    .collect::<::pest_typed::re_exported::Vec<_>>();
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#ascii_hex_digits , "Corresponds to expression: `(ASCII_HEX_DIGIT ~ ASCII_HEX_DIGIT*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ascii_hex_digits , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#ASCII_HEX_DIGIT , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#ASCII_HEX_DIGIT > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#ascii_hex_digits<'i, INHERITED> {
            #[doc = "A helper function to access [`ASCII_HEX_DIGIT`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII_HEX_DIGIT<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#ASCII_HEX_DIGIT,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII_HEX_DIGIT>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = res
                                    .content
                                    .iter()
                                    .map(|res| {
                                        let res = &res.matched;
                                        res
                                    })
                                    .collect::<::pest_typed::re_exported::Vec<_>>();
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#ascii_alpha_lowers , "Corresponds to expression: `(ASCII_ALPHA_LOWER ~ ASCII_ALPHA_LOWER*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ascii_alpha_lowers , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#ASCII_ALPHA_LOWER , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#ASCII_ALPHA_LOWER > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#ascii_alpha_lowers<'i, INHERITED> {
            #[doc = "A helper function to access [`ASCII_ALPHA_LOWER`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII_ALPHA_LOWER<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#ASCII_ALPHA_LOWER,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII_ALPHA_LOWER>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = res
                                    .content
                                    .iter()
                                    .map(|res| {
                                        let res = &res.matched;
                                        res
                                    })
                                    .collect::<::pest_typed::re_exported::Vec<_>>();
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#ascii_alpha_uppers , "Corresponds to expression: `(ASCII_ALPHA_UPPER ~ ASCII_ALPHA_UPPER*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ascii_alpha_uppers , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#ASCII_ALPHA_UPPER , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#ASCII_ALPHA_UPPER > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#ascii_alpha_uppers<'i, INHERITED> {
            #[doc = "A helper function to access [`ASCII_ALPHA_UPPER`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII_ALPHA_UPPER<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#ASCII_ALPHA_UPPER,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII_ALPHA_UPPER>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = res
                                    .content
                                    .iter()
                                    .map(|res| {
                                        let res = &res.matched;
                                        res
                                    })
                                    .collect::<::pest_typed::re_exported::Vec<_>>();
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#ascii_alphas , "Corresponds to expression: `(ASCII_ALPHA ~ ASCII_ALPHA*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ascii_alphas , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#ASCII_ALPHA , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#ASCII_ALPHA > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#ascii_alphas<'i, INHERITED> {
            #[doc = "A helper function to access [`ASCII_ALPHA`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII_ALPHA<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#ASCII_ALPHA,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII_ALPHA>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = res
                                    .content
                                    .iter()
                                    .map(|res| {
                                        let res = &res.matched;
                                        res
                                    })
                                    .collect::<::pest_typed::re_exported::Vec<_>>();
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#ascii_alphanumerics , "Corresponds to expression: `(ASCII_ALPHANUMERIC ~ ASCII_ALPHANUMERIC*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ascii_alphanumerics , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#ASCII_ALPHANUMERIC , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#ASCII_ALPHANUMERIC > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#ascii_alphanumerics<'i, INHERITED> {
            #[doc = "A helper function to access [`ASCII_ALPHANUMERIC`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII_ALPHANUMERIC<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#ASCII_ALPHANUMERIC,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII_ALPHANUMERIC>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = res
                                    .content
                                    .iter()
                                    .map(|res| {
                                        let res = &res.matched;
                                        res
                                    })
                                    .collect::<::pest_typed::re_exported::Vec<_>>();
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#asciis , "Corresponds to expression: `(ASCII ~ ASCII*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#asciis , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#ASCII , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#ASCII > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#asciis<'i, INHERITED> {
            #[doc = "A helper function to access [`ASCII`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#ASCII,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = res
                                    .content
                                    .iter()
                                    .map(|res| {
                                        let res = &res.matched;
                                        res
                                    })
                                    .collect::<::pest_typed::re_exported::Vec<_>>();
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#newline , "Corresponds to expression: `(NEWLINE ~ NEWLINE*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#newline , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#NEWLINE , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#NEWLINE > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#newline<'i, INHERITED> {
            #[doc = "A helper function to access [`NEWLINE`]."]
            #[allow(non_snake_case)]
            pub fn r#NEWLINE<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#NEWLINE,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#NEWLINE>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = res
                                    .content
                                    .iter()
                                    .map(|res| {
                                        let res = &res.matched;
                                        res
                                    })
                                    .collect::<::pest_typed::re_exported::Vec<_>>();
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#unicode , "Corresponds to expression: `(XID_START ~ XID_CONTINUE*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#unicode , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#XID_START , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#XID_CONTINUE > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#unicode<'i, INHERITED> {
            #[doc = "A helper function to access [`XID_CONTINUE`]."]
            #[allow(non_snake_case)]
            pub fn r#XID_CONTINUE<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#XID_CONTINUE>
            {
                let res = &self.content;
                {
                    let res = &res.content.1.matched;
                    {
                        let res = res
                            .content
                            .iter()
                            .map(|res| {
                                let res = &res.matched;
                                res
                            })
                            .collect::<::pest_typed::re_exported::Vec<_>>();
                        res
                    }
                }
            }
            #[doc = "A helper function to access [`XID_START`]."]
            #[allow(non_snake_case)]
            pub fn r#XID_START<'s>(&'s self) -> &'s super::super::rules::r#XID_START {
                let res = &self.content;
                {
                    let res = &res.content.0.matched;
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#SYMBOL , "Corresponds to expression: `\"shadows builtin\"`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#SYMBOL , super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_10 > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#SYMBOL<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#han , "Corresponds to expression: `(HAN ~ HAN*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#han , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#HAN , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#HAN > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#han<'i, INHERITED> {
            #[doc = "A helper function to access [`HAN`]."]
            #[allow(non_snake_case)]
            pub fn r#HAN<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#HAN,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#HAN>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = res
                                    .content
                                    .iter()
                                    .map(|res| {
                                        let res = &res.matched;
                                        res
                                    })
                                    .collect::<::pest_typed::re_exported::Vec<_>>();
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#hangul , "Corresponds to expression: `(HANGUL ~ HANGUL*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#hangul , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#HANGUL , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#HANGUL > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#hangul<'i, INHERITED> {
            #[doc = "A helper function to access [`HANGUL`]."]
            #[allow(non_snake_case)]
            pub fn r#HANGUL<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#HANGUL,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#HANGUL>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = res
                                    .content
                                    .iter()
                                    .map(|res| {
                                        let res = &res.matched;
                                        res
                                    })
                                    .collect::<::pest_typed::re_exported::Vec<_>>();
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#hiragana , "Corresponds to expression: `(HIRAGANA ~ HIRAGANA*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#hiragana , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#HIRAGANA , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#HIRAGANA > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#hiragana<'i, INHERITED> {
            #[doc = "A helper function to access [`HIRAGANA`]."]
            #[allow(non_snake_case)]
            pub fn r#HIRAGANA<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#HIRAGANA,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#HIRAGANA>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = res
                                    .content
                                    .iter()
                                    .map(|res| {
                                        let res = &res.matched;
                                        res
                                    })
                                    .collect::<::pest_typed::re_exported::Vec<_>>();
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#arabic , "Corresponds to expression: `(ARABIC ~ ARABIC*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#arabic , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#ARABIC , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#ARABIC > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#arabic<'i, INHERITED> {
            #[doc = "A helper function to access [`ARABIC`]."]
            #[allow(non_snake_case)]
            pub fn r#ARABIC<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#ARABIC,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ARABIC>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = res
                                    .content
                                    .iter()
                                    .map(|res| {
                                        let res = &res.matched;
                                        res
                                    })
                                    .collect::<::pest_typed::re_exported::Vec<_>>();
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#emoji , "Corresponds to expression: `(EMOJI ~ EMOJI*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#emoji , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#EMOJI , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#EMOJI > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#emoji<'i, INHERITED> {
            #[doc = "A helper function to access [`EMOJI`]."]
            #[allow(non_snake_case)]
            pub fn r#EMOJI<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#EMOJI,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#EMOJI>,
            ) {
                let res = &self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            res
                        },
                        {
                            let res = &res.content.1.matched;
                            {
                                let res = res
                                    .content
                                    .iter()
                                    .map(|res| {
                                        let res = &res.matched;
                                        res
                                    })
                                    .collect::<::pest_typed::re_exported::Vec<_>>();
                                res
                            }
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#WHITESPACE , "Corresponds to expression: `\" \"`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#WHITESPACE , super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_11 > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Expression , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#WHITESPACE<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#COMMENT , "Corresponds to expression: `(\"$\" ~ \"$\"*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#COMMENT , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_12 > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_13 > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Expression , false);
        impl<'i, const INHERITED: ::core::primitive::usize> r#COMMENT<'i, INHERITED> {}
        #[allow(unused_imports)]
        use super::super::unicode::*;
        ::pest_typed::rule_eoi!(EOI, super::super::Rule);
        #[allow(non_camel_case_types)]
        pub use pest_typed::predefined_node::ASCII;
        #[allow(non_camel_case_types)]
        pub use pest_typed::predefined_node::ASCII_ALPHA;
        #[allow(non_camel_case_types)]
        pub use pest_typed::predefined_node::ASCII_ALPHANUMERIC;
        #[allow(non_camel_case_types)]
        pub use pest_typed::predefined_node::ASCII_ALPHA_LOWER;
        #[allow(non_camel_case_types)]
        pub use pest_typed::predefined_node::ASCII_ALPHA_UPPER;
        #[allow(non_camel_case_types)]
        pub use pest_typed::predefined_node::ASCII_BIN_DIGIT;
        #[allow(non_camel_case_types)]
        pub use pest_typed::predefined_node::ASCII_DIGIT;
        #[allow(non_camel_case_types)]
        pub use pest_typed::predefined_node::ASCII_HEX_DIGIT;
        #[allow(non_camel_case_types)]
        pub use pest_typed::predefined_node::ASCII_NONZERO_DIGIT;
        #[allow(non_camel_case_types)]
        pub use pest_typed::predefined_node::ASCII_OCT_DIGIT;
        #[allow(non_camel_case_types)]
        pub use pest_typed::predefined_node::DROP;
        #[allow(non_camel_case_types)]
        pub use pest_typed::predefined_node::NEWLINE;
        #[allow(non_camel_case_types)]
        pub use pest_typed::predefined_node::PEEK;
        #[allow(non_camel_case_types)]
        pub use pest_typed::predefined_node::PEEK_ALL;
        #[allow(non_camel_case_types)]
        pub use pest_typed::predefined_node::POP;
        #[allow(non_camel_case_types)]
        pub use pest_typed::predefined_node::POP_ALL;
        #[allow(non_camel_case_types)]
        pub use pest_typed::predefined_node::SOI;
    }
}
pub use rules_impl::rules;
#[doc = "Used generics."]
pub mod generics {
    use pest_typed::predefined_node;
    #[doc = r" Skipped content."]
    pub type Skipped<'i> = predefined_node::AtomicRep<
        ::pest_typed::choices::Choice2<
            super::rules::WHITESPACE<'i, 0>,
            super::rules::COMMENT<'i, 0>,
        >,
    >;
    pub use pest_typed::choices::Choice2;
    pub use pest_typed::choices::Choice3;
    pub use pest_typed::sequence::Seq2;
    pub use pest_typed::sequence::Seq3;
    pub use pest_typed::sequence::Seq4;
    pub use pest_typed::sequence::Seq6;
    pub use predefined_node::{
        CharRange, Insens, Negative, PeekSlice1, PeekSlice2, Positive, Push, Skip, Str,
    };
    #[doc = r" Repeat arbitrary times."]
    pub type Rep<'i, const SKIP: ::core::primitive::usize, T> =
        predefined_node::Rep<T, Skipped<'i>, SKIP>;
    #[doc = r" Repeat at least once."]
    pub type RepOnce<'i, const SKIP: ::core::primitive::usize, T> =
        predefined_node::RepOnce<T, Skipped<'i>, SKIP>;
}
#[doc = "Re-export some types from rules to simplify the usage."]
pub use rules as pairs;
