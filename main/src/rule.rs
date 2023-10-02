// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Macros and functions for defining structs, most of which are [RuleStruct](crate::RuleStruct).

use crate::{
    predefined_node::EOI, tracker::Tracker, NeverFailedTypedNode, Position, RuleType, Span, Stack,
    TypedNode,
};

/// Implement [`Pairs`](crate::iterators::Pairs) for a struct that is a [`Pair`](crate::iterators::Pair).
///
/// Normally used by non-silent rules.
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$Rule:ty`. Rule type. Must implement [RuleType](`crate::RuleType`).
#[macro_export]
macro_rules! impl_pairs_with_self {
    ($name:ident, $Rule:ty) => {
        impl<'i: 'n, 'n, const INHERITED: ::core::primitive::usize>
            ::pest_typed::iterators::Pairs<'i, 'n, $Rule> for $name<'i, INHERITED>
        {
            type Iter = ::core::iter::Once<&'n dyn ::pest_typed::iterators::Pair<'i, 'n, $Rule>>;
            type IntoIter = ::core::iter::Once<
                ::pest_typed::re_exported::Box<
                    dyn ::pest_typed::iterators::Pair<'i, 'n, $Rule> + 'n,
                >,
            >;

            fn iter_pairs(&'n self) -> Self::Iter {
                ::core::iter::once(self)
            }
            fn into_iter_pairs(self) -> Self::IntoIter {
                ::core::iter::once(::pest_typed::re_exported::Box::new(self))
            }
        }
    };
}

/// Implement [`Pairs`](crate::iterators::Pairs) for a struct that contains [`Pair`](crate::iterators::Pair)s.
///
/// Normally used by [silent_rule](crate::silent_rule!).
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$Rule:ty`. Rule type. Must implement [RuleType](`crate::RuleType`).
/// - `$inner:ty`. Type of inner parsing expression.
#[macro_export]
macro_rules! impl_pairs_with_inner {
    ($name:ident, $Rule:ty, $inner:ty) => {
        impl<'i: 'n, 'n, const INHERITED: ::core::primitive::usize>
            ::pest_typed::iterators::Pairs<'i, 'n, $Rule> for $name<'i, INHERITED>
        {
            type Iter = ::pest_typed::re_exported::vec::IntoIter<
                &'n dyn ::pest_typed::iterators::Pair<'i, 'n, $Rule>,
            >;
            type IntoIter = ::pest_typed::re_exported::vec::IntoIter<
                ::pest_typed::re_exported::Box<
                    dyn ::pest_typed::iterators::Pair<'i, 'n, $Rule> + 'n,
                >,
            >;

            fn iter_pairs(&'n self) -> Self::Iter {
                let i = <$inner as ::pest_typed::iterators::Pairs<'i, 'n, $Rule>>::iter_pairs(
                    &self.content,
                );
                i.collect::<::pest_typed::re_exported::Vec<_>>().into_iter()
            }
            fn into_iter_pairs(self) -> Self::IntoIter {
                let i = self.content.into_iter_pairs();
                /*
                let i = <$inner as ::pest_typed::iterators::Pairs<'i, 'n, $Rule>>::into_iter(
                    self.content,
                );
                */
                i.collect::<::pest_typed::re_exported::Vec<_>>().into_iter()
            }
        }
    };
}

/// Implement [`Pairs`](crate::iterators::Pairs) for a struct.
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$Rule:ty`. Rule type. Must implement [RuleType](`crate::RuleType`).
/// - `$inner:ty`. Type of inner parsing expression.
/// - `$emission:tt`. `Span`, `Expression` or `Both`.
#[macro_export]
macro_rules! impl_pairs {
    ($name:ident, $Rule:ty, $inner:ty, Expression) => {
        ::pest_typed::impl_pairs_with_inner!($name, $Rule, $inner);
    };
    ($name:ident, $Rule:ty, $inner:ty, $emit:tt) => {
        ::pest_typed::impl_pairs_with_self!($name, $Rule);
    };
}

/// Implement [`core::ops::Deref`] for structs with content.
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$inner:ty`. Type of inner parsing expression.
#[macro_export]
macro_rules! impl_deref_with_content {
    ($name:ident, $inner:ty) => {
        impl<'i, const INHERITED: ::core::primitive::usize> ::core::ops::Deref
            for $name<'i, INHERITED>
        {
            type Target = $inner;
            fn deref(&self) -> &Self::Target {
                &self.content
            }
        }
        impl<'i, const INHERITED: ::core::primitive::usize> ::core::ops::DerefMut
            for $name<'i, INHERITED>
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.content
            }
        }
    };
}

