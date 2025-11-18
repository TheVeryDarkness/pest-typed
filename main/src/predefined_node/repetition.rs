// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Predefined tree nodes generics.
//! The generator may use this for convenience.
//! Normally you don't need to reference this module by yourself.

use crate::{
    predefined_node::{restore_on_none, Skipped},
    tracker::Tracker,
    wrapper::BoundWrapper,
    Cursor, NeverFailedTypedNode, RuleType, Span, Stack, TypedNode,
};
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};

type Iter<'n, T, IGNORED, const SKIP: usize> = core::iter::Map<
    alloc::slice::Iter<'n, Skipped<T, IGNORED, SKIP>>,
    fn(&'n Skipped<T, IGNORED, SKIP>) -> &'n T,
>;
type IntoIter<T, IGNORED, const SKIP: usize> = core::iter::Map<
    alloc::vec::IntoIter<Skipped<T, IGNORED, SKIP>>,
    fn(Skipped<T, IGNORED, SKIP>) -> T,
>;

/// Repeatably match `T` at least `MIN` times.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct AtomicRepeat<T> {
    /// Skipped and Matched expressions.
    pub content: Vec<T>,
}
impl<T> Default for AtomicRepeat<T> {
    fn default() -> Self {
        let content = Vec::new();
        Self { content }
    }
}
impl<C: Cursor, R: RuleType, T: TypedNode<C, R>> NeverFailedTypedNode<C, R> for AtomicRepeat<T> {
    #[inline]
    fn parse_with(mut input: C, stack: &mut Stack<Span<C::String>>) -> (C, Self) {
        let mut vec = Vec::new();
        let mut tracker = Tracker::new(input.as_position());

        for _ in 0usize.. {
            match restore_on_none(stack, |stack| {
                T::try_parse_partial_with(input.clone(), stack, &mut tracker)
            }) {
                Some((next, matched)) => {
                    input = next;
                    vec.push(matched);
                }
                None => break,
            }
        }
        (input, Self { content: vec })
    }

    #[inline]
    fn check_with(mut input: C, stack: &mut Stack<Span<C::String>>) -> C {
        let mut tracker = Tracker::new(input.as_position());

        for _ in 0usize.. {
            match restore_on_none(stack, |stack| {
                T::try_check_partial_with(input.clone(), stack, &mut tracker)
            }) {
                Some(next) => {
                    input = next;
                }
                None => break,
            }
        }
        input
    }
}
impl<C: Cursor, R: RuleType, T: TypedNode<C, R>> TypedNode<C, R> for AtomicRepeat<T> {
    #[inline]
    fn try_parse_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        Some(Self::parse_with(input, stack))
    }

    #[inline]
    fn try_check_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        Some(Self::check_with(input, stack))
    }
}
impl<T> Deref for AtomicRepeat<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
impl<T> DerefMut for AtomicRepeat<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}
impl<T: Clone + PartialEq> BoundWrapper for AtomicRepeat<T> {
    const MIN: usize = 0;
    const MAX: usize = usize::MAX;
}

