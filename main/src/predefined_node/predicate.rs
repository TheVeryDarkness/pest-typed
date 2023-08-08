// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::{position::Position, stack::Stack, Take};
use crate::{span::Span, tracker::Tracker, TypedNode};
use core::marker::PhantomData;
use core::{fmt::Debug, ops::Deref};
use pest::RuleType;

/// Positive predicate.
#[derive(Clone, PartialEq)]
pub struct Positive<'i, R: RuleType, T: TypedNode<'i, R>> {
    /// Mathed content.
    pub(super) content: T,
    _phantom: PhantomData<(&'i R, &'i T)>,
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> TypedNode<'i, R> for Positive<'i, R, T> {
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        tracker.positive_during(|tracker| {
            stack.snapshot();
            match T::try_parse_with::<ATOMIC>(input, stack, tracker) {
                Ok((_input, content)) => {
                    stack.restore();
                    Ok((input, Self::from(content)))
                }
                Err(_) => {
                    stack.restore();
                    Err(())
                }
            }
        })
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> From<T> for Positive<'i, R, T> {
    fn from(value: T) -> Self {
        Self {
            content: value,
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Deref for Positive<'i, R, T> {
    type Target = T::Target;
    fn deref(&self) -> &Self::Target {
        self.content.deref()
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Take for Positive<'i, R, T> {
    type Inner = T::Inner;
    fn take(self) -> Self::Inner {
        self.content.take()
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Debug for Positive<'i, R, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Positve")
            .field("content", &self.content)
            .finish()
    }
}

/// Negative predicate.
///
/// Will not contain anything.
#[derive(Clone, PartialEq)]
pub struct Negative<'i, R: RuleType, T: TypedNode<'i, R>> {
    content: (),
    _phantom: PhantomData<(&'i R, &'i T)>,
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> TypedNode<'i, R> for Negative<'i, R, T> {
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        tracker.negative_during(|tracker| {
            stack.snapshot();
            match T::try_parse_with::<ATOMIC>(input, stack, tracker) {
                Ok(_) => {
                    stack.restore();
                    Err(())
                }
                Err(_) => {
                    stack.restore();
                    Ok((input, Self::from(())))
                }
            }
        })
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> From<()> for Negative<'i, R, T> {
    fn from(value: ()) -> Self {
        Self {
            content: value,
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Deref for Negative<'i, R, T> {
    type Target = ();
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Take for Negative<'i, R, T> {
    type Inner = ();
    fn take(self) -> Self::Inner {
        self.content
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Debug for Negative<'i, R, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Negative").finish()
    }
}
