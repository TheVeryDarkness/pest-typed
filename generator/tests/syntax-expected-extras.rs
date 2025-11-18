//! Test `syntax`.
#![cfg(feature = "grammar-extras")]
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
impl ::pest_typed::RuleType for Rule {
    fn name(&self) -> &'static ::core::primitive::str {
        match self {
            Self::r#Regular => "Regular",
            Self::r#Atomic => "Atomic",
            Self::r#Silent => "Silent",
            Self::r#CompoundAtomic => "CompoundAtomic",
            Self::r#Tag => "Tag",
            Self::r#NonAtomic => "NonAtomic",
            Self::r#ExactString => "ExactString",
            Self::r#CaseInsensitive => "CaseInsensitive",
            Self::r#CharRange => "CharRange",
            Self::r#Any => "Any",
            Self::r#Seq => "Seq",
            Self::r#Choice => "Choice",
            Self::r#Rep => "Rep",
            Self::r#RepAtLeastOnce => "RepAtLeastOnce",
            Self::r#Opt => "Opt",
            Self::r#RepExact => "RepExact",
            Self::r#RepLeft => "RepLeft",
            Self::r#RepRight => "RepRight",
            Self::r#RepLeftRight => "RepLeftRight",
            Self::r#Pos => "Pos",
            Self::r#Neg => "Neg",
            Self::r#Push => "Push",
            Self::r#Pop => "Pop",
            Self::r#PopAll => "PopAll",
            Self::r#Peek => "Peek",
            Self::r#PeekLeft => "PeekLeft",
            Self::r#PeekRight => "PeekRight",
            Self::r#PeekLeftRight => "PeekLeftRight",
            Self::r#Drop => "Drop",
            Self::r#PeekAll => "PeekAll",
            Self::EOI => "EOI",
        }
    }
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
    #[doc = "A wrapper for `\"c\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_16;
    impl ::pest_typed::StringWrapper for r#w_16 {
        const CONTENT: &'static ::core::primitive::str = "c";
    }
    #[doc = "A wrapper for `\"d\"`."]
    #[allow(non_camel_case_types)]
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub struct r#w_17;
    impl ::pest_typed::StringWrapper for r#w_17 {
        const CONTENT: &'static ::core::primitive::str = "d";
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
#[doc = "Generated structs for tags."]
pub mod tags {
    #[doc = "Tags inside rule [super::super::rules::r#Tag]."]
    #[allow(non_snake_case)]
    pub mod r#Tag {
        #[doc = "Tag r#Second referenced by r#Tag."]
        #[allow(non_camel_case_types)]
        pub type r#Second<'s, S, const INHERITED: ::core::primitive::usize> =
            (&'s super::super::generics::Seq4<
                (::pest_typed::predefined_node::Skipped<
                    super::super::generics::Push<
                        super::super::rules::r#CompoundAtomic<S, INHERITED>,
                    >,
                    super::super::generics::Skipped<S>,
                    INHERITED,
                >),
                (::pest_typed::predefined_node::Skipped<
                    super::super::rules::r#Any<S, INHERITED>,
                    super::super::generics::Skipped<S>,
                    INHERITED,
                >),
                (::pest_typed::predefined_node::Skipped<
                    super::super::generics::Positive<super::super::rules::r#Silent<S, INHERITED>>,
                    super::super::generics::Skipped<S>,
                    INHERITED,
                >),
                (::pest_typed::predefined_node::Skipped<
                    super::super::generics::Negative<super::super::rules::r#Atomic<S, INHERITED>>,
                    super::super::generics::Skipped<S>,
                    INHERITED,
                >),
            >);
        #[doc = "Tag r#Third referenced by r#Tag."]
        #[allow(non_camel_case_types)]
        pub type r#Third<'s, S, const INHERITED: ::core::primitive::usize> =
            (&'s super::super::generics::Rep<
                super::super::rules::r#NonAtomic<S, INHERITED>,
                S,
                INHERITED,
            >);
        #[doc = "Tag r#inner referenced by r#Tag."]
        #[allow(non_camel_case_types)]
        pub type r#inner<'s, S, const INHERITED: ::core::primitive::usize> = (
            &'s super::super::rules::r#Any<S, INHERITED>,
            &'s super::super::rules::r#Any<S, INHERITED>,
        );
    }
}
#[doc = "Definitions of statically typed nodes generated by pest-generator."]
pub mod rules_impl {
    #[doc = "Definitions of statically typed nodes generated by pest-generator."]
    pub mod rules {
        :: pest_typed :: rule ! (pub r#Regular , "Corresponds to expression: `(CharRange+ ~ \"+\" ~ CharRange+)`. Normal rule." "" "Regular rule." , super :: super :: Rule , super :: super :: Rule :: r#Regular , super :: super :: generics :: Seq3 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#CharRange :: < S , INHERITED > , S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_0 > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#CharRange :: < S , INHERITED > , S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#Regular<S, INHERITED> {
            #[doc = "A helper function to access [`CharRange`]."]
            #[allow(non_snake_case)]
            pub fn r#CharRange<'s>(
                &'s self,
            ) -> (
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#CharRange<S, INHERITED>>,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#CharRange<S, INHERITED>>,
            ) {
                let res = &*self.content;
                {
                    let res = (
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
        :: pest_typed :: rule ! (pub r#Atomic , "Corresponds to expression: `('0'..'9')+`. Atomic rule." "" "Atomic rule." , super :: super :: Rule , super :: super :: Rule :: r#Atomic , super :: super :: generics :: RepOnce :: < super :: super :: generics :: CharRange :: < '0' , '9' > , S , 0 > , super :: super :: generics :: Skipped :: < S > , true , Span , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#Atomic<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#Silent , "Corresponds to expression: `(\"(\" | \")\")`. Normal rule." "" "Silent rule." , super :: super :: Rule , super :: super :: Rule :: r#Silent , super :: super :: generics :: Choice2 :: < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_1 > , super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_2 > , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Expression , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#Silent<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#CompoundAtomic , "Corresponds to expression: `(\"\\\"\" ~ (!\"\\\"\" ~ ANY)* ~ \"\\\"\")`. Atomic rule." "" "Compound atomic rule." , super :: super :: Rule , super :: super :: Rule :: r#CompoundAtomic , super :: super :: generics :: Seq3 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_3 > , super :: super :: generics :: Skipped < S > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Negative :: < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_4 > > , super :: super :: generics :: Skipped < S > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#ANY , super :: super :: generics :: Skipped < S > , 0 >) , > , S , 0 > , super :: super :: generics :: Skipped < S > , 0 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_5 > , super :: super :: generics :: Skipped < S > , 0 >) , > , super :: super :: generics :: Skipped :: < S > , true , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#CompoundAtomic<S, INHERITED> {
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
        :: pest_typed :: rule ! (pub r#Tag , "Corresponds to expression: `(Any ~ (#Second = (PUSH(CompoundAtomic) ~ (#inner = Any) ~ &Silent ~ !Atomic))? ~ (#inner = Any) ~ ((#Third = NonAtomic*) ~ Regular+)?)`. Normal rule." "" "Tagged rule." , super :: super :: Rule , super :: super :: Rule :: r#Tag , super :: super :: generics :: Seq4 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Any :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: generics :: Seq4 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Push :: < super :: super :: rules :: r#CompoundAtomic :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Any :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Positive :: < super :: super :: rules :: r#Silent :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Negative :: < super :: super :: rules :: r#Atomic :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , > > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Any :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < super :: super :: rules :: r#NonAtomic :: < S , INHERITED > , S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#Regular :: < S , INHERITED > , S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , > > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#Tag<S, INHERITED> {
            #[doc = "A helper function to access [`Any`]."]
            #[allow(non_snake_case)]
            pub fn r#Any<'s>(&'s self) -> &'s super::super::rules::r#Any<S, INHERITED> {
                let res = &*self.content;
                {
                    let res = &res.content.0.matched;
                    res
                }
            }
            #[doc = "A helper function to access [`Regular`]."]
            #[allow(non_snake_case)]
            pub fn r#Regular<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#Regular<S, INHERITED>>,
            > {
                let res = &*self.content;
                {
                    let res = &res.content.3.matched;
                    {
                        let res = res.as_ref().map(|res| {
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
                        });
                        res
                    }
                }
            }
            #[doc = "A helper function to access [`Second`]."]
            #[allow(non_snake_case)]
            pub fn r#Second<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<
                (&'s super::super::generics::Seq4<
                    (::pest_typed::predefined_node::Skipped<
                        super::super::generics::Push<
                            super::super::rules::r#CompoundAtomic<S, INHERITED>,
                        >,
                        super::super::generics::Skipped<S>,
                        INHERITED,
                    >),
                    (::pest_typed::predefined_node::Skipped<
                        super::super::rules::r#Any<S, INHERITED>,
                        super::super::generics::Skipped<S>,
                        INHERITED,
                    >),
                    (::pest_typed::predefined_node::Skipped<
                        super::super::generics::Positive<
                            super::super::rules::r#Silent<S, INHERITED>,
                        >,
                        super::super::generics::Skipped<S>,
                        INHERITED,
                    >),
                    (::pest_typed::predefined_node::Skipped<
                        super::super::generics::Negative<
                            super::super::rules::r#Atomic<S, INHERITED>,
                        >,
                        super::super::generics::Skipped<S>,
                        INHERITED,
                    >),
                >),
            > {
                let res = &*self.content;
                {
                    let res = &res.content.1.matched;
                    {
                        let res = res.as_ref().map(|res| res);
                        res
                    }
                }
            }
            #[doc = "A helper function to access [`Third`]."]
            #[allow(non_snake_case)]
            pub fn r#Third<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<
                (&'s super::super::generics::Rep<
                    super::super::rules::r#NonAtomic<S, INHERITED>,
                    S,
                    INHERITED,
                >),
            > {
                let res = &*self.content;
                {
                    let res = &res.content.3.matched;
                    {
                        let res = res.as_ref().map(|res| {
                            let res = &res.content.0.matched;
                            res
                        });
                        res
                    }
                }
            }
            #[doc = "A helper function to access [`inner`]."]
            #[allow(non_snake_case)]
            pub fn r#inner<'s>(&'s self) -> (&'s super::super::rules::r#Any<S, INHERITED>) {
                let res = &*self.content;
                {
                    let res = &res.content.2.matched;
                    res
                }
            }
        }
        :: pest_typed :: rule ! (pub r#NonAtomic , "Corresponds to expression: `((CaseInsensitive? ~ CharRange+ ~ Tag*)? ~ (\"b\" | (\"BB\" ~ \"b\"?))? ~ ^\"c\"* ~ (!Seq ~ Any)+)`. Non-atomic rule." "" "Non-atomic rule." , super :: super :: Rule , super :: super :: Rule :: r#NonAtomic , super :: super :: generics :: Seq4 :: < (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: generics :: Seq3 :: < (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#CaseInsensitive :: < S , 1 > > , super :: super :: generics :: Skipped < S > , 1 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#CharRange :: < S , 1 > , S , 1 > , super :: super :: generics :: Skipped < S > , 1 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < super :: super :: rules :: r#Tag :: < S , 1 > , S , 1 > , super :: super :: generics :: Skipped < S > , 1 >) , > > , super :: super :: generics :: Skipped < S > , 1 >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: generics :: Choice2 :: < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_6 > , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_7 > , super :: super :: generics :: Skipped < S > , 1 >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_8 > > , super :: super :: generics :: Skipped < S > , 1 >) , > , > > , super :: super :: generics :: Skipped < S > , 1 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < super :: super :: generics :: Insens :: < S , super :: super :: constant_wrappers :: r#w_9 > , S , 1 > , super :: super :: generics :: Skipped < S > , 1 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: RepOnce :: < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Negative :: < super :: super :: rules :: r#Seq :: < S , 1 > > , super :: super :: generics :: Skipped < S > , 1 >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Any :: < S , 1 > , super :: super :: generics :: Skipped < S > , 1 >) , > , S , 1 > , super :: super :: generics :: Skipped < S > , 1 >) , > , super :: super :: generics :: Skipped :: < S > , false , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#NonAtomic<S, INHERITED> {
            #[doc = "A helper function to access [`Any`]."]
            #[allow(non_snake_case)]
            pub fn r#Any<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#Any<S, 1>> {
                let res = &*self.content;
                {
                    let res = &res.content.3.matched;
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
            #[doc = "A helper function to access [`CaseInsensitive`]."]
            #[allow(non_snake_case)]
            pub fn r#CaseInsensitive<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#CaseInsensitive<S, 1>>
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
            ) -> ::pest_typed::re_exported::Option<
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#CharRange<S, 1>>,
            > {
                let res = &*self.content;
                {
                    let res = &res.content.0.matched;
                    {
                        let res = res.as_ref().map(|res| {
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
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#Tag<S, 1>>,
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
        :: pest_typed :: rule ! (pub r#ExactString , "Corresponds to expression: `\"r#\"`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#ExactString , super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_10 > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#ExactString<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#CaseInsensitive , "Corresponds to expression: `^\"0x\"`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#CaseInsensitive , super :: super :: generics :: Insens :: < S , super :: super :: constant_wrappers :: r#w_11 > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#CaseInsensitive<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#CharRange , "Corresponds to expression: `('0'..'9')`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#CharRange , super :: super :: generics :: CharRange :: < '0' , '9' > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#CharRange<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#Any , "Corresponds to expression: `ANY`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Any , super :: super :: rules :: r#ANY , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#Any<S, INHERITED> {
            #[doc = "A helper function to access [`ANY`]."]
            #[allow(non_snake_case)]
            pub fn r#ANY<'s>(&'s self) -> &'s super::super::rules::r#ANY {
                let res = &*self.content;
                res
            }
        }
        :: pest_typed :: rule ! (pub r#Seq , "Corresponds to expression: `(\"1\" ~ ('2'..'9') ~ \".\")`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Seq , super :: super :: generics :: Seq3 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_12 > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: CharRange :: < '2' , '9' > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_13 > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#Seq<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#Choice , "Corresponds to expression: `(\"a\" | (^\"b\"+ ~ RepAtLeastOnce) | (\"c\" ~ &\"d\" ~ Choice ~ Rep ~ Opt) | Peek | PeekLeft | PeekRight | PeekLeftRight | Drop | PeekAll)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Choice , super :: super :: generics :: Choice9 :: < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_14 > , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: RepOnce :: < super :: super :: generics :: Insens :: < S , super :: super :: constant_wrappers :: r#w_15 > , S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#RepAtLeastOnce :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Seq5 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_16 > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Positive :: < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_17 > > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Choice :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Rep :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Opt :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: rules :: r#Peek :: < S , INHERITED > , super :: super :: rules :: r#PeekLeft :: < S , INHERITED > , super :: super :: rules :: r#PeekRight :: < S , INHERITED > , super :: super :: rules :: r#PeekLeftRight :: < S , INHERITED > , super :: super :: rules :: r#Drop :: < S , INHERITED > , super :: super :: rules :: r#PeekAll :: < S , INHERITED > , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#Choice<S, INHERITED> {
            #[doc = "A helper function to access [`Choice`]."]
            #[allow(non_snake_case)]
            pub fn r#Choice<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#Choice<S, INHERITED>>
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
            #[doc = "A helper function to access [`Drop`]."]
            #[allow(non_snake_case)]
            pub fn r#Drop<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#Drop<S, INHERITED>>
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
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#Opt<S, INHERITED>>
            {
                let res = &*self.content;
                {
                    let res = res._2().map(|res| {
                        let res = &res.content.4.matched;
                        res
                    });
                    res
                }
            }
            #[doc = "A helper function to access [`Peek`]."]
            #[allow(non_snake_case)]
            pub fn r#Peek<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#Peek<S, INHERITED>>
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
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#PeekAll<S, INHERITED>>
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
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#PeekLeft<S, INHERITED>>
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
                &'s super::super::rules::r#PeekLeftRight<S, INHERITED>,
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
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#PeekRight<S, INHERITED>>
            {
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
            ) -> ::pest_typed::re_exported::Option<&'s super::super::rules::r#Rep<S, INHERITED>>
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
            #[doc = "A helper function to access [`RepAtLeastOnce`]."]
            #[allow(non_snake_case)]
            pub fn r#RepAtLeastOnce<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Option<
                &'s super::super::rules::r#RepAtLeastOnce<S, INHERITED>,
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
        :: pest_typed :: rule ! (pub r#Rep , "Corresponds to expression: `^\"b\"*`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Rep , super :: super :: generics :: Rep :: < super :: super :: generics :: Insens :: < S , super :: super :: constant_wrappers :: r#w_18 > , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#Rep<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#RepAtLeastOnce , "Corresponds to expression: `('0'..'9')+`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#RepAtLeastOnce , super :: super :: generics :: RepOnce :: < super :: super :: generics :: CharRange :: < '0' , '9' > , S , INHERITED > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#RepAtLeastOnce<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#Opt , "Corresponds to expression: `\"?\"?`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Opt , :: pest_typed :: re_exported :: Option :: < super :: super :: generics :: Str :: < super :: super :: constant_wrappers :: r#w_19 > > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#Opt<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#RepExact , "Corresponds to expression: `(RepAtLeastOnce ~ RepAtLeastOnce ~ RepAtLeastOnce)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#RepExact , super :: super :: generics :: Seq3 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#RepAtLeastOnce :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#RepAtLeastOnce :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#RepAtLeastOnce :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#RepExact<S, INHERITED> {
            #[doc = "A helper function to access [`RepAtLeastOnce`]."]
            #[allow(non_snake_case)]
            pub fn r#RepAtLeastOnce<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#RepAtLeastOnce<S, INHERITED>,
                &'s super::super::rules::r#RepAtLeastOnce<S, INHERITED>,
                &'s super::super::rules::r#RepAtLeastOnce<S, INHERITED>,
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
        :: pest_typed :: rule ! (pub r#RepLeft , "Corresponds to expression: `(RepExact ~ RepExact*)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#RepLeft , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#RepExact :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < super :: super :: rules :: r#RepExact :: < S , INHERITED > , S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#RepLeft<S, INHERITED> {
            #[doc = "A helper function to access [`RepExact`]."]
            #[allow(non_snake_case)]
            pub fn r#RepExact<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#RepExact<S, INHERITED>,
                ::pest_typed::re_exported::Vec<&'s super::super::rules::r#RepExact<S, INHERITED>>,
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
        :: pest_typed :: rule ! (pub r#RepRight , "Corresponds to expression: `(RepLeft? ~ RepLeft?)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#RepRight , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#RepLeft :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#RepLeft :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#RepRight<S, INHERITED> {
            #[doc = "A helper function to access [`RepLeft`]."]
            #[allow(non_snake_case)]
            pub fn r#RepLeft<'s>(
                &'s self,
            ) -> (
                ::pest_typed::re_exported::Option<&'s super::super::rules::r#RepLeft<S, INHERITED>>,
                ::pest_typed::re_exported::Option<&'s super::super::rules::r#RepLeft<S, INHERITED>>,
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
        :: pest_typed :: rule ! (pub r#RepLeftRight , "Corresponds to expression: `(RepRight ~ RepRight?)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#RepLeftRight , super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#RepRight :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#RepRight :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#RepLeftRight<S, INHERITED> {
            #[doc = "A helper function to access [`RepRight`]."]
            #[allow(non_snake_case)]
            pub fn r#RepRight<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#RepRight<S, INHERITED>,
                ::pest_typed::re_exported::Option<
                    &'s super::super::rules::r#RepRight<S, INHERITED>,
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
        :: pest_typed :: rule ! (pub r#Pos , "Corresponds to expression: `&(SOI ~ RepLeftRight ~ RepLeftRight ~ RepLeftRight? ~ RepLeftRight?)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Pos , super :: super :: generics :: Positive :: < super :: super :: generics :: Seq5 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#SOI , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#RepLeftRight :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#RepLeftRight :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#RepLeftRight :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < :: pest_typed :: re_exported :: Option :: < super :: super :: rules :: r#RepLeftRight :: < S , INHERITED > > , super :: super :: generics :: Skipped < S > , INHERITED >) , > > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#Pos<S, INHERITED> {
            #[doc = "A helper function to access [`RepLeftRight`]."]
            #[allow(non_snake_case)]
            pub fn r#RepLeftRight<'s>(
                &'s self,
            ) -> (
                &'s super::super::rules::r#RepLeftRight<S, INHERITED>,
                &'s super::super::rules::r#RepLeftRight<S, INHERITED>,
                ::pest_typed::re_exported::Option<
                    &'s super::super::rules::r#RepLeftRight<S, INHERITED>,
                >,
                ::pest_typed::re_exported::Option<
                    &'s super::super::rules::r#RepLeftRight<S, INHERITED>,
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
        :: pest_typed :: rule ! (pub r#Neg , "Corresponds to expression: `!(EOI ~ Pos)`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Neg , super :: super :: generics :: Negative :: < super :: super :: generics :: Seq2 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#EOI :: < S > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Pos :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , > > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#Neg<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#Push , "Corresponds to expression: `PUSH((RepLeft* ~ Neg ~ ExactString+ ~ Push ~ Pop ~ Push ~ PopAll))`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Push , super :: super :: generics :: Push :: < super :: super :: generics :: Seq7 :: < (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: Rep :: < super :: super :: rules :: r#RepLeft :: < S , INHERITED > , S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Neg :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: generics :: RepOnce :: < super :: super :: rules :: r#ExactString :: < S , INHERITED > , S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Push :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Pop :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#Push :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , (:: pest_typed :: predefined_node :: Skipped < super :: super :: rules :: r#PopAll :: < S , INHERITED > , super :: super :: generics :: Skipped < S > , INHERITED >) , > > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#Push<S, INHERITED> {
            #[doc = "A helper function to access [`ExactString`]."]
            #[allow(non_snake_case)]
            pub fn r#ExactString<'s>(
                &'s self,
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#ExactString<S, INHERITED>>
            {
                let res = &*self.content;
                {
                    let res = &res.content;
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
                    }
                }
            }
            #[doc = "A helper function to access [`Neg`]."]
            #[allow(non_snake_case)]
            pub fn r#Neg<'s>(&'s self) -> &'s super::super::rules::r#Neg<S, INHERITED> {
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
            pub fn r#Pop<'s>(&'s self) -> &'s super::super::rules::r#Pop<S, INHERITED> {
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
            pub fn r#PopAll<'s>(&'s self) -> &'s super::super::rules::r#PopAll<S, INHERITED> {
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
                &'s super::super::rules::r#Push<S, INHERITED>,
                &'s super::super::rules::r#Push<S, INHERITED>,
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
            ) -> ::pest_typed::re_exported::Vec<&'s super::super::rules::r#RepLeft<S, INHERITED>>
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
        :: pest_typed :: rule ! (pub r#Pop , "Corresponds to expression: `POP`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Pop , super :: super :: rules :: r#POP :: < S > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#Pop<S, INHERITED> {
            #[doc = "A helper function to access [`POP`]."]
            #[allow(non_snake_case)]
            pub fn r#POP<'s>(&'s self) -> &'s super::super::rules::r#POP<S> {
                let res = &*self.content;
                res
            }
        }
        :: pest_typed :: rule ! (pub r#PopAll , "Corresponds to expression: `POP_ALL`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#PopAll , super :: super :: rules :: r#POP_ALL :: < S > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#PopAll<S, INHERITED> {
            #[doc = "A helper function to access [`POP_ALL`]."]
            #[allow(non_snake_case)]
            pub fn r#POP_ALL<'s>(&'s self) -> &'s super::super::rules::r#POP_ALL<S> {
                let res = &*self.content;
                res
            }
        }
        :: pest_typed :: rule ! (pub r#Peek , "Corresponds to expression: `PEEK[0..]`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Peek , super :: super :: generics :: PeekSlice1 :: < 0i32 > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#Peek<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#PeekLeft , "Corresponds to expression: `PEEK[1..]`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#PeekLeft , super :: super :: generics :: PeekSlice1 :: < 1i32 > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#PeekLeft<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#PeekRight , "Corresponds to expression: `PEEK[0..]`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#PeekRight , super :: super :: generics :: PeekSlice1 :: < 0i32 > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#PeekRight<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#PeekLeftRight , "Corresponds to expression: `PEEK[1..2]`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#PeekLeftRight , super :: super :: generics :: PeekSlice2 :: < 1i32 , 2i32 > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#PeekLeftRight<S, INHERITED> {}
        :: pest_typed :: rule ! (pub r#Drop , "Corresponds to expression: `DROP`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#Drop , super :: super :: rules :: r#DROP , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#Drop<S, INHERITED> {
            #[doc = "A helper function to access [`DROP`]."]
            #[allow(non_snake_case)]
            pub fn r#DROP<'s>(&'s self) -> &'s super::super::rules::r#DROP {
                let res = &*self.content;
                res
            }
        }
        :: pest_typed :: rule ! (pub r#PeekAll , "Corresponds to expression: `PEEK_ALL`. Normal rule." "" , super :: super :: Rule , super :: super :: Rule :: r#PeekAll , super :: super :: rules :: r#PEEK_ALL :: < S > , super :: super :: generics :: Skipped :: < S > , INHERITED , Both , true);
        impl<S, const INHERITED: ::core::primitive::usize> r#PeekAll<S, INHERITED> {
            #[doc = "A helper function to access [`PEEK_ALL`]."]
            #[allow(non_snake_case)]
            pub fn r#PEEK_ALL<'s>(&'s self) -> &'s super::super::rules::r#PEEK_ALL<S> {
                let res = &*self.content;
                res
            }
        }
        #[allow(unused_imports)]
        use super::super::unicode::*;
        :: pest_typed :: rule_eoi ! (pub EOI , super :: super :: Rule);
        pub use ::pest_typed::predefined_node::ANY;
        pub use ::pest_typed::predefined_node::DROP;
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
    pub type Skipped<S> = predefined_node::Empty<S>;
    pub use pest_typed::choices::Choice2;
    pub use pest_typed::choices::Choice9;
    pub use pest_typed::sequence::Seq2;
    pub use pest_typed::sequence::Seq3;
    pub use pest_typed::sequence::Seq4;
    pub use pest_typed::sequence::Seq5;
    pub use pest_typed::sequence::Seq7;
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
