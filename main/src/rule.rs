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
    predefined_node::EOI, tracker::Tracker, Cursor, NeverFailedTypedNode, RuleType, Span, Stack,
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
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize>
            $crate::iterators::Pairs<S, $Rule> for $name<S, INHERITED>
        {
            #[inline]
            fn for_self_or_each_child(
                &self,
                f: &mut impl $crate::re_exported::FnMut($crate::iterators::Token<S, $Rule>),
            ) {
                f($crate::iterators::Pair::<S, $Rule>::as_token(self))
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
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize>
            $crate::iterators::Pairs<S, $Rule> for $name<S, INHERITED>
        {
            #[inline]
            fn for_self_or_each_child(
                &self,
                f: &mut impl $crate::re_exported::FnMut($crate::iterators::Token<S, $Rule>),
            ) {
                self.content.for_self_or_each_child(f);
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
        $crate::impl_pairs_with_inner!($name, $Rule, $inner);
    };
    ($name:ident, $Rule:ty, $inner:ty, $emit:tt) => {
        $crate::impl_pairs_with_self!($name, $Rule);
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
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> ::core::ops::Deref
            for $name<S, INHERITED>
        {
            type Target = $inner;
            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.content
            }
        }
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> ::core::ops::DerefMut
            for $name<S, INHERITED>
        {
            #[inline]
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
        $crate::impl_deref_with_content!($name, $inner);
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
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> $crate::Spanned<S, $Rule>
            for $name<S, INHERITED>
        {
            #[inline]
            fn span(&self) -> $crate::Span<S> {
                self.span.clone()
            }
        }
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize>
            $crate::iterators::Pair<S, $Rule> for $name<S, INHERITED>
        {
            #[inline]
            fn for_each_child(
                &self,
                f: impl $crate::re_exported::FnMut($crate::iterators::Token<S, $Rule>),
            ) {
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
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> $crate::TypeWrapper
            for $name<S, INHERITED>
        {
            type Inner = $inner;
        }
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> $crate::Spanned<S, $Rule>
            for $name<S, INHERITED>
        {
            fn span(&self) -> $crate::Span<S> {
                self.span.clone()
            }
        }
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize>
            $crate::iterators::Pair<S, $Rule> for $name<S, INHERITED>
        {
            #[inline]
            fn for_each_child(
                &self,
                mut f: impl $crate::re_exported::FnMut($crate::iterators::Token<S, $Rule>),
            ) {
                $crate::iterators::Pairs::<S, $Rule>::for_self_or_each_child(&self.content, &mut f);
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
/// - `$inner:ty`. Type of inner parsing expression.
/// - `$boxed:tt`. `true` or `false`.
#[macro_export]
macro_rules! impl_rule_struct {
    ($name:ident, $Rule:ty, $inner:ty, true) => {
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize>
            $crate::RuleStruct<S, $Rule> for $name<S, INHERITED>
        {
            type Inner = $inner;
            fn take_inner(self) -> $inner {
                *self.content
            }
            fn ref_inner(&self) -> &$inner {
                &self.content
            }
            fn mut_inner(&mut self) -> &mut $inner {
                &mut self.content
            }
        }
    };
    ($name:ident, $Rule:ty, $inner:ty, false) => {
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize>
            $crate::RuleStruct<S, $Rule> for $name<S, INHERITED>
        {
            type Inner = $inner;
            fn take_inner(self) -> $inner {
                self.content
            }
            fn ref_inner(&self) -> &$inner {
                &self.content
            }
            fn mut_inner(&mut self) -> &mut $inner {
                &mut self.content
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
        $crate::impl_pair_with_empty!($name, $Rule, $rule);
    };
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty, false, $emit:tt) => {
        $crate::impl_pair_with_content!($name, $Rule, $rule, $inner);
    };
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty, INHERITED, $emit:tt) => {
        $crate::impl_pair_with_content!($name, $Rule, $rule, $inner);
    };
}

/// Implement [ParsableTypedNode::try_parse](crate::ParsableTypedNode::try_parse()) for structs.
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
        impl<C: $crate::Cursor<String = S>, S: $crate::RefStr> $crate::ParsableTypedNode<C, $Rule>
            for $name<S, 1>
        {
            #[inline]
            fn try_parse_with(
                input: C,
                stack: &mut $crate::Stack<$crate::Span<C::String>>,
                tracker: &mut $crate::tracker::Tracker<C::String, $Rule>,
            ) -> ::core::option::Option<Self> {
                $crate::rule::parse_without_ignore::<C, $Rule, Self>(
                    input,
                    stack,
                    tracker,
                    <$Rule>::EOI,
                )
            }
            #[inline]
            fn try_check_with(
                input: C,
                stack: &mut $crate::Stack<$crate::Span<C::String>>,
                tracker: &mut $crate::tracker::Tracker<C::String, $Rule>,
            ) -> ::core::primitive::bool {
                $crate::rule::check_without_ignore::<C, $Rule, Self>(
                    input,
                    stack,
                    tracker,
                    <$Rule>::EOI,
                )
            }
        }
    };
    ($name:ident, $Rule:ty, $ignored:ty, $non_true:tt) => {
        impl<C: $crate::Cursor<String = S>, S: $crate::RefStr> $crate::ParsableTypedNode<C, $Rule>
            for $name<S, 1>
        {
            #[inline]
            fn try_parse_with(
                input: C,
                stack: &mut $crate::Stack<$crate::Span<C::String>>,
                tracker: &mut $crate::tracker::Tracker<C::String, $Rule>,
            ) -> ::core::option::Option<Self> {
                $crate::rule::parse::<C, $Rule, Self, $ignored>(input, stack, tracker, <$Rule>::EOI)
            }
            #[inline]
            fn try_check_with(
                input: C,
                stack: &mut $crate::Stack<$crate::Span<C::String>>,
                tracker: &mut $crate::tracker::Tracker<C::String, $Rule>,
            ) -> ::core::primitive::bool {
                $crate::rule::check::<C, $Rule, Self, $ignored>(input, stack, tracker, <$Rule>::EOI)
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
        impl<
                C: $crate::Cursor<String = S>,
                S: $crate::RefStr,
                const INHERITED: ::core::primitive::usize,
            > $crate::TypedNode<C, $Rule> for $name<C::String, INHERITED>
        {
            #[inline]
            fn try_parse_partial_with(
                input: C,
                stack: &mut $crate::Stack<$crate::Span<C::String>>,
                tracker: &mut $crate::tracker::Tracker<C::String, $Rule>,
            ) -> ::core::option::Option<(C, Self)> {
                let (input, content) = <$inner>::try_parse_partial_with(input, stack, tracker)?;
                let content = content.into();
                Some((
                    input,
                    Self {
                        content,
                        _phantom: ::core::marker::PhantomData,
                    },
                ))
            }
            #[inline]
            fn try_check_partial_with(
                input: C,
                stack: &mut $crate::Stack<$crate::Span<C::String>>,
                tracker: &mut $crate::tracker::Tracker<C::String, $Rule>,
            ) -> ::core::option::Option<C> {
                let input = <$inner>::try_check_partial_with(input, stack, tracker)?;
                Some(input)
            }
        }
    };
    ($name:ident, $Rule:ty, $inner:ty, $atomicity:expr, Span) => {
        impl<
                C: $crate::Cursor<String = S>,
                S: $crate::RefStr,
                const INHERITED: ::core::primitive::usize,
            > $crate::TypedNode<C, $Rule> for $name<C::String, INHERITED>
        {
            #[inline]
            fn try_parse_partial_with(
                input: C,
                stack: &mut $crate::Stack<$crate::Span<C::String>>,
                tracker: &mut $crate::tracker::Tracker<C::String, $Rule>,
            ) -> ::core::option::Option<(C, Self)> {
                tracker.record_during(input.clone(), |tracker| {
                    let start = input.clone();
                    let input = <$inner>::try_check_partial_with(input.clone(), stack, tracker)?;
                    let span = start.span(&input);
                    Some((input, Self { span }))
                })
            }
            #[inline]
            fn try_check_partial_with(
                input: C,
                stack: &mut $crate::Stack<$crate::Span<C::String>>,
                tracker: &mut $crate::tracker::Tracker<C::String, $Rule>,
            ) -> ::core::option::Option<C> {
                tracker.record_during_with(
                    input.clone(),
                    |tracker| {
                        let input = <$inner>::try_check_partial_with(input, stack, tracker)?;
                        Some(input)
                    },
                    <Self as $crate::RuleWrapper<$Rule>>::RULE,
                )
            }
        }
    };
    ($name:ident, $Rule:ty, $inner:ty, $atomicity:expr, Both) => {
        impl<
                C: $crate::Cursor<String = S>,
                S: $crate::RefStr,
                const INHERITED: ::core::primitive::usize,
            > $crate::TypedNode<C, $Rule> for $name<C::String, INHERITED>
        {
            #[inline]
            fn try_parse_partial_with(
                input: C,
                stack: &mut $crate::Stack<$crate::Span<C::String>>,
                tracker: &mut $crate::tracker::Tracker<C::String, $Rule>,
            ) -> ::core::option::Option<(C, Self)> {
                tracker.record_during(input.clone(), |tracker| {
                    let start = input.clone();
                    let (input, content) =
                        <$inner>::try_parse_partial_with(input.clone(), stack, tracker)?;
                    let span = start.span(&input);
                    let content = content.into();
                    Some((input, Self { content, span }))
                })
            }
            #[inline]
            fn try_check_partial_with(
                input: C,
                stack: &mut $crate::Stack<$crate::Span<C::String>>,
                tracker: &mut $crate::tracker::Tracker<C::String, $Rule>,
            ) -> ::core::option::Option<C> {
                tracker.record_during_with(
                    input.clone(),
                    |tracker| {
                        let input =
                            <$inner>::try_check_partial_with(input.clone(), stack, tracker)?;
                        Some(input)
                    },
                    <Self as $crate::RuleWrapper<$Rule>>::RULE,
                )
            }
        }
    };
}

/// Implement [Clone](core::clone::Clone), [Hash](core::hash::Hash), [PartialEq](core::cmp::PartialEq) and [Eq](core::cmp::Eq) for structs.
///
/// Arguments:
///
/// - `$name:ident`. Name of generated struct.
/// - `$Rule:ty`. Rule type. Must implement [RuleType](`crate::RuleType`).
/// - `$inner:ty`. Type of inner parsing expression.
/// - `$atomicity:tt`. `true`, `false` or `INHERITED`.
/// - `$emission:tt`. `Span`, `Expression` or `Both`.
#[macro_export]
macro_rules! impl_common_traits {
    ($name:ident, $Rule:ty, $inner:ty, $atomicity:expr, Expression) => {
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> ::core::clone::Clone
            for $name<S, INHERITED>
        {
            #[inline]
            fn clone(&self) -> Self {
                Self {
                    content: self.content.clone(),
                    _phantom: ::core::marker::PhantomData,
                }
            }
        }
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> ::core::hash::Hash
            for $name<S, INHERITED>
        {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                self.content.hash(state);
            }
        }
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> ::core::cmp::PartialEq
            for $name<S, INHERITED>
        {
            #[inline]
            fn eq(&self, other: &Self) -> ::core::primitive::bool {
                self.content == other.content
            }
        }
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> ::core::cmp::Eq
            for $name<S, INHERITED>
        {
        }
    };
    ($name:ident, $Rule:ty, $inner:ty, $atomicity:expr, Span) => {
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> ::core::clone::Clone
            for $name<S, INHERITED>
        {
            #[inline]
            fn clone(&self) -> Self {
                Self {
                    span: self.span.clone(),
                }
            }
        }
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> ::core::hash::Hash
            for $name<S, INHERITED>
        {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                self.span.hash(state);
            }
        }
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> ::core::cmp::PartialEq
            for $name<S, INHERITED>
        {
            #[inline]
            fn eq(&self, other: &Self) -> ::core::primitive::bool {
                self.span == other.span
            }
        }
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> ::core::cmp::Eq
            for $name<S, INHERITED>
        {
        }
    };
    ($name:ident, $Rule:ty, $inner:ty, $atomicity:expr, Both) => {
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> ::core::clone::Clone
            for $name<S, INHERITED>
        {
            #[inline]
            fn clone(&self) -> Self {
                Self {
                    span: self.span.clone(),
                    content: self.content.clone(),
                }
            }
        }
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> ::core::hash::Hash
            for $name<S, INHERITED>
        {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                self.span.hash(state);
                self.content.hash(state);
            }
        }
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> ::core::cmp::PartialEq
            for $name<S, INHERITED>
        {
            #[inline]
            fn eq(&self, other: &Self) -> ::core::primitive::bool {
                self.span == other.span && self.content == other.content
            }
        }
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> ::core::cmp::Eq
            for $name<S, INHERITED>
        {
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
        impl<S, const INHERITED: ::core::primitive::usize> $crate::RuleWrapper<$Rule>
            for $name<S, INHERITED>
        {
            const RULE: $Rule = $rule;
            type Rule = $Rule;
        }
    };
}

/// Get inner type.
///
/// Arguments:
///
/// - `$inner:ty`. Type of inner parsing expression.
/// - `$boxed:tt`. `true` or `false`.
#[macro_export]
macro_rules! rule_inner {
    ($inner:ty, true) => {
        $crate::re_exported::Box<$inner>
    };
    ($inner:ty, false) => {
        $inner
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
/// - `$boxed:tt`. `true` or `false`.
#[macro_export]
macro_rules! declare_rule_struct {
    ($vis:vis $name:ident, $($doc:literal)*, $Rule:ty, $inner:ty, Expression, $boxed:tt) => {
        $(
            #[doc = $doc]
        )*
        #[allow(non_camel_case_types)]
        $vis struct $name<S, const INHERITED: ::core::primitive::usize = 1> {
            /// Matched expression.
            pub content: $crate::rule_inner!($inner, $boxed),
            _phantom: ::core::marker::PhantomData<S>,
        }
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> ::core::fmt::Debug for $name<S, INHERITED> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(::core::stringify!($name))
                    .field("content", &self.content)
                    .finish()
            }
        }
        $crate::impl_rule_struct!($name, $Rule, $inner, $boxed);
    };
    ($vis:vis $name:ident, $($doc:literal)*, $Rule:ty, $inner:ty, Span, $boxed:tt) => {
        $(
            #[doc = $doc]
        )*
        #[allow(non_camel_case_types)]
        $vis struct $name<S, const INHERITED: ::core::primitive::usize = 1> {
            /// Span of matched expression.
            pub span: $crate::Span<S>,
        }
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> ::core::fmt::Debug for $name<S, INHERITED> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(::core::stringify!($name))
                    .field("span", &self.span)
                    .finish()
            }
        }
    };
    ($vis:vis $name:ident, $($doc:literal)*, $Rule:ty, $inner:ty, Both, $boxed:tt) => {
        $(
            #[doc = $doc]
        )*
        #[allow(non_camel_case_types)]
        $vis struct $name<S, const INHERITED: ::core::primitive::usize = 1> {
            /// Matched expression.
            pub content: $crate::rule_inner!($inner, $boxed),
            /// Span of matched expression.
            pub span: $crate::Span<S>,
        }
        impl<S: $crate::RefStr, const INHERITED: ::core::primitive::usize> ::core::fmt::Debug for $name<S, INHERITED> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(::core::stringify!($name))
                    .field("content", &self.content)
                    .field("span", &self.span)
                    .finish()
            }
        }
        $crate::impl_rule_struct!($name, $Rule, $inner, $boxed);
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
/// - `$boxed:tt`. `true` or `false`.
///
/// See the below macros that reference this:
/// - [atomic_rule](`crate::atomic_rule!`).
/// - [compound_atomic_rule](`crate::compound_atomic_rule!`).
/// - [non_atomic_rule](`crate::non_atomic_rule!`).
/// - [normal_rule](`crate::normal_rule!`).
/// - [silent_rule](`crate::silent_rule!`).
#[macro_export]
macro_rules! rule {
    ($vis:vis $name:ident, $($doc:literal)*, $Rule:ty, $rule:expr, $inner:ty, $ignored:ty, $atomicity:tt, $emission:tt, $boxed:tt) => {
        $crate::declare_rule_struct!($vis $name, $($doc)*, $Rule, $inner, $emission, $boxed);
        $crate::impl_rule_wrapper!($name, $Rule, $rule);
        $crate::impl_try_parse_with!($name, $Rule, $inner, $atomicity, $emission);
        $crate::impl_common_traits!($name, $Rule, $inner, $atomicity, $emission);
        $crate::impl_parse!($name, $Rule, $ignored, $atomicity);
        $crate::impl_deref!($name, $inner, $emission);
        $crate::impl_pairs!($name, $Rule, $inner, $emission);
        $crate::impl_pair!($name, $Rule, $rule, $inner, $atomicity, $emission);
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
    ($vis:vis $name:ident, $($doc:literal)*, $Rule:ty, $rule:expr, $inner:ty) => {
        $crate::rule!($vis $name, $($doc)*, $Rule, $rule, $inner, (), true, Span, false);
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
/// - `$boxed:tt`. Whether wrap inner type in a [Box](crate::re_exported::Box). `true` or `false`.
#[macro_export]
macro_rules! compound_atomic_rule {
    ($vis:vis $name:ident, $($doc:literal)*, $Rule:ty, $rule:expr, $inner:ty, $boxed:tt) => {
        $crate::rule!($vis $name, $($doc)*, $Rule, $rule, $inner, (), true, Both, $boxed);
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
///    Must implement [NeverFailedTypedNode](`crate::NeverFailedTypedNode`). Normally using [Skipped](`crate::predefined_node::Skipped`).
///
/// - `$boxed:tt`. Whether wrap inner type in a [Box](crate::re_exported::Box). `true` or `false`.
#[macro_export]
macro_rules! non_atomic_rule {
    ($vis:vis $name:ident, $($doc:literal)*, $Rule:ty, $rule:expr, $inner:ty, $ignored:ty, $boxed:tt) => {
        $crate::rule!($vis $name, $($doc)*, $Rule, $rule, $inner, $ignored, false, Both, $boxed);
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
///    Must implement [NeverFailedTypedNode](`crate::NeverFailedTypedNode`). Normally using [Skipped](`crate::predefined_node::Skipped`).
///
/// - `$boxed:tt`. Whether wrap inner type in a [Box](crate::re_exported::Box). `true` or `false`.
#[macro_export]
macro_rules! normal_rule {
    ($vis:vis $name:ident, $($doc:literal)*, $Rule:ty, $rule:expr, $inner:ty, $ignored:ty, $boxed:tt) => {
        $crate::rule!($vis $name, $($doc)*, $Rule, $rule, $inner, $ignored, INHERITED, Both, $boxed);
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
///    Must implement [NeverFailedTypedNode](`crate::NeverFailedTypedNode`). Normally using [Skipped](`crate::predefined_node::Skipped`).
///
/// - `$boxed:tt`. Whether wrap inner type in a [Box](crate::re_exported::Box). `true` or `false`.
#[macro_export]
macro_rules! silent_rule {
    ($vis:vis $name:ident, $($doc:literal)*, $Rule:ty, $rule:expr, $inner:ty, $ignored:ty, $boxed:tt) => {
        $crate::rule!($vis $name, $($doc)*, $Rule, $rule, $inner, $ignored, INHERITED, Expression, $boxed);
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
    ($vis:vis $name:ident, $Rule:ty) => {
        $crate::declare_rule_struct!(
            $vis $name,
            "The rule for end of input.",
            $Rule,
            $crate::predefined_node::EOI,
            Both,
            false
        );
        $crate::impl_rule_wrapper!($name, $Rule, <$Rule>::EOI);
        $crate::impl_try_parse_with!($name, $Rule, $crate::predefined_node::EOI, INHERITED, Both);
        $crate::impl_common_traits!($name, $Rule, $crate::predefined_node::EOI, INHERITED, Both);
        impl<C: $crate::Cursor, const INHERITED: usize> $crate::ParsableTypedNode<C, $Rule> for $name<C::String, INHERITED> {
            #[inline]
            fn try_parse_with(
                input: C,
                stack: &mut $crate::Stack<$crate::Span<C::String>>,
                tracker: &mut $crate::tracker::Tracker<C::String, $Rule>,
            ) -> ::core::option::Option<Self> {
                $crate::rule::parse_without_ignore::<C, $Rule, Self>(
                    input,
                    stack,
                    tracker,
                    <$Rule>::EOI,
                )
            }
            #[inline]
            fn try_check_with(
                input: C,
                stack: &mut $crate::Stack<$crate::Span<C::String>>,
                tracker: &mut $crate::tracker::Tracker<C::String, $Rule>,
            ) -> ::core::primitive::bool {
                $crate::rule::check_without_ignore::<C, $Rule, Self>(
                    input,
                    stack,
                    tracker,
                    <$Rule>::EOI,
                )
            }
        }
        $crate::impl_deref!($name, $crate::predefined_node::EOI, Expression);
        $crate::impl_pairs_with_self!($name, $Rule);
        $crate::impl_pair_with_empty!($name, $Rule, <$Rule>::EOI);
    };
}

/// Full parse as a non-atomic rule.
///
/// For [rule](crate::rule!) to implement [ParsableTypedNode](crate::ParsableTypedNode).
#[inline]
pub fn parse<
    C: Cursor,
    R: RuleType,
    _Self: TypedNode<C, R>,
    IGNORED: NeverFailedTypedNode<C, R>,
>(
    input: C,
    stack: &mut Stack<Span<C::String>>,
    tracker: &mut Tracker<C::String, R>,
    rule_eoi: R,
) -> Option<_Self> {
    let (input, res) = match _Self::try_parse_partial_with(input, stack, tracker) {
        Some((input, res)) => (input, res),
        None => return None,
    };
    let (input, _) = IGNORED::parse_with(input, stack);
    let (_, _) = match tracker.record_during_with(
        input.clone(),
        |tracker| EOI::try_parse_partial_with(input, stack, tracker),
        rule_eoi,
    ) {
        Some((input, res)) => (input, res),
        None => return None,
    };
    Some(res)
}

/// Check as a non-atomic rule.
///
/// For [rule](crate::rule!) to implement [ParsableTypedNode](crate::ParsableTypedNode).
#[inline]
pub fn check<
    C: Cursor,
    R: RuleType,
    _Self: TypedNode<C, R>,
    IGNORED: NeverFailedTypedNode<C, R>,
>(
    input: C,
    stack: &mut Stack<Span<C::String>>,
    tracker: &mut Tracker<C::String, R>,
    rule_eoi: R,
) -> bool {
    let input = match _Self::try_check_partial_with(input, stack, tracker) {
        Some(input) => input,
        None => return false,
    };
    let input = IGNORED::check_with(input, stack);
    tracker
        .record_during_with(
            input.clone(),
            |tracker| EOI::try_check_partial_with(input, stack, tracker),
            rule_eoi,
        )
        .is_some()
}

/// Full parse as an atomic rule.
///
/// For [rule](crate::rule!) to implement [ParsableTypedNode](crate::ParsableTypedNode).
#[inline]
pub fn parse_without_ignore<C: Cursor, R: RuleType, _Self: TypedNode<C, R>>(
    input: C,
    stack: &mut Stack<Span<C::String>>,
    tracker: &mut Tracker<C::String, R>,
    rule_eoi: R,
) -> Option<_Self> {
    let (input, res) = match _Self::try_parse_partial_with(input, stack, tracker) {
        Some((input, res)) => (input, res),
        None => return None,
    };
    let (_, _) = match tracker.record_during_with(
        input.clone(),
        |tracker| EOI::try_parse_partial_with(input, stack, tracker),
        rule_eoi,
    ) {
        Some((input, res)) => (input, res),
        None => return None,
    };
    Some(res)
}

/// Check without auto-skipped parsing expressions.
///
/// For [rule](crate::rule!) to implement [ParsableTypedNode](crate::ParsableTypedNode).
#[inline]
pub fn check_without_ignore<C: Cursor, R: RuleType, _Self: TypedNode<C, R>>(
    input: C,
    stack: &mut Stack<Span<C::String>>,
    tracker: &mut Tracker<C::String, R>,
    rule_eoi: R,
) -> bool {
    let input = match _Self::try_check_partial_with(input, stack, tracker) {
        Some(input) => input,
        None => return false,
    };
    tracker
        .record_during_with(
            input.clone(),
            |tracker| EOI::try_check_partial_with(input, stack, tracker),
            rule_eoi,
        )
        .is_some()
}