/// Implement [`core::ops::Deref`] for structs if they have content.
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$inner:ty`. Type of inner parsing expression.
/// - `$emission:tt`. `Span`, `Expression` or `Both`.
#[macro_export]
macro_rules! impl_deref {
    ($name:ident, $inner:ty, Span) => {};
    ($name:ident, $inner:ty, $emission:tt) => {
        ::pest_typed::impl_deref_with_content!($name, $inner);
    };
}

/// Implement [`Pair`](crate::iterators::Pair) for a struct without inner [`Pair`](crate::iterators::Pair)s.
///
/// Normally used by atomic rules.
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$Rule:ty`. Rule type. Must implement [RuleType](`crate::RuleType`).
/// - `$rule:expr`. Rule enumeration.
#[macro_export]
macro_rules! impl_pair_with_empty {
    ($name:ident, $Rule:ty, $rule:expr) => {
        impl<'i: 'n, 'n, const INHERITED: ::core::primitive::usize>
            ::pest_typed::RuleStruct<'i, $Rule> for $name<'i, INHERITED>
        {
            fn span(&self) -> ::pest_typed::Span<'i> {
                self.span
            }
        }
        impl<'i: 'n, 'n, const INHERITED: ::core::primitive::usize>
            ::pest_typed::iterators::Pair<'i, 'n, $Rule> for $name<'i, INHERITED>
        {
            fn inner(
                &'n self,
            ) -> ::pest_typed::re_exported::vec::IntoIter<
                &'n (dyn ::pest_typed::iterators::Pair<'i, 'n, $Rule> + 'n),
            > {
                ::pest_typed::re_exported::Vec::new().into_iter()
            }
            fn into_inner(
                self,
            ) -> ::pest_typed::re_exported::vec::IntoIter<
                ::pest_typed::re_exported::Box<
                    (dyn ::pest_typed::iterators::Pair<'i, 'n, $Rule> + 'n),
                >,
            > {
                ::pest_typed::re_exported::Vec::new().into_iter()
            }
        }
    };
}

