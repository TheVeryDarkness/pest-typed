// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Macros and functions for defining structs, most of which are [`crate::RuleStruct`].

use crate::{
    error::Error, predefined_node::EOI, tracker::Tracker, NeverFailedTypedNode, Position, RuleType,
    Stack, TypedNode,
};

/// Implement [`Pairs`](crate::iterators::Pairs) for a struct that is a [`Pair`](crate::iterators::Pair).
///
/// Normally used by non-silent rules.
#[macro_export]
macro_rules! impl_pairs_with_self {
    ($node:ident, $Rule:ty, $rule:expr) => {
        impl<'i: 'n, 'n> ::pest_typed::iterators::Pairs<'i, 'n, $Rule> for $node<'i> {
            type Iter = ::core::iter::Once<&'n dyn ::pest_typed::iterators::Pair<'i, 'n, $Rule>>;
            type IntoIter = ::core::iter::Once<
                ::pest_typed::re_exported::Box<
                    dyn ::pest_typed::iterators::Pair<'i, 'n, $Rule> + 'n,
                >,
            >;

            fn iter(&'n self) -> Self::Iter {
                ::core::iter::once(self)
            }
            fn into_iter(self) -> Self::IntoIter {
                ::core::iter::once(::pest_typed::re_exported::Box::new(self))
            }
        }
    };
}

/// Implement [`Pairs`](crate::iterators::Pairs) for a struct that contains [`Pair`](crate::iterators::Pair)s.
///
/// Normally used by [`crate::silent_rule!`].
#[macro_export]
macro_rules! impl_pairs_with_inner {
    ($node:ident, $Rule:ty, $rule:expr, $inner:ty) => {
        impl<'i: 'n, 'n> ::pest_typed::iterators::Pairs<'i, 'n, $Rule> for $node<'i> {
            type Iter = ::pest_typed::re_exported::vec::IntoIter<
                &'n dyn ::pest_typed::iterators::Pair<'i, 'n, $Rule>,
            >;
            type IntoIter = ::pest_typed::re_exported::vec::IntoIter<
                ::pest_typed::re_exported::Box<
                    dyn ::pest_typed::iterators::Pair<'i, 'n, $Rule> + 'n,
                >,
            >;

            fn iter(&'n self) -> Self::Iter {
                let i = <$inner as ::pest_typed::iterators::Pairs<'i, 'n, $Rule>>::iter(
                    self.content.as_ref(),
                );
                i.collect::<::pest_typed::re_exported::Vec<_>>().into_iter()
            }
            fn into_iter(self) -> Self::IntoIter {
                let i =
                    <$inner as ::pest_typed::iterators::Pairs<'i, 'n, $Rule>>::iter(*self.content);
                i.collect::<::pest_typed::re_exported::Vec<_>>().into_iter()
            }
        }
    };
}

/// Implement [`core::ops::Deref`] for structs with content.
#[macro_export]
macro_rules! impl_deref_with_content {
    ($name:ident, $inner:ty) => {
        impl<'i> ::core::ops::Deref for $name<'i> {
            type Target = $inner;
            fn deref(&self) -> &Self::Target {
                &self.content
            }
        }
        impl<'i> ::core::ops::DerefMut for $name<'i> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.content
            }
        }
    };
}

