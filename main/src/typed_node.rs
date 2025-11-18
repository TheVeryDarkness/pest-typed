// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::{
    error::Error, line_indexer::LineIndexer, predefined_node::restore_on_none, span::Span,
    tracker::Tracker, Cursor, Input, RuleType, RuleWrapper, Stack,
};
use alloc::{boxed::Box, vec::Vec};
use core::fmt::Debug;

/// Node of concrete syntax tree that never fails.
pub trait NeverFailedTypedNode<C: Cursor, R: RuleType>
where
    Self: Sized + Debug + Clone + PartialEq + Default,
{
    /// Create typed node.
    fn parse_with(cursor: C, stack: &mut Stack<Span<C::String>>) -> (C, Self);

    /// Check how much input can be matched.
    fn check_with(cursor: C, stack: &mut Stack<Span<C::String>>) -> C;
}

/// Node of concrete syntax tree.
pub trait TypedNode<C: Cursor, R: RuleType>
where
    Self: Sized + Debug + Clone + PartialEq,
{
    /// Try to create typed node.
    fn try_parse_partial_with(
        cursor: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)>;

    /// Check whether the typed node match some prefix of the input.
    fn try_check_partial_with(
        cursor: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<C>;
}

/// Node of concrete syntax tree.
pub trait ParsableTypedNode<C: Cursor, R: RuleType>: TypedNode<C, R> {
    /// Try to create typed node until the end.
    fn try_parse_with(
        cursor: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<Self>;
    /// Try to parse the whole input into given typed node.
    /// A rule is not atomic by default.
    #[inline]
    fn try_parse_with_cache(
        input: impl Input<Cursor = C>,
        indexer: impl LineIndexer<C::String>,
    ) -> Result<Self, Box<Error<R>>> {
        let cursor = input.as_cursor();
        let mut stack = Stack::new();
        let mut tracker = Tracker::new(cursor.as_position());
        match Self::try_parse_with(cursor, &mut stack, &mut tracker) {
            Some(res) => Ok(res),
            None => Err(Box::new(tracker.collect(indexer))),
        }
    }
    /// Try to parse the whole input into given typed node.
    /// A rule is not atomic by default.
    #[inline]
    fn try_parse_partial_with_cache(
        input: impl Input<Cursor = C>,
        indexer: impl LineIndexer<C::String>,
    ) -> Result<(C, Self), Box<Error<R>>> {
        let cursor = input.as_cursor();
        let mut stack = Stack::new();
        let mut tracker = Tracker::new(cursor.as_position());
        match Self::try_parse_partial_with(cursor, &mut stack, &mut tracker) {
            Some((cursor, res)) => Ok((cursor, res)),
            None => Err(Box::new(tracker.collect(indexer))),
        }
    }
    /// Try to parse the whole input into given typed node.
    /// A rule is not atomic by default.
    #[inline]
    fn try_parse(input: impl Input<Cursor = C>) -> Result<Self, Box<Error<R>>> {
        Self::try_parse_with_cache(input, ())
    }
    /// Try to parse the whole input into given typed node.
    /// A rule is not atomic by default.
    #[inline]
    fn try_parse_partial(input: impl Input<Cursor = C>) -> Result<(C, Self), Box<Error<R>>> {
        Self::try_parse_partial_with_cache(input, ())
    }

    /// Check whether the typed node match the whole input.
    fn try_check_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> bool;
    /// Check whether the typed node match the whole input.
    #[inline]
    fn try_check_with_cache(
        input: impl Input<Cursor = C>,
        indexer: impl LineIndexer<C::String>,
    ) -> Result<(), Box<Error<R>>> {
        let cursor = input.as_cursor();
        let mut stack = Stack::new();
        let mut tracker = Tracker::new(cursor.as_position());
        match Self::try_check_with(cursor, &mut stack, &mut tracker) {
            true => Ok(()),
            false => Err(Box::new(tracker.collect(indexer))),
        }
    }
    /// Try to parse the whole input into given typed node.
    /// A rule is not atomic by default.
    #[inline]
    fn try_check_partial_with_cache(
        input: impl Input<Cursor = C>,
        indexer: impl LineIndexer<C::String>,
    ) -> Result<C, Box<Error<R>>> {
        let cursor = input.as_cursor();
        let mut stack = Stack::new();
        let mut tracker = Tracker::new(cursor.as_position());
        match Self::try_check_partial_with(cursor, &mut stack, &mut tracker) {
            Some(input) => Ok(input),
            None => Err(Box::new(tracker.collect(indexer))),
        }
    }
    /// Check whether the typed node match the whole input.
    #[inline]
    fn try_check(input: impl Input<Cursor = C>) -> Result<(), Box<Error<R>>> {
        Self::try_check_with_cache(input, ())
    }
    /// Try to parse the whole input into given typed node.
    /// A rule is not atomic by default.
    #[inline]
    fn try_check_partial(input: impl Input<Cursor = C>) -> Result<C, Box<Error<R>>> {
        Self::try_check_partial_with_cache(input, ())
    }
}

/// Node of concrete syntax tree.
pub trait NeverFailedParsableTypedNode<C: Cursor, R: RuleType>: NeverFailedTypedNode<C, R> {
    /// Create typed node.
    fn parse_with_until_end(cursor: C, stack: &mut Stack<Span<C::String>>) -> Self;
    /// Parse the whole input into given typed node.
    /// A rule is not atomic by default.
    #[inline]
    fn parse(cursor: C) -> Self {
        let mut stack = Stack::new();
        Self::parse_with_until_end(cursor, &mut stack)
    }
    /// Parse the whole input into given typed node.
    /// A rule is not atomic by default.
    #[inline]
    fn parse_partial(cursor: C) -> (C, Self) {
        let mut stack = Stack::new();
        Self::parse_with(cursor, &mut stack)
    }
}

pub trait RuleStorage<R: RuleType> {
    fn rule(&self) -> R;
}
impl<R: RuleType, T: RuleWrapper<R>> RuleStorage<R> for T {
    #[inline(always)]
    fn rule(&self) -> R {
        T::RULE
    }
}

/// A trait for those struct that correspond to non-silent rules.
pub trait Spanned<I, R: RuleType> {
    /// The span of a matched expression by a non-silent rule.
    fn span(&self) -> Span<I>;
}

/// A trait for those struct that correspond to rules with inner expression.
pub trait RuleStruct<I, R: RuleType>: RuleStorage<R> {
    /// Inner type.
    type Inner;
    /// Take inner content.
    fn take_inner(self) -> Self::Inner;
    /// Reference inner content.
    fn ref_inner(&self) -> &Self::Inner;
    /// Reference inner content mutably.
    fn mut_inner(&mut self) -> &mut Self::Inner;
}

/// Match `[T; N]`.
impl<C: Cursor, R: RuleType, T: TypedNode<C, R>, const N: usize> TypedNode<C, R> for [T; N] {
    #[inline]
    fn try_parse_partial_with(
        mut input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        let mut vec = Vec::new();
        for _ in 0..N {
            let (next, res) = T::try_parse_partial_with(input, stack, tracker)?;
            input = next;
            vec.push(res);
        }
        match vec.try_into() {
            Ok(res) => Some((input, res)),
            // Actually impossible.
            Err(_) => None,
        }
    }

    #[inline]
    fn try_check_partial_with(
        mut input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        for _ in 0..N {
            let next = T::try_check_partial_with(input, stack, tracker)?;
            input = next;
        }
        Some(input)
    }
}

/// Match `(T1, T2)`.
impl<C: Cursor, R: RuleType, T1: TypedNode<C, R>, T2: TypedNode<C, R>> TypedNode<C, R>
    for (T1, T2)
{
    #[inline]
    fn try_parse_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        let (input, t1) = T1::try_parse_partial_with(input, stack, tracker)?;
        let (input, t2) = T2::try_parse_partial_with(input, stack, tracker)?;
        Some((input, (t1, t2)))
    }

    #[inline]
    fn try_check_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        let input = T1::try_check_partial_with(input, stack, tracker)?;
        T2::try_check_partial_with(input, stack, tracker)
    }
}

/// Optionally match `T`.
impl<C: Cursor, R: RuleType, T: TypedNode<C, R>> TypedNode<C, R> for Option<T> {
    #[inline]
    fn try_parse_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        let res = restore_on_none(stack, |stack| {
            T::try_parse_partial_with(input.clone(), stack, tracker)
        });
        match res {
            Some((input, inner)) => Some((input, Some(inner))),
            None => Some((input, None)),
        }
    }

    #[inline]
    fn try_check_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        match restore_on_none(stack, |stack| {
            T::try_check_partial_with(input.clone(), stack, tracker)
        }) {
            Some(input) => Some(input),
            None => Some(input),
        }
    }
}
