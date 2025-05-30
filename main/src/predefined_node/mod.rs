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

use super::{parser_state::constrain_idxs, Stack};
use super::{
    span::Span,
    tracker::Tracker,
    typed_node::NeverFailedTypedNode,
    wrapper::{StringArrayWrapper, StringWrapper},
    RuleType, TypedNode,
};
use core::ops::{Deref, DerefMut};
use core::{fmt::Debug, marker::PhantomData};
use custom_debug_derive::Debug as Dbg;
use derive_where::derive_where;
pub use repetition::{
    AtomicRepeat, Rep, RepExact, RepMin, RepMinMax, RepOnce, RepeatMin, RepeatMinMax,
};

/// Match given string case sensitively.
///
/// The `CONTENT` on the type (by [`StringWrapper`]) is the original string to match.
///
/// See [`Insens`] for case-insensitive matching.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Dbg, Hash, PartialEq, Eq)]
pub struct Str<T: StringWrapper + 'static> {
    #[debug(skip)]
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
impl<'i, R: RuleType, T: StringWrapper + 'static> TypedNode<'i, R> for Str<T> {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        mut input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        if input.match_string(Self::CONTENT) {
            Some((input, Self::from(())))
        } else {
            None
        }
    }

    #[inline]
    fn try_check_partial_with<I: Input<'i>>(
        mut input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
        if input.match_string(Self::CONTENT) {
            Some(input)
        } else {
            None
        }
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
#[derive(Clone, Dbg, Hash, PartialEq, Eq)]
pub struct Insens<'i, T: StringWrapper> {
    /// Matched content.
    pub content: &'i str,
    #[debug(skip)]
    _phantom: PhantomData<&'i T>,
}
impl<T: StringWrapper> StringWrapper for Insens<'_, T> {
    const CONTENT: &'static str = T::CONTENT;
}
impl<'i, T: StringWrapper> From<&'i str> for Insens<'i, T> {
    fn from(content: &'i str) -> Self {
        Self {
            content,
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, T: StringWrapper> TypedNode<'i, R> for Insens<'i, T> {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        mut input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        let start = input;
        if input.match_insensitive(Self::CONTENT) {
            let span = start.span(input);
            Some((input, Self::from(span.as_str())))
        } else {
            None
        }
    }

    #[inline]
    fn try_check_partial_with<I: Input<'i>>(
        mut input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
        if input.match_insensitive(Self::CONTENT) {
            Some(input)
        } else {
            None
        }
    }
}

/// Skips until one of the given strings.
#[derive_where(Clone, Hash, PartialEq, Eq)]
#[derive(Dbg)]
pub struct Skip<'i, Strings: StringArrayWrapper> {
    /// Skipped span.
    pub span: Span<'i>,
    #[debug(skip)]
    _phantom: PhantomData<&'i Strings>,
}
impl<Strings: StringArrayWrapper> StringArrayWrapper for Skip<'_, Strings> {
    const CONTENT: &'static [&'static str] = Strings::CONTENT;
}
impl<'i, Strings: StringArrayWrapper> From<Span<'i>> for Skip<'i, Strings> {
    fn from(span: Span<'i>) -> Self {
        Self {
            span,
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, Strings: StringArrayWrapper> TypedNode<'i, R> for Skip<'i, Strings> {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        mut input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        let start = input;
        match input.skip_until(Strings::CONTENT) {
            true => {
                let span = start.span(input);
                Some((input, Self::from(span)))
            }
            false => Some((input, Self::from(start.span(input)))), // return the original input if not found
        }
    }

    #[inline]
    fn try_check_partial_with<I: Input<'i>>(
        mut input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
        match input.skip_until(Strings::CONTENT) {
            true => Some(input),
            false => Some(input), // return the original input if not found
        }
    }
}

/// Skip `n` characters if there are.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SkipChar<'i, const N: usize> {
    /// Skipped span.
    pub span: Span<'i>,
}
impl<'i, const N: usize> From<Span<'i>> for SkipChar<'i, N> {
    fn from(span: Span<'i>) -> Self {
        Self { span }
    }
}
impl<'i, R: RuleType, const N: usize> TypedNode<'i, R> for SkipChar<'i, N> {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        mut input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        let start = input;
        match input.skip(N) {
            true => {
                let span = start.span(input);
                Some((input, Self::from(span)))
            }
            false => None,
        }
    }

    #[inline]
    fn try_check_partial_with<I: Input<'i>>(
        mut input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
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
impl<'i, R: RuleType, const MIN: char, const MAX: char> TypedNode<'i, R> for CharRange<MIN, MAX> {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        mut input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        let start = input;
        match input.match_range(MIN..MAX) {
            true => {
                let span = start.span(input);
                let content = span.as_str().chars().next().unwrap();
                Some((input, Self { content }))
            }
            false => None,
        }
    }

    #[inline]
    fn try_check_partial_with<I: Input<'i>>(
        mut input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
        match input.match_range(MIN..MAX) {
            true => Some(input),
            false => None,
        }
    }
}

