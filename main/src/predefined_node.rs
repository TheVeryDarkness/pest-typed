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

use crate::wrapper::BoundWrapper;

use super::{error::Error, parser_state::constrain_idxs, position::Position, stack::Stack};
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};
use core::{fmt, fmt::Debug, marker::PhantomData};
use pest::RuleType;

use super::{
    span::Span,
    tracker::Tracker,
    typed_node::{NeverFailedTypedNode, ParsableTypedNode},
    wrapper::{RuleWrapper, StringArrayWrapper, StringWrapper, TypeWrapper},
    TypedNode,
};

/// Match given string case sensitively.
///
/// The `CONTENT` on the type (by [`StringWrapper`]) is the original string to match.
///
/// See [`Insens`] for case-insensitive matching.
#[derive(Clone)]
pub struct Str<'i, R: RuleType, T: StringWrapper> {
    _phantom: PhantomData<(&'i R, &'i T)>,
}
impl<'i, R: RuleType, T: StringWrapper> StringWrapper for Str<'i, R, T> {
    const CONTENT: &'static str = T::CONTENT;
}
impl<'i, R: RuleType, T: StringWrapper> From<()> for Str<'i, R, T> {
    fn from(_value: ()) -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, T: StringWrapper> TypedNode<'i, R> for Str<'i, R, T> {
    fn try_parse_with<const _A: bool, Rule: RuleWrapper<R>>(
        mut input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        if input.match_string(Self::CONTENT) {
            Ok((input, Self::from(())))
        } else {
            Err(())
        }
    }
}
impl<'i, R: RuleType, T: StringWrapper> Debug for Str<'i, R, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Str").finish()
    }
}

/// Match given string case insensitively.
///
/// - The field `content` is the matched string.
/// - The `CONTENT` on the type (by [`StringWrapper`]) is the original string to match, and it may differ from `content` in case.
///   
///   For example, A `^"x"` may match `"X"`, and in the parsing result, `content` is `"X"`, while `CONTENT` is still `"x"`.    
///
/// See [`Str`] for case-sensitive matching.
#[derive(Clone, PartialEq)]
pub struct Insens<'i, R: RuleType, T: StringWrapper> {
    /// Matched content.
    pub content: &'i str,
    _phantom: PhantomData<(&'i R, &'i T)>,
}
impl<'i, R: RuleType, T: StringWrapper> StringWrapper for Insens<'i, R, T> {
    const CONTENT: &'static str = T::CONTENT;
}
impl<'i, R: RuleType, T: StringWrapper> From<&'i str> for Insens<'i, R, T> {
    fn from(content: &'i str) -> Self {
        Self {
            content,
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, T: StringWrapper> TypedNode<'i, R> for Insens<'i, R, T> {
    fn try_parse_with<const _A: bool, Rule: RuleWrapper<R>>(
        mut input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let start = input;
        if input.match_insensitive(Self::CONTENT) {
            let span = start.span(&input);
            Ok((input, Self::from(span.as_str())))
        } else {
            Err(())
        }
    }
}
impl<'i, R: RuleType, T: StringWrapper> Debug for Insens<'i, R, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Insens").finish()
    }
}

/// Inner tokens will be discarded, and only a [`Span`] will be contained.
///
/// And inner errors will **not** be tracked.
#[derive(Clone)]
pub struct Silent<'i, R: RuleType, T: TypedNode<'i, R>> {
    /// Span.
    pub span: Span<'i>,
    _phantom: PhantomData<(&'i R, &'i T)>,
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> From<Span<'i>> for Silent<'i, R, T> {
    fn from(span: Span<'i>) -> Self {
        Self {
            span,
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> TypedNode<'i, R> for Silent<'i, R, T> {
    fn try_parse_with<const ATOMIC: bool, Rule: RuleWrapper<R>>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let start = input;
        match T::try_parse_with::<ATOMIC, Rule>(input, stack, tracker) {
            Ok((input, _)) => {
                let span = start.span(&input);
                Ok((input, Self::from(span)))
            }
            Err(_) => Err(()),
        }
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Debug for Silent<'i, R, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Silent").finish()
    }
}

/// Skips until one of the given `strings`
#[derive(Clone)]
pub struct Skip<'i, R: RuleType, Strings: StringArrayWrapper> {
    /// Skipped span.
    pub span: Span<'i>,
    _phantom: PhantomData<(&'i R, &'i Strings)>,
}
impl<'i, R: RuleType, Strings: StringArrayWrapper> From<Span<'i>> for Skip<'i, R, Strings> {
    fn from(span: Span<'i>) -> Self {
        Self {
            span,
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, Strings: StringArrayWrapper> TypedNode<'i, R> for Skip<'i, R, Strings> {
    fn try_parse_with<const _A: bool, Rule: RuleWrapper<R>>(
        mut input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let start = input;
        match input.skip_until(Strings::CONTENT) {
            true => {
                let span = start.span(&input);
                Ok((
                    input,
                    Self {
                        span,
                        _phantom: PhantomData,
                    },
                ))
            }
            false => Err(()),
        }
    }
}
impl<'i, R: RuleType, Strings: StringArrayWrapper> Debug for Skip<'i, R, Strings> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Skip").finish()
    }
}

/// Skip `n` characters if there are.
#[derive(Clone)]
pub struct SkipChar<'i, R: RuleType, const N: usize> {
    _phantom: PhantomData<&'i R>,
}
impl<'i, R: RuleType, const N: usize> From<()> for SkipChar<'i, R, N> {
    fn from(_: ()) -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, const N: usize> TypedNode<'i, R> for SkipChar<'i, R, N> {
    fn try_parse_with<const ATOMIC: bool, Rule: RuleWrapper<R>>(
        mut input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        match input.skip(N) {
            true => Ok((input, Self::from(()))),
            false => Err(()),
        }
    }
}
impl<'i, R: RuleType, const N: usize> Debug for SkipChar<'i, R, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Range").finish()
    }
}

