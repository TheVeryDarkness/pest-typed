// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use super::choices::{Choice2, Choice3};
use super::terminal::CharRange;
use crate::{parser_state::constrain_idxs, position::Position, stack::Stack};
use crate::{span::Span, tracker::Tracker, RuleType, TypedNode};
use core::ops::DerefMut;
use core::{fmt::Debug, marker::PhantomData, ops::Deref};
use derive_debug::Dbg;

macro_rules! impl_with_content {
    ($name:ident, $type:ty) => {
        impl<'i> From<$type> for $name<'i> {
            fn from(value: $type) -> Self {
                Self {
                    content: value,
                    _phantom: PhantomData,
                }
            }
        }
        impl<'i> Deref for $name<'i> {
            type Target = $type;
            fn deref(&self) -> &Self::Target {
                &self.content
            }
        }
        impl<'i> DerefMut for $name<'i> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.content
            }
        }
        impl<'i> Take for $name<'i> {
            type Inner = $type;
            fn take(self) -> Self::Inner {
                self.content
            }
        }
    };
}
macro_rules! impl_with_span {
    ($name:ident) => {
        impl<'i> From<Span<'i>> for $name<'i> {
            fn from(span: Span<'i>) -> Self {
                Self { span }
            }
        }
        impl<'i> Deref for $name<'i> {
            type Target = Span<'i>;
            fn deref(&self) -> &Self::Target {
                &self.span
            }
        }
        impl<'i> DerefMut for $name<'i> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.span
            }
        }
        impl<'i> Take for $name<'i> {
            type Inner = Span<'i>;
            fn take(self) -> Self::Inner {
                self.span
            }
        }
    };
}
macro_rules! impl_generics_with_span {
    ($name:ident, ($($args:tt)*), ($($params:tt)*)) => {
        impl<$($args)*> From<Span<'i>> for $name<$($params)*> {
            fn from(span: Span<'i>) -> Self {
                Self { span }
            }
        }
        impl<$($args)*> Deref for $name<$($params)*> {
            type Target = Span<'i>;
            fn deref(&self) -> &Self::Target {
                &self.span
            }
        }
        impl<$($args)*> DerefMut for $name<$($params)*> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.span
            }
        }
        impl<$($args)*> Take for $name<$($params)*> {
            type Inner = Span<'i>;
            fn take(self) -> Self::Inner {
                self.span
            }
        }
    };
}

/// Match any character.
#[derive(Dbg, Clone, PartialEq)]
pub struct ANY<'i> {
    /// Matched character.
    pub content: char,
    #[dbg(skip)]
    _phantom: PhantomData<&'i str>,
}
impl<'i, R: RuleType> TypedNode<'i, R> for ANY<'i> {
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
            true => Ok((input, Self::from(c))),
            false => Err(()),
        }
    }
}
impl_with_content!(ANY, char);

/// Match the start of input.
#[derive(Dbg, Clone, PartialEq)]
pub struct SOI<'i> {
    #[dbg(skip)]
    pub content: (),
    #[dbg(skip)]
    _phantom: PhantomData<&'i str>,
}
impl<'i, R: RuleType> TypedNode<'i, R> for SOI<'i> {
    #[inline]
    fn try_parse_with<const _A: bool>(
        input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        if input.at_start() {
            Ok((input, Self::from(())))
        } else {
            Err(())
        }
    }
}
impl_with_content!(SOI, ());

/// Match the end of input.
///
/// [`EOI`] will record its rule if not matched.
#[derive(Dbg, Clone, PartialEq)]
pub struct EOI<'i> {
    #[dbg(skip)]
    pub content: (),
    #[dbg(skip)]
    _phantom: PhantomData<&'i str>,
}
impl<'i, R: RuleType> TypedNode<'i, R> for EOI<'i> {
    #[inline]
    fn try_parse_with<const _A: bool>(
        input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        if input.at_end() {
            Ok((input, Self::from(())))
        } else {
            Err(())
        }
    }
}
impl_with_content!(EOI, ());

/// Type of a new-line character.
#[derive(Debug, Clone, PartialEq)]
pub enum NewLineType {
    /// `\r\n`.
    CRLF,
    /// `\n`.
    CR,
    /// `\r`.
    LF,
}
/// Match a new-line character.
/// A built-in rule. Equivalent to `"\r\n" | "\n" | "\r"`.
#[derive(Dbg, Clone, PartialEq)]
pub struct NEWLINE<'i> {
    pub content: NewLineType,
    #[dbg(skip)]
    _phantom: PhantomData<&'i str>,
}
impl<'i, R: RuleType> TypedNode<'i, R> for NEWLINE<'i> {
    #[inline]
    fn try_parse_with<const _A: bool>(
        mut input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        if input.match_string("\r\n") {
            Ok((input, Self::from(NewLineType::CRLF)))
        } else if input.match_string("\n") {
            Ok((input, Self::from(NewLineType::CR)))
        } else if input.match_string("\r") {
            Ok((input, Self::from(NewLineType::LF)))
        } else {
            Err(())
        }
    }
}
impl_with_content!(NEWLINE, NewLineType);

