// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use super::builtin::EOI;
use crate::typed_node::Take;
use crate::{error::Error, position::Position, stack::Stack};
use crate::{
    span::Span,
    tracker::Tracker,
    typed_node::{NeverFailedTypedNode, ParsableTypedNode},
    wrapper::RuleWrapper,
    TypedNode,
};
use core::ops::{Deref, DerefMut};
use core::{fmt, fmt::Debug, marker::PhantomData};
use pest::RuleType;

/// Errors on current rule will **not** be tracked.
#[derive(Clone, PartialEq)]
pub struct Silent<'i, R: RuleType, T: TypedNode<'i, R>> {
    /// Matched content.
    pub(super) content: T,
    _phantom: PhantomData<(&'i R, &'i T)>,
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> From<T> for Silent<'i, R, T> {
    fn from(content: T) -> Self {
        Self {
            content,
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> TypedNode<'i, R> for Silent<'i, R, T> {
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        match T::try_parse_with::<ATOMIC>(input, stack, tracker) {
            Ok((input, content)) => Ok((input, Self::from(content))),
            Err(_) => Err(()),
        }
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Deref for Silent<'i, R, T> {
    type Target = T::Target;
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> DerefMut for Silent<'i, R, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Take for Silent<'i, R, T> {
    type Inner = T::Inner;
    fn take(self) -> Self::Inner {
        self.content.take()
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Debug for Silent<'i, R, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Silent")
            .field("content", &self.content)
            .finish()
    }
}

/// Start point of an atomic rule.
///
/// Force inner tokens to be atomic.
///
/// See [`Rule`] and [`NonAtomicRule`].
#[derive(Clone, PartialEq)]
pub struct AtomicRule<
    'i,
    R: RuleType,
    T: TypedNode<'i, R>,
    RULE: RuleWrapper<R>,
    _EOI: RuleWrapper<R>,
> {
    /// Matched content.
    content: T,
    /// Matched span.
    pub(super) span: Span<'i>,
    _phantom: PhantomData<(&'i R, &'i RULE, &'i _EOI)>,
}
impl<'i, R: RuleType, T: TypedNode<'i, R>, RULE: RuleWrapper<R>, _EOI: RuleWrapper<R>>
    From<(T, Span<'i>)> for AtomicRule<'i, R, T, RULE, _EOI>
{
    fn from((content, span): (T, Span<'i>)) -> Self {
        Self {
            content,
            span,
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>, RULE: RuleWrapper<R>, _EOI: RuleWrapper<R>>
    RuleWrapper<R> for AtomicRule<'i, R, T, RULE, _EOI>
{
    const RULE: R = RULE::RULE;
    type Rule = R;
}
impl<'i, R: RuleType, T: TypedNode<'i, R>, RULE: RuleWrapper<R>, _EOI: RuleWrapper<R>>
    TypedNode<'i, R> for AtomicRule<'i, R, T, RULE, _EOI>
{
    #[inline]
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let start = input;
        tracker.record_during(start, |tracker| {
            let (input, res) = T::try_parse_with::<true>(input, stack, tracker)?;
            let res = Self::from((res, start.span(&input)));
            Ok((input, res))
        })
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>, RULE: RuleWrapper<R>, _EOI: RuleWrapper<R>>
    ParsableTypedNode<'i, R> for AtomicRule<'i, R, T, RULE, _EOI>
{
    fn parse(input: &'i str) -> Result<Self, Error<R>> {
        parse_without_ignore::<R, _EOI, Self>(input)
    }
    fn parse_partial(input: &'i str) -> Result<(Position<'i>, Self), Error<R>> {
        parse_partial::<R, Self>(input)
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>, RULE: RuleWrapper<R>, _EOI: RuleWrapper<R>> Deref
    for AtomicRule<'i, R, T, RULE, _EOI>
{
    type Target = T::Target;
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>, RULE: RuleWrapper<R>, _EOI: RuleWrapper<R>> DerefMut
    for AtomicRule<'i, R, T, RULE, _EOI>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>, RULE: RuleWrapper<R>, _EOI: RuleWrapper<R>> Take
    for AtomicRule<'i, R, T, RULE, _EOI>
{
    type Inner = T::Inner;
    fn take(self) -> Self::Inner {
        self.content.take()
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>, RULE: RuleWrapper<R>, _EOI: RuleWrapper<R>> Debug
    for AtomicRule<'i, R, T, RULE, _EOI>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AtomicRule")
            .field("rule", &RULE::RULE)
            .field("content", &self.content)
            .finish()
    }
}

/// Start point of a non-atomic rule.
///
/// Force inner tokens to be not atomic.
///
/// See [`Rule`] and [`AtomicRule`].
#[derive(Clone, PartialEq)]
pub struct NonAtomicRule<
    'i,
    R: RuleType,
    T: TypedNode<'i, R>,
    RULE: RuleWrapper<R>,
    _EOI: RuleWrapper<R>,
    IGNORED: NeverFailedTypedNode<'i, R>,
> {
    /// Matched content.
    content: T,
    /// Matched span.
    pub(super) span: Span<'i>,
    _phantom: PhantomData<(&'i R, &'i T, &'i RULE, &'i _EOI, &'i IGNORED)>,
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        RULE: RuleWrapper<R>,
        _EOI: RuleWrapper<R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > From<(T, Span<'i>)> for NonAtomicRule<'i, R, T, RULE, _EOI, IGNORED>
{
    fn from((content, span): (T, Span<'i>)) -> Self {
        Self {
            content,
            span,
            _phantom: PhantomData,
        }
    }
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        RULE: RuleWrapper<R>,
        _EOI: RuleWrapper<R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > RuleWrapper<R> for NonAtomicRule<'i, R, T, RULE, _EOI, IGNORED>
{
    const RULE: R = RULE::RULE;

    type Rule = R;
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        RULE: RuleWrapper<R>,
        _EOI: RuleWrapper<R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > TypedNode<'i, R> for NonAtomicRule<'i, R, T, RULE, _EOI, IGNORED>
{
    #[inline]
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        tracker.record_during(input, |tracker| {
            let start = input;
            let (input, res) = T::try_parse_with::<false>(input, stack, tracker)?;
            Ok((input, Self::from((res, start.span(&input)))))
        })
    }
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        RULE: RuleWrapper<R>,
        _EOI: RuleWrapper<R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > ParsableTypedNode<'i, R> for NonAtomicRule<'i, R, T, RULE, _EOI, IGNORED>
{
    fn parse(input: &'i str) -> Result<Self, Error<R>> {
        parse::<R, _EOI, Self, IGNORED>(input)
    }
    fn parse_partial(input: &'i str) -> Result<(Position<'i>, Self), Error<R>> {
        parse_partial::<R, Self>(input)
    }
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        RULE: RuleWrapper<R>,
        _EOI: RuleWrapper<R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > Deref for NonAtomicRule<'i, R, T, RULE, _EOI, IGNORED>
{
    type Target = T::Target;
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        RULE: RuleWrapper<R>,
        _EOI: RuleWrapper<R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > DerefMut for NonAtomicRule<'i, R, T, RULE, _EOI, IGNORED>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        RULE: RuleWrapper<R>,
        _EOI: RuleWrapper<R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > Take for NonAtomicRule<'i, R, T, RULE, _EOI, IGNORED>
{
    type Inner = T::Inner;
    fn take(self) -> Self::Inner {
        self.content.take()
    }
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        RULE: RuleWrapper<R>,
        _EOI: RuleWrapper<R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > Debug for NonAtomicRule<'i, R, T, RULE, _EOI, IGNORED>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NonAtomicRule")
            .field("rule", &RULE::RULE)
            .field("content", &self.content)
            .finish()
    }
}

/// Start point of a normal rule.
///
/// Will not change atomicity.
///
/// See [`AtomicRule`] and [`NonAtomicRule`].
#[derive(Clone, PartialEq)]
pub struct Rule<
    'i,
    R: RuleType,
    T: TypedNode<'i, R>,
    RULE: RuleWrapper<R>,
    _EOI: RuleWrapper<R>,
    IGNORED: NeverFailedTypedNode<'i, R>,
> {
    /// Matched content.
    content: T,
    /// Matched span
    pub(super) span: Span<'i>,
    _phantom: PhantomData<(&'i R, &'i RULE, &'i _EOI, &'i IGNORED)>,
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        RULE: RuleWrapper<R>,
        _EOI: RuleWrapper<R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > From<(T, Span<'i>)> for Rule<'i, R, T, RULE, _EOI, IGNORED>
{
    fn from((content, span): (T, Span<'i>)) -> Self {
        Self {
            content,
            span,
            _phantom: PhantomData,
        }
    }
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        RULE: RuleWrapper<R>,
        _EOI: RuleWrapper<R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > RuleWrapper<R> for Rule<'i, R, T, RULE, _EOI, IGNORED>
{
    const RULE: R = RULE::RULE;
    type Rule = R;
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        RULE: RuleWrapper<R>,
        _EOI: RuleWrapper<R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > TypedNode<'i, R> for Rule<'i, R, T, RULE, _EOI, IGNORED>
{
    #[inline]
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        tracker.record_during(input, |tracker| {
            let start = input;
            let (input, res) = T::try_parse_with::<ATOMIC>(input, stack, tracker)?;
            Ok((input, Self::from((res, start.span(&input)))))
        })
    }
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        RULE: RuleWrapper<R>,
        _EOI: RuleWrapper<R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > ParsableTypedNode<'i, R> for Rule<'i, R, T, RULE, _EOI, IGNORED>
{
    #[inline]
    fn parse(input: &'i str) -> Result<Self, Error<R>> {
        parse::<R, _EOI, Self, IGNORED>(input)
    }

    fn parse_partial(input: &'i str) -> Result<(Position<'i>, Self), Error<R>> {
        parse_partial::<R, Self>(input)
    }
}
impl<
        'i,
        R: RuleType,
        RULE: RuleWrapper<R>,
        T: TypedNode<'i, R>,
        _EOI: RuleWrapper<R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > Deref for Rule<'i, R, T, RULE, _EOI, IGNORED>
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
impl<
        'i,
        R: RuleType,
        RULE: RuleWrapper<R>,
        T: TypedNode<'i, R>,
        _EOI: RuleWrapper<R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > DerefMut for Rule<'i, R, T, RULE, _EOI, IGNORED>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}
impl<
        'i,
        R: RuleType,
        RULE: RuleWrapper<R>,
        T: TypedNode<'i, R>,
        _EOI: RuleWrapper<R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > Take for Rule<'i, R, T, RULE, _EOI, IGNORED>
{
    type Inner = T::Inner;
    fn take(self) -> Self::Inner {
        self.content.take()
    }
}
impl<
        'i,
        R: RuleType,
        RULE: RuleWrapper<R>,
        T: TypedNode<'i, R>,
        _EOI: RuleWrapper<R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > Debug for Rule<'i, R, T, RULE, _EOI, IGNORED>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rule")
            .field("rule", &RULE::RULE)
            .field("content", &self.content)
            .finish()
    }
}

fn parse<
    'i,
    R: RuleType + 'i,
    _EOI: RuleWrapper<R> + 'i,
    _Self: TypedNode<'i, R>,
    IGNORED: NeverFailedTypedNode<'i, R>,
>(
    input: &'i str,
) -> Result<_Self, Error<R>> {
    let mut stack = Stack::new();
    let input = Position::from_start(input);
    let mut tracker = Tracker::new(input);
    let (input, res) = match _Self::try_parse_with::<false>(input, &mut stack, &mut tracker) {
        Ok((input, res)) => (input, res),
        Err(_) => return Err(tracker.collect()),
    };
    let (input, _) = IGNORED::parse_with::<false>(input, &mut stack);
    let (_, _) = match AtomicRule::<'i, R, EOI<'i>, _EOI, _EOI>::try_parse_with::<false>(
        input,
        &mut stack,
        &mut tracker,
    ) {
        Ok((input, res)) => (input, res),
        Err(_) => return Err(tracker.collect()),
    };
    Ok(res)
}

fn parse_without_ignore<
    'i,
    R: RuleType + 'i,
    _EOI: RuleWrapper<R> + 'i,
    _Self: TypedNode<'i, R>,
>(
    input: &'i str,
) -> Result<_Self, Error<R>> {
    let mut stack = Stack::new();
    let input = Position::from_start(input);
    let mut tracker = Tracker::new(input);
    let (input, res) = match _Self::try_parse_with::<false>(input, &mut stack, &mut tracker) {
        Ok((input, res)) => (input, res),
        Err(_) => return Err(tracker.collect()),
    };
    let (_, _) = match AtomicRule::<'i, R, EOI<'i>, _EOI, _EOI>::try_parse_with::<false>(
        input,
        &mut stack,
        &mut tracker,
    ) {
        Ok((input, res)) => (input, res),
        Err(_) => return Err(tracker.collect()),
    };
    Ok(res)
}

fn parse_partial<'i, R: RuleType, _Self: TypedNode<'i, R>>(
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