/// Match a character in the range `[MIN, MAX]`.
/// Inclusively both below and above.
#[derive(Clone, PartialEq)]
pub struct CharRange<'i, R: RuleType, const MIN: char, const MAX: char> {
    /// Matched character.
    pub content: char,
    _phantom: PhantomData<&'i R>,
}
impl<'i, R: RuleType, const MIN: char, const MAX: char> From<char> for CharRange<'i, R, MIN, MAX> {
    fn from(content: char) -> Self {
        Self {
            content,
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, const MIN: char, const MAX: char> TypedNode<'i, R>
    for CharRange<'i, R, MIN, MAX>
{
    fn try_parse_with<const _A: bool, Rule: RuleWrapper<R>>(
        mut input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let start = input;
        match input.match_range(MIN..MAX) {
            true => {
                let span = start.span(&input);
                let content = span.as_str().chars().next().unwrap();
                Ok((input, Self::from(content)))
            }
            false => Err(()),
        }
    }
}

impl<'i, R: RuleType, const MIN: char, const MAX: char> Debug for CharRange<'i, R, MIN, MAX> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Range")
            .field("content", &self.content)
            .finish()
    }
}

/// Try to create stack slice.
#[inline]
fn stack_slice<'i, 's, R: RuleType>(
    input: Position<'i>,
    start: i32,
    end: Option<i32>,
    stack: &'s Stack<Span<'i>>,
    tracker: &mut Tracker<'i, R>,
) -> Result<core::slice::Iter<'s, Span<'i>>, ()> {
    let range = match constrain_idxs(start, end, stack.len()) {
        Some(range) => range,
        None => {
            tracker.out_of_bound(input, start, end);
            return Err(());
        }
    };
    // return true if an empty sequence is requested
    if range.end <= range.start {
        return Ok(core::slice::Iter::default());
    }
    Ok(stack[range].iter())
}

/// Match a part of the stack without popping.
/// Will match (consume) input.
#[inline]
fn peek_spans<'s, 'i: 's, R: RuleType, Rule: RuleWrapper<R>>(
    input: Position<'i>,
    iter: impl Iterator<Item = &'s Span<'i>>,
    _tracker: &mut Tracker<'i, R>,
) -> Result<(Position<'i>, Span<'i>), ()> {
    let mut matching_pos = input;
    for span in iter {
        match matching_pos.match_string(span.as_str()) {
            true => (),
            false => {
                return Err(());
            }
        }
    }
    Ok((matching_pos, input.span(&matching_pos)))
}

/// Positive predicate.
#[derive(Clone)]
pub struct Positive<'i, R: RuleType, N: TypedNode<'i, R>> {
    /// Mathed content.
    pub content: N,
    _phantom: PhantomData<(&'i R, &'i N)>,
}
impl<'i, R: RuleType, N: TypedNode<'i, R>> From<N> for Positive<'i, R, N> {
    fn from(content: N) -> Self {
        Self {
            content,
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, N: TypedNode<'i, R>> Deref for Positive<'i, R, N> {
    type Target = N;
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
impl<'i, R: RuleType, N: TypedNode<'i, R>> DerefMut for Positive<'i, R, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}
impl<'i, R: RuleType, N: TypedNode<'i, R>> TypedNode<'i, R> for Positive<'i, R, N> {
    fn try_parse_with<const ATOMIC: bool, Rule: RuleWrapper<R>>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        tracker.positive_during(|tracker| {
            stack.snapshot();
            match N::try_parse_with::<ATOMIC, Rule>(input, stack, tracker) {
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
impl<'i, R: RuleType, N: TypedNode<'i, R>> Debug for Positive<'i, R, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Positive").finish()
    }
}

/// Negative predicate.
///
/// Will not contain anything.
#[derive(Clone)]
pub struct Negative<'i, R: RuleType, N: TypedNode<'i, R>> {
    _phantom: PhantomData<(&'i R, &'i N)>,
}
impl<'i, R: RuleType, N: TypedNode<'i, R>> From<()> for Negative<'i, R, N> {
    fn from(_value: ()) -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, N: TypedNode<'i, R>> TypedNode<'i, R> for Negative<'i, R, N> {
    fn try_parse_with<const ATOMIC: bool, Rule: RuleWrapper<R>>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        tracker.negative_during(|tracker| {
            stack.snapshot();
            match N::try_parse_with::<ATOMIC, Rule>(input, stack, tracker) {
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
impl<'i, R: RuleType, N: TypedNode<'i, R>> Debug for Negative<'i, R, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Negative").finish()
    }
}

/// Match any character.
#[derive(Debug, Clone)]
pub struct ANY<'i> {
    /// Pair span.
    pub span: Span<'i>,
    /// Matched character.
    pub content: char,
}
impl<'i, R: RuleType> TypedNode<'i, R> for ANY<'i> {
    #[inline]
    fn try_parse_with<const _A: bool, Rule: RuleWrapper<R>>(
        mut input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let original_input = input;
        let mut c: char = ' ';
        match input.match_char_by(|ch| {
            c = ch;
            true
        }) {
            true => {
                let span = original_input.span(&input);
                Ok((input, Self { span, content: c }))
            }
            false => Err(()),
        }
    }
}

/// Match the start of input.
#[derive(Clone)]
pub struct SOI<'i> {
    _phantom: PhantomData<&'i str>,
}
impl<'i, R: RuleType> TypedNode<'i, R> for SOI<'i> {
    #[inline]
    fn try_parse_with<const _A: bool, Rule: RuleWrapper<R>>(
        input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        if input.at_start() {
            Ok((
                input,
                Self {
                    _phantom: PhantomData,
                },
            ))
        } else {
            Err(())
        }
    }
}
impl<'i> Debug for SOI<'i> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SOI").finish()
    }
}