/// Implement [`Pair`](crate::iterators::Pair) for a struct without inner [`Pair`](crate::iterators::Pair)s.
///
/// Normally used by atomic rules.
///
/// Arguments: `($node:ident, $Rule:ty, $rule:expr)`.
#[macro_export]
macro_rules! impl_pair_with_empty {
    ($node:ident, $Rule:ty, $rule:expr) => {
        impl<'i: 'n, 'n> ::pest_typed::RuleStruct<'i, $Rule> for $node<'i> {
            fn span(&self) -> ::pest_typed::Span<'i> {
                self.span
            }
        }
        impl<'i: 'n, 'n> ::pest_typed::iterators::Pair<'i, 'n, $Rule> for $node<'i> {
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
/// Arguments: `($node:ident, $Rule:ty, $rule:expr)`.
#[macro_export]
macro_rules! impl_pair_with_content {
    ($node:ident, $Rule:ty, $rule:expr, $inner:ty) => {
        impl<'i: 'n, 'n> ::pest_typed::RuleStruct<'i, $Rule> for $node<'i> {
            fn span(&self) -> ::pest_typed::Span<'i> {
                self.span
            }
        }
        impl<'i: 'n, 'n> ::pest_typed::iterators::Pair<'i, 'n, $Rule> for $node<'i> {
            fn inner(
                &'n self,
            ) -> ::pest_typed::re_exported::vec::IntoIter<
                &'n (dyn ::pest_typed::iterators::Pair<'i, 'n, $Rule> + 'n),
            > {
                let i = <$inner as ::pest_typed::iterators::Pairs<'i, 'n, $Rule>>::iter(
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
                let i = <$inner as ::pest_typed::iterators::Pairs<'i, 'n, $Rule>>::into_iter(
                    *self.content,
                );
                i.collect::<::pest_typed::re_exported::Vec<_>>().into_iter()
            }
        }
    };
}

/// Inner tokens will be discarded, and only a [`Span`] will be contained.
///
/// And inner errors will **not** be tracked.
#[macro_export]
macro_rules! silent_rule {
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name<'i> {
            /// Matched expression.
            pub content: $inner,
        }
        impl<'i> ::pest_typed::TypedNode<'i, $Rule> for $name<'i> {
            fn try_parse_with<const ATOMIC: ::core::primitive::bool>(
                input: ::pest_typed::Position<'i>,
                stack: &mut ::pest_typed::Stack<::pest_typed::Span<'i>>,
                tracker: &mut ::pest_typed::tracker::Tracker<'i, $Rule>,
            ) -> ::core::result::Result<(::pest_typed::Position<'i>, Self), ()> {
                let start = input;
                match $inner::try_parse_with::<ATOMIC>(input, stack, tracker) {
                    ::core::result::Result::Ok((input, _)) => {
                        let span = start.span(&input);
                        ::core::result::Result::Ok((input, Self { span }))
                    }
                    ::core::result::Result::Err(_) => ::core::result::Result::Err(()),
                }
            }
        }
        ::pest_typed::impl_deref_with_content!($name, $inner);
        ::pest_typed::impl_pairs_with_inner!($name, $Rule, $rule, $inner);
    };
}

/// Start point of a normal rule.
///
/// Will not change atomicity.
///
/// See [`crate::atomic_rule!`] and [`crate::non_atomic_rule!`].
#[macro_export]
macro_rules! rule {
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty, $ignored:ty) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name<'i> {
            /// Matched content.
            pub content: ::pest_typed::re_exported::Box<$inner>,
            /// Matched span
            pub span: ::pest_typed::Span<'i>,
        }
        impl<'i> ::pest_typed::TypeWrapper for $name<'i> {
            type Inner = $inner;
        }
        impl<'i> ::pest_typed::RuleWrapper<$Rule> for $name<'i> {
            const RULE: $Rule = $rule;
            type Rule = $Rule;
        }
        impl<'i> ::pest_typed::TypedNode<'i, $Rule> for $name<'i> {
            #[inline]
            fn try_parse_with<const ATOMIC: ::core::primitive::bool>(
                input: ::pest_typed::Position<'i>,
                stack: &mut ::pest_typed::Stack<::pest_typed::Span<'i>>,
                tracker: &mut ::pest_typed::tracker::Tracker<'i, $Rule>,
            ) -> ::core::result::Result<(::pest_typed::Position<'i>, Self), ()> {
                tracker.record_during(input, |tracker| {
                    let start = input;
                    let (input, content) =
                        <$inner>::try_parse_with::<ATOMIC>(input, stack, tracker)?;
                    let span = start.span(&input);
                    let content = ::pest_typed::re_exported::Box::new(content);
                    Ok((input, Self { content, span }))
                })
            }
        }
        impl<'i> ::pest_typed::ParsableTypedNode<'i, $Rule> for $name<'i> {
            #[inline]
            fn parse(input: &'i str) -> Result<Self, ::pest_typed::error::Error<$Rule>> {
                ::pest_typed::rule::parse::<$Rule, Self, $ignored>(input, <$Rule>::EOI)
            }

            fn parse_partial(
                input: &'i str,
            ) -> Result<(::pest_typed::Position<'i>, Self), ::pest_typed::error::Error<$Rule>> {
                ::pest_typed::rule::parse_partial::<$Rule, Self>(input)
            }
        }
        ::pest_typed::impl_deref_with_content!($name, $inner);
        ::pest_typed::impl_pairs_with_self!($name, $Rule, $rule);
        ::pest_typed::impl_pair_with_content!($name, $Rule, $rule, $inner);
    };
}

