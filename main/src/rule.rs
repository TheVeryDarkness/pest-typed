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
    ($name:ident, $Rule:ty) => {
        impl<'i: 'n, 'n> ::pest_typed::iterators::Pairs<'i, 'n, $Rule> for $name<'i> {
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
    ($name:ident, $Rule:ty, $inner:ty) => {
        impl<'i: 'n, 'n> ::pest_typed::iterators::Pairs<'i, 'n, $Rule> for $name<'i> {
            type Iter = ::pest_typed::re_exported::vec::IntoIter<
                &'n dyn ::pest_typed::iterators::Pair<'i, 'n, $Rule>,
            >;
            type IntoIter = ::pest_typed::re_exported::vec::IntoIter<
                ::pest_typed::re_exported::Box<
                    dyn ::pest_typed::iterators::Pair<'i, 'n, $Rule> + 'n,
                >,
            >;

            fn iter(&'n self) -> Self::Iter {
                let i =
                    <$inner as ::pest_typed::iterators::Pairs<'i, 'n, $Rule>>::iter(&self.content);
                i.collect::<::pest_typed::re_exported::Vec<_>>().into_iter()
            }
            fn into_iter(self) -> Self::IntoIter {
                let i = self.content.into_iter();
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
/// - `$name:ident`: struct name.
/// - `$Rule:ty`: rule type.
/// - `$rule:expr`: rule value.
/// - `$inner:ty`: inner type.
/// - `InnerExpression`, `Span` or `Both`: emission.
#[macro_export]
macro_rules! impl_pairs {
    ($name:ident, $Rule:ty, $inner:ty, InnerExpression) => {
        ::pest_typed::impl_pairs_with_inner!($name, $Rule, $inner);
    };
    ($name:ident, $Rule:ty, $inner:ty, $emit:tt) => {
        ::pest_typed::impl_pairs_with_self!($name, $Rule);
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

/// Implement [`core::ops::Deref`] for structs if they have content.
#[macro_export]
macro_rules! impl_deref {
    ($name:ident, $inner:ty, true) => {};
    ($name:ident, $inner:ty, false) => {
        ::pest_typed::impl_deref_with_content!($name, $inner);
    };
    ($name:ident, $inner:ty, ATOMIC) => {
        ::pest_typed::impl_deref_with_content!($name, $inner);
    };
}

/// Implement [`Pair`](crate::iterators::Pair) for a struct without inner [`Pair`](crate::iterators::Pair)s.
///
/// Normally used by atomic rules.
///
/// Arguments: `($name:ident, $Rule:ty, $rule:expr)`.
#[macro_export]
macro_rules! impl_pair_with_empty {
    ($name:ident, $Rule:ty, $rule:expr) => {
        impl<'i: 'n, 'n> ::pest_typed::RuleStruct<'i, $Rule> for $name<'i> {
            fn span(&self) -> ::pest_typed::Span<'i> {
                self.span
            }
        }
        impl<'i: 'n, 'n> ::pest_typed::iterators::Pair<'i, 'n, $Rule> for $name<'i> {
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
/// - `$name:ident`: struct name.
/// - `$Rule:ty`: rule type.
/// - `$rule:expr`: rule value.
/// - `$inner:ty`: inner type.
#[macro_export]
macro_rules! impl_pair_with_content {
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty) => {
        impl<'i> ::pest_typed::TypeWrapper for $name<'i> {
            type Inner = $inner;
        }
        impl<'i: 'n, 'n> ::pest_typed::RuleStruct<'i, $Rule> for $name<'i> {
            fn span(&self) -> ::pest_typed::Span<'i> {
                self.span
            }
        }
        impl<'i: 'n, 'n> ::pest_typed::iterators::Pair<'i, 'n, $Rule> for $name<'i> {
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

/// Implement [`Pair`](crate::iterators::Pair) for a struct.
///
/// Arguments:
/// - `$name:ident`: struct name.
/// - `$Rule:ty`: rule type.
/// - `$rule:expr`: rule value.
/// - `$inner:ty`: inner type.
/// - `true`, `false` or `None`: atomicity.
/// - `InnerExpression`, `Span` or `Both`: emission.
#[macro_export]
macro_rules! impl_pair {
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty, $atomicity:expr, InnerExpression) => {};
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty, true, $emit:tt) => {
        ::pest_typed::impl_pair_with_empty!($name, $Rule, $rule);
    };
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty, false, $emit:tt) => {
        ::pest_typed::impl_pair_with_content!($name, $Rule, $rule, $inner);
    };
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty, ATOMIC, $emit:tt) => {
        ::pest_typed::impl_pair_with_content!($name, $Rule, $rule, $inner);
    };
}

/// Implement [`crate::ParsableTypedNode::parse()`] for structs.
#[macro_export]
macro_rules! impl_parse {
    ($name:ident, $Rule:ty, $ignored:ty, true) => {
        impl<'i> ::pest_typed::ParsableTypedNode<'i, $Rule> for $name<'i> {
            #[inline]
            fn parse(
                input: &'i ::core::primitive::str,
            ) -> ::core::result::Result<Self, ::pest_typed::error::Error<$Rule>> {
                ::pest_typed::rule::parse_without_ignore::<$Rule, Self>(input, <$Rule>::EOI)
            }
        }
    };
    ($name:ident, $Rule:ty, $ignored:ty, $non_true:tt) => {
        impl<'i> ::pest_typed::ParsableTypedNode<'i, $Rule> for $name<'i> {
            #[inline]
            fn parse(
                input: &'i ::core::primitive::str,
            ) -> ::core::result::Result<Self, ::pest_typed::error::Error<$Rule>> {
                ::pest_typed::rule::parse::<$Rule, Self, $ignored>(input, <$Rule>::EOI)
            }
        }
    };
}

/// Implement [`crate::TypedNode::try_parse_with()`] for structs.
#[macro_export]
macro_rules! impl_try_parse_with {
    ($name:ident, $Rule:ty, $inner:ty, $atomicity:expr, InnerExpression) => {
        impl<'i> ::pest_typed::TypedNode<'i, $Rule> for $name<'i> {
            #[inline]
            fn try_parse_with<const ATOMIC: ::core::primitive::bool>(
                input: ::pest_typed::Position<'i>,
                stack: &mut ::pest_typed::Stack<::pest_typed::Span<'i>>,
                tracker: &mut ::pest_typed::tracker::Tracker<'i, $Rule>,
            ) -> ::core::result::Result<(::pest_typed::Position<'i>, Self), ()> {
                let (input, content) =
                    <$inner>::try_parse_with::<$atomicity>(input, stack, tracker)?;
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
        impl<'i> ::pest_typed::TypedNode<'i, $Rule> for $name<'i> {
            #[inline]
            fn try_parse_with<const ATOMIC: ::core::primitive::bool>(
                input: ::pest_typed::Position<'i>,
                stack: &mut ::pest_typed::Stack<::pest_typed::Span<'i>>,
                tracker: &mut ::pest_typed::tracker::Tracker<'i, $Rule>,
            ) -> ::core::result::Result<(::pest_typed::Position<'i>, Self), ()> {
                tracker.record_during(input, |tracker| {
                    let start = input;
                    let (input, _) = <$inner>::try_parse_with::<$atomicity>(input, stack, tracker)?;
                    let span = start.span(&input);
                    Ok((input, Self { span }))
                })
            }
        }
    };
    ($name:ident, $Rule:ty, $inner:ty, $atomicity:expr, Both) => {
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
                        <$inner>::try_parse_with::<$atomicity>(input, stack, tracker)?;
                    let span = start.span(&input);
                    let content = ::pest_typed::re_exported::Box::new(content);
                    Ok((input, Self { content, span }))
                })
            }
        }
    };
}