/// Match the end of input.
///
/// [`EOI`] will record its rule if not matched.
#[derive(Clone)]
pub struct EOI<'i> {
    _phantom: PhantomData<&'i str>,
}
impl<'i, R: RuleType> TypedNode<'i, R> for EOI<'i> {
    #[inline]
    fn try_parse_with<const _A: bool, Rule: RuleWrapper<R>>(
        input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        if input.at_end() {
            Ok((
                input,
                Self {
                    _phantom: PhantomData,
                },
            ))
        } else {
            Err(())
        }
    }
}
impl<'i> Debug for EOI<'i> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EOI").finish()
    }
}

/// Match a new line character.
/// A built-in rule. Equivalent to `"\r\n" | "\n" | "\r"`.
#[derive(Debug, Clone)]
pub struct NEWLINE<'i> {
    /// Pair span.
    pub span: Span<'i>,
}
impl<'i, R: RuleType> TypedNode<'i, R> for NEWLINE<'i> {
    #[inline]
    fn try_parse_with<const _A: bool, Rule: RuleWrapper<R>>(
        mut input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let start = input;
        if input.match_string("\r\n") {
            let span = start.span(&input);
            Ok((input, Self { span }))
        } else if input.match_string("\n") {
            let span = start.span(&input);
            Ok((input, Self { span }))
        } else if input.match_string("\r") {
            let span = start.span(&input);
            Ok((input, Self { span }))
        } else {
            Err(())
        }
    }
}

/// Peek all spans in stack reversely.
/// Will consume input.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct PEEK_ALL<'i> {
    /// Pair span.
    pub span: Span<'i>,
}
impl<'i, R: RuleType> TypedNode<'i, R> for PEEK_ALL<'i> {
    #[inline]
    fn try_parse_with<const _A: bool, Rule: RuleWrapper<R>>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let spans = stack[0..stack.len()].iter().rev();
        let (input, span) = peek_spans::<R, Rule>(input, spans, tracker)?;
        Ok((input, Self { span }))
    }
}

/// Peek top span in stack.
/// Will consume input.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct PEEK<'i> {
    /// Pair span.
    pub span: Span<'i>,
}
impl<'i> From<Span<'i>> for PEEK<'i> {
    fn from(span: Span<'i>) -> Self {
        Self { span }
    }
}
impl<'i, R: RuleType> TypedNode<'i, R> for PEEK<'i> {
    #[inline]
    fn try_parse_with<const _A: bool, Rule: RuleWrapper<R>>(
        mut input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let start = input;
        match stack.peek() {
            Some(string) => match input.match_string(string.as_str()) {
                true => Ok((input, Self::from(start.span(&input)))),
                false => Err(()),
            },
            None => {
                tracker.empty_stack(input);
                Err(())
            }
        }
    }
}

/// Optionally match `T`.
#[derive(Clone)]
pub struct Opt<'i, R: RuleType, T: TypedNode<'i, R>> {
    /// Matched content.
    pub content: Option<T>,
    _phantom: PhantomData<&'i R>,
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> TypedNode<'i, R> for Opt<'i, R, T> {
    #[inline]
    fn try_parse_with<const ATOMIC: bool, Rule: RuleWrapper<R>>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        match T::try_parse_with::<ATOMIC, Rule>(input, stack, tracker) {
            Ok((input, inner)) => Ok((
                input,
                Self {
                    content: Some(inner),
                    _phantom: PhantomData,
                },
            )),
            Err(_) => Ok((
                input,
                Self {
                    content: None,
                    _phantom: PhantomData,
                },
            )),
        }
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Debug for Opt<'i, R, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Opt")
            .field("content", &self.content)
            .finish()
    }
}

/// Skip comments (by rule `COMMENT`) or white spaces (by rule `WHITESPACE`) if there is any.
/// Never fail.
#[derive(Clone)]
pub struct Ign<'i, R: RuleType, COMMENT: TypedNode<'i, R>, WHITESPACE: TypedNode<'i, R>> {
    _phantom: PhantomData<(&'i R, &'i COMMENT, &'i WHITESPACE)>,
}

impl<'i, R: RuleType, COMMENT: TypedNode<'i, R>, WHITESPACE: TypedNode<'i, R>>
    NeverFailedTypedNode<'i, R> for Ign<'i, R, COMMENT, WHITESPACE>
{
    #[inline]
    fn parse_with<const ATOMIC: bool, Rule: RuleWrapper<R>>(
        mut input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
    ) -> (Position<'i>, Self) {
        if ATOMIC {
            return (
                input,
                Self {
                    _phantom: PhantomData,
                },
            );
        }
        let mut flag = true;
        let mut tracker = Tracker::new(input);
        while flag {
            flag = false;
            while let Ok((remained, _)) =
                WHITESPACE::try_parse_with::<true, Rule>(input, stack, &mut tracker)
            {
                input = remained;
                flag = true;
            }
            while let Ok((remained, _)) =
                COMMENT::try_parse_with::<true, Rule>(input, stack, &mut tracker)
            {
                input = remained;
                flag = true;
            }
        }
        (
            input,
            Self {
                _phantom: PhantomData,
            },
        )
    }
}
impl<'i, R: RuleType, COMMENT: TypedNode<'i, R>, WHITESPACE: TypedNode<'i, R>> TypedNode<'i, R>
    for Ign<'i, R, COMMENT, WHITESPACE>
{
    #[inline]
    fn try_parse_with<const ATOMIC: bool, RULE: RuleWrapper<R>>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        Ok(Self::parse_with::<ATOMIC, RULE>(input, stack))
    }
}
impl<'i, R: RuleType, COMMENT: TypedNode<'i, R>, WHITESPACE: TypedNode<'i, R>> Debug
    for Ign<'i, R, COMMENT, WHITESPACE>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ign").finish()
    }
}

