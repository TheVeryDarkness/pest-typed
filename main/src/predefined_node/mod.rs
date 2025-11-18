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

mod repetition;
pub mod unicode;

use super::{
    parser_state::constrain_idxs,
    span::Span,
    tracker::Tracker,
    typed_node::NeverFailedTypedNode,
    wrapper::{StringArrayWrapper, StringWrapper},
    RuleType, Stack, TypedNode,
};
use core::{
    fmt::{self, Debug},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};
use derive_where::derive_where;
pub use repetition::{
    AtomicRepeat, Rep, RepExact, RepMax, RepMin, RepMinMax, RepOnce, RepeatMin, RepeatMinMax,
};

/// Match given string case sensitively.
///
/// The `CONTENT` on the type (by [`StringWrapper`]) is the original string to match.
///
/// See [`Insens`] for case-insensitive matching.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive_where(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Str<T: StringWrapper + 'static> {
    _phantom: PhantomData<&'static T>,
}
impl<T: StringWrapper> StringWrapper for Str<T> {
    const CONTENT: &'static str = T::CONTENT;
}
impl<T: StringWrapper> From<()> for Str<T> {
    fn from(_value: ()) -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}
impl<C: Cursor, R: RuleType, T: StringWrapper + 'static> TypedNode<C, R> for Str<T> {
    #[inline]
    fn try_parse_partial_with(
        mut input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        if input.match_string(Self::CONTENT) {
            Some((input, Self::from(())))
        } else {
            None
        }
    }

    #[inline]
    fn try_check_partial_with(
        mut input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        if input.match_string(Self::CONTENT) {
            Some(input)
        } else {
            None
        }
    }
}
impl<T: StringWrapper> Debug for Str<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Str").finish()
    }
}

/// Match given string case insensitively.
///
/// - The field `content` is the matched string.
/// - The `CONTENT` on the type (by [`StringWrapper`]) is the original string to match, and it may differ from `content` in case.
///   
///   For example, A `^"x"` may match `"X"`, and in the parsing result, `self.content` is `"X"`, while `Self::CONTENT` is still `"x"`.    
///
/// See [`Str`] for case-sensitive matching.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive_where(Clone, Hash, PartialEq, Eq; S: RefStr)]
pub struct Insens<S, T> {
    /// Matched content.
    pub content: S,
    _phantom: PhantomData<T>,
}
impl<S, T: StringWrapper> StringWrapper for Insens<S, T> {
    const CONTENT: &'static str = T::CONTENT;
}
impl<S, T> From<S> for Insens<S, T> {
    fn from(content: S) -> Self {
        Self {
            content,
            _phantom: PhantomData,
        }
    }
}
impl<C: Cursor, R: RuleType, T: StringWrapper> TypedNode<C, R> for Insens<C::String, T> {
    #[inline]
    fn try_parse_partial_with(
        mut input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        let start = input.clone();
        if input.match_insensitive(Self::CONTENT) {
            let span = start.span(&input);
            Some((input, Self::from(span.as_str())))
        } else {
            None
        }
    }

    #[inline]
    fn try_check_partial_with(
        mut input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        if input.match_insensitive(Self::CONTENT) {
            Some(input)
        } else {
            None
        }
    }
}
impl<S: Debug, T> Debug for Insens<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Insens")
            .field("content", &self.content)
            .finish()
    }
}

/// Skips until one of the given strings.
#[derive_where(Clone, Hash, PartialEq, Eq; S: RefStr)]
pub struct Skip<S, Strings> {
    /// Skipped span.
    pub span: Span<S>,
    _phantom: PhantomData<Strings>,
}
impl<S, Strings: StringArrayWrapper> StringArrayWrapper for Skip<S, Strings> {
    const CONTENT: &'static [&'static str] = Strings::CONTENT;
}
impl<S, Strings> From<Span<S>> for Skip<S, Strings> {
    fn from(span: Span<S>) -> Self {
        Self {
            span,
            _phantom: PhantomData,
        }
    }
}
impl<C: Cursor, R: RuleType, Strings: StringArrayWrapper> TypedNode<C, R>
    for Skip<C::String, Strings>
{
    #[inline]
    fn try_parse_partial_with(
        mut input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        let start = input.clone();
        match input.skip_until(Strings::CONTENT) {
            true => {
                let span = start.span(&input);
                Some((input, Self::from(span)))
            }
            false => {
                let span = start.span(&input);
                Some((input, Self::from(span))) // return the original input if not found
            }
        }
    }

    #[inline]
    fn try_check_partial_with(
        mut input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        match input.skip_until(Strings::CONTENT) {
            true => Some(input),
            false => Some(input), // return the original input if not found
        }
    }
}
impl<S: RefStr, Strings> Debug for Skip<S, Strings> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Skip").field("span", &self.span).finish()
    }
}