/// Start point of an atomic rule.
///
/// Force inner tokens to be atomic.
///
/// See [`crate::rule!`] and [`crate::non_atomic_rule!`].
#[macro_export]
macro_rules! atomic_rule {
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name<'i> {
            /// Matched content.
            pub content: ::pest_typed::re_exported::Box<$inner>,
            /// Matched span.
            pub span: ::pest_typed::Span<'i>,
        }
        impl<'i> ::pest_typed::RuleWrapper<$Rule> for $name<'i> {
            const RULE: $Rule = $rule;
            type Rule = $Rule;
        }
        impl<'i> ::pest_typed::TypedNode<'i, $Rule> for $name<'i> {
            #[inline]
            fn try_parse_with<const ATOMIC: ::core::primitive::bool>(
                input: ::pest_typed::Position<'i>,
                stack: &mut ::pest_typed::Stack<::pest_typed::Span<'i>>,
                tracker: &mut ::pest_typed::tracker::Tracker<'i, $Rule>,
            ) -> ::core::result::Result<(::pest_typed::Position<'i>, Self), ()> {
                let start = input;
                tracker.record_during(start, |tracker| {
                    let (input, content) = <$inner>::try_parse_with::<true>(input, stack, tracker)?;
                    let content = ::pest_typed::re_exported::Box::new(content);
                    let span = start.span(&input);
                    let res = Self { content, span };
                    ::core::result::Result::Ok((input, res))
                })
            }
        }
        impl<'i> ::pest_typed::ParsableTypedNode<'i, $Rule> for $name<'i> {
            fn parse(
                input: &'i ::core::primitive::str,
            ) -> ::core::result::Result<Self, ::pest_typed::error::Error<$Rule>> {
                ::pest_typed::rule::parse_without_ignore::<$Rule, Self>(input, <$Rule>::EOI)
            }
            fn parse_partial(
                input: &'i str,
            ) -> ::core::result::Result<
                (::pest_typed::Position<'i>, Self),
                ::pest_typed::error::Error<$Rule>,
            > {
                ::pest_typed::rule::parse_partial::<$Rule, Self>(input)
            }
        }
        // ::pest_typed::impl_deref_with_content!($name, $inner);
        ::pest_typed::impl_pairs_with_self!($name, $Rule, $rule);
        ::pest_typed::impl_pair_with_empty!($name, $Rule, $rule);
    };
}