/// Implement [`Pair`](crate::iterators::Pair) for a struct with inner [`Pair`](crate::iterators::Pair)s.
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$Rule:ty`. Rule type. Must implement [RuleType](`crate::RuleType`).
/// - `$rule:expr`. Rule enumeration.
/// - `$inner:ty`. Type of inner parsing expression.
#[macro_export]
macro_rules! impl_pair_with_content {
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty) => {
        impl<'i, const INHERITED: ::core::primitive::usize> ::pest_typed::TypeWrapper
            for $name<'i, INHERITED>
        {
            type Inner = $inner;
        }
        impl<'i: 'n, 'n, const INHERITED: ::core::primitive::usize>
            ::pest_typed::RuleStruct<'i, $Rule> for $name<'i, INHERITED>
        {
            fn span(&self) -> ::pest_typed::Span<'i> {
                self.span
            }
        }
        impl<'i: 'n, 'n, const INHERITED: ::core::primitive::usize>
            ::pest_typed::iterators::Pair<'i, 'n, $Rule> for $name<'i, INHERITED>
        {
            fn inner(
                &'n self,
            ) -> ::pest_typed::re_exported::vec::IntoIter<
                &'n (dyn ::pest_typed::iterators::Pair<'i, 'n, $Rule> + 'n),
            > {
                let i = <$inner as ::pest_typed::iterators::Pairs<'i, 'n, $Rule>>::iter_pairs(
                    self.content.as_ref(),
                );
                i.collect::<::pest_typed::re_exported::Vec<_>>().into_iter()
            }
            fn into_inner(
                self,
            ) -> ::pest_typed::re_exported::vec::IntoIter<
                ::pest_typed::re_exported::Box<
                    (dyn ::pest_typed::iterators::Pair<'i, 'n, $Rule> + 'n),
                >,
            > {
                let i = <$inner as ::pest_typed::iterators::Pairs<'i, 'n, $Rule>>::into_iter_pairs(
                    *self.content,
                );
                i.collect::<::pest_typed::re_exported::Vec<_>>().into_iter()
            }
        }
    };
}

/// Implement [`Pair`](crate::iterators::Pair) for a struct.
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$Rule:ty`. Rule type. Must implement [RuleType](`crate::RuleType`).
/// - `$rule:expr`. Rule enumeration.
/// - `$inner:ty`. Type of inner parsing expression.
/// - `$atomicity:tt`. `true`, `false` or `INHERITED`.
/// - `$emission:tt`. `Span`, `Expression` or `Both`.
#[macro_export]
macro_rules! impl_pair {
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty, $atomicity:expr, Expression) => {};
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty, true, $emit:tt) => {
        ::pest_typed::impl_pair_with_empty!($name, $Rule, $rule);
    };
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty, false, $emit:tt) => {
        ::pest_typed::impl_pair_with_content!($name, $Rule, $rule, $inner);
    };
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty, INHERITED, $emit:tt) => {
        ::pest_typed::impl_pair_with_content!($name, $Rule, $rule, $inner);
    };
}