/// Match a sequence of two expressions.
#[derive(Clone)]
pub struct Seq<
    'i,
    R: RuleType,
    T1: TypedNode<'i, R>,
    T2: TypedNode<'i, R>,
    IGNORED: NeverFailedTypedNode<'i, R>,
> {
    /// Matched first expression.
    pub first: T1,
    /// Matched second expression.
    pub second: T2,
    _phantom: PhantomData<(&'i R, &'i IGNORED)>,
}
impl<
        'i,
        R: RuleType,
        T1: TypedNode<'i, R>,
        T2: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > Seq<'i, R, T1, T2, IGNORED>
{
    pub fn next(&self) -> (&T1, &T2) {
        (&self.first, &self.second)
    }
}
impl<
        'i,
        R: RuleType,
        T1: TypedNode<'i, R>,
        T2: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > TypedNode<'i, R> for Seq<'i, R, T1, T2, IGNORED>
{
    #[inline]
    fn try_parse_with<const ATOMIC: bool, Rule: RuleWrapper<R>>(
        mut input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let (next, first) = T1::try_parse_with::<ATOMIC, Rule>(input, stack, tracker)?;
        input = next;
        let (next, _) = IGNORED::parse_with::<ATOMIC, Rule>(input, stack);
        input = next;
        let (next, second) = T2::try_parse_with::<ATOMIC, Rule>(input, stack, tracker)?;
        input = next;

        Ok((
            input,
            Self {
                first,
                second,
                _phantom: PhantomData,
            },
        ))
    }
}
impl<
        'i,
        R: RuleType,
        T1: TypedNode<'i, R>,
        T2: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > Debug for Seq<'i, R, T1, T2, IGNORED>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Seq")
            .field("first", &self.first)
            .field("second", &self.second)
            .finish()
    }
}

/// Help to invoke if-else-if-else-if-else structure.
pub struct ChoiceHelper<'n, T, Ret> {
    result: Result<Ret, &'n T>,
}
impl<'n, T, Ret> ChoiceHelper<'n, T, Ret> {
    pub(super) fn from(value: &'n T) -> Self {
        Self { result: Err(value) }
    }
    pub fn else_then(self, f: impl FnOnce(&T) -> Ret) -> Ret {
        match self.result {
            Ok(ret) => ret,
            Err(r) => f(r),
        }
    }
}
impl<'i, 'n, R: RuleType, T1: TypedNode<'i, R>, T2: TypedNode<'i, R>, Ret>
    ChoiceHelper<'n, Choice<'i, R, T1, T2>, Ret>
{
    pub fn else_if(self, f: impl FnOnce(&T1) -> Ret) -> ChoiceHelper<'n, T2, Ret> {
        match self.result {
            Err(Choice::First(first, _)) => {
                let result = f(first);
                ChoiceHelper { result: Ok(result) }
            }
            Err(Choice::Second(second, _)) => ChoiceHelper::from(second),
            Ok(ret) => ChoiceHelper { result: Ok(ret) },
        }
    }
}

/// Match either of two expressions.
#[derive(Clone)]
pub enum Choice<'i, R: RuleType, T1: TypedNode<'i, R>, T2: TypedNode<'i, R>> {
    /// Matched first expression.
    First(T1, PhantomData<&'i R>),
    /// Matched second expression.
    Second(T2, PhantomData<&'i R>),
}
impl<'i, R: RuleType, T1: TypedNode<'i, R>, T2: TypedNode<'i, R>> Choice<'i, R, T1, T2> {
    /// Get the first case if exists.
    #[inline]
    pub fn get_first(&self) -> Option<&T1> {
        match self {
            Self::First(res, _) => Some(res),
            Self::Second(_, _) => None,
        }
    }
    /// Get the second case if exists.
    #[inline]
    pub fn get_second(&self) -> Option<&T2> {
        match self {
            Self::First(_, _) => None,
            Self::Second(res, _) => Some(res),
        }
    }
    /// Invoke if is not None and is the first case.
    pub fn if_then<Ret>(&self, f: impl FnOnce(&T1) -> Ret) -> ChoiceHelper<T2, Ret> {
        ChoiceHelper::from(self).else_if(f)
    }
}
impl<'i, R: RuleType, T1: TypedNode<'i, R>, T2: TypedNode<'i, R>> TypedNode<'i, R>
    for Choice<'i, R, T1, T2>
{
    #[inline]
    fn try_parse_with<const ATOMIC: bool, Rule: RuleWrapper<R>>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        match T1::try_parse_with::<ATOMIC, Rule>(input, stack, tracker) {
            Ok((input, first)) => Ok((input, Self::First(first, PhantomData))),
            Err(_) => match T2::try_parse_with::<ATOMIC, Rule>(input, stack, tracker) {
                Ok((input, second)) => Ok((input, Self::Second(second, PhantomData))),
                Err(_) => Err(()),
            },
        }
    }
}
impl<'i, R: RuleType, T1: TypedNode<'i, R>, T2: TypedNode<'i, R>> Debug for Choice<'i, R, T1, T2> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::First(first, _) => f.debug_tuple("First").field(first).finish(),
            Self::Second(second, _) => f.debug_tuple("Second").field(second).finish(),
        }
    }
}