/// Skip `n` characters if there are.
#[derive_where(Clone, Debug, Hash, PartialEq, Eq; S: RefStr)]
pub struct SkipChar<S, const N: usize> {
    /// Skipped span.
    pub span: Span<S>,
}
impl<S, const N: usize> From<Span<S>> for SkipChar<S, N> {
    fn from(span: Span<S>) -> Self {
        Self { span }
    }
}
impl<C: Cursor, R: RuleType, const N: usize> TypedNode<C, R> for SkipChar<C::String, N> {
    #[inline]
    fn try_parse_partial_with(
        mut input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        let start = input.clone();
        match input.skip(N) {
            true => {
                let span = start.span(&input);
                Some((input, Self::from(span)))
            }
            false => None,
        }
    }

    #[inline]
    fn try_check_partial_with(
        mut input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        match input.skip(N) {
            true => Some(input),
            false => None,
        }
    }
}

/// Match a character in the range `[MIN, MAX]`.
/// Inclusively both below and above.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CharRange<const MIN: char, const MAX: char> {
    /// Matched character.
    pub content: char,
}
impl<C: Cursor, R: RuleType, const MIN: char, const MAX: char> TypedNode<C, R>
    for CharRange<MIN, MAX>
{
    #[inline]
    fn try_parse_partial_with(
        mut input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        let start = input.clone();
        match input.match_range(MIN..MAX) {
            true => {
                let span = start.span(&input);
                let content = span.as_str().chars().next().unwrap();
                Some((input, Self { content }))
            }
            false => None,
        }
    }

    #[inline]
    fn try_check_partial_with(
        mut input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        match input.match_range(MIN..MAX) {
            true => Some(input),
            false => None,
        }
    }
}

/// Try to create stack slice.
#[inline]
fn stack_slice<'s, C: Cursor, R: RuleType>(
    input: C,
    start: i32,
    end: Option<i32>,
    stack: &'s Stack<Span<C::String>>,
    tracker: &mut Tracker<C::String, R>,
) -> Option<core::slice::Iter<'s, Span<C::String>>> {
    let range = match constrain_idxs(start, end, stack.len()) {
        Some(range) => range,
        None => {
            tracker.out_of_bound(input, start, end);
            return None;
        }
    };
    // return true if an empty sequence is requested
    if range.end <= range.start {
        return Some(core::slice::Iter::default());
    }
    Some(stack[range].iter())
}

/// Match a part of the stack without popping.
/// Will match (consume) input.
#[inline]
fn peek_spans<'s, C: Cursor, R: RuleType>(
    input: C,
    iter: impl Iterator<Item = &'s Span<C::String>>,
    _tracker: &mut Tracker<C::String, R>,
) -> Option<(C, Span<C::String>)>
where
    C::String: 's,
{
    let mut matching_pos = input.clone();
    for span in iter {
        match matching_pos.match_string(span.as_str().as_str()) {
            true => (),
            false => {
                return None;
            }
        }
    }
    let span = input.span(&matching_pos);
    Some((matching_pos, span))
}