/// Peek all spans in stack reversely.
/// Will consume input.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
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
impl_with_span!(PEEK_ALL);

/// Peek top span in stack.
/// Will consume input.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub struct PEEK<'i> {
    /// Pair span.
    pub span: Span<'i>,
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
impl_with_span!(PEEK);

/// Drop the top of the stack.
/// Fail if there is no span in the stack.
#[derive(Dbg, Clone, PartialEq)]
pub struct DROP<'i> {
    #[dbg(skip)]
    pub content: (),
    #[dbg(skip)]
    _phantom: PhantomData<&'i str>,
}
impl<'i, R: RuleType> TypedNode<'i, R> for DROP<'i> {
    #[inline]
    fn try_parse_with<const _A: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        match stack.pop() {
            Some(_) => Ok((input, Self::from(()))),
            None => {
                tracker.empty_stack(input);
                Err(())
            }
        }
    }
}
impl_with_content!(DROP, ());

/// Match and pop the top span of the stack.
#[derive(Debug, Clone, PartialEq)]
pub struct POP<'i> {
    /// Matched span.
    pub span: Span<'i>,
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
impl_with_span!(POP);

/// Match and pop all spans in the stack in top-to-bottom-order.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub struct POP_ALL<'i> {
    /// Matched span.
    pub span: Span<'i>,
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
impl_with_span!(POP_ALL);

/// Always fail.
#[derive(Dbg, Clone, PartialEq)]
pub struct AlwaysFail<'i> {
    #[dbg(skip)]
    pub content: (),
    #[dbg(skip)]
    _phantom: PhantomData<&'i ()>,
}
impl<'i, R: RuleType> TypedNode<'i, R> for AlwaysFail<'i> {
    #[inline]
    fn try_parse_with<const ATOMIC: bool>(
        _input: Position<'i>,
        _stack: &mut Stack<Span<'i>>,
        _tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        Err(())
    }
}
impl_with_content!(AlwaysFail, ());

/// Match `[START:END]` in top-to-bottom order of the stack.
#[derive(Debug, Clone, PartialEq)]
pub struct PeekSlice2<'i, const START: i32, const END: i32> {
    pub span: Span<'i>,
}
impl<'i, R: RuleType, const START: i32, const END: i32> TypedNode<'i, R>
    for PeekSlice2<'i, START, END>
{
    #[inline]
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let start = input;
        let spans = stack_slice(input, START, Some(END), stack, tracker)?;
        let (input, _) = peek_spans::<R>(input, spans, tracker)?;
        Ok((input, Self::from(start.span(&input))))
    }
}
impl_generics_with_span!(PeekSlice2, ('i, const START: i32, const END: i32), ('i, START, END));

/// Match `[START:END]` in top-to-bottom order of the stack.
#[derive(Debug, Clone, PartialEq)]
pub struct PeekSlice1<'i, const START: i32> {
    pub span: Span<'i>,
}
impl<'i, R: RuleType, const START: i32> TypedNode<'i, R> for PeekSlice1<'i, START> {
    #[inline]
    fn try_parse_with<const ATOMIC: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let start = input;
        let spans = stack_slice(input, START, None, stack, tracker)?;
        let (input, _) = peek_spans::<R>(input, spans, tracker)?;
        Ok((input, Self::from(start.span(&input))))
    }
}
impl_generics_with_span!(PeekSlice1, ('i, const START: i32), ('i, START));

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
pub type ASCII_HEX_DIGIT<'i, R> =
    Choice3<'i, R, ASCII_DIGIT<'i, R>, CharRange<'i, R, 'a', 'f'>, CharRange<'i, R, 'A', 'F'>>;

/// Lower case ASCII alphabet.
#[allow(non_camel_case_types)]
pub type ASCII_ALPHA_LOWER<'i, R> = CharRange<'i, R, 'a', 'z'>;

/// Upper case ASCII alphabet.
#[allow(non_camel_case_types)]
pub type ASCII_ALPHA_UPPER<'i, R> = CharRange<'i, R, 'A', 'Z'>;

/// ASCII alphabet.
#[allow(non_camel_case_types)]
pub type ASCII_ALPHA<'i, R> = Choice2<'i, R, ASCII_ALPHA_LOWER<'i, R>, ASCII_ALPHA_UPPER<'i, R>>;

/// ASCII alphabet or digit.
#[allow(non_camel_case_types)]
pub type ASCII_ALPHANUMERIC<'i, R> = Choice2<'i, R, ASCII_ALPHA<'i, R>, ASCII_DIGIT<'i, R>>;

/// ASCII alphabet.
#[allow(non_camel_case_types)]
pub type ASCII<'i, R> = CharRange<'i, R, '\x00', '\x7f'>;
