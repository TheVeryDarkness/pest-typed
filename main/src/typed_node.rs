// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use super::{error::Error, position::Position, span::Span, stack::Stack, tracker::Tracker};
use crate::RuleType;
use crate::RuleWrapper;
pub use alloc::rc::Rc;
use core::{fmt::Debug, ops::Deref};

/// Node of concrete syntax tree that never fails.
pub trait NeverFailedTypedNode<'i, R: RuleType>
where
    Self: Sized + Debug + Clone + PartialEq,
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
    Self: Sized + Debug + Clone + PartialEq + Deref + Take,
{
    /// Wrapped type.
    type Inner: Sized;
    /// Create typed node.
    /// `ATOMIC` refers to the external status, and it can be overriden by rule definition.
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()>;
    /// Dereference once.
    fn deref_once<'n>(node: &'n Self) -> &'n Self::Inner;
    /// Dereference self once.
    fn deref_self_once<'n>(&'n self) -> &'n Self::Inner {
        Self::deref_once(self)
    }
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

/// A trait for taking a node's content.
pub trait Take {
    /// Type of taken value.
    type Taken: Sized;
    /// Take something's content.
    fn take(self) -> Self::Taken;
}