/// Implement [ParsableTypedNode::parse](crate::ParsableTypedNode::parse()) for structs.
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$Rule:ty`. Rule type. Must implement [RuleType](`crate::RuleType`).
/// - `$ignored:ty`. Type of auto-skipped parsing  expressions.
///
///   Must implement [NeverFailedTypedNode](`crate::NeverFailedTypedNode`). Normally using [Skipped](`crate::predefined_node::Skipped`).
///
/// - `$atomicity:tt`. `true`, `false` or `INHERITED`.
#[macro_export]
macro_rules! impl_parse {
    ($name:ident, $Rule:ty, $ignored:ty, true) => {
        impl<'i> ::pest_typed::ParsableTypedNode<'i, $Rule> for $name<'i, 1> {
            #[inline]
            fn try_parse_with_until_end(
                input: $crate::Position<'i>,
                stack: &mut $crate::Stack<$crate::Span<'i>>,
                tracker: &mut $crate::tracker::Tracker<'i, $Rule>,
            ) -> ::core::result::Result<Self, ()> {
                ::pest_typed::rule::parse_without_ignore::<$Rule, Self>(
                    input,
                    stack,
                    tracker,
                    <$Rule>::EOI,
                )
            }
        }
    };
    ($name:ident, $Rule:ty, $ignored:ty, $non_true:tt) => {
        impl<'i> ::pest_typed::ParsableTypedNode<'i, $Rule> for $name<'i, 1> {
            #[inline]
            fn try_parse_with_until_end(
                input: $crate::Position<'i>,
                stack: &mut $crate::Stack<$crate::Span<'i>>,
                tracker: &mut $crate::tracker::Tracker<'i, $Rule>,
            ) -> ::core::result::Result<Self, ()> {
                ::pest_typed::rule::parse::<$Rule, Self, $ignored>(
                    input,
                    stack,
                    tracker,
                    <$Rule>::EOI,
                )
            }
        }
    };
}

/// Implement [TypedNode::try_parse_with](crate::TypedNode::try_parse_with()) for structs.
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$Rule:ty`. Rule type. Must implement [RuleType](`crate::RuleType`).
/// - `$inner:ty`. Type of inner parsing expression.
/// - `$atomicity:tt`. `true`, `false` or `INHERITED`.
/// - `$emission:tt`. `Span`, `Expression` or `Both`.
#[macro_export]
macro_rules! impl_try_parse_with {
    ($name:ident, $Rule:ty, $inner:ty, $atomicity:expr, Expression) => {
        impl<'i, const INHERITED: ::core::primitive::usize> ::pest_typed::TypedNode<'i, $Rule>
            for $name<'i, INHERITED>
        {
            #[inline]
            fn try_parse_with(
                input: ::pest_typed::Position<'i>,
                stack: &mut ::pest_typed::Stack<::pest_typed::Span<'i>>,
                tracker: &mut ::pest_typed::tracker::Tracker<'i, $Rule>,
            ) -> ::core::result::Result<(::pest_typed::Position<'i>, Self), ()> {
                let (input, content) = <$inner>::try_parse_with(input, stack, tracker)?;
                let content = ::pest_typed::re_exported::Box::new(content);
                Ok((
                    input,
                    Self {
                        content,
                        _phantom: ::core::marker::PhantomData,
                    },
                ))
            }
        }
    };
    ($name:ident, $Rule:ty, $inner:ty, $atomicity:expr, Span) => {
        impl<'i, const INHERITED: ::core::primitive::usize> ::pest_typed::TypedNode<'i, $Rule>
            for $name<'i, INHERITED>
        {
            #[inline]
            fn try_parse_with(
                input: ::pest_typed::Position<'i>,
                stack: &mut ::pest_typed::Stack<::pest_typed::Span<'i>>,
                tracker: &mut ::pest_typed::tracker::Tracker<'i, $Rule>,
            ) -> ::core::result::Result<(::pest_typed::Position<'i>, Self), ()> {
                tracker.record_during(input, |tracker| {
                    let start = input;
                    let (input, _) = <$inner>::try_parse_with(input, stack, tracker)?;
                    let span = start.span(&input);
                    Ok((input, Self { span }))
                })
            }
        }
    };
    ($name:ident, $Rule:ty, $inner:ty, $atomicity:expr, Both) => {
        impl<'i, const INHERITED: ::core::primitive::usize> ::pest_typed::TypedNode<'i, $Rule>
            for $name<'i, INHERITED>
        {
            #[inline]
            fn try_parse_with(
                input: ::pest_typed::Position<'i>,
                stack: &mut ::pest_typed::Stack<::pest_typed::Span<'i>>,
                tracker: &mut ::pest_typed::tracker::Tracker<'i, $Rule>,
            ) -> ::core::result::Result<(::pest_typed::Position<'i>, Self), ()> {
                tracker.record_during(input, |tracker| {
                    let start = input;
                    let (input, content) = <$inner>::try_parse_with(input, stack, tracker)?;
                    let span = start.span(&input);
                    let content = ::pest_typed::re_exported::Box::new(content);
                    Ok((input, Self { content, span }))
                })
            }
        }
    };
}

/// Implement [RuleWrapper](crate::RuleWrapper) for the struct.
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$Rule:ty`. Rule type. Must implement [RuleType](`crate::RuleType`).
/// - `$rule:expr`. Rule enumeration.
#[macro_export]
macro_rules! impl_rule_wrapper {
    ($name:ident, $Rule:ty, $rule:expr) => {
        impl<'i, const INHERITED: ::core::primitive::usize> ::pest_typed::RuleWrapper<$Rule>
            for $name<'i, INHERITED>
        {
            const RULE: $Rule = $rule;
            type Rule = $Rule;
        }
    };
}

/// Declare the body of the struct.
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$($doc:literal)*`. A list of strings that is prepended to generated struct as document comments.
/// - `$inner:ty`. Type of inner parsing expression.
/// - `$emission:tt`. `Span`, `Expression` or `Both`.
#[macro_export]
macro_rules! declare_rule_struct {
    ($name:ident, $($doc:literal)*, $inner:ty, Expression) => {
        $(
            #[doc = $doc]
        )*
        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq)]
        pub struct $name<'i, const INHERITED: ::core::primitive::usize = 1> {
            /// Matched expression.
            pub content: ::pest_typed::re_exported::Box<$inner>,
            _phantom: ::core::marker::PhantomData<&'i ::core::primitive::str>,
        }
        impl<'i, const INHERITED: ::core::primitive::usize> ::core::fmt::Debug for $name<'i, INHERITED> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(::core::stringify!($name))
                    .field("content", &self.content)
                    .finish()
            }
        }
    };
    ($name:ident, $($doc:literal)*, $inner:ty, Span) => {
        $(
            #[doc = $doc]
        )*
        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq)]
        pub struct $name<'i, const INHERITED: ::core::primitive::usize = 1> {
            /// Span of matched expression.
            pub span: ::pest_typed::Span<'i>,
        }
        impl<'i, const INHERITED: ::core::primitive::usize> ::core::fmt::Debug for $name<'i, INHERITED> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(::core::stringify!($name))
                    .field("span", &self.span)
                    .finish()
            }
        }
    };
    ($name:ident, $($doc:literal)*, $inner:ty, Both) => {
        $(
            #[doc = $doc]
        )*
        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq)]
        pub struct $name<'i, const INHERITED: ::core::primitive::usize = 1> {
            /// Matched expression.
            pub content: ::pest_typed::re_exported::Box<$inner>,
            /// Span of matched expression.
            pub span: ::pest_typed::Span<'i>,
        }
        impl<'i, const INHERITED: ::core::primitive::usize> ::core::fmt::Debug for $name<'i, INHERITED> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(::core::stringify!($name))
                    .field("content", &self.content)
                    .field("span", &self.span)
                    .finish()
            }
        }
    };
}