/// Start point of a non-atomic rule.
///
/// Force inner tokens to be not atomic.
///
/// See [`crate::rule!`] and [`crate::atomic_rule!`].
#[macro_export]
macro_rules! non_atomic_rule {
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name<'i> {
            /// Matched content.
            pub content: ::pest_typed::re_exported::Box<$inner>,
            /// Matched span.
            pub span: ::pest_typed::Span<'i>,
        }
        impl<'i> ::pest_typed::RuleWrapper<$Rule> for $name<'i> {
            const RULE: $Rule = $rule;
            type Rule = $Rule;
        }
        impl<'i> ::pest_typed::TypeWrapper for $name<'i> {
            type Inner = $inner;
        }
        impl<'i> ::pest_typed::TypedNode<'i, $Rule> for $name<'i> {
            #[inline]
            fn try_parse_with<const ATOMIC: ::core::primitive::bool>(
                input: ::pest_typed::Position<'i>,
                stack: &mut ::pest_typed::Stack<::pest_typed::Span<'i>>,
                tracker: &mut ::pest_typed::tracker::Tracker<'i, $Rule>,
            ) -> ::core::result::Result<(::pest_typed::Position<'i>, Self), ()> {
                tracker.record_during(input, |tracker| {
                    let start = input;
                    let (input, content) = $inner::try_parse_with::<false>(input, stack, tracker)?;
                    let content = ::pest_typed::re_exported::Box::new(content);
                    let span = start.span(&input);
                    ::core::result::Result::Ok((input, Self { content, span }))
                })
            }
        }
        impl<'i> ::pest_typed::ParsableTypedNode<'i, $Rule> for $name<'i> {
            fn parse(
                input: &'i ::core::primitive::str,
            ) -> ::core::result::Result<Self, ::pest_typed::error::Error<$Rule>> {
                ::pest_typed::rule::parse::<$Rule, _EOI, Self, IGNORED>(input)
            }
            fn parse_partial(
                input: &'i ::core::primitive::str,
            ) -> ::core::result::Result<
                (::pest_typed::Position<'i>, Self),
                ::pest_typed::error::Error<$Rule>,
            > {
                ::pest_typed::rule::parse_partial::<$Rule, Self>(input)
            }
        }
        ::pest_typed::impl_deref_with_content!($name, $inner);
        ::pest_typed::impl_pairs_with_self!($name, $Rule, $rule);
        ::pest_typed::impl_pair_with_content!($name, $Rule, $rule, $inner);
    };
}

/// Full parse as non-atomic rule.
///
/// For [`crate::rule!`].
pub fn parse<
    'i,
    R: RuleType + 'i,
    _Self: TypedNode<'i, R>,
    IGNORED: NeverFailedTypedNode<'i, R>,
>(
    input: &'i str,
    rule_eoi: R,
) -> Result<_Self, Error<R>> {
    let mut stack = Stack::new();
    let input = Position::from_start(input);
    let mut tracker = Tracker::new(input);
    let (input, res) = match _Self::try_parse_with::<false>(input, &mut stack, &mut tracker) {
        Ok((input, res)) => (input, res),
        Err(_) => return Err(tracker.collect()),
    };
    let (input, _) = IGNORED::parse_with::<false>(input, &mut stack);
    let (_, _) = match tracker.record_during_with(
        input,
        |tracker| EOI::try_parse_with::<false>(input, &mut stack, tracker),
        rule_eoi,
    ) {
        Ok((input, res)) => (input, res),
        Err(_) => return Err(tracker.collect()),
    };
    Ok(res)
}

/// Full parse as non-atomic rule.
///
/// For [`crate::atomic_rule!`].
pub fn parse_without_ignore<'i, R: RuleType + 'i, _Self: TypedNode<'i, R>>(
    input: &'i str,
    rule_eoi: R,
) -> Result<_Self, Error<R>> {
    let mut stack = Stack::new();
    let input = Position::from_start(input);
    let mut tracker = Tracker::new(input);
    let (input, res) = match _Self::try_parse_with::<false>(input, &mut stack, &mut tracker) {
        Ok((input, res)) => (input, res),
        Err(_) => return Err(tracker.collect()),
    };
    let (_, _) = match tracker.record_during_with(
        input,
        |tracker| EOI::try_parse_with::<false>(input, &mut stack, tracker),
        rule_eoi,
    ) {
        Ok((input, res)) => (input, res),
        Err(_) => return Err(tracker.collect()),
    };
    Ok(res)
}

/// Partially parse.
///
/// For [`crate::ParsableTypedNode::parse_partial()`]
pub fn parse_partial<'i, R: RuleType, _Self: TypedNode<'i, R>>(
    input: &'i str,
) -> Result<(Position<'i>, _Self), Error<R>> {
    let mut stack = Stack::new();
    let input = Position::from_start(input);
    let mut tracker = Tracker::new(input);
    match _Self::try_parse_with::<false>(input, &mut stack, &mut tracker) {
        Ok((input, res)) => Ok((input, res)),
        Err(_) => Err(tracker.collect()),
    }
}
