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

use super::{parser_state::constrain_idxs, position::Position, stack::Stack};
use super::{
    span::Span,
    tracker::Tracker,
    typed_node::NeverFailedTypedNode,
    wrapper::{StringArrayWrapper, StringWrapper},
    RuleType, TypedNode,
};
use crate::wrapper::BoundWrapper;
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};
use core::{fmt::Debug, marker::PhantomData};
use custom_debug_derive::Debug as Dbg;

/// Match given string case sensitively.
///
/// The `CONTENT` on the type (by [`StringWrapper`]) is the original string to match.
///
/// See [`Insens`] for case-insensitive matching.
#[derive(Clone, Dbg, PartialEq)]
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
impl<'i, R: RuleType, T: StringWrapper> TypedNode<'i, R> for Str<T> {
    fn try_parse_with<const _A: bool>(
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

/// Match given string case insensitively.
///
/// - The field `content` is the matched string.
/// - The `CONTENT` on the type (by [`StringWrapper`]) is the original string to match, and it may differ from `content` in case.
///   
///   For example, A `^"x"` may match `"X"`, and in the parsing result, `self.content` is `"X"`, while `Self::CONTENT` is still `"x"`.    
///
/// See [`Str`] for case-sensitive matching.
#[derive(Clone, Dbg, PartialEq)]
pub struct Insens<'i, T: StringWrapper> {
    /// Matched content.
    pub content: &'i str,
    #[debug(skip)]
    _phantom: PhantomData<&'i T>,
}
impl<'i, T: StringWrapper> StringWrapper for Insens<'i, T> {
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
    fn try_parse_with<const _A: bool>(
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

/// Skips until one of the given strings.
#[derive(Clone, Dbg, PartialEq)]
pub struct Skip<'i, Strings: StringArrayWrapper> {
    /// Skipped span.
    pub span: Span<'i>,
    #[debug(skip)]
    _phantom: PhantomData<&'i Strings>,
}
impl<'i, Strings: StringArrayWrapper> StringArrayWrapper for Skip<'i, Strings> {
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
    fn try_parse_with<const _A: bool>(
        mut input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let start = input;
        match input.skip_until(Strings::CONTENT) {
            true => {
                let span = start.span(&input);
                Ok((input, Self::from(span)))
            }
            false => Err(()),
        }
    }
}

/// Skip `n` characters if there are.
#[derive(Clone, Debug, PartialEq)]
pub struct SkipChar<'i, const N: usize> {
    /// Skipped span.
    pub span: Span<'i>,
}
impl<'i, R: RuleType, const N: usize> TypedNode<'i, R> for SkipChar<'i, N> {
    fn try_parse_with<const ATOMIC: bool>(
        mut input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let start = input;
        match input.skip(N) {
            true => {
                let span = start.span(&input);
                Ok((input, Self { span }))
            }
            false => Err(()),
        }
    }
}

