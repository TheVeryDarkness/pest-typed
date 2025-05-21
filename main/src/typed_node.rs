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
    tracker::Tracker, AsInput, Input, RuleType, RuleWrapper, Stack,
};
use alloc::{boxed::Box, vec::Vec};
use core::fmt::Debug;

/// Node of concrete syntax tree that never fails.
pub trait NeverFailedTypedNode<'i, R: RuleType>
where
    Self: Sized + Debug + Clone + PartialEq + Default,
{
    /// Create typed node.
    fn parse_with<I: Input<'i>>(input: I, stack: &mut Stack<Span<'i>>) -> (I, Self);

    /// Check how much input can be matched.
    fn check_with<I: Input<'i>>(input: I, stack: &mut Stack<Span<'i>>) -> I;
}

/// Node of concrete syntax tree.
pub trait TypedNode<'i, R: RuleType>
where
    Self: Sized + Debug + Clone + PartialEq,
{
    /// Try to create typed node.
    fn try_parse_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)>;

    /// Check whether the typed node match some prefix of the input.
    fn try_check_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<I>;
}

/// Node of concrete syntax tree.
pub trait ParsableTypedNode<'i, R: RuleType>: TypedNode<'i, R> {
    /// Try to create typed node until the end.
    fn try_parse_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<Self>;
    /// Try to parse the whole input into given typed node.
    /// A rule is not atomic by default.
    fn try_parse_with_cache(
        input: impl AsInput<'i>,
        indexer: impl LineIndexer<'i>,
    ) -> Result<Self, Box<Error<R>>> {
        let mut stack = Stack::new();
        let input = input.as_input();
        let mut tracker = Tracker::new(input);
        match Self::try_parse_with(input, &mut stack, &mut tracker) {
            Some(res) => Ok(res),
            None => Err(Box::new(tracker.collect(indexer))),
        }
    }
    /// Try to parse the whole input into given typed node.
    /// A rule is not atomic by default.
    fn try_parse_partial_with_cache<I: AsInput<'i>>(
        input: I,
        indexer: impl LineIndexer<'i>,
    ) -> Result<(I::Output, Self), Box<Error<R>>> {
        let mut stack = Stack::new();
        let input = input.as_input();
        let mut tracker = Tracker::new(input);
        match Self::try_parse_partial_with(input, &mut stack, &mut tracker) {
            Some((input, res)) => Ok((input, res)),
            None => Err(Box::new(tracker.collect(indexer))),
        }
    }
    /// Try to parse the whole input into given typed node.
    /// A rule is not atomic by default.
    fn try_parse(input: impl AsInput<'i>) -> Result<Self, Box<Error<R>>> {
        Self::try_parse_with_cache(input, ())
    }
    /// Try to parse the whole input into given typed node.
    /// A rule is not atomic by default.
    fn try_parse_partial<I: AsInput<'i>>(input: I) -> Result<(I::Output, Self), Box<Error<R>>> {
        Self::try_parse_partial_with_cache(input, ())
    }

    /// Check whether the typed node match the whole input.
    fn try_check_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> bool;
    /// Check whether the typed node match the whole input.
    fn try_check_with_cache(
        input: impl AsInput<'i>,
        indexer: impl LineIndexer<'i>,
    ) -> Result<(), Box<Error<R>>> {
        let mut stack = Stack::new();
        let input = input.as_input();
        let mut tracker = Tracker::new(input);
        match Self::try_check_with(input, &mut stack, &mut tracker) {
            true => Ok(()),
            false => Err(Box::new(tracker.collect(indexer))),
        }
    }
    /// Try to parse the whole input into given typed node.
    /// A rule is not atomic by default.
    fn try_check_partial_with_cache<I: AsInput<'i>>(
        input: I,
        indexer: impl LineIndexer<'i>,
    ) -> Result<I::Output, Box<Error<R>>> {
        let mut stack = Stack::new();
        let input = input.as_input();
        let mut tracker = Tracker::new(input);
        match Self::try_check_partial_with(input, &mut stack, &mut tracker) {
            Some(input) => Ok(input),
            None => Err(Box::new(tracker.collect(indexer))),
        }
    }
    /// Check whether the typed node match the whole input.
    fn try_check(input: impl AsInput<'i>) -> Result<(), Box<Error<R>>> {
        Self::try_check_with_cache(input, ())
    }
    /// Try to parse the whole input into given typed node.
    /// A rule is not atomic by default.
    fn try_check_partial<I: AsInput<'i>>(input: I) -> Result<I::Output, Box<Error<R>>> {
        Self::try_check_partial_with_cache(input, ())
    }
}

/// Node of concrete syntax tree.
pub trait NeverFailedParsableTypedNode<'i, R: RuleType>: NeverFailedTypedNode<'i, R> {
    /// Create typed node.
    fn parse_with_until_end<I: Input<'i>>(input: I, stack: &mut Stack<Span<'i>>) -> Self;
    /// Parse the whole input into given typed node.
    /// A rule is not atomic by default.
    fn parse(input: impl AsInput<'i>) -> Self {
        let mut stack = Stack::new();
        let input = input.as_input();
        Self::parse_with_until_end(input, &mut stack)
    }
    /// Parse the whole input into given typed node.
    /// A rule is not atomic by default.
    fn parse_partial<I: AsInput<'i>>(input: I) -> (I::Output, Self) {
        let mut stack = Stack::new();
        let input = input.as_input();
        Self::parse_with(input, &mut stack)
    }
}

pub trait RuleStorage<R: RuleType> {
    fn rule(&self) -> R;
}
impl<R: RuleType, T: RuleWrapper<R>> RuleStorage<R> for T {
    fn rule(&self) -> R {
        T::RULE
    }
}

/// A trait for those struct that correspond to non-silent rules.
pub trait Spanned<'i, R: RuleType> {
    /// The span of a matched expression by a non-silent rule.
    fn span(&self) -> Span<'i>;
}

/// A trait for those struct that correspond to rules with inner expression.
pub trait RuleStruct<'i, R: RuleType>: RuleStorage<R> {
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
impl<'i, R: RuleType, T: TypedNode<'i, R>, const N: usize> TypedNode<'i, R> for [T; N] {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        mut input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
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
    fn try_check_partial_with<I: Input<'i>>(
        mut input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
        for _ in 0..N {
            let next = T::try_check_partial_with(input, stack, tracker)?;
            input = next;
        }
        Some(input)
    }
}

/// Match `(T1, T2)`.
impl<'i, R: RuleType, T1: TypedNode<'i, R>, T2: TypedNode<'i, R>> TypedNode<'i, R> for (T1, T2) {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        let (input, t1) = T1::try_parse_partial_with(input, stack, tracker)?;
        let (input, t2) = T2::try_parse_partial_with(input, stack, tracker)?;
        Some((input, (t1, t2)))
    }

    #[inline]
    fn try_check_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
        let input = T1::try_check_partial_with(input, stack, tracker)?;
        T2::try_check_partial_with(input, stack, tracker)
    }
}

/// Optionally match `T`.
impl<'i, R: RuleType, T: TypedNode<'i, R>> TypedNode<'i, R> for Option<T> {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        let res = restore_on_none(stack, |stack| {
            T::try_parse_partial_with(input, stack, tracker)
        });
        match res {
            Some((input, inner)) => Some((input, Some(inner))),
            None => Some((input, None)),
        }
    }

    fn try_check_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
        match restore_on_none(stack, |stack| {
            T::try_check_partial_with(input, stack, tracker)
        }) {
            Some(input) => Some(input),
            None => Some(input),
        }
    }
}
