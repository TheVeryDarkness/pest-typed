// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use core::fmt::Debug;

pub use alloc::rc::Rc;
use pest::RuleType;

use crate::{predefined_node::restore_on_err, RuleWrapper};

use super::{error::Error, position::Position, span::Span, stack::Stack, tracker::Tracker};

/// Node of concrete syntax tree that never fails.
pub trait NeverFailedTypedNode<'i, R: RuleType>
where
    Self: Sized + Debug + Clone + PartialEq + Default,
{
    /// Create typed node.
    /// `ATOMIC` refers to the external status, and it can be overriden by rule definition.
    fn parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
    ) -> (Position<'i>, Self);
}

/// Node of concrete syntax tree.
pub trait TypedNode<'i, R: RuleType>
where
    Self: Sized + Debug + Clone + PartialEq,
{
    /// Create typed node.
    /// `ATOMIC` refers to the external status, and it can be overriden by rule definition.
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()>;
}

/// Node of concrete syntax tree.
pub trait ParsableTypedNode<'i, R: RuleType>: TypedNode<'i, R> {
    /// Parse the whole input into given typed node.
    /// A rule is not atomic by default.
    fn parse(input: &'i str) -> Result<Self, Error<R>>;
    /// Parse the whole input into given typed node.
    /// A rule is not atomic by default.
    fn parse_partial(input: &'i str) -> Result<(Position<'i>, Self), Error<R>>;
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
pub trait RuleStruct<'i, R: RuleType>: RuleStorage<R> {
    /// The span of a matched expression by a non-silent rule.
    fn span(&self) -> Span<'i>;
}

/// Optionally match `T`.
impl<'i, R: RuleType, T: TypedNode<'i, R>> TypedNode<'i, R> for Option<T> {
    #[inline]
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let res = restore_on_err(stack, |stack| {
            T::try_parse_with::<ATOMIC>(input, stack, tracker)
        });
        match res {
            Ok((input, inner)) => {
                stack.clear_snapshot();
                Ok((input, Self::from(Some(inner))))
            }
            Err(_) => {
                stack.restore();
                Ok((input, Self::from(None)))
            }
        }
    }
}