/// Positive predicate.
///
/// Peeked expressions will not occur in Pair/Pairs API.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Positive<N> {
    /// Peeked content.
    pub content: N,
}
impl<N> From<N> for Positive<N> {
    fn from(content: N) -> Self {
        Self { content }
    }
}
impl<N> Deref for Positive<N> {
    type Target = N;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
impl<N> DerefMut for Positive<N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}
impl<C: Cursor, R: RuleType, N: TypedNode<C, R>> TypedNode<C, R> for Positive<N> {
    #[inline]
    fn try_parse_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        tracker.positive_during(|tracker| {
            stack.snapshot();
            match N::try_parse_partial_with(input.clone(), stack, tracker) {
                Some((_, content)) => {
                    stack.restore();
                    Some((input, Self::from(content)))
                }
                None => {
                    stack.restore();
                    None
                }
            }
        })
    }

    #[inline]
    fn try_check_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        tracker.positive_during(|tracker| {
            stack.snapshot();
            match N::try_check_partial_with(input.clone(), stack, tracker) {
                Some(_) => {
                    stack.restore();
                    Some(input)
                }
                None => {
                    stack.restore();
                    None
                }
            }
        })
    }
}

/// Negative predicate.
///
/// Will not contain anything.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive_where(Clone, Hash, PartialEq, Eq)]
pub struct Negative<T> {
    _phantom: PhantomData<T>,
}
impl<T> From<()> for Negative<T> {
    fn from(_value: ()) -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}
impl<C: Cursor, R: RuleType, T: TypedNode<C, R>> TypedNode<C, R> for Negative<T> {
    #[inline]
    fn try_parse_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        tracker.negative_during(|tracker| {
            stack.snapshot();
            match T::try_check_partial_with(input.clone(), stack, tracker) {
                Some(_) => {
                    stack.restore();
                    None
                }
                None => {
                    stack.restore();
                    Some((input, Self::from(())))
                }
            }
        })
    }

    #[inline]
    fn try_check_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        tracker.negative_during(|tracker| {
            stack.snapshot();
            match T::try_check_partial_with(input.clone(), stack, tracker) {
                Some(_) => {
                    stack.restore();
                    None
                }
                None => {
                    stack.restore();
                    Some(input)
                }
            }
        })
    }
}
impl<T> Debug for Negative<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Negative").finish()
    }
}

/// Match any character.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ANY {
    /// Matched character.
    pub content: char,
}
impl<C: Cursor, R: RuleType> TypedNode<C, R> for ANY {
    #[inline]
    fn try_parse_partial_with(
        mut input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        input.advance_char().map(|c| (input, Self { content: c }))
    }

    #[inline]
    fn try_check_partial_with(
        mut input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        input.advance_char().map(|_| input)
    }
}

/// Match the start of input.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SOI;
impl<C: Cursor, R: RuleType> TypedNode<C, R> for SOI {
    #[inline]
    fn try_parse_partial_with(
        input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        if input.at_start() {
            Some((input, Self))
        } else {
            None
        }
    }

    #[inline]
    fn try_check_partial_with(
        input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        if input.at_start() {
            Some(input)
        } else {
            None
        }
    }
}

/// Match the end of input.
///
/// [`EOI`] will record its rule if not matched.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct EOI;
impl<C: Cursor, R: RuleType> TypedNode<C, R> for EOI {
    #[inline]
    fn try_parse_partial_with(
        input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        if input.at_end() {
            Some((input, Self))
        } else {
            None
        }
    }

    #[inline]
    fn try_check_partial_with(
        input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        if input.at_end() {
            Some(input)
        } else {
            None
        }
    }
}

/// Type of eol.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum NewLineType {
    /// `\r\n`
    CRLF,
    /// `\n`
    LF,
    /// `\r`
    CR,
}

/// Match a new line character.
/// A built-in rule. Equivalent to `"\r\n" | "\n" | "\r"`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct NEWLINE {
    /// Type of matched character.
    pub content: NewLineType,
}
impl<C: Cursor, R: RuleType> TypedNode<C, R> for NEWLINE {
    #[inline]
    fn try_parse_partial_with(
        mut input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        let (input, t) = if input.match_string("\r\n") {
            (input, NewLineType::CRLF)
        } else if input.match_string("\n") {
            (input, NewLineType::LF)
        } else if input.match_string("\r") {
            (input, NewLineType::CR)
        } else {
            return None;
        };
        Some((input, Self { content: t }))
    }

    #[inline]
    fn try_check_partial_with(
        mut input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        if input.match_string("\r\n") || input.match_string("\n") || input.match_string("\r") {
            Some(input)
        } else {
            None
        }
    }
}