/// Try to create stack slice.
#[inline]
fn stack_slice<'i, 's, I: Input<'i>, R: RuleType>(
    input: I,
    start: i32,
    end: Option<i32>,
    stack: &'s Stack<Span<'i>>,
    tracker: &mut Tracker<'i, R>,
) -> Option<core::slice::Iter<'s, Span<'i>>> {
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
fn peek_spans<'s, 'i: 's, I: Input<'i>, R: RuleType>(
    input: I,
    iter: impl Iterator<Item = &'s Span<'i>>,
    _tracker: &mut Tracker<'i, R>,
) -> Option<(I, Span<'i>)> {
    let mut matching_pos = input;
    for span in iter {
        match matching_pos.match_string(span.as_str()) {
            true => (),
            false => {
                return None;
            }
        }
    }
    Some((matching_pos, input.span(matching_pos)))
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
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
impl<N> DerefMut for Positive<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}
impl<'i, R: RuleType, N: TypedNode<'i, R>> TypedNode<'i, R> for Positive<N> {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        tracker.positive_during(|tracker| {
            stack.snapshot();
            match N::try_parse_partial_with(input, stack, tracker) {
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
    fn try_check_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
        tracker.positive_during(|tracker| {
            stack.snapshot();
            match N::try_check_partial_with(input, stack, tracker) {
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
#[derive(Clone, Dbg, Hash, PartialEq, Eq)]
pub struct Negative<T> {
    #[debug(skip)]
    _phantom: PhantomData<T>,
}
impl<T> From<()> for Negative<T> {
    fn from(_value: ()) -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, T: TypedNode<'i, R>> TypedNode<'i, R> for Negative<T> {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        tracker.negative_during(|tracker| {
            stack.snapshot();
            match T::try_check_partial_with(input, stack, tracker) {
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
    fn try_check_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
        tracker.negative_during(|tracker| {
            stack.snapshot();
            match T::try_check_partial_with(input, stack, tracker) {
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

/// Match any character.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ANY {
    /// Matched character.
    pub content: char,
}
impl<'i, R: RuleType> TypedNode<'i, R> for ANY {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        mut input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        input.next().map(|c| (input, Self { content: c }))
    }

    #[inline]
    fn try_check_partial_with<I: Input<'i>>(
        mut input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
        input.next().map(|_| input)
    }
}

/// Match the start of input.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SOI;
impl<'i, R: RuleType> TypedNode<'i, R> for SOI {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        if input.at_start() {
            Some((input, Self))
        } else {
            None
        }
    }

    #[inline]
    fn try_check_partial_with<I: Input<'i>>(
        input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
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
impl<'i, R: RuleType> TypedNode<'i, R> for EOI {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        if input.at_end() {
            Some((input, Self))
        } else {
            None
        }
    }

    #[inline]
    fn try_check_partial_with<I: Input<'i>>(
        input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
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
impl<'i, R: RuleType> TypedNode<'i, R> for NEWLINE {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        mut input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
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
    fn try_check_partial_with<I: Input<'i>>(
        mut input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
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
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct PEEK_ALL<'i> {
    /// Pair span.
    pub span: Span<'i>,
}
impl<'i> From<Span<'i>> for PEEK_ALL<'i> {
    fn from(span: Span<'i>) -> Self {
        Self { span }
    }
}
impl<'i, R: RuleType> TypedNode<'i, R> for PEEK_ALL<'i> {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        let spans = stack[0..stack.len()].iter().rev();
        let (input, span) = peek_spans::<I, R>(input, spans, tracker)?;
        Some((input, Self::from(span)))
    }

    #[inline]
    fn try_check_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
        let spans = stack[0..stack.len()].iter().rev();
        let (input, _) = peek_spans::<I, R>(input, spans, tracker)?;
        Some(input)
    }
}

/// Peek top span in stack.
/// Will consume input.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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
    fn try_parse_partial_with<I: Input<'i>>(
        mut input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        let start = input;
        match stack.peek() {
            Some(string) => match input.match_string(string.as_str()) {
                true => Some((input, Self::from(start.span(input)))),
                false => None,
            },
            None => {
                tracker.empty_stack(input);
                None
            }
        }
    }

    #[inline]
    fn try_check_partial_with<I: Input<'i>>(
        mut input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
        match stack.peek() {
            Some(string) => match input.match_string(string.as_str()) {
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
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
impl<'i, R: RuleType> TypedNode<'i, R> for DROP {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        match stack.pop() {
            Some(_) => Some((input, Self)),
            None => {
                tracker.empty_stack(input);
                None
            }
        }
    }

    #[inline]
    fn try_check_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
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
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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
    fn try_parse_partial_with<I: Input<'i>>(
        mut input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        match stack.pop() {
            Some(span) => match input.match_string(span.as_str()) {
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
    fn try_check_partial_with<I: Input<'i>>(
        mut input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
        match stack.pop() {
            Some(span) => match input.match_string(span.as_str()) {
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
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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
    fn try_parse_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        let (input, res) = PEEK_ALL::try_parse_partial_with(input, stack, tracker)?;
        while stack.pop().is_some() {}
        Some((input, Self::from(res.span)))
    }

    #[inline]
    fn try_check_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
        let input = PEEK_ALL::try_check_partial_with(input, stack, tracker)?;
        while stack.pop().is_some() {}
        Some(input)
    }
}

/// Always fail.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Dbg, Hash, PartialEq, Eq)]
pub struct AlwaysFail<'i>(#[debug(skip)] PhantomData<&'i char>);
impl Default for AlwaysFail<'_> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<'i, R: RuleType> TypedNode<'i, R> for AlwaysFail<'i> {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        _input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        None
    }

    #[inline]
    fn try_check_partial_with<I: Input<'i>>(
        _input: I,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
        None
    }
}

/// Empty.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Dbg, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Empty<'i>(#[debug(skip)] PhantomData<&'i char>);
impl Default for Empty<'_> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<'i, R: RuleType> NeverFailedTypedNode<'i, R> for Empty<'i> {
    #[inline]
    fn parse_with<I: Input<'i>>(input: I, _stack: &mut Stack<Span<'i>>) -> (I, Self) {
        (input, Self::default())
    }

    fn check_with<I: Input<'i>>(input: I, _stack: &mut Stack<Span<'i>>) -> I {
        input
    }
}
impl<'i, R: RuleType> TypedNode<'i, R> for Empty<'i> {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        Some(<Self as NeverFailedTypedNode<'i, R>>::parse_with(
            input, stack,
        ))
    }

    #[inline]
    fn try_check_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
        Some(<Self as NeverFailedTypedNode<'i, R>>::check_with(
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
impl<'i, R: RuleType, T: TypedNode<'i, R>> TypedNode<'i, R> for Push<T> {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        let start = input;
        let (input, content) = T::try_parse_partial_with(input, stack, tracker)?;
        stack.push(start.span(input));
        Some((input, Self::from(content)))
    }

    #[inline]
    fn try_check_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
        let start = input;
        let input = T::try_check_partial_with(input, stack, tracker)?;
        stack.push(start.span(input));
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

/// Match `[START..END]` in top-to-bottom order of the stack.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PeekSlice2<const START: i32, const END: i32>;
impl<'i, R: RuleType, const START: i32, const END: i32> TypedNode<'i, R>
    for PeekSlice2<START, END>
{
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        let spans = stack_slice(input, START, Some(END), stack, tracker)?;
        let (input, _) = peek_spans::<I, R>(input, spans, tracker)?;
        Some((input, Self))
    }

    #[inline]
    fn try_check_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
        let spans = stack_slice(input, START, Some(END), stack, tracker)?;
        let (input, _) = peek_spans::<I, R>(input, spans, tracker)?;
        Some(input)
    }
}

