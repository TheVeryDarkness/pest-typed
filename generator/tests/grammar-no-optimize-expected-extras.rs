//! Test `grammar`.
#![cfg(feature = "grammar-extras")]
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
impl ::pest_typed::RuleType for Rule {
    fn name(&self) -> &'static ::core::primitive::str {
        match self {
            Self::r#string => "string",
            Self::r#insensitive => "insensitive",
            Self::r#range => "range",
            Self::r#ident => "ident",
            Self::r#pos_pred => "pos_pred",
            Self::r#neg_pred => "neg_pred",
            Self::r#double_neg_pred => "double_neg_pred",
            Self::r#sequence => "sequence",
            Self::r#sequence_compound => "sequence_compound",
            Self::r#sequence_atomic => "sequence_atomic",
            Self::r#sequence_non_atomic => "sequence_non_atomic",
            Self::r#sequence_atomic_compound => "sequence_atomic_compound",
            Self::r#sequence_nested => "sequence_nested",
            Self::r#sequence_compound_nested => "sequence_compound_nested",
            Self::r#node_tag => "node_tag",
            Self::r#choice => "choice",
            Self::r#choice_prefix => "choice_prefix",
            Self::r#optional => "optional",
            Self::r#repeat => "repeat",
            Self::r#repeat_atomic => "repeat_atomic",
            Self::r#repeat_once => "repeat_once",
            Self::r#repeat_once_atomic => "repeat_once_atomic",
            Self::r#repeat_min_max => "repeat_min_max",
            Self::r#repeat_min_max_atomic => "repeat_min_max_atomic",
            Self::r#repeat_exact => "repeat_exact",
            Self::r#repeat_min => "repeat_min",
            Self::r#repeat_min_atomic => "repeat_min_atomic",
            Self::r#repeat_max => "repeat_max",
            Self::r#repeat_max_atomic => "repeat_max_atomic",
            Self::r#soi_at_start => "soi_at_start",
            Self::r#repeat_mutate_stack => "repeat_mutate_stack",
            Self::r#repeat_mutate_stack_pop_all => "repeat_mutate_stack_pop_all",
            Self::r#will_fail => "will_fail",
            Self::r#stack_resume_after_fail => "stack_resume_after_fail",
            Self::r#peek_ => "peek_",
            Self::r#peek_all => "peek_all",
            Self::r#peek_slice_23 => "peek_slice_23",
            Self::r#pop_ => "pop_",
            Self::r#pop_all => "pop_all",
            Self::r#pop_fail => "pop_fail",
            Self::r#checkpoint_restore => "checkpoint_restore",
            Self::r#ascii_digits => "ascii_digits",
            Self::r#ascii_nonzero_digits => "ascii_nonzero_digits",
            Self::r#ascii_bin_digits => "ascii_bin_digits",
            Self::r#ascii_oct_digits => "ascii_oct_digits",
            Self::r#ascii_hex_digits => "ascii_hex_digits",
            Self::r#ascii_alpha_lowers => "ascii_alpha_lowers",
            Self::r#ascii_alpha_uppers => "ascii_alpha_uppers",
            Self::r#ascii_alphas => "ascii_alphas",
            Self::r#ascii_alphanumerics => "ascii_alphanumerics",
            Self::r#asciis => "asciis",
            Self::r#newline => "newline",
            Self::r#unicode => "unicode",
            Self::r#SYMBOL => "SYMBOL",
            Self::r#han => "han",
            Self::r#hangul => "hangul",
            Self::r#hiragana => "hiragana",
            Self::r#arabic => "arabic",
            Self::r#emoji => "emoji",
            Self::r#WHITESPACE => "WHITESPACE",
            Self::r#COMMENT => "COMMENT",
            Self::EOI => "EOI",
        }
    }
}
#[doc = "Unicode rules."]
pub mod unicode {
    #[allow(unused_imports)]
    pub use ::pest_typed::predefined_node::unicode::{
        ARABIC, EMOJI, HAN, HANGUL, HIRAGANA, XID_CONTINUE, XID_START,
    };
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
}
#[doc = "Generated structs for tags."]
pub mod tags {
    #[doc = "Tags inside rule [super::super::rules::r#node_tag]."]
    #[allow(non_snake_case)]
    pub mod r#node_tag {
        #[doc = "Tag r#string referenced by r#node_tag."]
        #[allow(non_camel_case_types)]
        pub type r#string<'s, S, const INHERITED: ::core::primitive::usize> =
            (&'s super::super::rules::r#node_tag<S>);
    }
}
#[doc = "Definitions of statically typed nodes generated by pest-generator."]
pub mod rules_impl {
    #[doc = "Definitions of statically typed nodes generated by pest-generator."]
    pub mod rules {
        :: pest_typed :: rule ! (pub r#string , "Corresponds to expression: `\"abc\"`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#string , super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_0 > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#string<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#insensitive , "Corresponds to expression: `^\"abc\"`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#insensitive , super :: super :: generics :: Insens :: < S , super :: super :: constant_wrappers :: r#w_1 > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#insensitive<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#range , "Corresponds to expression: `('0'..'9')`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#range , super :: super :: generics :: CharRange :: < '0' , '9' > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#range<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#ident , "Corresponds to expression: `string`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ident , super :: super :: rules :: r#string :: < S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#ident<S, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(&'s self) -> &'s super::super::rules::r#string<S, INHERITED> {
                let res = &*self.content;
                res
            }
        }
        :: pest_typed :: rule ! (pub r#pos_pred , "Corresponds to expression: `&string`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#pos_pred , super :: super :: generics :: Positive :: < super :: super :: rules :: r#string :: < S , INHERITED > > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#pos_pred<S, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(&'s self) -> &'s super::super::rules::r#string<S, INHERITED> {
                let res = &*self.content;
                {
                    let res = &res.content;
                    res
                }
            }
        }
        :: pest_typed :: rule ! (pub r#neg_pred , "Corresponds to expression: `!string`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#neg_pred , super :: super :: generics :: Negative :: < super :: super :: rules :: r#string :: < S , INHERITED > > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#neg_pred<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#double_neg_pred , "Corresponds to expression: `!!string`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#double_neg_pred , super :: super :: generics :: Negative :: < super :: super :: generics :: Negative :: < super :: super :: rules :: r#string :: < S , INHERITED > > > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#double_neg_pred<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#sequence , "Corresponds to expression: `(string ~ string)`. Non-atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#sequence , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < S , 1 > , super :: super :: generics :: Skipped < S > , 1 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < S , 1 > , super :: super :: generics :: Skipped < S > , 1 >) , > , super :: super :: generics :: Skipped :: < S > , false , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#sequence<S, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#string<S, 1>,
                &'s super::super::rules::r#string<S, 1>,
            ) {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#sequence_compound , "Corresponds to expression: `(string ~ string)`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#sequence_compound , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < S , 0 > , super :: super :: generics :: Skipped < S > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < S , 0 > , super :: super :: generics :: Skipped < S > , 0 >) , > , super :: super :: generics :: Skipped :: < S > , true , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#sequence_compound<S, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#string<S, 0>,
                &'s super::super::rules::r#string<S, 0>,
            ) {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#sequence_atomic , "Corresponds to expression: `(string ~ string)`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#sequence_atomic , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < S , 0 > , super :: super :: generics :: Skipped < S > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < S , 0 > , super :: super :: generics :: Skipped < S > , 0 >) , > , super :: super :: generics :: Skipped :: < S > , true , Span , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#sequence_atomic<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#sequence_non_atomic , "Corresponds to expression: `sequence`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#sequence_non_atomic , super :: super :: rules :: r#sequence :: < S , 0 > , super :: super :: generics :: Skipped :: < S > , true , Span , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#sequence_non_atomic<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#sequence_atomic_compound , "Corresponds to expression: `sequence_compound`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#sequence_atomic_compound , super :: super :: rules :: r#sequence_compound :: < S , 0 > , super :: super :: generics :: Skipped :: < S > , true , Span , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#sequence_atomic_compound<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#sequence_nested , "Corresponds to expression: `(string ~ string)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#sequence_nested , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#sequence_nested<S, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#string<S, INHERITED>,
                &'s super::super::rules::r#string<S, INHERITED>,
            ) {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#sequence_compound_nested , "Corresponds to expression: `sequence_nested`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#sequence_compound_nested , super :: super :: rules :: r#sequence_nested :: < S , 0 > , super :: super :: generics :: Skipped :: < S > , true , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#sequence_compound_nested<S, INHERITED> {
            #[doc = "A helper function to access [`sequence_nested`]."]
            #[allow(non_snake_case)]
            pub fn r#sequence_nested<'s>(
                &'s self,
            ) -> &'s super::super::rules::r#sequence_nested<S, 0> {
                let res = &*self.content;
                res
            }
        }
        :: pest_typed :: rule ! (pub r#node_tag , "Corresponds to expression: `(#string = string)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#node_tag , super :: super :: rules :: r#string :: < S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#node_tag<S, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(&'s self) -> &'s super::super::rules::r#string<S, INHERITED> {
                let res = &*self.content;
                res
            }
        }
        :: pest_typed :: rule ! (pub r#choice , "Corresponds to expression: `(string | range)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#choice , super :: super :: generics :: Choice2 :: < super :: super :: rules :: r#string :: < S , INHERITED > , super :: super :: rules :: r#range :: < S , INHERITED > , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#choice<S, INHERITED> {
            #[doc = "A helper function to access [`range`]."]
            #[allow(non_snake_case)]
            pub fn r#range<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#range<S, INHERITED>>
            {
                let res = &*self.content;
                {
                    let res = res._1().map(|res| res);
                    res
                }
            }
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#string<S, INHERITED>>
            {
                let res = &*self.content;
                {
                    let res = res._0().map(|res| res);
                    res
                }
            }
        }
        :: pest_typed :: rule ! (pub r#choice_prefix , "Corresponds to expression: `(string | range)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#choice_prefix , super :: super :: generics :: Choice2 :: < super :: super :: rules :: r#string :: < S , INHERITED > , super :: super :: rules :: r#range :: < S , INHERITED > , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#choice_prefix<S, INHERITED> {
            #[doc = "A helper function to access [`range`]."]
            #[allow(non_snake_case)]
            pub fn r#range<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#range<S, INHERITED>>
            {
                let res = &*self.content;
                {
                    let res = res._1().map(|res| res);
                    res
                }
            }
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#string<S, INHERITED>>
            {
                let res = &*self.content;
                {
                    let res = res._0().map(|res| res);
                    res
                }
            }
        }
        :: pest_typed :: rule ! (pub r#optional , "Corresponds to expression: `string?`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#optional , :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#string :: < S , INHERITED > > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#optional<S, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#string<S, INHERITED>>
            {
                let res = &*self.content;
                {
                    let res = res.as_ref().map(|res| res);
                    res
                }
            }
        }
        :: pest_typed :: rule ! (pub r#repeat , "Corresponds to expression: `string*`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat , super :: super :: generics :: Rep :: < super :: super :: rules :: r#string :: < S , INHERITED > , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#repeat<S, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#string<S, INHERITED>>
            {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#repeat_atomic , "Corresponds to expression: `string*`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_atomic , super :: super :: generics :: Rep :: < super :: super :: rules :: r#string :: < S , 0 > , S , 0 > , super :: super :: generics :: Skipped :: < S > , true , Span , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#repeat_atomic<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#repeat_once , "Corresponds to expression: `string+`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_once , super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#string :: < S , INHERITED > , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#repeat_once<S, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#string<S, INHERITED>>
            {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#repeat_once_atomic , "Corresponds to expression: `string+`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_once_atomic , super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#string :: < S , 0 > , S , 0 > , super :: super :: generics :: Skipped :: < S > , true , Span , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#repeat_once_atomic<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#repeat_min_max , "Corresponds to expression: `string{2, 3}`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_min_max , super :: super :: generics :: RepMinMax :: < super :: super :: rules :: r#string :: < S , INHERITED > , S , INHERITED , 2usize , 3usize > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#repeat_min_max<S, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#string<S, INHERITED>>
            {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#repeat_min_max_atomic , "Corresponds to expression: `string{2, 3}`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_min_max_atomic , super :: super :: generics :: RepMinMax :: < super :: super :: rules :: r#string :: < S , 0 > , S , 0 , 2usize , 3usize > , super :: super :: generics :: Skipped :: < S > , true , Span , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#repeat_min_max_atomic<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#repeat_exact , "Corresponds to expression: `string{2}`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_exact , super :: super :: generics :: RepExact :: < super :: super :: rules :: r#string :: < S , INHERITED > , S , INHERITED , 2usize > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#repeat_exact<S, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#string<S, INHERITED>>
            {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#repeat_min , "Corresponds to expression: `string{2,}`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_min , super :: super :: generics :: RepMin :: < super :: super :: rules :: r#string :: < S , INHERITED > , S , INHERITED , 2usize > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#repeat_min<S, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#string<S, INHERITED>>
            {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#repeat_min_atomic , "Corresponds to expression: `string{2,}`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_min_atomic , super :: super :: generics :: RepMin :: < super :: super :: rules :: r#string :: < S , 0 > , S , 0 , 2usize > , super :: super :: generics :: Skipped :: < S > , true , Span , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#repeat_min_atomic<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#repeat_max , "Corresponds to expression: `string{,2}`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_max , super :: super :: generics :: RepMax :: < super :: super :: rules :: r#string :: < S , INHERITED > , S , INHERITED , 2usize > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#repeat_max<S, INHERITED> {
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#string<S, INHERITED>>
            {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#repeat_max_atomic , "Corresponds to expression: `string{,2}`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_max_atomic , super :: super :: generics :: RepMax :: < super :: super :: rules :: r#string :: < S , 0 > , S , 0 , 2usize > , super :: super :: generics :: Skipped :: < S > , true , Span , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#repeat_max_atomic<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#soi_at_start , "Corresponds to expression: `(SOI ~ string)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#soi_at_start , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#SOI , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#string :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#soi_at_start<S, INHERITED> {
            #[doc = "A helper function to access [`SOI`]."]
            #[allow(non_snake_case)]
            pub fn r#SOI<'s>(&'s self) -> &'s super::super::rules::r#SOI {
                let res = &*self.content;
                {
                    let res = &res.content.0.matched;
                    res
                }
            }
            #[doc = "A helper function to access [`string`]."]
            #[allow(non_snake_case)]
            pub fn r#string<'s>(&'s self) -> &'s super::super::rules::r#string<S, INHERITED> {
                let res = &*self.content;
                {
                    let res = &res.content.1.matched;
                    res
                }
            }
        }
        :: pest_typed :: rule ! (pub r#repeat_mutate_stack , "Corresponds to expression: `((((PUSH(('a'..'c')) ~ \",\")* ~ POP) ~ POP) ~ POP)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_mutate_stack , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: generics :: CharRange :: < 'a' , 'c' > > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_2 > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP :: < S > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP :: < S > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP :: < S > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#repeat_mutate_stack<S, INHERITED> {
            #[doc = "A helper function to access [`POP`]."]
            #[allow(non_snake_case)]
            pub fn r#POP<'s>(
                &'s self,
            ) -> (
                (
                    &'s super::super::rules::r#POP<S>,
                    &'s super::super::rules::r#POP<S>,
                ),
                &'s super::super::rules::r#POP<S>,
            ) {
                let res = &*self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            {
                                let res = (
                                    {
                                        let res = &res.content.0.matched;
                                        {
                                            let res = &res.content.1.matched;
                                            res
                                        }
                                    },
                                    {
                                        let res = &res.content.1.matched;
                                        res
                                    },
                                );
                                res
                            }
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
        :: pest_typed :: rule ! (pub r#repeat_mutate_stack_pop_all , "Corresponds to expression: `((PUSH(('a'..'c')) ~ \",\")* ~ POP_ALL)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#repeat_mutate_stack_pop_all , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: generics :: CharRange :: < 'a' , 'c' > > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_3 > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP_ALL :: < S > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#repeat_mutate_stack_pop_all<S, INHERITED> {
            #[doc = "A helper function to access [`POP_ALL`]."]
            #[allow(non_snake_case)]
            pub fn r#POP_ALL<'s>(&'s self) -> &'s super::super::rules::r#POP_ALL<S> {
                let res = &*self.content;
                {
                    let res = &res.content.1.matched;
                    res
                }
            }
        }
        :: pest_typed :: rule ! (pub r#will_fail , "Corresponds to expression: `(repeat_mutate_stack_pop_all ~ \"FAIL\")`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#will_fail , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#repeat_mutate_stack_pop_all :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_4 > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#will_fail<S, INHERITED> {
            #[doc = "A helper function to access [`repeat_mutate_stack_pop_all`]."]
            #[allow(non_snake_case)]
            pub fn r#repeat_mutate_stack_pop_all<'s>(
                &'s self,
            ) -> &'s super::super::rules::r#repeat_mutate_stack_pop_all<S, INHERITED> {
                let res = &*self.content;
                {
                    let res = &res.content.0.matched;
                    res
                }
            }
        }
        :: pest_typed :: rule ! (pub r#stack_resume_after_fail , "Corresponds to expression: `(will_fail | repeat_mutate_stack_pop_all)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#stack_resume_after_fail , super :: super :: generics :: Choice2 :: < super :: super :: rules :: r#will_fail :: < S , INHERITED > , super :: super :: rules :: r#repeat_mutate_stack_pop_all :: < S , INHERITED > , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#stack_resume_after_fail<S, INHERITED> {
            #[doc = "A helper function to access [`repeat_mutate_stack_pop_all`]."]
            #[allow(non_snake_case)]
            pub fn r#repeat_mutate_stack_pop_all<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<
                &'s super::super::rules::r#repeat_mutate_stack_pop_all<S, INHERITED>,
            > {
                let res = &*self.content;
                {
                    let res = res._1().map(|res| res);
                    res
                }
            }
            #[doc = "A helper function to access [`will_fail`]."]
            #[allow(non_snake_case)]
            pub fn r#will_fail<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#will_fail<S, INHERITED>>
            {
                let res = &*self.content;
                {
                    let res = res._0().map(|res| res);
                    res
                }
            }
        }
        :: pest_typed :: rule ! (pub r#peek_ , "Corresponds to expression: `(((PUSH(range) ~ PUSH(range)) ~ PEEK) ~ PEEK)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#peek_ , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#PEEK :: < S > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#PEEK :: < S > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#peek_<S, INHERITED> {
            #[doc = "A helper function to access [`PEEK`]."]
            #[allow(non_snake_case)]
            pub fn r#PEEK<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#PEEK<S>,
                &'s super::super::rules::r#PEEK<S>,
            ) {
                let res = &*self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            {
                                let res = &res.content.1.matched;
                                res
                            }
                        },
                        {
                            let res = &res.content.1.matched;
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
                &'s super::super::rules::r#range<S, INHERITED>,
                &'s super::super::rules::r#range<S, INHERITED>,
            ) {
                let res = &*self.content;
                {
                    let res = &res.content.0.matched;
                    {
                        let res = &res.content.0.matched;
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
            }
        }
        :: pest_typed :: rule ! (pub r#peek_all , "Corresponds to expression: `((PUSH(range) ~ PUSH(range)) ~ PEEK_ALL)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#peek_all , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#PEEK_ALL :: < S > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#peek_all<S, INHERITED> {
            #[doc = "A helper function to access [`PEEK_ALL`]."]
            #[allow(non_snake_case)]
            pub fn r#PEEK_ALL<'s>(&'s self) -> &'s super::super::rules::r#PEEK_ALL<S> {
                let res = &*self.content;
                {
                    let res = &res.content.1.matched;
                    res
                }
            }
            #[doc = "A helper function to access [`range`]."]
            #[allow(non_snake_case)]
            pub fn r#range<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#range<S, INHERITED>,
                &'s super::super::rules::r#range<S, INHERITED>,
            ) {
                let res = &*self.content;
                {
                    let res = &res.content.0.matched;
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
        }
        :: pest_typed :: rule ! (pub r#peek_slice_23 , "Corresponds to expression: `(((((PUSH(range) ~ PUSH(range)) ~ PUSH(range)) ~ PUSH(range)) ~ PUSH(range)) ~ PEEK[1..-2])`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#peek_slice_23 , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: PeekSlice2 :: < 1i32 , - 2i32 > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#peek_slice_23<S, INHERITED> {
            #[doc = "A helper function to access [`range`]."]
            #[allow(non_snake_case)]
            pub fn r#range<'s>(
                &'s self,
            ) -> (
                (
                    (
                        (
                            &'s super::super::rules::r#range<S, INHERITED>,
                            &'s super::super::rules::r#range<S, INHERITED>,
                        ),
                        &'s super::super::rules::r#range<S, INHERITED>,
                    ),
                    &'s super::super::rules::r#range<S, INHERITED>,
                ),
                &'s super::super::rules::r#range<S, INHERITED>,
            ) {
                let res = &*self.content;
                {
                    let res = &res.content.0.matched;
                    {
                        let res = (
                            {
                                let res = &res.content.0.matched;
                                {
                                    let res = (
                                        {
                                            let res = &res.content.0.matched;
                                            {
                                                let res = (
                                                    {
                                                        let res = &res.content.0.matched;
                                                        {
                                                            let res = (
                                                                {
                                                                    let res =
                                                                        &res.content.0.matched;
                                                                    {
                                                                        let res = &res.content;
                                                                        res
                                                                    }
                                                                },
                                                                {
                                                                    let res =
                                                                        &res.content.1.matched;
                                                                    {
                                                                        let res = &res.content;
                                                                        res
                                                                    }
                                                                },
                                                            );
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
        }
        :: pest_typed :: rule ! (pub r#pop_ , "Corresponds to expression: `(((PUSH(range) ~ PUSH(range)) ~ POP) ~ POP)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#pop_ , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP :: < S > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP :: < S > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#pop_<S, INHERITED> {
            #[doc = "A helper function to access [`POP`]."]
            #[allow(non_snake_case)]
            pub fn r#POP<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#POP<S>,
                &'s super::super::rules::r#POP<S>,
            ) {
                let res = &*self.content;
                {
                    let res = (
                        {
                            let res = &res.content.0.matched;
                            {
                                let res = &res.content.1.matched;
                                res
                            }
                        },
                        {
                            let res = &res.content.1.matched;
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
                &'s super::super::rules::r#range<S, INHERITED>,
                &'s super::super::rules::r#range<S, INHERITED>,
            ) {
                let res = &*self.content;
                {
                    let res = &res.content.0.matched;
                    {
                        let res = &res.content.0.matched;
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
            }
        }
        :: pest_typed :: rule ! (pub r#pop_all , "Corresponds to expression: `((PUSH(range) ~ PUSH(range)) ~ POP_ALL)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#pop_all , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP_ALL :: < S > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#pop_all<S, INHERITED> {
            #[doc = "A helper function to access [`POP_ALL`]."]
            #[allow(non_snake_case)]
            pub fn r#POP_ALL<'s>(&'s self) -> &'s super::super::rules::r#POP_ALL<S> {
                let res = &*self.content;
                {
                    let res = &res.content.1.matched;
                    res
                }
            }
            #[doc = "A helper function to access [`range`]."]
            #[allow(non_snake_case)]
            pub fn r#range<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#range<S, INHERITED>,
                &'s super::super::rules::r#range<S, INHERITED>,
            ) {
                let res = &*self.content;
                {
                    let res = &res.content.0.matched;
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
        }
        :: pest_typed :: rule ! (pub r#pop_fail , "Corresponds to expression: `(((PUSH(range) ~ !POP) ~ range) ~ POP)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#pop_fail , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#range :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Negative :: < super :: super :: rules :: r#POP :: < S > > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#range :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP :: < S > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#pop_fail<S, INHERITED> {
            #[doc = "A helper function to access [`POP`]."]
            #[allow(non_snake_case)]
            pub fn r#POP<'s>(&'s self) -> &'s super::super::rules::r#POP<S> {
                let res = &*self.content;
                {
                    let res = &res.content.1.matched;
                    res
                }
            }
            #[doc = "A helper function to access [`range`]."]
            #[allow(non_snake_case)]
            pub fn r#range<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#range<S, INHERITED>,
                &'s super::super::rules::r#range<S, INHERITED>,
            ) {
                let res = &*self.content;
                {
                    let res = &res.content.0.matched;
                    {
                        let res = (
                            {
                                let res = &res.content.0.matched;
                                {
                                    let res = &res.content.0.matched;
                                    {
                                        let res = &res.content;
                                        res
                                    }
                                }
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
        }
        :: pest_typed :: rule ! (pub r#checkpoint_restore , "Corresponds to expression: `((PUSH(\"\") ~ ((((PUSH(\"a\") ~ \"b\") ~ POP) | (DROP ~ \"b\")) | (POP ~ \"a\"))) ~ EOI)`. Atomic rule." "" , super :: super :: Rule , super :: super :: Rule :: r#checkpoint_restore , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_5 > > , super :: super :: generics :: Skipped < S > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Choice2 :: < super :: super :: generics :: Choice2 :: < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_6 > > , super :: super :: generics :: Skipped < S > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_7 > , super :: super :: generics :: Skipped < S > , 0 >) , > , super :: super :: generics :: Skipped < S > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP :: < S > , super :: super :: generics :: Skipped < S > , 0 >) , > , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#DROP , super :: super :: generics :: Skipped < S > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_8 > , super :: super :: generics :: Skipped < S > , 0 >) , > , > , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#POP :: < S > , super :: super :: generics :: Skipped < S > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_9 > , super :: super :: generics :: Skipped < S > , 0 >) , > , > , super :: super :: generics :: Skipped < S > , 0 >) , > , super :: super :: generics :: Skipped < S > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#EOI :: < S > , super :: super :: generics :: Skipped < S > , 0 >) , > , super :: super :: generics :: Skipped :: < S > , true , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#checkpoint_restore<S, INHERITED> {
            #[doc = "A helper function to access [`DROP`]."]
            #[allow(non_snake_case)]
            pub fn r#DROP<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#DROP> {
                let res = &*self.content;
                {
                    let res = &res.content.0.matched;
                    {
                        let res = &res.content.1.matched;
                        {
                            let res = res
                                ._0()
                                .map(|res| {
                                    let res = res._1().map(|res| {
                                        let res = &res.content.0.matched;
                                        res
                                    });
                                    res
                                })
                                .flatten();
                            res
                        }
                    }
                }
            }
            #[doc = "A helper function to access [`EOI`]."]
            #[allow(non_snake_case)]
            pub fn r#EOI<'s>(&'s self) -> &'s super::super::rules::r#EOI<S> {
                let res = &*self.content;
                {
                    let res = &res.content.1.matched;
                    res
                }
            }
            #[doc = "A helper function to access [`POP`]."]
            #[allow(non_snake_case)]
            pub fn r#POP<'s>(
                &'s self,
            ) -> (
                ::pest_typed::re_exported::Option<&'s super::super::rules::r#POP<S>>,
                ::pest_typed::re_exported::Option<&'s super::super::rules::r#POP<S>>,
            ) {
                let res = &*self.content;
                {
                    let res = &res.content.0.matched;
                    {
                        let res = &res.content.1.matched;
                        {
                            let res = (
                                {
                                    let res = res
                                        ._0()
                                        .map(|res| {
                                            let res = res._0().map(|res| {
                                                let res = &res.content.1.matched;
                                                res
                                            });
                                            res
                                        })
                                        .flatten();
                                    res
                                },
                                {
                                    let res = res._1().map(|res| {
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
        }
        :: pest_typed :: rule ! (pub r#ascii_digits , "Corresponds to expression: `ASCII_DIGIT+`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ascii_digits , super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#ASCII_DIGIT , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#ascii_digits<S, INHERITED> {
            #[doc = "A helper function to access [`ASCII_DIGIT`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII_DIGIT<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII_DIGIT>
            {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#ascii_nonzero_digits , "Corresponds to expression: `ASCII_NONZERO_DIGIT+`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ascii_nonzero_digits , super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#ASCII_NONZERO_DIGIT , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#ascii_nonzero_digits<S, INHERITED> {
            #[doc = "A helper function to access [`ASCII_NONZERO_DIGIT`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII_NONZERO_DIGIT<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII_NONZERO_DIGIT>
            {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#ascii_bin_digits , "Corresponds to expression: `ASCII_BIN_DIGIT+`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ascii_bin_digits , super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#ASCII_BIN_DIGIT , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#ascii_bin_digits<S, INHERITED> {
            #[doc = "A helper function to access [`ASCII_BIN_DIGIT`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII_BIN_DIGIT<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII_BIN_DIGIT>
            {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#ascii_oct_digits , "Corresponds to expression: `ASCII_OCT_DIGIT+`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ascii_oct_digits , super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#ASCII_OCT_DIGIT , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#ascii_oct_digits<S, INHERITED> {
            #[doc = "A helper function to access [`ASCII_OCT_DIGIT`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII_OCT_DIGIT<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII_OCT_DIGIT>
            {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#ascii_hex_digits , "Corresponds to expression: `ASCII_HEX_DIGIT+`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ascii_hex_digits , super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#ASCII_HEX_DIGIT , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#ascii_hex_digits<S, INHERITED> {
            #[doc = "A helper function to access [`ASCII_HEX_DIGIT`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII_HEX_DIGIT<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII_HEX_DIGIT>
            {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#ascii_alpha_lowers , "Corresponds to expression: `ASCII_ALPHA_LOWER+`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ascii_alpha_lowers , super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#ASCII_ALPHA_LOWER , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#ascii_alpha_lowers<S, INHERITED> {
            #[doc = "A helper function to access [`ASCII_ALPHA_LOWER`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII_ALPHA_LOWER<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII_ALPHA_LOWER>
            {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#ascii_alpha_uppers , "Corresponds to expression: `ASCII_ALPHA_UPPER+`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ascii_alpha_uppers , super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#ASCII_ALPHA_UPPER , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#ascii_alpha_uppers<S, INHERITED> {
            #[doc = "A helper function to access [`ASCII_ALPHA_UPPER`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII_ALPHA_UPPER<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII_ALPHA_UPPER>
            {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#ascii_alphas , "Corresponds to expression: `ASCII_ALPHA+`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ascii_alphas , super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#ASCII_ALPHA , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#ascii_alphas<S, INHERITED> {
            #[doc = "A helper function to access [`ASCII_ALPHA`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII_ALPHA<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII_ALPHA>
            {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#ascii_alphanumerics , "Corresponds to expression: `ASCII_ALPHANUMERIC+`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ascii_alphanumerics , super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#ASCII_ALPHANUMERIC , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#ascii_alphanumerics<S, INHERITED> {
            #[doc = "A helper function to access [`ASCII_ALPHANUMERIC`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII_ALPHANUMERIC<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII_ALPHANUMERIC>
            {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#asciis , "Corresponds to expression: `ASCII+`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#asciis , super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#ASCII , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#asciis<S, INHERITED> {
            #[doc = "A helper function to access [`ASCII`]."]
            #[allow(non_snake_case)]
            pub fn r#ASCII<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ASCII> {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#newline , "Corresponds to expression: `NEWLINE+`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#newline , super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#NEWLINE , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#newline<S, INHERITED> {
            #[doc = "A helper function to access [`NEWLINE`]."]
            #[allow(non_snake_case)]
            pub fn r#NEWLINE<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#NEWLINE> {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#unicode , "Corresponds to expression: `(XID_START ~ XID_CONTINUE*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#unicode , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#XID_START , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < super :: super :: rules :: r#XID_CONTINUE , S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#unicode<S, INHERITED> {
            #[doc = "A helper function to access [`XID_CONTINUE`]."]
            #[allow(non_snake_case)]
            pub fn r#XID_CONTINUE<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#XID_CONTINUE>
            {
                let res = &*self.content;
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
                let res = &*self.content;
                {
                    let res = &res.content.0.matched;
                    res
                }
            }
        }
        :: pest_typed :: rule ! (pub r#SYMBOL , "Corresponds to expression: `\"shadows builtin\"`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#SYMBOL , super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_10 > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#SYMBOL<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#han , "Corresponds to expression: `HAN+`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#han , super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#HAN , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#han<S, INHERITED> {
            #[doc = "A helper function to access [`HAN`]."]
            #[allow(non_snake_case)]
            pub fn r#HAN<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#HAN> {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#hangul , "Corresponds to expression: `HANGUL+`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#hangul , super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#HANGUL , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#hangul<S, INHERITED> {
            #[doc = "A helper function to access [`HANGUL`]."]
            #[allow(non_snake_case)]
            pub fn r#HANGUL<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#HANGUL> {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#hiragana , "Corresponds to expression: `HIRAGANA+`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#hiragana , super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#HIRAGANA , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#hiragana<S, INHERITED> {
            #[doc = "A helper function to access [`HIRAGANA`]."]
            #[allow(non_snake_case)]
            pub fn r#HIRAGANA<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#HIRAGANA> {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#arabic , "Corresponds to expression: `ARABIC+`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#arabic , super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#ARABIC , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#arabic<S, INHERITED> {
            #[doc = "A helper function to access [`ARABIC`]."]
            #[allow(non_snake_case)]
            pub fn r#ARABIC<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ARABIC> {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#emoji , "Corresponds to expression: `EMOJI+`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#emoji , super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#EMOJI , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#emoji<S, INHERITED> {
            #[doc = "A helper function to access [`EMOJI`]."]
            #[allow(non_snake_case)]
            pub fn r#EMOJI<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#EMOJI> {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (pub r#WHITESPACE , "Corresponds to expression: `\" \"`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#WHITESPACE , super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_11 > , super :: super :: generics :: Skipped :: < S > , INHERITED , Expression , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#WHITESPACE<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#COMMENT , "Corresponds to expression: `\"$\"+`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#COMMENT , super :: super :: generics :: RepOnce :: < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_12 > , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Expression , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#COMMENT<S, INHERITED> {}
        #[allow(unused_imports)]
        use super::super::unicode::*;
        :: pest_typed :: rule_eoi ! (pub EOI , super :: super :: Rule);
        pub use ::pest_typed::predefined_node::ASCII;
        pub use ::pest_typed::predefined_node::ASCII_ALPHA;
        pub use ::pest_typed::predefined_node::ASCII_ALPHANUMERIC;
        pub use ::pest_typed::predefined_node::ASCII_ALPHA_LOWER;
        pub use ::pest_typed::predefined_node::ASCII_ALPHA_UPPER;
        pub use ::pest_typed::predefined_node::ASCII_BIN_DIGIT;
        pub use ::pest_typed::predefined_node::ASCII_DIGIT;
        pub use ::pest_typed::predefined_node::ASCII_HEX_DIGIT;
        pub use ::pest_typed::predefined_node::ASCII_NONZERO_DIGIT;
        pub use ::pest_typed::predefined_node::ASCII_OCT_DIGIT;
        pub use ::pest_typed::predefined_node::DROP;
        pub use ::pest_typed::predefined_node::NEWLINE;
        pub use ::pest_typed::predefined_node::PEEK;
        pub use ::pest_typed::predefined_node::PEEK_ALL;
        pub use ::pest_typed::predefined_node::POP;
        pub use ::pest_typed::predefined_node::POP_ALL;
        pub use ::pest_typed::predefined_node::SOI;
    }
}
pub use rules_impl::rules;
#[doc = "Used generics."]
pub mod generics {
    use ::pest_typed::predefined_node;
    #[doc = r" Skipped content."]
    pub type Skipped<S> = predefined_node::AtomicRepeat<
        ::pest_typed::choices::Choice2<super::rules::WHITESPACE<S, 0>, super::rules::COMMENT<S, 0>>,
    >;
    pub use pest_typed::choices::Choice2;
    pub use pest_typed::sequence::Seq2;
    pub use predefined_node::{
        CharRange, Insens, Negative, PeekSlice1, PeekSlice2, Positive, Push, PushLiteral, Skip, Str,
    };
    #[doc = r" Repeat arbitrary times."]
    pub type Rep<T, S, const SKIP: ::core::primitive::usize> =
        predefined_node::Rep<T, Skipped<S>, SKIP>;
    #[doc = r" Repeat at least once."]
    pub type RepOnce<T, S, const SKIP: ::core::primitive::usize> =
        predefined_node::RepOnce<T, Skipped<S>, SKIP>;
    #[doc = r" Repeat at least `MIN` times."]
    pub type RepMin<
        T,
        S,
        const SKIP: ::core::primitive::usize,
        const MIN: ::core::primitive::usize,
    > = predefined_node::RepMin<T, Skipped<S>, SKIP, MIN>;
    #[doc = r" Repeat at most `MAX` times."]
    pub type RepMax<
        T,
        S,
        const SKIP: ::core::primitive::usize,
        const MAX: ::core::primitive::usize,
    > = predefined_node::RepMax<T, Skipped<S>, SKIP, MAX>;
    #[doc = r" Repeat between `MIN` and `MAX` times."]
    pub type RepMinMax<
        T,
        S,
        const SKIP: ::core::primitive::usize,
        const MIN: ::core::primitive::usize,
        const MAX: ::core::primitive::usize,
    > = predefined_node::RepMinMax<T, Skipped<S>, SKIP, MIN, MAX>;
    #[doc = r" Repeat exactly `TIMES` times."]
    pub type RepExact<
        T,
        S,
        const SKIP: ::core::primitive::usize,
        const TIMES: ::core::primitive::usize,
    > = predefined_node::RepExact<T, Skipped<S>, SKIP, TIMES>;
}
#[doc = "Re-export some types from rules to simplify the usage."]
pub use rules as pairs;