/// Peek all spans in stack reversely.
/// Will consume input.
#[allow(non_camel_case_types)]
#[derive_where(Clone, Debug, Hash, PartialEq, Eq; S: RefStr)]
pub struct PEEK_ALL<S> {
    /// Pair span.
    pub span: Span<S>,
}
impl<S> From<Span<S>> for PEEK_ALL<S> {
    fn from(span: Span<S>) -> Self {
        Self { span }
    }
}
impl<C: Cursor, R: RuleType> TypedNode<C, R> for PEEK_ALL<C::String> {
    #[inline]
    fn try_parse_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        let spans = stack[0..stack.len()].iter().rev();
        let (input, span) = peek_spans::<C, R>(input, spans, tracker)?;
        Some((input, Self::from(span)))
    }

    #[inline]
    fn try_check_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        let spans = stack[0..stack.len()].iter().rev();
        let (input, _) = peek_spans::<C, R>(input, spans, tracker)?;
        Some(input)
    }
}

/// Peek top span in stack.
/// Will consume input.
#[allow(non_camel_case_types)]
#[derive_where(Clone, Debug, Hash, PartialEq, Eq; S: RefStr)]
pub struct PEEK<S> {
    /// Pair span.
    pub span: Span<S>,
}
impl<S> From<Span<S>> for PEEK<S> {
    fn from(span: Span<S>) -> Self {
        Self { span }
    }
}
impl<C: Cursor, R: RuleType> TypedNode<C, R> for PEEK<C::String> {
    #[inline]
    fn try_parse_partial_with(
        mut input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        let start = input.clone();
        match stack.peek() {
            Some(string) => match input.match_string(string.as_str().as_str()) {
                true => {
                    let span = Self::from(start.span(&input));
                    Some((input, span))
                }
                false => None,
            },
            None => {
                tracker.empty_stack(input);
                None
            }
        }
    }

    #[inline]
    fn try_check_partial_with(
        mut input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        match stack.peek() {
            Some(string) => match input.match_string(string.as_str().as_str()) {
                true => Some(input),
                false => None,
            },
            None => {
                tracker.empty_stack(input);
                None
            }
        }
    }
}

/// Skip comments (by rule `COMMENT`) or white spaces (by rule `WHITESPACE`) if there is any.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Skipped<T, Skip, const SKIP: usize> {
    /// Skipped content.
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub skipped: [Skip; SKIP],
    /// Matched content.
    pub matched: T,
}
impl<T: Debug, Skip: Debug, const SKIP: usize> Debug for Skipped<T, Skip, SKIP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if SKIP > 0 {
            f.debug_struct("Skipped")
                .field("skipped", &self.skipped)
                .field("matched", &self.matched)
                .finish()
        } else {
            Debug::fmt(&self.matched, f)
        }
    }
}

/// Drop the top of the stack.
///
/// Fail if there is no span in the stack.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct DROP;
impl<C: Cursor, R: RuleType> TypedNode<C, R> for DROP {
    #[inline]
    fn try_parse_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        match stack.pop() {
            Some(_) => Some((input, Self)),
            None => {
                tracker.empty_stack(input);
                None
            }
        }
    }

    #[inline]
    fn try_check_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        match stack.pop() {
            Some(_) => Some(input),
            None => {
                tracker.empty_stack(input);
                None
            }
        }
    }
}

/// Match and pop the top span of the stack.
#[derive_where(Clone, Debug, Hash, PartialEq, Eq; S: RefStr)]
pub struct POP<S> {
    /// Matched span.
    pub span: Span<S>,
}

impl<S> From<Span<S>> for POP<S> {
    fn from(span: Span<S>) -> Self {
        Self { span }
    }
}
impl<C: Cursor, R: RuleType> TypedNode<C, R> for POP<C::String> {
    #[inline]
    fn try_parse_partial_with(
        mut input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        match stack.pop() {
            Some(span) => match input.match_string(span.as_str().as_str()) {
                true => Some((input, Self::from(span))),
                false => None,
            },
            None => {
                tracker.empty_stack(input);
                None
            }
        }
    }

    #[inline]
    fn try_check_partial_with(
        mut input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        match stack.pop() {
            Some(span) => match input.match_string(span.as_str().as_str()) {
                true => Some(input),
                false => None,
            },
            None => {
                tracker.empty_stack(input);
                None
            }
        }
    }
}