/// Repeatably match `T` at least `MIN` times.
#[derive(Clone)]
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
    > TypedNode<'i, R> for RepMin<'i, R, T, IGNORED, MIN>
{
    #[inline]
    fn try_parse_with<const ATOMIC: bool, Rule: RuleWrapper<R>>(
        mut input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let mut vec = Vec::<T>::new();

        {
            let mut i: usize = 0;
            loop {
                if i != 0 {
                    let (next, _) = IGNORED::parse_with::<ATOMIC, Rule>(input, stack);
                    input = next;
                }
                match T::try_parse_with::<ATOMIC, Rule>(input, stack, tracker) {
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
    > Debug for RepMin<'i, R, T, IGNORED, MIN>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RepMin")
            .field("content", &self.content)
            .finish()
    }
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
        const MIN: usize,
    > BoundWrapper for RepMin<'i, R, T, IGNORED, MIN>
{
    const MIN: usize = MIN;
    const MAX: usize = usize::MAX;
}
pub type Rep<'i, R, T, IGNORED> = RepMin<'i, R, T, IGNORED, 0>;
pub type RepOnce<'i, R, T, IGNORED> = RepMin<'i, R, T, IGNORED, 1>;

/// Drop the top of the stack.
/// Fail if there is no span in the stack.
#[derive(Clone)]
pub struct DROP<'i> {
    _phantom: PhantomData<&'i str>,
}
impl<'i, R: RuleType> TypedNode<'i, R> for DROP<'i> {
    #[inline]
    fn try_parse_with<const _A: bool, Rule: RuleWrapper<R>>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        match stack.pop() {
            Some(_) => Ok((
                input,
                Self {
                    _phantom: PhantomData,
                },
            )),
            None => {
                tracker.empty_stack(input);
                Err(())
            }
        }
    }
}
impl<'i> Debug for DROP<'i> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DROP").finish()
    }
}

/// Match and pop the top span of the stack.
#[derive(Clone)]
pub struct POP<'i> {
    /// Matched span.
    pub span: Span<'i>,
}

impl<'i> From<Span<'i>> for POP<'i> {
    fn from(span: Span<'i>) -> Self {
        Self { span }
    }
}
impl<'i, R: RuleType> TypedNode<'i, R> for POP<'i> {
    #[inline]
    fn try_parse_with<const _A: bool, Rule: RuleWrapper<R>>(
        mut input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        match stack.pop() {
            Some(span) => match input.match_string(span.as_str()) {
                true => Ok((input, Self::from(span))),
                false => Err(()),
            },
            None => {
                tracker.empty_stack(input);
                Err(())
            }
        }
    }
}
impl<'i> Debug for POP<'i> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("POP").finish()
    }
}

/// Match and pop all spans in the stack in top-to-bottom-order.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct POP_ALL<'i> {
    /// Matched span.
    pub span: Span<'i>,
}

impl<'i> From<Span<'i>> for POP_ALL<'i> {
    fn from(span: Span<'i>) -> Self {
        Self { span }
    }
}
impl<'i, R: RuleType> TypedNode<'i, R> for POP_ALL<'i> {
    #[inline]
    fn try_parse_with<const ATOMIC: bool, Rule: RuleWrapper<R>>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let (input, res) = PEEK_ALL::try_parse_with::<ATOMIC, Rule>(input, stack, tracker)?;
        while let Some(_) = stack.pop() {}
        Ok((input, Self::from(res.span)))
    }
}

