// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::typed_node::Take;
use crate::{position::Position, stack::Stack};
use crate::{
    span::Span,
    tracker::Tracker,
    wrapper::{StringArrayWrapper, StringWrapper},
    TypedNode,
};
use core::ops::Deref;
use core::{fmt, fmt::Debug, marker::PhantomData};
use pest::RuleType;

trait FromSpan<'i>: Sized {
    fn from_span(span: Span<'i>) -> Self;
}
trait FromStr<'i>: Sized {
    fn from_str(span: &'i str) -> Self;
}
trait FromChar: Sized {
    fn from_char(content: char) -> Self;
}

/// Match given string case sensitively.
///
/// The `CONTENT` on the type (by [`StringWrapper`]) is the original string to match.
///
/// See [`Insens`] for case-insensitive matching.
#[derive(Clone, PartialEq)]
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
impl<'i, R: RuleType, T: StringWrapper> Deref for Str<'i, R, T> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        Self::CONTENT
    }
}
impl<'i, R: RuleType, T: StringWrapper> Take for Str<'i, R, T> {
    type Inner = &'static str;
    fn take(self) -> Self::Inner {
        Self::CONTENT
    }
}
impl<'i, R: RuleType, T: StringWrapper> Debug for Str<'i, R, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Str")
            .field("content", &Self::CONTENT)
            .finish()
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
    content: &'i str,
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
impl<'i, R: RuleType, T: StringWrapper> Deref for Insens<'i, R, T> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.content
    }
}
impl<'i, R: RuleType, T: StringWrapper> Take for Insens<'i, R, T> {
    type Inner = &'i str;
    fn take(self) -> Self::Inner {
        self.content
    }
}
impl<'i, R: RuleType, T: StringWrapper> Debug for Insens<'i, R, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Insens")
            .field("content", &self.content)
            .finish()
    }
}

/// Skips until one of the given `strings`
#[derive(Clone, PartialEq)]
pub struct Skip<'i, R: RuleType, Strings: StringArrayWrapper> {
    /// Skipped span.
    span: &'i str,
    _phantom: PhantomData<(&'i R, &'i Strings)>,
}
impl<'i, R: RuleType, Strings: StringArrayWrapper> FromSpan<'i> for Skip<'i, R, Strings> {
    fn from_span(span: Span<'i>) -> Self {
        Self {
            span: span.as_str(),
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, Strings: StringArrayWrapper> TypedNode<'i, R> for Skip<'i, R, Strings> {
    fn try_parse_with<const _A: bool>(
        mut input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let start = input;
        match input.skip_until(Strings::CONTENT) {
            true => {
                let span = start.span(&input);
                Ok((input, Self::from_span(span)))
            }
            false => Err(()),
        }
    }
}
impl<'i, R: RuleType, Strings: StringArrayWrapper> Deref for Skip<'i, R, Strings> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.span
    }
}
impl<'i, R: RuleType, Strings: StringArrayWrapper> Take for Skip<'i, R, Strings> {
    type Inner = &'i str;
    fn take(self) -> Self::Inner {
        self.span
    }
}
impl<'i, R: RuleType, Strings: StringArrayWrapper> Debug for Skip<'i, R, Strings> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Skip").field("span", &self.span).finish()
    }
}

/// Skip `n` characters if there are.
#[derive(Clone, PartialEq)]
pub struct SkipChar<'i, R: RuleType, const N: usize> {
    span: &'i str,
    _phantom: PhantomData<&'i R>,
}
impl<'i, R: RuleType, const N: usize> FromStr<'i> for SkipChar<'i, R, N> {
    fn from_str(span: &'i str) -> Self {
        Self {
            span,
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, const N: usize> TypedNode<'i, R> for SkipChar<'i, R, N> {
    fn try_parse_with<const ATOMIC: bool>(
        mut input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let start = input;
        match input.skip(N) {
            true => Ok((input, Self::from_str(start.span(&input).as_str()))),
            false => Err(()),
        }
    }
}
impl<'i, R: RuleType, const N: usize> Deref for SkipChar<'i, R, N> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.span
    }
}
impl<'i, R: RuleType, const N: usize> Take for SkipChar<'i, R, N> {
    type Inner = &'i str;
    fn take(self) -> Self::Inner {
        self.span
    }
}
impl<'i, R: RuleType, const N: usize> Debug for SkipChar<'i, R, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SkipChar")
            .field("span", &self.span)
            .finish()
    }
}

/// Match a character in the range `[MIN, MAX]`.
/// Inclusively both below and above.
#[derive(Clone, PartialEq)]
pub struct CharRange<'i, R: RuleType, const MIN: char, const MAX: char> {
    /// Matched character.
    content: char,
    _phantom: PhantomData<&'i R>,
}
impl<'i, R: RuleType, const MIN: char, const MAX: char> FromChar for CharRange<'i, R, MIN, MAX> {
    fn from_char(content: char) -> Self {
        Self {
            content,
            _phantom: PhantomData,
        }
    }
}
impl<'i, R: RuleType, const MIN: char, const MAX: char> TypedNode<'i, R>
    for CharRange<'i, R, MIN, MAX>
{
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
                Ok((input, Self::from_char(content)))
            }
            false => Err(()),
        }
    }
}
impl<'i, R: RuleType, const MIN: char, const MAX: char> Deref for CharRange<'i, R, MIN, MAX> {
    type Target = char;
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
impl<'i, R: RuleType, const MIN: char, const MAX: char> Take for CharRange<'i, R, MIN, MAX> {
    type Inner = char;
    fn take(self) -> Self::Inner {
        self.content
    }
}
impl<'i, R: RuleType, const MIN: char, const MAX: char> Debug for CharRange<'i, R, MIN, MAX> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CharRange")
            .field("content", &self.content)
            .finish()
    }
}