/// The start point of a node tag.
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$Rule:ty`. Rule type. Must implement [RuleType](`crate::RuleType`).
/// - `$inner:ty`. Type of inner parsing expression.
#[macro_export]
macro_rules! tag {
    ($name:ident, $Rule:ty, $inner:ty) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name<'i, const INHERITED: usize> {
            /// Matched expression.
            pub content: $inner,
            /// Span of matched expression.
            pub span: ::pest_typed::Span<'i>,
        }
        impl<'i, const INHERITED: usize> ::pest_typed::TypedNode<'i, $Rule>
            for $name<'i, INHERITED>
        {
            fn try_parse_with(
                input: ::pest_typed::Position<'i>,
                stack: &mut ::pest_typed::Stack<::pest_typed::Span<'i>>,
                tracker: &mut ::pest_typed::tracker::Tracker<'i, $Rule>,
            ) -> ::core::result::Result<(::pest_typed::Position<'i>, Self), ()> {
                let start = input;
                match <$inner>::try_parse_with(input, stack, tracker) {
                    ::core::result::Result::Ok((input, content)) => {
                        let span = start.span(&input);
                        ::core::result::Result::Ok((input, Self { content, span }))
                    }
                    ::core::result::Result::Err(_) => ::core::result::Result::Err(()),
                }
            }
        }
        ::pest_typed::impl_deref_with_content!($name, $inner);
        ::pest_typed::impl_pairs_with_inner!($name, $Rule, $inner);
    };
}

/// Start point of a rule.
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$($doc:literal)*`. A list of strings that is prepended to generated struct as document comments.
/// - `$Rule:ty`. Rule type. Must implement [RuleType](`crate::RuleType`).
/// - `$rule:expr`. Rule enumeration.
/// - `$inner:ty`. Type of inner parsing expression.
/// - `$ignored:ty`. Type of auto-skipped parsing  expressions.
///
///   Must implement [NeverFailedTypedNode](`crate::NeverFailedTypedNode`). Normally using [Skipped](`crate::predefined_node::Skipped`).
///
/// - `$atomicity:tt`. `true`, `false` or `INHERITED`.
/// - `$emission:tt`. `Span`, `Expression` or `Both`.
///
/// See the below macros that reference this:
/// - [atomic_rule](`crate::atomic_rule!`).
/// - [compound_atomic_rule](`crate::compound_atomic_rule!`).
/// - [non_atomic_rule](`crate::non_atomic_rule!`).
/// - [normal_rule](`crate::normal_rule!`).
/// - [silent_rule](`crate::silent_rule!`).
#[macro_export]
macro_rules! rule {
    ($name:ident, $($doc:literal)*, $Rule:ty, $rule:expr, $inner:ty, $ignored:ty, $atomicity:tt, $emission:tt) => {
        ::pest_typed::declare_rule_struct!($name, $($doc)*, $inner, $emission);
        ::pest_typed::impl_rule_wrapper!($name, $Rule, $rule);
        ::pest_typed::impl_try_parse_with!($name, $Rule, $inner, $atomicity, $emission);
        ::pest_typed::impl_parse!($name, $Rule, $ignored, $atomicity);
        ::pest_typed::impl_deref!($name, $inner, $emission);
        ::pest_typed::impl_pairs!($name, $Rule, $inner, $emission);
        ::pest_typed::impl_pair!($name, $Rule, $rule, $inner, $atomicity, $emission);
    };
}