/// Match a character in the range `[MIN, MAX]`.
/// Inclusively both below and above.
#[derive(Clone, Debug, PartialEq)]
pub struct CharRange<const MIN: char, const MAX: char> {
    /// Matched character.
    pub content: char,
}
impl<'i, R: RuleType, const MIN: char, const MAX: char> TypedNode<'i, R> for CharRange<MIN, MAX> {
    fn try_parse_with<const _A: bool>(
        mut input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let start = input;
        match input.match_range(MIN..MAX) {
            true => {
                let span = start.span(&input);
                let content = span.as_str().chars().next().unwrap();
                Ok((input, Self { content }))
            }
            false => Err(()),
        }
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
fn peek_spans<'s, 'i: 's, R: RuleType>(
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
///
/// Peeked expressions will not occur in Pair/Pairs API.
#[derive(Clone, Debug, PartialEq)]
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
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        tracker.positive_during(|tracker| {
            stack.snapshot();
            match N::try_parse_with::<ATOMIC>(input, stack, tracker) {
                Ok((_, content)) => {
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

/// Negative predicate.
///
/// Will not contain anything.
#[derive(Clone, Dbg, PartialEq)]
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

/// Match any character.
#[derive(Clone, Debug, PartialEq)]
pub struct ANY {
    /// Matched character.
    pub content: char,
}
impl<'i, R: RuleType> TypedNode<'i, R> for ANY {
    #[inline]
    fn try_parse_with<const _A: bool>(
        mut input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let mut c: char = ' ';
        match input.match_char_by(|ch| {
            c = ch;
            true
        }) {
            true => Ok((input, Self { content: c })),
            false => Err(()),
        }
    }
}

/// Match the start of input.
#[derive(Clone, Debug, PartialEq)]
pub struct SOI;
impl<'i, R: RuleType> TypedNode<'i, R> for SOI {
    #[inline]
    fn try_parse_with<const _A: bool>(
        input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        if input.at_start() {
            Ok((input, Self))
        } else {
            Err(())
        }
    }
}

/// Match the end of input.
///
/// [`EOI`] will record its rule if not matched.
#[derive(Clone, Debug, PartialEq)]
pub struct EOI;
impl<'i, R: RuleType> TypedNode<'i, R> for EOI {
    #[inline]
    fn try_parse_with<const _A: bool>(
        input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        if input.at_end() {
            Ok((input, Self))
        } else {
            Err(())
        }
    }
}

/// Type of eol.
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
pub struct NEWLINE {
    /// Type of matched character.
    pub content: NewLineType,
}
impl<'i, R: RuleType> TypedNode<'i, R> for NEWLINE {
    #[inline]
    fn try_parse_with<const _A: bool>(
        mut input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let (input, t) = if input.match_string("\r\n") {
            (input, NewLineType::CRLF)
        } else if input.match_string("\n") {
            (input, NewLineType::LF)
        } else if input.match_string("\r") {
            (input, NewLineType::CR)
        } else {
            return Err(());
        };
        Ok((input, Self { content: t }))
    }
}

/// Peek all spans in stack reversely.
/// Will consume input.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub struct PEEK_ALL<'i> {
    /// Pair span.
    pub span: Span<'i>,
}
impl<'i, R: RuleType> TypedNode<'i, R> for PEEK_ALL<'i> {
    #[inline]
    fn try_parse_with<const _A: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let spans = stack[0..stack.len()].iter().rev();
        let (input, span) = peek_spans::<R>(input, spans, tracker)?;
        Ok((input, Self { span }))
    }
}

/// Peek top span in stack.
/// Will consume input.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
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
    fn try_parse_with<const _A: bool>(
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

/// Skip single whitespace or comment.
#[derive(Clone, Debug, PartialEq)]
pub enum SkippedUnit<COMMENT: Clone + Debug + PartialEq, WHITESPACE: Clone + Debug + PartialEq> {
    /// Comment.
    Comment(COMMENT),
    /// White space.
    WhiteSpace(WHITESPACE),
}

/// Skip comments (by rule `COMMENT`) or white spaces (by rule `WHITESPACE`) if there is any.
///
/// Never fail.
#[derive(Clone, Debug, PartialEq)]
pub struct Skipped<WHITESPACE, COMMENT> {
    /// Skipped comments and white spaces.
    pub content: Vec<Choice2<WHITESPACE, COMMENT>>,
}
impl<WHITESPACE, COMMENT> Default for Skipped<WHITESPACE, COMMENT> {
    fn default() -> Self {
        Self {
            content: Vec::new(),
        }
    }
}
impl<'i, R: RuleType, WHITESPACE: TypedNode<'i, R>, COMMENT: TypedNode<'i, R>>
    NeverFailedTypedNode<'i, R> for Skipped<WHITESPACE, COMMENT>
{
    #[inline]
    fn parse_with<const ATOMIC: bool>(
        mut input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
    ) -> (Position<'i>, Self) {
        if ATOMIC {
            return (input, Self::default());
        }
        let mut flag = true;
        let mut vec = Vec::new();
        let mut tracker = Tracker::new(input);
        while flag {
            flag = false;
            while let Ok((remained, ws)) =
                WHITESPACE::try_parse_with::<true>(input, stack, &mut tracker)
            {
                vec.push(Choice2::_0(ws));
                input = remained;
                flag = true;
            }
            while let Ok((remained, c)) =
                COMMENT::try_parse_with::<true>(input, stack, &mut tracker)
            {
                vec.push(Choice2::_1(c));
                input = remained;
                flag = true;
            }
        }
        (input, Self { content: vec })
    }
}
impl<'i, R: RuleType, COMMENT: TypedNode<'i, R>, WHITESPACE: TypedNode<'i, R>> TypedNode<'i, R>
    for Skipped<COMMENT, WHITESPACE>
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

/// Repeatably match `T` at least `MIN` times.
#[derive(Clone, Debug, PartialEq)]
pub struct RepMin<T, IGNORED, const MIN: usize> {
    /// Skipped and Matched expressions.
    pub content: Vec<(IGNORED, T)>,
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
        const MIN: usize,
    > TypedNode<'i, R> for RepMin<T, IGNORED, MIN>
{
    #[inline]
    fn try_parse_with<const ATOMIC: bool>(
        mut input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let mut vec = Vec::<(IGNORED, T)>::new();

        {
            for i in 0.. {
                let (next, ignored) = IGNORED::parse_with::<ATOMIC>(input, stack);
                input = next;

                match restore_on_err(stack, |stack| {
                    T::try_parse_with::<ATOMIC>(input, stack, tracker)
                }) {
                    Ok((next, matched)) => {
                        input = next;
                        vec.push((ignored, matched));
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
impl<T, IGNORED, const MIN: usize> RepMin<T, IGNORED, MIN> {
    /// Returns an iterator over all matched expressions.
    pub fn iter_matched<'n>(
        &'n self,
    ) -> core::iter::Map<alloc::slice::Iter<'n, (IGNORED, T)>, fn(&'n (IGNORED, T)) -> &'n T> {
        self.content.iter().map(|(_, e)| e)
    }
    /// Returns an iterator over all skipped or matched expressions.
    pub fn iter_all<'n>(&'n self) -> alloc::slice::Iter<'n, (IGNORED, T)> {
        self.content.iter()
    }
}
impl<T: Clone + PartialEq, IGNORED: Clone + PartialEq, const MIN: usize> BoundWrapper
    for RepMin<T, IGNORED, MIN>
{
    const MIN: usize = MIN;
    const MAX: usize = usize::MAX;
}

/// Repeatably match `T` at least `MIN` times and at most `MAX` times.
#[derive(Clone, Debug, PartialEq)]
pub struct RepMinMax<T, IGNORED, const MIN: usize, const MAX: usize> {
    /// Skipped and Matched expressions.
    pub content: Vec<(IGNORED, T)>,
}
impl<
        'i,
        R: RuleType,
        T: TypedNode<'i, R>,
        IGNORED: NeverFailedTypedNode<'i, R>,
        const MIN: usize,
        const MAX: usize,
    > TypedNode<'i, R> for RepMinMax<T, IGNORED, MIN, MAX>
{
    #[inline]
    fn try_parse_with<const ATOMIC: bool>(
        mut input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let mut vec = Vec::<(IGNORED, T)>::new();

        {
            for i in 0..MAX {
                let (next, ignored) = IGNORED::parse_with::<ATOMIC>(input, stack);
                input = next;

                match restore_on_err(stack, |stack| {
                    T::try_parse_with::<ATOMIC>(input, stack, tracker)
                }) {
                    Ok((next, matched)) => {
                        input = next;
                        vec.push((ignored, matched));
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
impl<T, IGNORED, const MIN: usize, const MAX: usize> RepMinMax<T, IGNORED, MIN, MAX> {
    /// Returns an iterator over all matched expressions.
    pub fn iter_matched<'n>(
        &'n self,
    ) -> core::iter::Map<alloc::slice::Iter<'n, (IGNORED, T)>, fn(&'n (IGNORED, T)) -> &'n T> {
        self.content.iter().map(|(_, e)| e)
    }
    /// Returns an iterator over all skipped or matched expressions.
    pub fn iter_all<'n>(&'n self) -> alloc::slice::Iter<'n, (IGNORED, T)> {
        self.content.iter()
    }
}
impl<T: Clone + PartialEq, IGNORED: Clone + PartialEq, const MIN: usize, const MAX: usize>
    BoundWrapper for RepMinMax<T, IGNORED, MIN, MAX>
{
    const MIN: usize = MIN;
    const MAX: usize = MAX;
}

/// Repeat arbitrary times.
pub type Rep<T, IGNORED> = RepMin<T, IGNORED, 0>;
/// Repeat at least one times.
pub type RepOnce<T, IGNORED> = RepMin<T, IGNORED, 1>;

/// Drop the top of the stack.
///
/// Fail if there is no span in the stack.
#[derive(Clone, Debug, PartialEq)]
pub struct DROP;
impl<'i, R: RuleType> TypedNode<'i, R> for DROP {
    #[inline]
    fn try_parse_with<const _A: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        match stack.pop() {
            Some(_) => Ok((input, Self)),
            None => {
                tracker.empty_stack(input);
                Err(())
            }
        }
    }
}

/// Match and pop the top span of the stack.
#[derive(Clone, Debug, PartialEq)]
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
    fn try_parse_with<const _A: bool>(
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

/// Match and pop all spans in the stack in top-to-bottom-order.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
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
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let (input, res) = PEEK_ALL::try_parse_with::<ATOMIC>(input, stack, tracker)?;
        while stack.pop().is_some() {}
        Ok((input, Self::from(res.span)))
    }
}

/// Always fail.
#[derive(Clone, Dbg, PartialEq)]
pub struct AlwaysFail<'i>(#[debug(skip)] PhantomData<&'i char>);
impl<'i> Default for AlwaysFail<'i> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
/// A trait that only [AlwaysFail] implements.
pub trait AlwaysFailed: Debug + Default + Clone + PartialEq {}
impl<'i> AlwaysFailed for AlwaysFail<'i> {}
impl<'i, R: RuleType, T: AlwaysFailed> TypedNode<'i, R> for T {
    #[inline]
    fn try_parse_with<const ATOMIC: bool>(
        _input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        Err(())
    }
}

/// Match an expression and push it to the [Stack].
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
pub struct PeekSlice2<const START: i32, const END: i32>;
impl<'i, R: RuleType, const START: i32, const END: i32> TypedNode<'i, R>
    for PeekSlice2<START, END>
{
    #[inline]
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let spans = stack_slice(input, START, Some(END), stack, tracker)?;
        let (input, _) = peek_spans::<R>(input, spans, tracker)?;
        Ok((input, Self))
    }
}

/// Match `[START..]` in top-to-bottom order of the stack.
#[derive(Clone, Debug, PartialEq)]
pub struct PeekSlice1<const START: i32>;
impl<'i, R: RuleType, const START: i32> TypedNode<'i, R> for PeekSlice1<START> {
    #[inline]
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let spans = stack_slice(input, START, None, stack, tracker)?;
        let (input, _) = peek_spans::<R>(input, spans, tracker)?;
        Ok((input, Self))
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

/// Restore on error.
pub fn restore_on_err<'i, T, E>(
    stack: &mut Stack<Span<'i>>,
    f: impl FnOnce(&mut Stack<Span<'i>>) -> Result<T, E>,
) -> Result<T, E> {
    stack.snapshot();
    let res = f(stack);
    match res {
        Ok(_) => stack.clear_snapshot(),
        Err(_) => stack.restore(),
    }
    res
}