/// Repeatably match `T` at least `MIN` times.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct RepeatMin<T, const MIN: usize> {
    /// Skipped and Matched expressions.
    pub content: Vec<T>,
}
impl<
        C: Cursor,
        R: RuleType,
        T: TypedNode<C, R>,
        Skip: NeverFailedTypedNode<C, R>,
        const SKIP: usize,
    > NeverFailedTypedNode<C, R> for RepeatMin<Skipped<T, Skip, SKIP>, 0>
{
    #[inline]
    fn parse_with(mut input: C, stack: &mut Stack<Span<C::String>>) -> (C, Self) {
        let mut vec = Vec::new();
        let mut tracker = Tracker::new(input.as_position());

        for i in 0usize.. {
            match restore_on_none(stack, |stack| {
                try_parse_unit(input.clone(), stack, &mut tracker, i)
            }) {
                Some((next, matched)) => {
                    input = next;
                    vec.push(matched);
                }
                None => break,
            }
        }
        (input, Self { content: vec })
    }

    #[inline]
    fn check_with(mut input: C, stack: &mut Stack<Span<C::String>>) -> C {
        let mut tracker = Tracker::new(input.as_position());

        for i in 0usize.. {
            match restore_on_none(stack, |stack| {
                try_check_unit::<C, R, T, Skip, SKIP>(input.clone(), stack, &mut tracker, i)
            }) {
                Some(next) => {
                    input = next;
                }
                None => break,
            }
        }
        input
    }
}
impl<T> Default for RepeatMin<T, 0> {
    fn default() -> Self {
        let content = Vec::new();
        Self { content }
    }
}
impl<
        C: Cursor,
        R: RuleType,
        T: TypedNode<C, R>,
        Skip: NeverFailedTypedNode<C, R>,
        const SKIP: usize,
        const MIN: usize,
    > TypedNode<C, R> for RepeatMin<Skipped<T, Skip, SKIP>, MIN>
{
    #[inline]
    fn try_parse_partial_with(
        mut input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        let mut vec = Vec::new();

        for i in 0usize.. {
            match restore_on_none(stack, |stack| {
                try_parse_unit(input.clone(), stack, tracker, i)
            }) {
                Some((next, matched)) => {
                    input = next;
                    vec.push(matched);
                }
                None => {
                    if i < MIN {
                        return None;
                    } else {
                        break;
                    }
                }
            }
        }

        Some((input, Self { content: vec }))
    }

    #[inline]
    fn try_check_partial_with(
        mut input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        for i in 0usize.. {
            match restore_on_none(stack, |stack| {
                try_check_unit::<C, R, T, Skip, SKIP>(input.clone(), stack, tracker, i)
            }) {
                Some(next) => {
                    input = next;
                }
                None => {
                    if i < MIN {
                        return None;
                    } else {
                        break;
                    }
                }
            }
        }
        Some(input)
    }
}
impl<T, IGNORED, const SKIP: usize, const MIN: usize> RepeatMin<Skipped<T, IGNORED, SKIP>, MIN> {
    /// Returns an iterator over all matched expressions by reference.
    pub fn iter_matched(&'_ self) -> Iter<'_, T, IGNORED, SKIP> {
        self.content.iter().map(|s| &s.matched)
    }
    /// Returns an iterator over all matched expressions by value.
    pub fn into_iter_matched(self) -> IntoIter<T, IGNORED, SKIP> {
        self.content.into_iter().map(|s| s.matched)
    }
}
impl<T, const MIN: usize> RepeatMin<T, MIN> {
    /// Returns an iterator over all skipped or matched expressions by reference.
    pub fn iter_all(&'_ self) -> alloc::slice::Iter<'_, T> {
        self.content.iter()
    }
    /// Returns an iterator over all skipped or matched expressions by value.
    pub fn into_iter_all(self) -> alloc::vec::IntoIter<T> {
        self.content.into_iter()
    }
}
impl<T: Clone + PartialEq, const MIN: usize> BoundWrapper for RepeatMin<T, MIN> {
    const MIN: usize = MIN;
    const MAX: usize = usize::MAX;
}

/// Repeatably match `T` at least `MIN` times and at most `MAX` times.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct RepeatMinMax<T, const MIN: usize, const MAX: usize> {
    /// Skipped and Matched expressions.
    pub content: Vec<T>,
}

impl<T, const MAX: usize> Default for RepeatMinMax<T, 0, MAX> {
    fn default() -> Self {
        Self {
            content: Vec::new(),
        }
    }
}
impl<
        C: Cursor,
        R: RuleType,
        T: TypedNode<C, R>,
        Skip: NeverFailedTypedNode<C, R>,
        const SKIP: usize,
        const MAX: usize,
    > NeverFailedTypedNode<C, R> for RepeatMinMax<Skipped<T, Skip, SKIP>, 0, MAX>
{
    #[inline]
    fn parse_with(mut input: C, stack: &mut Stack<Span<C::String>>) -> (C, Self) {
        let mut vec = Vec::new();

        let mut tracker = Tracker::new(input.as_position());

        for i in 0..MAX {
            match restore_on_none(stack, |stack| {
                try_parse_unit(input.clone(), stack, &mut tracker, i)
            }) {
                Some((next, matched)) => {
                    input = next;
                    vec.push(matched);
                }
                None => {
                    break;
                }
            }
        }

        (input, Self { content: vec })
    }

    #[inline]
    fn check_with(mut input: C, stack: &mut Stack<Span<C::String>>) -> C {
        let mut tracker = Tracker::new(input.as_position());

        for i in 0..MAX {
            match restore_on_none(stack, |stack| {
                try_check_unit::<C, R, T, Skip, SKIP>(input.clone(), stack, &mut tracker, i)
            }) {
                Some(next) => {
                    input = next;
                }
                None => {
                    break;
                }
            }
        }

        input
    }
}
impl<
        C: Cursor,
        R: RuleType,
        T: TypedNode<C, R>,
        Skip: NeverFailedTypedNode<C, R>,
        const SKIP: usize,
        const MIN: usize,
        const MAX: usize,
    > TypedNode<C, R> for RepeatMinMax<Skipped<T, Skip, SKIP>, MIN, MAX>
{
    #[inline]
    fn try_parse_partial_with(
        mut input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        let mut vec = Vec::new();

        for i in 0..MAX {
            match restore_on_none(stack, |stack| {
                try_parse_unit(input.clone(), stack, tracker, i)
            }) {
                Some((next, matched)) => {
                    input = next;
                    vec.push(matched);
                }
                None => {
                    if i < MIN {
                        return None;
                    } else {
                        break;
                    }
                }
            }
        }

        Some((input, Self { content: vec }))
    }

    #[inline]
    fn try_check_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        let mut input = input;

        for i in 0..MAX {
            match restore_on_none(stack, |stack| {
                try_check_unit::<C, R, T, Skip, SKIP>(input.clone(), stack, tracker, i)
            }) {
                Some(next) => {
                    input = next;
                }
                None => {
                    if i < MIN {
                        return None;
                    } else {
                        break;
                    }
                }
            }
        }

        Some(input)
    }
}
impl<T, IGNORED, const SKIP: usize, const MIN: usize, const MAX: usize>
    RepeatMinMax<Skipped<T, IGNORED, SKIP>, MIN, MAX>
{
    /// Returns an iterator over all matched expressions by reference.
    pub fn iter_matched(&'_ self) -> Iter<'_, T, IGNORED, SKIP> {
        self.content.iter().map(|s| &s.matched)
    }
    /// Returns an iterator over all matched expressions by value.
    pub fn into_iter_matched(self) -> IntoIter<T, IGNORED, SKIP> {
        self.content.into_iter().map(|s| s.matched)
    }
}
impl<T, const MIN: usize, const MAX: usize> RepeatMinMax<T, MIN, MAX> {
    /// Returns an iterator over all skipped or matched expressions by reference.
    pub fn iter_all(&'_ self) -> alloc::slice::Iter<'_, T> {
        self.content.iter()
    }
    /// Returns an iterator over all skipped or matched expressions by value.
    pub fn into_iter_all(self) -> alloc::vec::IntoIter<T> {
        self.content.into_iter()
    }
}
impl<T: Clone + PartialEq, const MIN: usize, const MAX: usize> BoundWrapper
    for RepeatMinMax<T, MIN, MAX>
{
    const MIN: usize = MIN;
    const MAX: usize = MAX;
}

/// Repeat exactly `TIMES` times.
pub type RepExact<T, IGNORED, const SKIP: usize, const TIMES: usize> =
    RepeatMinMax<Skipped<T, IGNORED, SKIP>, TIMES, TIMES>;
/// Repeat at least `MIN` times.
pub type RepMin<T, IGNORED, const SKIP: usize, const MIN: usize> =
    RepeatMin<Skipped<T, IGNORED, SKIP>, MIN>;
/// Repeat at most `MAX` times.
pub type RepMax<T, IGNORED, const SKIP: usize, const MAX: usize> =
    RepeatMinMax<Skipped<T, IGNORED, SKIP>, 0, MAX>;
/// Repeat at least `MIN` and at most `MAX` times (both inclusive).
pub type RepMinMax<T, IGNORED, const SKIP: usize, const MIN: usize, const MAX: usize> =
    RepeatMinMax<Skipped<T, IGNORED, SKIP>, MIN, MAX>;
/// Repeat arbitrary times.
pub type Rep<T, IGNORED, const SKIP: usize> = RepeatMin<Skipped<T, IGNORED, SKIP>, 0>;
/// Repeat at least one times.
pub type RepOnce<T, IGNORED, const SKIP: usize> = RepeatMin<Skipped<T, IGNORED, SKIP>, 1>;

fn try_parse_unit<
    C: Cursor,
    R: RuleType,
    T: TypedNode<C, R>,
    Skip: NeverFailedTypedNode<C, R>,
    const SKIP: usize,
>(
    mut input: C,
    stack: &mut Stack<Span<C::String>>,
    tracker: &mut Tracker<C::String, R>,
    i: usize,
) -> Option<(C, Skipped<T, Skip, SKIP>)> {
    let skipped = core::array::from_fn(|_| {
        if i == 0 {
            Skip::default()
        } else {
            let (next, skipped) = Skip::parse_with(input.clone(), stack);
            input = next;
            skipped
        }
    });
    let (next, matched) = T::try_parse_partial_with(input, stack, tracker)?;
    input = next;
    let res = Skipped { skipped, matched };
    Some((input, res))
}

fn try_check_unit<
    C: Cursor,
    R: RuleType,
    T: TypedNode<C, R>,
    Skip: NeverFailedTypedNode<C, R>,
    const SKIP: usize,
>(
    mut input: C,
    stack: &mut Stack<Span<C::String>>,
    tracker: &mut Tracker<C::String, R>,
    i: usize,
) -> Option<C> {
    for _ in 0..SKIP {
        if i > 0 {
            let next = Skip::check_with(input, stack);
            input = next;
        }
    }
    let next = T::try_check_partial_with(input, stack, tracker)?;
    input = next;
    Some(input)
}