/// Shortcut for atomic rule in [pest].
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$($doc:literal)*`. A list of strings that is prepended to generated struct as document comments.
/// - `$Rule:ty`. Rule type. Must implement [RuleType](`crate::RuleType`).
/// - `$rule:expr`. Rule enumeration.
/// - `$inner:ty`. Type of inner parsing expression.
#[macro_export]
macro_rules! atomic_rule {
    ($name:ident, $($doc:literal)*, $Rule:ty, $rule:expr, $inner:ty) => {
        ::pest_typed::rule!($name, $($doc)*, $Rule, $rule, $inner, (), true, Span);
    };
}

/// Shortcut for compound atomic rule in [pest].
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$($doc:literal)*`. A list of strings that is prepended to generated struct as document comments.
/// - `$Rule:ty`. Rule type. Must implement [RuleType](`crate::RuleType`).
/// - `$rule:expr`. Rule enumeration.
/// - `$inner:ty`. Type of inner parsing expression.
#[macro_export]
macro_rules! compound_atomic_rule {
    ($name:ident, $($doc:literal)*, $Rule:ty, $rule:expr, $inner:ty) => {
        ::pest_typed::rule!($name, $($doc)*, $Rule, $rule, $inner, (), true, Both);
    };
}

/// Shortcut for non-atomic rule in [pest].
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$($doc:literal)*`. A list of strings that is prepended to generated struct as document comments.
/// - `$Rule:ty`. Rule type. Must implement [RuleType](`crate::RuleType`).
/// - `$rule:expr`. Rule enumeration.
/// - `$inner:ty`. Type of inner parsing expression.
/// - `$ignored:ty`. Type of auto-skipped parsing  expressions.
///
///   Must implement [NeverFailedTypedNode](`crate::NeverFailedTypedNode`). Normally using [Skipped](`crate::predefined_node::Skipped`).
#[macro_export]
macro_rules! non_atomic_rule {
    ($name:ident, $($doc:literal)*, $Rule:ty, $rule:expr, $inner:ty, $ignored:ty) => {
        ::pest_typed::rule!($name, $($doc)*, $Rule, $rule, $inner, $ignored, false, Both);
    };
}

/// Shortcut for normal rule in [pest].
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$($doc:literal)*`. A list of strings that is prepended to generated struct as document comments.
/// - `$Rule:ty`. Rule type. Must implement [RuleType](`crate::RuleType`).
/// - `$rule:expr`. Rule enumeration.
/// - `$inner:ty`. Type of inner parsing expression.
/// - `$ignored:ty`. Type of auto-skipped parsing expressions.
///
///   Must implement [NeverFailedTypedNode](`crate::NeverFailedTypedNode`). Normally using [Skipped](`crate::predefined_node::Skipped`).
#[macro_export]
macro_rules! normal_rule {
    ($name:ident, $($doc:literal)*, $Rule:ty, $rule:expr, $inner:ty, $ignored:ty) => {
        ::pest_typed::rule!($name, $($doc)*, $Rule, $rule, $inner, $ignored, INHERITED, Both);
    };
}