/// Match `[START..]` in top-to-bottom order of the stack.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PeekSlice1<const START: i32>;
impl<'i, R: RuleType, const START: i32> TypedNode<'i, R> for PeekSlice1<START> {
    #[inline]
    fn try_parse_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<(I, Self)> {
        let spans = stack_slice(input, START, None, stack, tracker)?;
        let (input, _) = peek_spans::<I, R>(input, spans, tracker)?;
        Some((input, Self))
    }

    #[inline]
    fn try_check_partial_with<I: Input<'i>>(
        input: I,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Option<I> {
        let spans = stack_slice(input, START, None, stack, tracker)?;
        let (input, _) = peek_spans::<I, R>(input, spans, tracker)?;
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

use crate::choices::{Choice2, Choice3};
use crate::Input;
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
pub fn match_char_by<'i>(
    position: &mut impl Input<'i>,
    pred: impl FnOnce(char) -> bool,
) -> Option<char> {
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
pub fn restore_on_none<'i, T>(
    stack: &mut Stack<Span<'i>>,
    f: impl FnOnce(&mut Stack<Span<'i>>) -> Option<T>,
) -> Option<T> {
    stack.snapshot();
    let res = f(stack);
    match res.as_ref() {
        Some(_) => stack.clear_snapshot(),
        None => stack.restore(),
    }
    res
}