/// Boxed node for `T`.
#[derive(Clone)]
pub struct Box<'i, R: RuleType, T: TypedNode<'i, R>> {
    /// Boxed content.
    pub content: ::alloc::boxed::Box<T>,
    _phantom: PhantomData<&'i R>,
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Deref for Box<'i, R, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.content.as_ref()
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> DerefMut for Box<'i, R, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.content.as_mut()
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> AsRef<T> for Box<'i, R, T> {
    fn as_ref(&self) -> &T {
        &self.content
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> AsMut<T> for Box<'i, R, T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.content
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> TypedNode<'i, R> for Box<'i, R, T> {
    #[inline]
    fn try_parse_with<const ATOMIC: bool, Rule: RuleWrapper<R>>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let (input, res) = T::try_parse_with::<ATOMIC, Rule>(input, stack, tracker)?;
        Ok((
            input,
            Self {
                content: ::alloc::boxed::Box::new(res),
                _phantom: PhantomData,
            },
        ))
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Debug for Box<'i, R, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.content.fmt(f)
    }
}

/// Restore stack state on error.
#[derive(Clone)]
pub struct Restorable<'i, R: RuleType, T: TypedNode<'i, R>> {
    /// Matched content.
    pub content: T,
    _phantom: PhantomData<&'i R>,
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> From<T> for Restorable<'i, R, T> {
    fn from(content: T) -> Self {
        Self {
            content,
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> TypedNode<'i, R> for Restorable<'i, R, T> {
    fn try_parse_with<const ATOMIC: bool, Rule: RuleWrapper<R>>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        stack.snapshot();
        match T::try_parse_with::<ATOMIC, Rule>(input, stack, tracker) {
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
impl<'i, R: RuleType, T: TypedNode<'i, R>> Debug for Restorable<'i, R, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.content.fmt(f)
    }
}

/// Always fail.
#[derive(Clone)]
pub struct AlwaysFail<'i> {
    _phantom: PhantomData<&'i ()>,
}
/// A trait that only `AlwaysFail` implements.
pub trait AlwaysFailed: Debug + Clone {}
impl<'i> AlwaysFailed for AlwaysFail<'i> {}
impl<'i, R: RuleType, T: AlwaysFailed> TypedNode<'i, R> for T {
    #[inline]
    fn try_parse_with<const ATOMIC: bool, Rule: RuleWrapper<R>>(
        _input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        Err(())
    }
}
impl<'i> Debug for AlwaysFail<'i> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AlwaysFail").finish()
    }
}

/// Start point of an atomic rule.
///
/// Force inner tokens to be atomic.
///
/// See [`Rule`] and [`NonAtomicRule`].
#[derive(Clone)]
pub struct AtomicRule<
    'i,
    R: RuleType,
    T: TypedNode<'i, R>,
    RULE: RuleWrapper<R>,
    _EOI: RuleWrapper<R>,
> {
    /// Matched content.
    pub content: T,
    /// Matched span.
    pub span: Span<'i>,
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
    fn try_parse_with<const ATOMIC: bool, UpperRule: RuleWrapper<R>>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let start = input;
        tracker.record_during(start, |tracker| {
            let (input, res) = T::try_parse_with::<true, RULE>(input, stack, tracker)?;
            let res = Self::from((res, start.span(&input)));
            Ok((input, res))
        })
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>, RULE: RuleWrapper<R>, _EOI: RuleWrapper<R>>
    ParsableTypedNode<'i, R> for AtomicRule<'i, R, T, RULE, _EOI>
{
    fn parse(input: &'i str) -> Result<Self, Error<R>> {
        parse_without_ignore::<R, RULE, _EOI, Self>(input)
    }
    fn parse_partial(input: &'i str) -> Result<(Position<'i>, Self), Error<R>> {
        parse_partial::<R, RULE, Self>(input)
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
#[derive(Clone)]
pub struct NonAtomicRule<
    'i,
    R: RuleType,
    T: TypedNode<'i, R>,
    RULE: RuleWrapper<R>,
    _EOI: RuleWrapper<R>,
    IGNORED: NeverFailedTypedNode<'i, R>,
> {
    /// Matched content.
    pub content: T,
    _phantom: PhantomData<(&'i R, &'i T, &'i RULE, &'i _EOI, &'i IGNORED)>,
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        RULE: RuleWrapper<R>,
        _EOI: RuleWrapper<R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > From<T> for NonAtomicRule<'i, R, T, RULE, _EOI, IGNORED>
{
    fn from(content: T) -> Self {
        Self {
            content,
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
    > TypeWrapper for NonAtomicRule<'i, R, T, RULE, _EOI, IGNORED>
{
    type Inner = T;
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
    fn try_parse_with<const ATOMIC: bool, UpperRule: RuleWrapper<R>>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        tracker.record_during(input, |tracker| {
            let (input, res) = T::try_parse_with::<false, RULE>(input, stack, tracker)?;
            Ok((input, Self::from(res)))
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
        parse::<R, RULE, _EOI, Self, IGNORED>(input)
    }
    fn parse_partial(input: &'i str) -> Result<(Position<'i>, Self), Error<R>> {
        parse_partial::<R, RULE, Self>(input)
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

/// Match an expression and push it.
#[derive(Clone)]
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
    fn try_parse_with<const ATOMIC: bool, Rule: RuleWrapper<R>>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let start = input;
        let (input, content) = T::try_parse_with::<ATOMIC, Rule>(input, stack, tracker)?;
        stack.push(start.span(&input));
        Ok((input, Self::from(content)))
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> Debug for Push<'i, R, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Push")
            .field("content", &self.content)
            .finish()
    }
}

/// Match `[START:END]` in top-to-bottom order of the stack.
#[derive(Clone)]
pub struct PeekSlice2<'i, R: RuleType, const START: i32, const END: i32> {
    _phantom: PhantomData<&'i R>,
}
impl<'i, R: RuleType, const START: i32, const END: i32> TypedNode<'i, R>
    for PeekSlice2<'i, R, START, END>
{
    #[inline]
    fn try_parse_with<const ATOMIC: bool, Rule: RuleWrapper<R>>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let spans = stack_slice(input, START, Some(END), stack, tracker)?;
        let (input, _) = peek_spans::<R, Rule>(input, spans, tracker)?;
        Ok((
            input,
            Self {
                _phantom: PhantomData,
            },
        ))
    }
}
impl<'i, R: RuleType, const START: i32, const END: i32> Debug for PeekSlice2<'i, R, START, END> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PeekSlice2").finish()
    }
}

/// Match `[START:END]` in top-to-bottom order of the stack.
#[derive(Clone)]
pub struct PeekSlice1<'i, R: RuleType, const START: i32> {
    _phantom: PhantomData<&'i R>,
}
impl<'i, R: RuleType, const START: i32> TypedNode<'i, R> for PeekSlice1<'i, R, START> {
    #[inline]
    fn try_parse_with<const ATOMIC: bool, Rule: RuleWrapper<R>>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let spans = stack_slice(input, START, None, stack, tracker)?;
        let (input, _) = peek_spans::<R, Rule>(input, spans, tracker)?;
        Ok((
            input,
            Self {
                _phantom: PhantomData,
            },
        ))
    }
}
impl<'i, R: RuleType, const START: i32> Debug for PeekSlice1<'i, R, START> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PeekSlice1").finish()
    }
}

/// Start point of a normal rule.
///
/// Will not change atomicity.
///
/// See [`AtomicRule`] and [`NonAtomicRule`].
#[derive(Clone)]
pub struct Rule<
    'i,
    R: RuleType,
    RULE: RuleWrapper<R>,
    _EOI: RuleWrapper<R>,
    T: TypedNode<'i, R>,
    IGNORED: NeverFailedTypedNode<'i, R>,
> {
    /// Matched content.
    pub content: T,
    _phantom: PhantomData<(&'i R, &'i RULE, &'i _EOI, &'i IGNORED)>,
}
impl<
        'i,
        R: RuleType,
        RULE: RuleWrapper<R>,
        _EOI: RuleWrapper<R>,
        T: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > From<T> for Rule<'i, R, RULE, _EOI, T, IGNORED>
{
    fn from(content: T) -> Self {
        Self {
            content,
            _phantom: PhantomData,
        }
    }
}
impl<
        'i,
        R: RuleType,
        RULE: RuleWrapper<R>,
        _EOI: RuleWrapper<R>,
        T: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > TypeWrapper for Rule<'i, R, RULE, _EOI, T, IGNORED>
{
    type Inner = T;
}
impl<
        'i,
        R: RuleType,
        RULE: RuleWrapper<R>,
        _EOI: RuleWrapper<R>,
        T: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > RuleWrapper<R> for Rule<'i, R, RULE, _EOI, T, IGNORED>
{
    const RULE: R = RULE::RULE;
    type Rule = R;
}
impl<
        'i,
        R: RuleType,
        RULE: RuleWrapper<R>,
        _EOI: RuleWrapper<R>,
        T: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > TypedNode<'i, R> for Rule<'i, R, RULE, _EOI, T, IGNORED>
{
    #[inline]
    fn try_parse_with<const ATOMIC: bool, UpperRule: RuleWrapper<R>>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        tracker.record_during(input, |tracker| {
            let (input, res) = T::try_parse_with::<ATOMIC, RULE>(input, stack, tracker)?;
            Ok((input, Self::from(res)))
        })
    }
}
impl<
        'i,
        R: RuleType,
        RULE: RuleWrapper<R>,
        _EOI: RuleWrapper<R>,
        T: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > ParsableTypedNode<'i, R> for Rule<'i, R, RULE, _EOI, T, IGNORED>
{
    #[inline]
    fn parse(input: &'i str) -> Result<Self, Error<R>> {
        parse::<R, RULE, _EOI, Self, IGNORED>(input)
    }

    fn parse_partial(input: &'i str) -> Result<(Position<'i>, Self), Error<R>> {
        parse_partial::<R, RULE, Self>(input)
    }
}
impl<
        'i,
        R: RuleType,
        RULE: RuleWrapper<R>,
        _EOI: RuleWrapper<R>,
        T: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > Debug for Rule<'i, R, RULE, _EOI, T, IGNORED>
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
    R: RuleType,
    RULE: RuleWrapper<R>,
    _EOI: RuleWrapper<R>,
    _Self: TypedNode<'i, R>,
    IGNORED: NeverFailedTypedNode<'i, R>,