/// Fields of structs.
#[macro_export]
macro_rules! impl_rule_wrapper {
    ($name:ident, $Rule:ty, $rule:expr) => {
        impl<'i> ::pest_typed::RuleWrapper<$Rule> for $name<'i> {
            const RULE: $Rule = $rule;
            type Rule = $Rule;
        }
    };
}

/// Fields of structs.
#[macro_export]
macro_rules! declare_rule_struct {
    ($name:ident, $inner:ty, InnerExpression) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq)]
        pub struct $name<'i> {
            /// Matched expression.
            pub content: ::pest_typed::re_exported::Box<$inner>,
            _phantom: ::core::marker::PhantomData<&'i ::core::primitive::str>,
        }
        impl<'i> ::core::fmt::Debug for $name<'i> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct("Rule")
                    .field("name", &::core::stringify!($name))
                    .field("content", &self.content)
                    .finish()
            }
        }
    };
    ($name:ident, $inner:ty, Span) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq)]
        pub struct $name<'i> {
            /// Span of matched expression.
            pub span: ::pest_typed::Span<'i>,
        }
        impl<'i> ::core::fmt::Debug for $name<'i> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct("Rule")
                    .field("name", &::core::stringify!($name))
                    .field("span", &self.span)
                    .finish()
            }
        }
    };
    ($name:ident, $inner:ty, Both) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq)]
        pub struct $name<'i> {
            /// Matched expression.
            pub content: ::pest_typed::re_exported::Box<$inner>,
            /// Span of matched expression.
            pub span: ::pest_typed::Span<'i>,
        }
        impl<'i> ::core::fmt::Debug for $name<'i> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct("Rule")
                    .field("name", &::core::stringify!($name))
                    .field("content", &self.content)
                    .field("span", &self.span)
                    .finish()
            }
        }
    };
}