/// Match and pop all spans in the stack in top-to-bottom-order.
#[allow(non_camel_case_types)]
#[derive_where(Clone, Debug, Hash, PartialEq, Eq; S: RefStr)]
pub struct POP_ALL<S> {
    /// Matched span.
    pub span: Span<S>,
}
impl<S> From<Span<S>> for POP_ALL<S> {
    fn from(span: Span<S>) -> Self {
        Self { span }
    }
}
impl<C: Cursor, R: RuleType> TypedNode<C, R> for POP_ALL<C::String> {
    #[inline]
    fn try_parse_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        let (input, res) = PEEK_ALL::try_parse_partial_with(input, stack, tracker)?;
        while stack.pop().is_some() {}
        Some((input, Self::from(res.span)))
    }

    #[inline]
    fn try_check_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        let input = PEEK_ALL::try_check_partial_with(input, stack, tracker)?;
        while stack.pop().is_some() {}
        Some(input)
    }
}

/// Always fail.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive_where(Clone, Hash, PartialEq, Eq)]
pub struct AlwaysFail<S>(PhantomData<S>);
impl<S> Default for AlwaysFail<S> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<C: Cursor, R: RuleType> TypedNode<C, R> for AlwaysFail<C::String> {
    #[inline]
    fn try_parse_partial_with(
        _input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        None
    }

    #[inline]
    fn try_check_partial_with(
        _input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        None
    }
}
impl<S> Debug for AlwaysFail<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AlwaysFail").finish()
    }
}

/// Empty.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive_where(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Empty<S>(PhantomData<S>);
impl<S> Default for Empty<S> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<C: Cursor, R: RuleType> NeverFailedTypedNode<C, R> for Empty<C::String> {
    #[inline]
    fn parse_with(input: C, _stack: &mut Stack<Span<C::String>>) -> (C, Self) {
        (input, Self::default())
    }

    fn check_with(input: C, _stack: &mut Stack<Span<C::String>>) -> C {
        input
    }
}
impl<C: Cursor, R: RuleType> TypedNode<C, R> for Empty<C::String> {
    #[inline]
    fn try_parse_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        Some(<Self as NeverFailedTypedNode<C, R>>::parse_with(
            input, stack,
        ))
    }

    #[inline]
    fn try_check_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        Some(<Self as NeverFailedTypedNode<C, R>>::check_with(
            input, stack,
        ))
    }
}

/// Match an expression and push it to the [Stack].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Push<T> {
    /// Matched content.
    pub content: T,
}
impl<T> From<T> for Push<T> {
    fn from(content: T) -> Self {
        Self { content }
    }
}
impl<C: Cursor, R: RuleType, T: TypedNode<C, R>> TypedNode<C, R> for Push<T> {
    #[inline]
    fn try_parse_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        let start = input.clone();
        let (input, content) = T::try_parse_partial_with(input, stack, tracker)?;
        stack.push(start.span(&input));
        Some((input, Self::from(content)))
    }

    #[inline]
    fn try_check_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        let start = input.clone();
        let input = T::try_check_partial_with(input, stack, tracker)?;
        stack.push(start.span(&input));
        Some(input)
    }
}
impl<T> Deref for Push<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
impl<T> DerefMut for Push<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

/// Simply push a literal to the [Stack].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive_where(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PushLiteral<T: StringWrapper + 'static> {
    /// Matched content.
    _phantom: PhantomData<&'static T>,
}
impl<T: StringWrapper> PushLiteral<T> {
    const fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}
impl<T: StringWrapper> StringWrapper for PushLiteral<T> {
    const CONTENT: &'static str = T::CONTENT;
}
impl<C: Cursor, R: RuleType, T: StringWrapper + 'static> TypedNode<C, R> for PushLiteral<T> {
    #[inline]
    fn try_parse_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        stack.push(Span::new_full(C::String::from_static(T::CONTENT)));
        Some((input, Self::new()))
    }

    #[inline]
    fn try_check_partial_with(
        input: C,
        _stack: &mut Stack<Span<C::String>>,
        _tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        Some(input)
    }
}
impl<T: StringWrapper> Debug for PushLiteral<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PushLiteral").finish()
    }
}