>(
    input: &'i str,
) -> Result<_Self, Error<R>> {
    let mut stack = Stack::new();
    let input = Position::from_start(input);
    let mut tracker = Tracker::new(input);
    let (input, res) = match _Self::try_parse_with::<false, RULE>(input, &mut stack, &mut tracker) {
        Ok((input, res)) => (input, res),
        Err(_) => return Err(tracker.collect()),
    };
    let (input, _) = IGNORED::parse_with::<false, _EOI>(input, &mut stack);
    let (_, _) = match EOI::try_parse_with::<false, _EOI>(input, &mut stack, &mut tracker) {
        Ok((input, res)) => (input, res),
        Err(_) => return Err(tracker.collect()),
    };
    Ok(res)
}

fn parse_without_ignore<
    'i,
    R: RuleType,
    RULE: RuleWrapper<R>,
    _EOI: RuleWrapper<R>,
    _Self: TypedNode<'i, R>,
>(
    input: &'i str,
) -> Result<_Self, Error<R>> {
    let mut stack = Stack::new();
    let input = Position::from_start(input);
    let mut tracker = Tracker::new(input);
    let (input, res) = match _Self::try_parse_with::<false, RULE>(input, &mut stack, &mut tracker) {
        Ok((input, res)) => (input, res),
        Err(_) => return Err(tracker.collect()),
    };
    let (_, _) = match EOI::try_parse_with::<false, _EOI>(input, &mut stack, &mut tracker) {
        Ok((input, res)) => (input, res),
        Err(_) => return Err(tracker.collect()),
    };
    Ok(res)
}

fn parse_partial<'i, R: RuleType, RULE: RuleWrapper<R>, _Self: TypedNode<'i, R>>(
    input: &'i str,
) -> Result<(Position<'i>, _Self), Error<R>> {
    let mut stack = Stack::new();
    let input = Position::from_start(input);
    let mut tracker = Tracker::new(input);
    match _Self::try_parse_with::<false, RULE>(input, &mut stack, &mut tracker) {
        Ok((input, res)) => Ok((input, res)),
        Err(_) => return Err(tracker.collect()),
    }
}

/// ASCII Digit. `'0'..'9'`
#[allow(non_camel_case_types)]
pub type ASCII_DIGIT<'i, R> = CharRange<'i, R, '0', '9'>;

/// Non-zero ASCII Digit. `'1'..'9'`
#[allow(non_camel_case_types)]
pub type ASCII_NONZERO_DIGIT<'i, R> = CharRange<'i, R, '1', '9'>;

/// Binary ASCII Digit. `'0'..'1'`
#[allow(non_camel_case_types)]
pub type ASCII_BIN_DIGIT<'i, R> = CharRange<'i, R, '0', '1'>;

/// Octal ASCII Digit. `'0'..'7'`
#[allow(non_camel_case_types)]
pub type ASCII_OCT_DIGIT<'i, R> = CharRange<'i, R, '0', '7'>;

