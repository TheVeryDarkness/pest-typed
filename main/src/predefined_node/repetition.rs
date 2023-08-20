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
    predefined_node::restore_on_err, tracker::Tracker, wrapper::BoundWrapper, NeverFailedTypedNode,
    Position, RuleType, Span, Stack, TypedNode,
};
use alloc::vec::Vec;

/// Repeatably match `T` at least `MIN` times.
#[derive(Clone, Debug, PartialEq)]
pub struct RepMin<T, IGNORED, const SKIP: usize, const MIN: usize> {
    /// Skipped and Matched expressions.
    pub content: Vec<([IGNORED; SKIP], T)>,
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
        const SKIP: usize,
        const MIN: usize,
    > TypedNode<'i, R> for RepMin<T, IGNORED, SKIP, MIN>
{
    #[inline]
    fn try_parse_with(
        mut input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let mut vec = Vec::<([IGNORED; SKIP], T)>::new();

        {
            for i in 0.. {
                let skipped = core::array::from_fn(|_| {
                    let (next, ignored) = IGNORED::parse_with(input, stack);
                    input = next;
                    ignored
                });

                match restore_on_err(stack, |stack| T::try_parse_with(input, stack, tracker)) {
                    Ok((next, matched)) => {
                        input = next;
                        vec.push((skipped, matched));
                    }
                    Err(err) => {
                        if i < MIN {
                            return Err(err);
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        Ok((input, Self { content: vec }))
    }
}
impl<T, IGNORED, const SKIP: usize, const MIN: usize> RepMin<T, IGNORED, SKIP, MIN> {
    /// Returns an iterator over all matched expressions.
    pub fn iter_matched<'n>(
        &'n self,
    ) -> core::iter::Map<
        alloc::slice::Iter<'n, ([IGNORED; SKIP], T)>,
        fn(&'n ([IGNORED; SKIP], T)) -> &'n T,
    > {
        self.content.iter().map(|(_, e)| e)
    }
    /// Returns an iterator over all skipped or matched expressions.
    pub fn iter_all<'n>(&'n self) -> alloc::slice::Iter<'n, ([IGNORED; SKIP], T)> {
        self.content.iter()
    }
}
impl<T: Clone + PartialEq, IGNORED: Clone + PartialEq, const SKIP: usize, const MIN: usize>
    BoundWrapper for RepMin<T, IGNORED, SKIP, MIN>
{
    const MIN: usize = MIN;
    const MAX: usize = usize::MAX;
}

/// Repeatably match `T` at least `MIN` times and at most `MAX` times.
#[derive(Clone, Debug, PartialEq)]
pub struct RepMinMax<T, IGNORED, const SKIP: usize, const MIN: usize, const MAX: usize> {
    /// Skipped and Matched expressions.
    pub content: Vec<([IGNORED; SKIP], T)>,
}

impl<T, IGNORED, const SKIP: usize, const MAX: usize> Default
    for RepMinMax<T, IGNORED, SKIP, 0, MAX>
{
    fn default() -> Self {
        Self {
            content: Vec::new(),
        }
    }
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
        const SKIP: usize,
        const MAX: usize,
    > NeverFailedTypedNode<'i, R> for RepMinMax<T, IGNORED, SKIP, 0, MAX>
{
    #[inline]
    fn parse_with(mut input: Position<'i>, stack: &mut Stack<Span<'i>>) -> (Position<'i>, Self) {
        let mut vec = Vec::<([IGNORED; SKIP], T)>::new();

        let mut tracker = Tracker::new(input);
        {
            for i in 0..MAX {
                let skipped = core::array::from_fn(|_| {
                    let (next, ignored) = IGNORED::parse_with(input, stack);
                    input = next;
                    ignored
                });

                match restore_on_err(stack, |stack| T::try_parse_with(input, stack, &mut tracker)) {
                    Ok((next, matched)) => {
                        input = next;
                        vec.push((skipped, matched));
                    }
                    Err(err) => {
                        break;
                    }
                }
            }
        }
        (input, Self { content: vec })
    }
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
        const SKIP: usize,
        const MIN: usize,
        const MAX: usize,
    > TypedNode<'i, R> for RepMinMax<T, IGNORED, SKIP, MIN, MAX>
{
    #[inline]
    fn try_parse_with(
        mut input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let mut vec = Vec::<([IGNORED; SKIP], T)>::new();

        {
            for i in 0..MAX {
                let skipped = core::array::from_fn(|_| {
                    let (next, ignored) = IGNORED::parse_with(input, stack);
                    input = next;
                    ignored
                });

                match restore_on_err(stack, |stack| T::try_parse_with(input, stack, tracker)) {
                    Ok((next, matched)) => {
                        input = next;
                        vec.push((skipped, matched));
                    }
                    Err(err) => {
                        if i < MIN {
                            return Err(err);
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        Ok((input, Self { content: vec }))
    }
}
impl<T, IGNORED, const SKIP: usize, const MIN: usize, const MAX: usize>
    RepMinMax<T, IGNORED, SKIP, MIN, MAX>
{
    /// Returns an iterator over all matched expressions.
    pub fn iter_matched<'n>(
        &'n self,
    ) -> core::iter::Map<
        alloc::slice::Iter<'n, ([IGNORED; SKIP], T)>,
        fn(&'n ([IGNORED; SKIP], T)) -> &'n T,
    > {
        self.content.iter().map(|(_, e)| e)
    }
    /// Returns an iterator over all skipped or matched expressions.
    pub fn iter_all<'n>(&'n self) -> alloc::slice::Iter<'n, ([IGNORED; SKIP], T)> {
        self.content.iter()
    }
}
impl<
        T: Clone + PartialEq,
        IGNORED: Clone + PartialEq,
        const SKIP: usize,
        const MIN: usize,
        const MAX: usize,
    > BoundWrapper for RepMinMax<T, IGNORED, SKIP, MIN, MAX>
{
    const MIN: usize = MIN;
    const MAX: usize = MAX;
}

/// Repeat arbitrary times.
pub type Rep<T, IGNORED, const SKIP: usize> = RepMin<T, IGNORED, SKIP, 0>;
/// Repeat at least one times.
pub type RepOnce<T, IGNORED, const SKIP: usize> = RepMin<T, IGNORED, SKIP, 1>;
