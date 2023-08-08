// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::{
    position::Position,
    span::Span,
    stack::Stack,
    tracker::Tracker,
    typed_node::{NeverFailedTypedNode, Take},
    wrapper::BoundWrapper,
    RuleType, TypedNode,
};
use alloc::{vec, vec::Vec};
use core::ops::{Deref, DerefMut};
use core::{fmt, fmt::Debug, marker::PhantomData};

macro_rules! impl_with_content {
    ($name:ident, ($($generics_args:tt)*), ($($generics_params:tt)*), $type:ty) => {
        impl<$($generics_args)*> Deref for $name<$($generics_params)*> {
            type Target = $type;
            fn deref(&self) -> &Self::Target {
                &self.content
            }
        }
        impl<$($generics_args)*> DerefMut for $name<$($generics_params)*> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.content
            }
        }
        impl<$($generics_args)*> Take for $name<$($generics_params)*> {
            type Inner = $type;
            fn take(self) -> Self::Inner {
                self.content
            }
        }
        impl<$($generics_args)*> Debug for $name<$($generics_params)*> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("content", &self.content)
                    .finish()
            }
        }
    };
}

/// Optionally match `T`.
#[derive(Clone, PartialEq)]
pub struct Opt<'i, R: RuleType, T: TypedNode<'i, R>> {
    /// Matched content.
    content: Option<T>,
    _phantom: PhantomData<&'i R>,
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> From<Option<T>> for Opt<'i, R, T> {
    fn from(content: Option<T>) -> Self {
        Self {
            content,
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> TypedNode<'i, R> for Opt<'i, R, T> {
    #[inline]
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        match T::try_parse_with::<ATOMIC>(input, stack, tracker) {
            Ok((input, inner)) => Ok((input, Self::from(Some(inner)))),
            Err(_) => Ok((input, Self::from(None))),
        }
    }
}
impl_with_content!(
    Opt,
    ('i, R: RuleType, T: TypedNode<'i, R>),
    ('i, R, T),
    Option<T>
);

/// Ignored single comment or white space.
#[derive(Debug, Clone, PartialEq)]
pub enum IgnoredUnit<'i, R: RuleType, COMMENT: TypedNode<'i, R>, WHITESPACE: TypedNode<'i, R>> {
    /// Auto-skipped comment.
    Comment(COMMENT),
    /// Auto-skipped white space.
    WhiteSpace(WHITESPACE),
    /// An impossible case.
    ERROR(PhantomData<&'i R>),
}
/// Skip comments (by rule `COMMENT`) or white spaces (by rule `WHITESPACE`) if there is any.
/// Never fail.
#[derive(Clone, PartialEq)]
pub struct Ignored<'i, R: RuleType, COMMENT: TypedNode<'i, R>, WHITESPACE: TypedNode<'i, R>> {
    content: Vec<IgnoredUnit<'i, R, COMMENT, WHITESPACE>>,
    _phantom: PhantomData<&'i R>,
}
impl<'i, R: RuleType, COMMENT: TypedNode<'i, R>, WHITESPACE: TypedNode<'i, R>>
    From<Vec<IgnoredUnit<'i, R, COMMENT, WHITESPACE>>> for Ignored<'i, R, COMMENT, WHITESPACE>
{
    fn from(value: Vec<IgnoredUnit<'i, R, COMMENT, WHITESPACE>>) -> Self {
        Self {
            content: value,
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, COMMENT: TypedNode<'i, R>, WHITESPACE: TypedNode<'i, R>>
    NeverFailedTypedNode<'i, R> for Ignored<'i, R, COMMENT, WHITESPACE>
{
    #[inline]
    fn parse_with<const ATOMIC: bool>(
        mut input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
    ) -> (Position<'i>, Self) {
        if ATOMIC {
            return (input, Self::from(vec![]));
        }
        let mut vec = Vec::new();
        let mut flag = true;
        let mut tracker = Tracker::new(input);
        while flag {
            flag = false;
            while let Ok((remained, comment)) =
                COMMENT::try_parse_with::<true>(input, stack, &mut tracker)
            {
                vec.push(IgnoredUnit::Comment(comment));
                input = remained;
                flag = true;
            }
            while let Ok((remained, white_space)) =
                WHITESPACE::try_parse_with::<true>(input, stack, &mut tracker)
            {
                vec.push(IgnoredUnit::WhiteSpace(white_space));
                input = remained;
                flag = true;
            }
        }
        (input, Self::from(vec))
    }
}
impl<'i, R: RuleType, COMMENT: TypedNode<'i, R>, WHITESPACE: TypedNode<'i, R>> TypedNode<'i, R>
    for Ignored<'i, R, COMMENT, WHITESPACE>
{
    #[inline]
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        Ok(Self::parse_with::<ATOMIC>(input, stack))
    }
}
impl_with_content!(
    Ignored,
    ('i, R: RuleType, COMMENT: TypedNode<'i, R>, WHITESPACE: TypedNode<'i, R>),
    ('i, R, COMMENT, WHITESPACE),
    Vec<IgnoredUnit<'i, R, COMMENT, WHITESPACE>>
);

/// Repeatably match `T` at least `MIN` times.
#[derive(Clone, PartialEq)]
pub struct RepMin<
    'i,
    R: RuleType,
    T: TypedNode<'i, R>,
    IGNORED: NeverFailedTypedNode<'i, R>,
    const MIN: usize,
> {
    /// Matched nodes.
    pub content: Vec<T>,
    _phantom: PhantomData<(&'i R, &'i IGNORED)>,
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
        const MIN: usize,
    > From<RepMin<'i, R, T, IGNORED, MIN>> for Vec<T>
{
    fn from(value: RepMin<'i, R, T, IGNORED, MIN>) -> Self {
        value.content
    }
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
        const MIN: usize,
    > TypedNode<'i, R> for RepMin<'i, R, T, IGNORED, MIN>
{
    #[inline]
    fn try_parse_with<const ATOMIC: bool>(
        mut input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let mut vec = Vec::<T>::new();

        {
            let mut i: usize = 0;
            loop {
                if i != 0 {
                    let (next, _) = IGNORED::parse_with::<ATOMIC>(input, stack);
                    input = next;
                }
                match T::try_parse_with::<ATOMIC>(input, stack, tracker) {
                    Ok((next, elem)) => {
                        input = next;
                        vec.push(elem);
                    }
                    Err(_err) => {
                        if i < MIN {
                            return Err(());
                        }
                        break;
                    }
                }
                i += 1;
            }
        }
        Ok((
            input,
            Self {
                content: vec,
                _phantom: PhantomData,
            },
        ))
    }
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
        const MIN: usize,
    > BoundWrapper<T> for RepMin<'i, R, T, IGNORED, MIN>
{
    const MIN: usize = MIN;
    const MAX: usize = usize::MAX;
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
        const MIN: usize,
    > Deref for RepMin<'i, R, T, IGNORED, MIN>
{
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
        const MIN: usize,
    > DerefMut for RepMin<'i, R, T, IGNORED, MIN>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
        const MIN: usize,
    > Take for RepMin<'i, R, T, IGNORED, MIN>
{
    type Inner = Self::Target;
    fn take(self) -> Self::Inner {
        self.content
    }
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
        const MIN: usize,
    > Debug for RepMin<'i, R, T, IGNORED, MIN>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RepMin")
            .field("content", &self.content)
            .finish()
    }
}

/// Repeat arbitrary times.
pub type Rep<'i, R, T, IGNORED> = RepMin<'i, R, T, IGNORED, 0>;
/// Repeat at least one times.
pub type RepOnce<'i, R, T, IGNORED> = RepMin<'i, R, T, IGNORED, 1>;

/// Boxed node for `T`.
///
/// Often used for rules references.
///
/// It's a pity that we can only return a `Box<T>`
/// Replace this with `T` when [`alloc::boxed::Box::take`] is stablizied.
#[derive(Clone, PartialEq)]
pub struct Ref<'i, R: RuleType, T: TypedNode<'i, R>> {
    /// Boxed content.
    pub(super) content: ::alloc::boxed::Box<T>,
    _phantom: PhantomData<&'i R>,
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> From<::alloc::boxed::Box<T>> for Ref<'i, R, T> {
    fn from(content: ::alloc::boxed::Box<T>) -> Self {
        Self {
            content,
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> From<T> for Ref<'i, R, T> {
    fn from(content: T) -> Self {
        Self::from(::alloc::boxed::Box::new(content))
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Deref for Ref<'i, R, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.content.as_ref()
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> DerefMut for Ref<'i, R, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.content.as_mut()
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Take for Ref<'i, R, T> {
    type Inner = T;
    fn take(self) -> Self::Inner {
        *self.content
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> TypedNode<'i, R> for Ref<'i, R, T> {
    #[inline]
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let (input, res) = T::try_parse_with::<ATOMIC>(input, stack, tracker)?;
        Ok((input, Self::from(res)))
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Debug for Ref<'i, R, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.content.fmt(f)
    }
}

/// Restore stack state on error.
#[derive(Clone, PartialEq)]
pub struct RestoreOnError<'i, R: RuleType, T: TypedNode<'i, R>> {
    /// Matched content.
    pub(super) content: T,
    _phantom: PhantomData<&'i R>,
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> From<T> for RestoreOnError<'i, R, T> {
    fn from(content: T) -> Self {
        Self {
            content,
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Deref for RestoreOnError<'i, R, T> {
    type Target = T::Target;
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Take for RestoreOnError<'i, R, T> {
    type Inner = T::Inner;
    fn take(self) -> Self::Inner {
        self.content.take()
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> TypedNode<'i, R> for RestoreOnError<'i, R, T> {
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        stack.snapshot();
        match T::try_parse_with::<ATOMIC>(input, stack, tracker) {
            Ok((input, res)) => {
                stack.clear_snapshot();
                Ok((input, Self::from(res)))
            }
            Err(err) => {
                stack.restore();
                Err(err)
            }
        }
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Debug for RestoreOnError<'i, R, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.content.fmt(f)
    }
}

/// Match an expression and push it.
#[derive(Clone, PartialEq)]
pub struct Push<'i, R: RuleType, T: TypedNode<'i, R>> {
    /// Matched content.
    pub content: T,
    _phantom: PhantomData<(&'i R, &'i T)>,
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> From<T> for Push<'i, R, T> {
    fn from(content: T) -> Self {
        Self {
            content,
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> TypedNode<'i, R> for Push<'i, R, T> {
    #[inline]
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let start = input;
        let (input, content) = T::try_parse_with::<ATOMIC>(input, stack, tracker)?;
        stack.push(start.span(&input));
        Ok((input, Self::from(content)))
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Deref for Push<'i, R, T> {
    type Target = T::Target;
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Take for Push<'i, R, T> {
    type Inner = T::Inner;
    fn take(self) -> Self::Inner {
        self.content.take()
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Debug for Push<'i, R, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Push")
            .field("content", &self.content)
            .finish()
    }
}