/// Hexadecimal ASCII Digit. `'0'..'9' | 'a'..'f' | 'A'..'F'`
#[allow(non_camel_case_types)]
pub type ASCII_HEX_DIGIT<'i, R> = Choice<
    'i,
    R,
    ASCII_DIGIT<'i, R>,
    Choice<'i, R, CharRange<'i, R, 'a', 'f'>, CharRange<'i, R, 'A', 'F'>>,
>;

/// Lower case ASCII alphabet.
#[allow(non_camel_case_types)]
pub type ASCII_ALPHA_LOWER<'i, R> = CharRange<'i, R, 'a', 'z'>;

/// Upper case ASCII alphabet.
#[allow(non_camel_case_types)]
pub type ASCII_ALPHA_UPPER<'i, R> = CharRange<'i, R, 'A', 'Z'>;

/// ASCII alphabet.
#[allow(non_camel_case_types)]
pub type ASCII_ALPHA<'i, R> = Choice<'i, R, ASCII_ALPHA_LOWER<'i, R>, ASCII_ALPHA_UPPER<'i, R>>;

/// ASCII alphabet or digit.
#[allow(non_camel_case_types)]
pub type ASCII_ALPHANUMERIC<'i, R> = Choice<'i, R, ASCII_ALPHA<'i, R>, ASCII_DIGIT<'i, R>>;

/// ASCII alphabet.
#[allow(non_camel_case_types)]
pub type ASCII<'i, R> = CharRange<'i, R, '\x00', '\x7f'>;

/// Match char by a predicate.
///
/// Return Some(char) if matched.
pub fn match_char_by(position: &mut Position<'_>, pred: impl FnOnce(char) -> bool) -> Option<char> {
    let mut res = None;
    position.match_char_by(|c| {
        let matched = pred(c);
        if matched {
            res = Some(c);
        }
        matched
    });
    res
}

#[cfg(test)]
mod tests {
    use super::super::Storage;
    use super::*;
    use alloc::format;

    macro_rules! make_rules {
        ($($ids:ident,)*) => {
            #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            enum Rule {
                $($ids),*
            }
            mod rule_wrappers {
                $(
                    #[derive(Clone)]
                    pub struct $ids {}
                    impl super::RuleWrapper<super::Rule> for $ids {
                        const RULE:super::Rule = super::Rule::$ids;
                        type Rule = super::Rule;
                    }
                )*
            }
        };
    }

    make_rules! {
        Foo,
        RepFoo,
        WHITESPACE,
        COMMENT,
        EOI,
    }

    #[derive(Clone)]
    struct Foo;
    impl StringWrapper for Foo {
        const CONTENT: &'static str = "foo";
    }
    impl RuleWrapper<Rule> for Foo {
        const RULE: Rule = Rule::Foo;
        type Rule = Rule;
    }

    type WHITESPACE<'i> = AtomicRule<
        'i,
        Rule,
        CharRange<'i, Rule, ' ', ' '>,
        rule_wrappers::WHITESPACE,
        rule_wrappers::EOI,
    >;
    type COMMENT<'i> = AtomicRule<
        'i,
        Rule,
        CharRange<'i, Rule, '\t', '\t'>,
        rule_wrappers::COMMENT,
        rule_wrappers::EOI,
    >;
    type StrFoo<'i> = super::Rule<
        'i,
        Rule,
        rule_wrappers::Foo,
        rule_wrappers::EOI,
        Str<'i, Rule, Foo>,
        Ignore<'i>,
    >;
    #[test]
    fn string() {
        assert_eq!(<StrFoo<'_> as TypeWrapper>::Inner::CONTENT, Foo::CONTENT);
        let s = StrFoo::parse("foo").unwrap();
        assert_eq!(s.content.get_content(), "foo");
        assert_eq!(format!("{:?}", s), r#"Rule { rule: Foo, content: Str }"#)
    }
    #[test]
    fn range() {
        let whitespace = WHITESPACE::parse(" ").unwrap();
        assert_eq!(
            format!("{:?}", whitespace),
            r#"AtomicRule { rule: WHITESPACE, content: Range { content: ' ' } }"#
        );
        let comment = COMMENT::parse("\t").unwrap();
        assert_eq!(
            format!("{:?}", comment),
            r#"AtomicRule { rule: COMMENT, content: Range { content: '\t' } }"#
        );
    }
    type Ignore<'i> = Ign<'i, Rule, COMMENT<'i>, WHITESPACE<'i>>;
    #[test]
    fn ignore() {
        super::Rule::<Rule, rule_wrappers::RepFoo, rule_wrappers::EOI, Ignore<'_>, Ignore<'_>>::parse(
            " \t  ",
        )
        .unwrap();
    }

    type R<'i> = super::Rule<
        'i,
        Rule,
        rule_wrappers::RepFoo,
        rule_wrappers::EOI,
        Rep<'i, Rule, Str<'i, Rule, Foo>, Ignore<'i>>,
        Ignore<'i>,
    >;
    #[test]
    fn repetition() {
        let rep = R::parse("foofoofoo").unwrap();
        assert_eq!(
            format!("{:?}", rep),
            "Rule { rule: RepFoo, content: RepMin { content: [Str, Str, Str] } }"
        );
        let rep = R::parse("foo foo foo").unwrap();
        assert_eq!(
            format!("{:?}", rep),
            "Rule { rule: RepFoo, content: RepMin { content: [Str, Str, Str] } }"
        );
        let rep = R::parse("foo foo\tfoo").unwrap();
        assert_eq!(
            format!("{:?}", rep),
            "Rule { rule: RepFoo, content: RepMin { content: [Str, Str, Str] } }"
        );
    }
}