/// The start point of a node tag.
#[macro_export]
macro_rules! tag {
    ($name:ident, $Rule:ty, $inner:ty) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name<'i> {
            /// Matched expression.
            pub content: $inner,
            /// Span of matched expression.
            pub span: ::pest_typed::Span<'i>,
        }
        impl<'i> ::pest_typed::TypedNode<'i, $Rule> for $name<'i> {
            fn try_parse_with<const ATOMIC: ::core::primitive::bool>(
                input: ::pest_typed::Position<'i>,
                stack: &mut ::pest_typed::Stack<::pest_typed::Span<'i>>,
                tracker: &mut ::pest_typed::tracker::Tracker<'i, $Rule>,
            ) -> ::core::result::Result<(::pest_typed::Position<'i>, Self), ()> {
                let start = input;
                match <$inner>::try_parse_with::<ATOMIC>(input, stack, tracker) {
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

/// Start point of a normal rule.
///
/// Will not change atomicity.
///
/// See [`crate::atomic_rule!`] and [`crate::non_atomic_rule!`].
#[macro_export]
macro_rules! rule {
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty, $ignored:ty, $atomicity:tt, $emission:tt) => {
        ::pest_typed::declare_rule_struct! {$name, $inner, $emission}
        ::pest_typed::impl_rule_wrapper!($name, $Rule, $rule);
        ::pest_typed::impl_try_parse_with!($name, $Rule, $inner, $atomicity, $emission);
        ::pest_typed::impl_parse!($name, $Rule, $ignored, $atomicity);
        ::pest_typed::impl_deref!($name, $inner, $atomicity);
        ::pest_typed::impl_pairs!($name, $Rule, $inner, $emission);
        ::pest_typed::impl_pair!($name, $Rule, $rule, $inner, $atomicity, $emission);
    };
}
/// Shortcut for atomic rule in pest.
#[macro_export]
macro_rules! atomic_rule {
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty) => {
        ::pest_typed::rule!($name, $Rule, $rule, $inner, (), true, Span);
    };
}

/// Shortcut for compound atomic rule in pest.
#[macro_export]
macro_rules! compound_atomic_rule {
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty) => {
        ::pest_typed::rule!($name, $Rule, $rule, $inner, (), true, Both);
    };
}

/// Shortcut for non-atomic rule in pest.
#[macro_export]
macro_rules! non_atomic_rule {
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty, $ignored:ty) => {
        ::pest_typed::rule!($name, $Rule, $rule, $inner, $ignored, false, Both);
    };
}

/// Shortcut for normal rule in pest.
#[macro_export]
macro_rules! normal_rule {
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty, $ignored:ty) => {
        ::pest_typed::rule!($name, $Rule, $rule, $inner, $ignored, ATOMIC, Both);
    };
}

/// Shortcut for silent rule in pest.
#[macro_export]
macro_rules! silent_rule {
    ($name:ident, $Rule:ty, $rule:expr, $inner:ty, $ignored:ty) => {
        ::pest_typed::rule!(
            $name,
            $Rule,
            $rule,
            $inner,
            $ignored,
            ATOMIC,
            InnerExpression
        );
    };
}

/// Start point of a normal rule.
///
/// Will not change atomicity.
///
/// See [`crate::atomic_rule!`] and [`crate::non_atomic_rule!`].
#[macro_export]
macro_rules! rule_eoi {
    ($name:ident, $Rule:ty) => {
        ::pest_typed::declare_rule_struct! {$name, ::pest_typed::predefined_node::EOI, Both}
        ::pest_typed::impl_rule_wrapper!($name, $Rule, <$Rule>::EOI);
        ::pest_typed::impl_try_parse_with!(
            $name,
            $Rule,
            ::pest_typed::predefined_node::EOI,
            ATOMIC,
            Both
        );
        impl<'i> ::pest_typed::ParsableTypedNode<'i, $Rule> for $name<'i> {
            #[inline]
            fn parse(
                input: &'i ::core::primitive::str,
            ) -> ::core::result::Result<Self, ::pest_typed::error::Error<$Rule>> {
                ::pest_typed::rule::parse_without_ignore::<$Rule, Self>(input, <$Rule>::EOI)
            }
        }
        ::pest_typed::impl_deref!($name, ::pest_typed::predefined_node::EOI, ATOMIC);
        ::pest_typed::impl_pairs_with_self!($name, $Rule);
        ::pest_typed::impl_pair_with_empty!($name, $Rule, <$Rule>::EOI);
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
