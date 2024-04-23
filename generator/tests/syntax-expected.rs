#![cfg(not(feature = "grammar-extras"))]
#![allow(unused_parens)]
#[doc = ""]
#[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    #[doc = "Regular rule."]
    r#Regular,
    #[doc = "Atomic rule."]
    r#Atomic,
    #[doc = "Silent rule."]
    r#Silent,
    #[doc = "Compound atomic rule."]
    r#CompoundAtomic,
    #[doc = "Tagged rule."]
    r#Tag,
    #[doc = "Non-atomic rule."]
    r#NonAtomic,
    r#ExactString,
    r#CaseInsensitive,
    r#CharRange,
    r#Any,
    r#Seq,
    r#Choice,
    r#Rep,
    r#RepAtLeastOnce,
    r#Opt,
    r#RepExact,
    r#RepLeft,
    r#RepRight,
    r#RepLeftRight,
    r#Pos,
    r#Neg,
    r#Push,
    r#Pop,
    r#PopAll,
    r#Peek,
    r#PeekLeft,
    r#PeekRight,
    r#PeekLeftRight,
    r#Drop,
    r#PeekAll,
}
#[doc = "Unicode rules."]
pub mod unicode {}
mod constant_wrappers {
    #[doc = "A wrapper for `\"+\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_0;
    impl ::pest_typed::StringWrapper for r#w_0 {
        const CONTENT: &'static ::core::primitive::str = "+";
    }
    #[doc = "A wrapper for `\"(\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_1;
    impl ::pest_typed::StringWrapper for r#w_1 {
        const CONTENT: &'static ::core::primitive::str = "(";
    }
    #[doc = "A wrapper for `\")\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_2;
    impl ::pest_typed::StringWrapper for r#w_2 {
        const CONTENT: &'static ::core::primitive::str = ")";
    }
    #[doc = "A wrapper for `\"\\\"\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_3;
    impl ::pest_typed::StringWrapper for r#w_3 {
        const CONTENT: &'static ::core::primitive::str = "\"";
    }
    #[doc = "A wrapper for `\"\\\"\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_4;
    impl ::pest_typed::StringWrapper for r#w_4 {
        const CONTENT: &'static ::core::primitive::str = "\"";
    }
    #[doc = "A wrapper for `\"\\\"\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_5;
    impl ::pest_typed::StringWrapper for r#w_5 {
        const CONTENT: &'static ::core::primitive::str = "\"";
    }
    #[doc = "A wrapper for `\"b\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_6;
    impl ::pest_typed::StringWrapper for r#w_6 {
        const CONTENT: &'static ::core::primitive::str = "b";
    }
    #[doc = "A wrapper for `\"BB\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_7;
    impl ::pest_typed::StringWrapper for r#w_7 {
        const CONTENT: &'static ::core::primitive::str = "BB";
    }
    #[doc = "A wrapper for `\"b\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_8;
    impl ::pest_typed::StringWrapper for r#w_8 {
        const CONTENT: &'static ::core::primitive::str = "b";
    }
    #[doc = "A wrapper for `\"c\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_9;
    impl ::pest_typed::StringWrapper for r#w_9 {
        const CONTENT: &'static ::core::primitive::str = "c";
    }
    #[doc = "A wrapper for `\"r#\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_10;
    impl ::pest_typed::StringWrapper for r#w_10 {
        const CONTENT: &'static ::core::primitive::str = "r#";
    }
    #[doc = "A wrapper for `\"0x\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_11;
    impl ::pest_typed::StringWrapper for r#w_11 {
        const CONTENT: &'static ::core::primitive::str = "0x";
    }
    #[doc = "A wrapper for `\"1\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_12;
    impl ::pest_typed::StringWrapper for r#w_12 {
        const CONTENT: &'static ::core::primitive::str = "1";
    }
    #[doc = "A wrapper for `\".\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_13;
    impl ::pest_typed::StringWrapper for r#w_13 {
        const CONTENT: &'static ::core::primitive::str = ".";
    }
    #[doc = "A wrapper for `\"a\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_14;
    impl ::pest_typed::StringWrapper for r#w_14 {
        const CONTENT: &'static ::core::primitive::str = "a";
    }
    #[doc = "A wrapper for `\"b\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_15;
    impl ::pest_typed::StringWrapper for r#w_15 {
        const CONTENT: &'static ::core::primitive::str = "b";
    }
    #[doc = "A wrapper for `\"b\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_16;
    impl ::pest_typed::StringWrapper for r#w_16 {
        const CONTENT: &'static ::core::primitive::str = "b";
    }
    #[doc = "A wrapper for `\"c\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_17;
    impl ::pest_typed::StringWrapper for r#w_17 {
        const CONTENT: &'static ::core::primitive::str = "c";
    }
    #[doc = "A wrapper for `\"b\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_18;
    impl ::pest_typed::StringWrapper for r#w_18 {
        const CONTENT: &'static ::core::primitive::str = "b";
    }
    #[doc = "A wrapper for `\"?\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_19;
    impl ::pest_typed::StringWrapper for r#w_19 {
        const CONTENT: &'static ::core::primitive::str = "?";
    }
}
#[doc = "Definitions of statically typed nodes generated by pest-generator."]
pub mod rules_impl {
    #[doc = "Definitions of statically typed nodes generated by pest-generator."]
    pub mod rules {
        :: pest_typed :: rule ! (r#Regular , "Corresponds to expression: `((CharRange ~ CharRange*) ~ \"+\" ~ CharRange ~ CharRange*)`. Normal rule." "" "Regular rule." , super :: super :: Rule , super :: super :: Rule :: r#Regular , super :: super :: generics :: Seq4 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#CharRange :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#CharRange :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_0 > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#CharRange :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#CharRange :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#Regular<'i, INHERITED> {
            #[doc = "A helper function to access [`CharRange`]."]
            #[allow(non_snake_case)]
            pub fn r#CharRange<'s>(
                &'s self,
            ) -> (
                (
                    &'s super::super::rules::r#CharRange<'i, INHERITED>,
                    ::pest_typed::re_exported::Vec<
                        &'s super::super::rules::r#CharRange<'i, INHERITED>,
                    >,
                ),
                &'s super::super::rules::r#CharRange<'i, INHERITED>,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#CharRange<'i, INHERITED>>,
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
                        },
                        {
                            let res = &res.content.2.matched;
                            res
                        },
                        {
                            let res = &res.content.3.matched;
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
        :: pest_typed :: rule ! (r#Atomic , "Corresponds to expression: `(('0'..'9') ~ ('0'..'9')*)`. Atomic rule." "" "Atomic rule." , super :: super :: Rule , super :: super :: Rule :: r#Atomic , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: CharRange :: < '0' , '9' > , super :: super :: generics :: Skipped < 'i > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , 0 , super :: super :: generics :: CharRange :: < '0' , '9' > > , super :: super :: generics :: Skipped < 'i > , 0 >) , > , super :: super :: generics :: Skipped :: < 'i > , true , Span , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#Atomic<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#Silent , "Corresponds to expression: `(\"(\" | \")\")`. Normal rule." "" "Silent rule." , super :: super :: Rule , super :: super :: Rule :: r#Silent , super :: super :: generics :: Choice2 :: < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_1 > , super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_2 > , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Expression , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#Silent<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#CompoundAtomic , "Corresponds to expression: `(\"\\\"\" ~ (!\"\\\"\" ~ ANY)* ~ \"\\\"\")`. Atomic rule." "" "Compound atomic rule." , super :: super :: Rule , super :: super :: Rule :: r#CompoundAtomic , super :: super :: generics :: Seq3 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_3 > , super :: super :: generics :: Skipped < 'i > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , 0 , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Negative :: < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_4 > > , super :: super :: generics :: Skipped < 'i > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#ANY , super :: super :: generics :: Skipped < 'i > , 0 >) , > > , super :: super :: generics :: Skipped < 'i > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_5 > , super :: super :: generics :: Skipped < 'i > , 0 >) , > , super :: super :: generics :: Skipped :: < 'i > , true , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#CompoundAtomic<'i, INHERITED> {
            #[doc = "A helper function to access [`ANY`]."]
            #[allow(non_snake_case)]
            pub fn r#ANY<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ANY> {
                let res = &*self.content;
                {
                    let res = &res.content.1.matched;
                    {
                        let res = res
                            .content
                            .iter()
                            .map(|res| {
                                let res = &res.matched;
                                {
                                    let res = &res.content.1.matched;
                                    res
                                }
                            })
                            .collect::<::pest_typed::re_exported::Vec<_>>();
                        res
                    }
                }
            }
        }
        :: pest_typed :: rule ! (r#Tag , "Corresponds to expression: `(Any ~ (PUSH(CompoundAtomic) ~ Any ~ &Silent ~ !Atomic)? ~ Any ~ (NonAtomic* ~ Regular ~ Regular*)?)`. Normal rule." "" "Tagged rule." , super :: super :: Rule , super :: super :: Rule :: r#Tag , super :: super :: generics :: Seq4 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Any :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: generics :: Seq4 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#CompoundAtomic :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Any :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Positive :: < super :: super :: rules :: r#Silent :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Negative :: < super :: super :: rules :: r#Atomic :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Any :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: generics :: Seq3 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#NonAtomic :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Regular :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#Regular :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#Tag<'i, INHERITED> {
            #[doc = "A helper function to access [`Any`]."]
            #[allow(non_snake_case)]
            pub fn r#Any<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#Any<'i, INHERITED>,
                ::pest_typed::re_exported::Option<&'s super::super::rules::r#Any<'i, INHERITED>>,
                &'s super::super::rules::r#Any<'i, INHERITED>,
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
                            {
                                let res = res.as_ref().map(|res| {
                                    let res = &res.content.1.matched;
                                    res
                                });
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
            #[doc = "A helper function to access [`CompoundAtomic`]."]
            #[allow(non_snake_case)]
            pub fn r#CompoundAtomic<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<
                &'s super::super::rules::r#CompoundAtomic<'i, INHERITED>,
            > {
                let res = &*self.content;
                {
                    let res = &res.content.1.matched;
                    {
                        let res = res.as_ref().map(|res| {
                            let res = &res.content.0.matched;
                            {
                                let res = &res.content;
                                res
                            }
                        });
                        res
                    }
                }
            }
            #[doc = "A helper function to access [`NonAtomic`]."]
            #[allow(non_snake_case)]
            pub fn r#NonAtomic<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#NonAtomic<'i, INHERITED>>,
            > {
                let res = &*self.content;
                {
                    let res = &res.content.3.matched;
                    {
                        let res = res.as_ref().map(|res| {
                            let res = &res.content.0.matched;
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
                        });
                        res
                    }
                }
            }
            #[doc = "A helper function to access [`Regular`]."]
            #[allow(non_snake_case)]
            pub fn r#Regular<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<(
                &'s super::super::rules::r#Regular<'i, INHERITED>,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#Regular<'i, INHERITED>>,
            )> {
                let res = &*self.content;
                {
                    let res = &res.content.3.matched;
                    {
                        let res = res.as_ref().map(|res| {
                            let res = (
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
                        });
                        res
                    }
                }
            }
            #[doc = "A helper function to access [`Silent`]."]
            #[allow(non_snake_case)]
            pub fn r#Silent<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#Silent<'i, INHERITED>>
            {
                let res = &*self.content;
                {
                    let res = &res.content.1.matched;
                    {
                        let res = res.as_ref().map(|res| {
                            let res = &res.content.2.matched;
                            {
                                let res = &res.content;
                                res
                            }
                        });
                        res
                    }
                }
            }
        }
        :: pest_typed :: rule ! (r#NonAtomic , "Corresponds to expression: `((CaseInsensitive? ~ (CharRange ~ CharRange*) ~ Tag*)? ~ (\"b\" | (\"BB\" ~ \"b\"?))? ~ ^\"c\"* ~ (!Seq ~ Any) ~ (!Seq ~ Any)*)`. Non-atomic rule." "" "Non-atomic rule." , super :: super :: Rule , super :: super :: Rule :: r#NonAtomic , super :: super :: generics :: Seq5 :: < (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: generics :: Seq3 :: < (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#CaseInsensitive :: < 'i , 1 > > , super :: super :: generics :: Skipped < 'i > , 1 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#CharRange :: < 'i , 1 > , super :: super :: generics :: Skipped < 'i > , 1 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , 1 , super :: super :: rules :: r#CharRange :: < 'i , 1 > > , super :: super :: generics :: Skipped < 'i > , 1 >) , > , super :: super :: generics :: Skipped < 'i > , 1 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , 1 , super :: super :: rules :: r#Tag :: < 'i , 1 > > , super :: super :: generics :: Skipped < 'i > , 1 >) , > > , super :: super :: generics :: Skipped < 'i > , 1 >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: generics :: Choice2 :: < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_6 > , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_7 > , super :: super :: generics :: Skipped < 'i > , 1 >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_8 > > , super :: super :: generics :: Skipped < 'i > , 1 >) , > , > > , super :: super :: generics :: Skipped < 'i > , 1 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , 1 , super :: super :: generics :: Insens :: < 'i , super :: super :: constant_wrappers :: r#w_9 > > , super :: super :: generics :: Skipped < 'i > , 1 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Negative :: < super :: super :: rules :: r#Seq :: < 'i , 1 > > , super :: super :: generics :: Skipped < 'i > , 1 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Any :: < 'i , 1 > , super :: super :: generics :: Skipped < 'i > , 1 >) , > , super :: super :: generics :: Skipped < 'i > , 1 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , 1 , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Negative :: < super :: super :: rules :: r#Seq :: < 'i , 1 > > , super :: super :: generics :: Skipped < 'i > , 1 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Any :: < 'i , 1 > , super :: super :: generics :: Skipped < 'i > , 1 >) , > > , super :: super :: generics :: Skipped < 'i > , 1 >) , > , super :: super :: generics :: Skipped :: < 'i > , false , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#NonAtomic<'i, INHERITED> {
            #[doc = "A helper function to access [`Any`]."]
            #[allow(non_snake_case)]
            pub fn r#Any<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#Any<'i, 1>,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#Any<'i, 1>>,
            ) {
                let res = &*self.content;
                {
                    let res = (
                        {
                            let res = &res.content.3.matched;
                            {
                                let res = &res.content.1.matched;
                                res
                            }
                        },
                        {
                            let res = &res.content.4.matched;
                            {
                                let res = res
                                    .content
                                    .iter()
                                    .map(|res| {
                                        let res = &res.matched;
                                        {
                                            let res = &res.content.1.matched;
                                            res
                                        }
                                    })
                                    .collect::<::pest_typed::re_exported::Vec<_>>();
                                res
                            }
                        },
                    );
                    res
                }
            }
            #[doc = "A helper function to access [`CaseInsensitive`]."]
            #[allow(non_snake_case)]
            pub fn r#CaseInsensitive<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#CaseInsensitive<'i, 1>>
            {
                let res = &*self.content;
                {
                    let res = &res.content.0.matched;
                    {
                        let res = res
                            .as_ref()
                            .map(|res| {
                                let res = &res.content.0.matched;
                                {
                                    let res = res.as_ref().map(|res| res);
                                    res
                                }
                            })
                            .flatten();
                        res
                    }
                }
            }
            #[doc = "A helper function to access [`CharRange`]."]
            #[allow(non_snake_case)]
            pub fn r#CharRange<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<(
                &'s super::super::rules::r#CharRange<'i, 1>,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#CharRange<'i, 1>>,
            )> {
                let res = &*self.content;
                {
                    let res = &res.content.0.matched;
                    {
                        let res = res.as_ref().map(|res| {
                            let res = &res.content.1.matched;
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
                        });
                        res
                    }
                }
            }
            #[doc = "A helper function to access [`Tag`]."]
            #[allow(non_snake_case)]
            pub fn r#Tag<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#Tag<'i, 1>>,
            > {
                let res = &*self.content;
                {
                    let res = &res.content.0.matched;
                    {
                        let res = res.as_ref().map(|res| {
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
                        });
                        res
                    }
                }
            }
        }
        :: pest_typed :: rule ! (r#ExactString , "Corresponds to expression: `\"r#\"`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ExactString , super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_10 > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#ExactString<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#CaseInsensitive , "Corresponds to expression: `^\"0x\"`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#CaseInsensitive , super :: super :: generics :: Insens :: < 'i , super :: super :: constant_wrappers :: r#w_11 > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#CaseInsensitive<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#CharRange , "Corresponds to expression: `('0'..'9')`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#CharRange , super :: super :: generics :: CharRange :: < '0' , '9' > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#CharRange<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#Any , "Corresponds to expression: `ANY`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Any , super :: super :: rules :: r#ANY , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Expression , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#Any<'i, INHERITED> {
            #[doc = "A helper function to access [`ANY`]."]
            #[allow(non_snake_case)]
            pub fn r#ANY<'s>(&'s self) -> &'s super::super::rules::r#ANY {
                let res = &*self.content;
                res
            }
        }
        :: pest_typed :: rule ! (r#Seq , "Corresponds to expression: `(\"1\" ~ ('2'..'9') ~ \".\")`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Seq , super :: super :: generics :: Seq3 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_12 > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: CharRange :: < '2' , '9' > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_13 > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#Seq<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#Choice , "Corresponds to expression: `(\"a\" | ((^\"b\" ~ ^\"b\"*) ~ RepAtLeastOnce) | (&\"c\" ~ Choice ~ Rep ~ Opt) | Peek | PeekLeft | PeekRight | PeekLeftRight | Drop | PeekAll)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Choice , super :: super :: generics :: Choice9 :: < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_14 > , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Insens :: < 'i , super :: super :: constant_wrappers :: r#w_15 > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: generics :: Insens :: < 'i , super :: super :: constant_wrappers :: r#w_16 > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#RepAtLeastOnce :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Seq4 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Positive :: < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_17 > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Choice :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Rep :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Opt :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: rules :: r#Peek :: < 'i , INHERITED > , super :: super :: rules :: r#PeekLeft :: < 'i , INHERITED > , super :: super :: rules :: r#PeekRight :: < 'i , INHERITED > , super :: super :: rules :: r#PeekLeftRight :: < 'i , INHERITED > , super :: super :: rules :: r#Drop :: < 'i , INHERITED > , super :: super :: rules :: r#PeekAll :: < 'i , INHERITED > , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#Choice<'i, INHERITED> {
            #[doc = "A helper function to access [`Choice`]."]
            #[allow(non_snake_case)]
            pub fn r#Choice<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#Choice<'i, INHERITED>>
            {
                let res = &*self.content;
                {
                    let res = res._2().map(|res| {
                        let res = &res.content.1.matched;
                        res
                    });
                    res
                }
            }
            #[doc = "A helper function to access [`Drop`]."]
            #[allow(non_snake_case)]
            pub fn r#Drop<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#Drop<'i, INHERITED>>
            {
                let res = &*self.content;
                {
                    let res = res._7().map(|res| res);
                    res
                }
            }
            #[doc = "A helper function to access [`Opt`]."]
            #[allow(non_snake_case)]
            pub fn r#Opt<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#Opt<'i, INHERITED>>
            {
                let res = &*self.content;
                {
                    let res = res._2().map(|res| {
                        let res = &res.content.3.matched;
                        res
                    });
                    res
                }
            }
            #[doc = "A helper function to access [`Peek`]."]
            #[allow(non_snake_case)]
            pub fn r#Peek<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#Peek<'i, INHERITED>>
            {
                let res = &*self.content;
                {
                    let res = res._3().map(|res| res);
                    res
                }
            }
            #[doc = "A helper function to access [`PeekAll`]."]
            #[allow(non_snake_case)]
            pub fn r#PeekAll<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#PeekAll<'i, INHERITED>>
            {
                let res = &*self.content;
                {
                    let res = res._8().map(|res| res);
                    res
                }
            }
            #[doc = "A helper function to access [`PeekLeft`]."]
            #[allow(non_snake_case)]
            pub fn r#PeekLeft<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#PeekLeft<'i, INHERITED>>
            {
                let res = &*self.content;
                {
                    let res = res._4().map(|res| res);
                    res
                }
            }
            #[doc = "A helper function to access [`PeekLeftRight`]."]
            #[allow(non_snake_case)]
            pub fn r#PeekLeftRight<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<
                &'s super::super::rules::r#PeekLeftRight<'i, INHERITED>,
            > {
                let res = &*self.content;
                {
                    let res = res._6().map(|res| res);
                    res
                }
            }
            #[doc = "A helper function to access [`PeekRight`]."]
            #[allow(non_snake_case)]
            pub fn r#PeekRight<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<
                &'s super::super::rules::r#PeekRight<'i, INHERITED>,
            > {
                let res = &*self.content;
                {
                    let res = res._5().map(|res| res);
                    res
                }
            }
            #[doc = "A helper function to access [`Rep`]."]
            #[allow(non_snake_case)]
            pub fn r#Rep<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#Rep<'i, INHERITED>>
            {
                let res = &*self.content;
                {
                    let res = res._2().map(|res| {
                        let res = &res.content.2.matched;
                        res
                    });
                    res
                }
            }
            #[doc = "A helper function to access [`RepAtLeastOnce`]."]
            #[allow(non_snake_case)]
            pub fn r#RepAtLeastOnce<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<
                &'s super::super::rules::r#RepAtLeastOnce<'i, INHERITED>,
            > {
                let res = &*self.content;
                {
                    let res = res._1().map(|res| {
                        let res = &res.content.1.matched;
                        res
                    });
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#Rep , "Corresponds to expression: `^\"b\"*`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Rep , super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: generics :: Insens :: < 'i , super :: super :: constant_wrappers :: r#w_18 > > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#Rep<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#RepAtLeastOnce , "Corresponds to expression: `(('0'..'9') ~ ('0'..'9')*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#RepAtLeastOnce , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: CharRange :: < '0' , '9' > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: generics :: CharRange :: < '0' , '9' > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#RepAtLeastOnce<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#Opt , "Corresponds to expression: `\"?\"?`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Opt , :: pest_typed :: re_exported :: Option :: < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_19 > > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#Opt<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#RepExact , "Corresponds to expression: `(RepAtLeastOnce ~ RepAtLeastOnce ~ RepAtLeastOnce)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#RepExact , super :: super :: generics :: Seq3 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#RepAtLeastOnce :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#RepAtLeastOnce :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#RepAtLeastOnce :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#RepExact<'i, INHERITED> {
            #[doc = "A helper function to access [`RepAtLeastOnce`]."]
            #[allow(non_snake_case)]
            pub fn r#RepAtLeastOnce<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#RepAtLeastOnce<'i, INHERITED>,
                &'s super::super::rules::r#RepAtLeastOnce<'i, INHERITED>,
                &'s super::super::rules::r#RepAtLeastOnce<'i, INHERITED>,
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
                        {
                            let res = &res.content.2.matched;
                            res
                        },
                    );
                    res
                }
            }
        }
        :: pest_typed :: rule ! (r#RepLeft , "Corresponds to expression: `(RepExact ~ RepExact*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#RepLeft , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#RepExact :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#RepExact :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#RepLeft<'i, INHERITED> {
            #[doc = "A helper function to access [`RepExact`]."]
            #[allow(non_snake_case)]
            pub fn r#RepExact<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#RepExact<'i, INHERITED>,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#RepExact<'i, INHERITED>>,
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
        :: pest_typed :: rule ! (r#RepRight , "Corresponds to expression: `(RepLeft? ~ RepLeft?)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#RepRight , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#RepLeft :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#RepLeft :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#RepRight<'i, INHERITED> {
            #[doc = "A helper function to access [`RepLeft`]."]
            #[allow(non_snake_case)]
            pub fn r#RepLeft<'s>(
                &'s self,
            ) -> (
                ::pest_typed::re_exported::Option<
                    &'s super::super::rules::r#RepLeft<'i, INHERITED>,
                >,
                ::pest_typed::re_exported::Option<
                    &'s super::super::rules::r#RepLeft<'i, INHERITED>,
                >,
            ) {
                let res = &*self.content;
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
        :: pest_typed :: rule ! (r#RepLeftRight , "Corresponds to expression: `(RepRight ~ RepRight?)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#RepLeftRight , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#RepRight :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#RepRight :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#RepLeftRight<'i, INHERITED> {
            #[doc = "A helper function to access [`RepRight`]."]
            #[allow(non_snake_case)]
            pub fn r#RepRight<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#RepRight<'i, INHERITED>,
                ::pest_typed::re_exported::Option<
                    &'s super::super::rules::r#RepRight<'i, INHERITED>,
                >,
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
        :: pest_typed :: rule ! (r#Pos , "Corresponds to expression: `&(SOI ~ RepLeftRight ~ RepLeftRight ~ RepLeftRight? ~ RepLeftRight?)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Pos , super :: super :: generics :: Positive :: < super :: super :: generics :: Seq5 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#SOI , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#RepLeftRight :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#RepLeftRight :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#RepLeftRight :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#RepLeftRight :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#Pos<'i, INHERITED> {
            #[doc = "A helper function to access [`RepLeftRight`]."]
            #[allow(non_snake_case)]
            pub fn r#RepLeftRight<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#RepLeftRight<'i, INHERITED>,
                &'s super::super::rules::r#RepLeftRight<'i, INHERITED>,
                ::pest_typed::re_exported::Option<
                    &'s super::super::rules::r#RepLeftRight<'i, INHERITED>,
                >,
                ::pest_typed::re_exported::Option<
                    &'s super::super::rules::r#RepLeftRight<'i, INHERITED>,
                >,
            ) {
                let res = &*self.content;
                {
                    let res = &res.content;
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
                                {
                                    let res = res.as_ref().map(|res| res);
                                    res
                                }
                            },
                            {
                                let res = &res.content.4.matched;
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
            #[doc = "A helper function to access [`SOI`]."]
            #[allow(non_snake_case)]
            pub fn r#SOI<'s>(&'s self) -> &'s super::super::rules::r#SOI {
                let res = &*self.content;
                {
                    let res = &res.content;
                    {
                        let res = &res.content.0.matched;
                        res
                    }
                }
            }
        }
        :: pest_typed :: rule ! (r#Neg , "Corresponds to expression: `!(EOI ~ Pos)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Neg , super :: super :: generics :: Negative :: < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#EOI :: < 'i > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Pos :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#Neg<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#Push , "Corresponds to expression: `PUSH((RepLeft* ~ Neg ~ (ExactString ~ ExactString*) ~ Push ~ Pop ~ Push ~ PopAll))`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Push , super :: super :: generics :: Push :: < super :: super :: generics :: Seq7 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#RepLeft :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Neg :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#ExactString :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < 'i , INHERITED , super :: super :: rules :: r#ExactString :: < 'i , INHERITED > > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Push :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Pop :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Push :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#PopAll :: < 'i , INHERITED > , super :: super :: generics :: Skipped < 'i > , INHERITED >) , > > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#Push<'i, INHERITED> {
            #[doc = "A helper function to access [`ExactString`]."]
            #[allow(non_snake_case)]
            pub fn r#ExactString<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#ExactString<'i, INHERITED>,
                ::pest_typed::re_exported::Vec<
                    &'s super::super::rules::r#ExactString<'i, INHERITED>,
                >,
            ) {
                let res = &*self.content;
                {
                    let res = &res.content;
                    {
                        let res = &res.content.2.matched;
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
            }
            #[doc = "A helper function to access [`Neg`]."]
            #[allow(non_snake_case)]
            pub fn r#Neg<'s>(&'s self) -> &'s super::super::rules::r#Neg<'i, INHERITED> {
                let res = &*self.content;
                {
                    let res = &res.content;
                    {
                        let res = &res.content.1.matched;
                        res
                    }
                }
            }
            #[doc = "A helper function to access [`Pop`]."]
            #[allow(non_snake_case)]
            pub fn r#Pop<'s>(&'s self) -> &'s super::super::rules::r#Pop<'i, INHERITED> {
                let res = &*self.content;
                {
                    let res = &res.content;
                    {
                        let res = &res.content.4.matched;
                        res
                    }
                }
            }
            #[doc = "A helper function to access [`PopAll`]."]
            #[allow(non_snake_case)]
            pub fn r#PopAll<'s>(&'s self) -> &'s super::super::rules::r#PopAll<'i, INHERITED> {
                let res = &*self.content;
                {
                    let res = &res.content;
                    {
                        let res = &res.content.6.matched;
                        res
                    }
                }
            }
            #[doc = "A helper function to access [`Push`]."]
            #[allow(non_snake_case)]
            pub fn r#Push<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#Push<'i, INHERITED>,
                &'s super::super::rules::r#Push<'i, INHERITED>,
            ) {
                let res = &*self.content;
                {
                    let res = &res.content;
                    {
                        let res = (
                            {
                                let res = &res.content.3.matched;
                                res
                            },
                            {
                                let res = &res.content.5.matched;
                                res
                            },
                        );
                        res
                    }
                }
            }
            #[doc = "A helper function to access [`RepLeft`]."]
            #[allow(non_snake_case)]
            pub fn r#RepLeft<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#RepLeft<'i, INHERITED>>
            {
                let res = &*self.content;
                {
                    let res = &res.content;
                    {
                        let res = &res.content.0.matched;
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
            }
        }
        :: pest_typed :: rule ! (r#Pop , "Corresponds to expression: `POP`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Pop , super :: super :: rules :: r#POP :: < 'i > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#Pop<'i, INHERITED> {
            #[doc = "A helper function to access [`POP`]."]
            #[allow(non_snake_case)]
            pub fn r#POP<'s>(&'s self) -> &'s super::super::rules::r#POP<'i> {
                let res = &*self.content;
                res
            }
        }
        :: pest_typed :: rule ! (r#PopAll , "Corresponds to expression: `POP_ALL`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#PopAll , super :: super :: rules :: r#POP_ALL :: < 'i > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#PopAll<'i, INHERITED> {
            #[doc = "A helper function to access [`POP_ALL`]."]
            #[allow(non_snake_case)]
            pub fn r#POP_ALL<'s>(&'s self) -> &'s super::super::rules::r#POP_ALL<'i> {
                let res = &*self.content;
                res
            }
        }
        :: pest_typed :: rule ! (r#Peek , "Corresponds to expression: `PEEK[0..]`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Peek , super :: super :: generics :: PeekSlice1 :: < 0i32 > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#Peek<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#PeekLeft , "Corresponds to expression: `PEEK[1..]`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#PeekLeft , super :: super :: generics :: PeekSlice1 :: < 1i32 > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#PeekLeft<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#PeekRight , "Corresponds to expression: `PEEK[0..]`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#PeekRight , super :: super :: generics :: PeekSlice1 :: < 0i32 > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#PeekRight<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#PeekLeftRight , "Corresponds to expression: `PEEK[1..2]`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#PeekLeftRight , super :: super :: generics :: PeekSlice2 :: < 1i32 , 2i32 > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#PeekLeftRight<'i, INHERITED> {}
        :: pest_typed :: rule ! (r#Drop , "Corresponds to expression: `DROP`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Drop , super :: super :: rules :: r#DROP , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#Drop<'i, INHERITED> {
            #[doc = "A helper function to access [`DROP`]."]
            #[allow(non_snake_case)]
            pub fn r#DROP<'s>(&'s self) -> &'s super::super::rules::r#DROP {
                let res = &*self.content;
                res
            }
        }
        :: pest_typed :: rule ! (r#PeekAll , "Corresponds to expression: `PEEK_ALL`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#PeekAll , super :: super :: rules :: r#PEEK_ALL :: < 'i > , super :: super :: generics :: Skipped :: < 'i > , INHERITED , Both , true);
        impl<'i, const INHERITED: ::core::primitive::usize> r#PeekAll<'i, INHERITED> {
            #[doc = "A helper function to access [`PEEK_ALL`]."]
            #[allow(non_snake_case)]
            pub fn r#PEEK_ALL<'s>(&'s self) -> &'s super::super::rules::r#PEEK_ALL<'i> {
                let res = &*self.content;
                res
            }
        }
        #[allow(unused_imports)]
        use super::super::unicode::*;
        ::pest_typed::rule_eoi!(EOI, super::super::Rule);
        pub use pest_typed::predefined_node::ANY;
        pub use pest_typed::predefined_node::DROP;
        pub use pest_typed::predefined_node::PEEK_ALL;
        pub use pest_typed::predefined_node::POP;
        pub use pest_typed::predefined_node::POP_ALL;
        pub use pest_typed::predefined_node::SOI;
    }
}
pub use rules_impl::rules;
#[doc = "Used generics."]
pub mod generics {
    use pest_typed::predefined_node;
    #[doc = r" Skipped content."]
    pub type Skipped<'i> = predefined_node::Empty<'i>;
    pub use pest_typed::choices::Choice2;
    pub use pest_typed::choices::Choice9;
    pub use pest_typed::sequence::Seq2;
    pub use pest_typed::sequence::Seq3;
    pub use pest_typed::sequence::Seq4;
    pub use pest_typed::sequence::Seq5;
    pub use pest_typed::sequence::Seq7;
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