/// Shortcut for silent rule in [pest].
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$($doc:literal)*`. A list of strings that is prepended to generated struct as document comments.
/// - `$Rule:ty`. Rule type. Must implement [RuleType](`crate::RuleType`).
/// - `$rule:expr`. Rule enumeration.
/// - `$inner:ty`. Type of inner parsing expression.
/// - `$ignored:ty`. Type of auto-skipped parsing expressions.
///
///   Must implement [NeverFailedTypedNode](`crate::NeverFailedTypedNode`). Normally using [Skipped](`crate::predefined_node::Skipped`).
#[macro_export]
macro_rules! silent_rule {
    ($name:ident, $($doc:literal)*, $Rule:ty, $rule:expr, $inner:ty, $ignored:ty) => {
        ::pest_typed::rule!($name, $($doc)*, $Rule, $rule, $inner, $ignored, INHERITED, Expression);
    };
}

/// Start point of a end-of-input rule.
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$Rule:ty`. Rule type. Must implement [RuleType].
#[macro_export]
macro_rules! rule_eoi {
    ($name:ident, $Rule:ty) => {
        ::pest_typed::declare_rule_struct!(
            $name,
            "The rule for end of input.",
            ::pest_typed::predefined_node::EOI,
            Both
        );
        ::pest_typed::impl_rule_wrapper!($name, $Rule, <$Rule>::EOI);
        ::pest_typed::impl_try_parse_with!(
            $name,
            $Rule,
            ::pest_typed::predefined_node::EOI,
            INHERITED,
            Both
        );
        impl<'i, const INHERITED: usize> ::pest_typed::ParsableTypedNode<'i, $Rule>
            for $name<'i, INHERITED>
        {
            #[inline]
            fn try_parse_with_until_end(
                input: $crate::Position<'i>,
                stack: &mut $crate::Stack<$crate::Span<'i>>,
                tracker: &mut $crate::tracker::Tracker<'i, $Rule>,
            ) -> ::core::result::Result<Self, ()> {
                ::pest_typed::rule::parse_without_ignore::<$Rule, Self>(
                    input,
                    stack,
                    tracker,
                    <$Rule>::EOI,
                )
            }
        }
        ::pest_typed::impl_deref!($name, ::pest_typed::predefined_node::EOI, Expression);
        ::pest_typed::impl_pairs_with_self!($name, $Rule);
        ::pest_typed::impl_pair_with_empty!($name, $Rule, <$Rule>::EOI);
    };
}

/// Full parse as a non-atomic rule.
///
/// For [rule](crate::rule!) to implement [ParsableTypedNode](crate::ParsableTypedNode).
pub fn parse<
    'i,
    R: RuleType + 'i,
    _Self: TypedNode<'i, R>,
    IGNORED: NeverFailedTypedNode<'i, R>,
>(
    input: Position<'i>,
    stack: &mut Stack<Span<'i>>,
    tracker: &mut Tracker<'i, R>,
    rule_eoi: R,
) -> Result<_Self, ()> {
    let (input, res) = match _Self::try_parse_with(input, stack, tracker) {
        Ok((input, res)) => (input, res),
        Err(_) => return Err(()),
    };
    let (input, _) = IGNORED::parse_with(input, stack);
    let (_, _) = match tracker.record_during_with(
        input,
        |tracker| EOI::try_parse_with(input, stack, tracker),
        rule_eoi,
    ) {
        Ok((input, res)) => (input, res),
        Err(_) => return Err(()),
    };
    Ok(res)
}

/// Full parse as an atomic rule.
///
/// For [rule](crate::rule!) to implement [ParsableTypedNode](crate::ParsableTypedNode).
pub fn parse_without_ignore<'i, R: RuleType + 'i, _Self: TypedNode<'i, R>>(
    input: Position<'i>,
    stack: &mut Stack<Span<'i>>,
    tracker: &mut Tracker<'i, R>,
    rule_eoi: R,
) -> Result<_Self, ()> {
    let (input, res) = match _Self::try_parse_with(input, stack, tracker) {
        Ok((input, res)) => (input, res),
        Err(_) => return Err(()),
    };
    let (_, _) = match tracker.record_during_with(
        input,
        |tracker| EOI::try_parse_with(input, stack, tracker),
        rule_eoi,
    ) {
        Ok((input, res)) => (input, res),
        Err(_) => return Err(()),
    };
    Ok(res)
}