/// Match `[START..END]` in top-to-bottom order of the stack.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PeekSlice2<const START: i32, const END: i32>;
impl<C: Cursor, R: RuleType, const START: i32, const END: i32> TypedNode<C, R>
    for PeekSlice2<START, END>
{
    #[inline]
    fn try_parse_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        let spans = stack_slice(input.clone(), START, Some(END), stack, tracker)?;
        let (input, _) = peek_spans::<C, R>(input, spans, tracker)?;
        Some((input, Self))
    }

    #[inline]
    fn try_check_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        let spans = stack_slice(input.clone(), START, Some(END), stack, tracker)?;
        let (input, _) = peek_spans::<C, R>(input, spans, tracker)?;
        Some(input)
    }
}

/// Match `[START..]` in top-to-bottom order of the stack.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PeekSlice1<const START: i32>;
impl<C: Cursor, R: RuleType, const START: i32> TypedNode<C, R> for PeekSlice1<START> {
    #[inline]
    fn try_parse_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<(C, Self)> {
        let spans = stack_slice(input.clone(), START, None, stack, tracker)?;
        let (input, _) = peek_spans::<C, R>(input, spans, tracker)?;
        Some((input, Self))
    }

    #[inline]
    fn try_check_partial_with(
        input: C,
        stack: &mut Stack<Span<C::String>>,
        tracker: &mut Tracker<C::String, R>,
    ) -> Option<C> {
        let spans = stack_slice(input.clone(), START, None, stack, tracker)?;
        let (input, _) = peek_spans::<C, R>(input, spans, tracker)?;
        Some(input)
    }
}

/// ASCII Digit. `'0'..'9'`
#[allow(non_camel_case_types)]
pub type ASCII_DIGIT = CharRange<'0', '9'>;

/// Non-zero ASCII Digit. `'1'..'9'`
#[allow(non_camel_case_types)]
pub type ASCII_NONZERO_DIGIT = CharRange<'1', '9'>;

/// Binary ASCII Digit. `'0'..'1'`
#[allow(non_camel_case_types)]
pub type ASCII_BIN_DIGIT = CharRange<'0', '1'>;

/// Octal ASCII Digit. `'0'..'7'`
#[allow(non_camel_case_types)]
pub type ASCII_OCT_DIGIT = CharRange<'0', '7'>;

use crate::{
    choices::{Choice2, Choice3},
    input::RefStr,
    Cursor,
};
/// Hexadecimal ASCII Digit. `'0'..'9' | 'a'..'f' | 'A'..'F'`
#[allow(non_camel_case_types)]
pub type ASCII_HEX_DIGIT = Choice3<ASCII_DIGIT, CharRange<'a', 'f'>, CharRange<'A', 'F'>>;

/// Lower case ASCII alphabet.
#[allow(non_camel_case_types)]
pub type ASCII_ALPHA_LOWER = CharRange<'a', 'z'>;

/// Upper case ASCII alphabet.
#[allow(non_camel_case_types)]
pub type ASCII_ALPHA_UPPER = CharRange<'A', 'Z'>;

/// ASCII alphabet.
#[allow(non_camel_case_types)]
pub type ASCII_ALPHA = Choice2<ASCII_ALPHA_LOWER, ASCII_ALPHA_UPPER>;

/// ASCII alphabet or digit.
#[allow(non_camel_case_types)]
pub type ASCII_ALPHANUMERIC = Choice2<ASCII_ALPHA, ASCII_DIGIT>;

/// ASCII alphabet.
#[allow(non_camel_case_types)]
pub type ASCII = CharRange<'\x00', '\x7f'>;

/// Match char by a predicate.
///
/// Return Some(char) if matched.
#[inline]
pub fn match_char_by(position: &mut impl Cursor, pred: impl FnOnce(char) -> bool) -> Option<char> {
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

/// Restore on error.
#[inline]
pub fn restore_on_none<S: RefStr, T>(
    stack: &mut Stack<Span<S>>,
    f: impl FnOnce(&mut Stack<Span<S>>) -> Option<T>,
) -> Option<T> {
    stack.snapshot();
    let res = f(stack);
    match res.as_ref() {
        Some(_) => stack.clear_snapshot(),
        None => stack.restore(),
    }
    res
}
