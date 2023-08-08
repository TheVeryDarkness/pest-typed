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
use super::{Pair, Pairs};
use crate::typed_node::Take;
use crate::{parser_state::constrain_idxs, position::Position, stack::Stack};
use crate::{span::Span, tracker::Tracker, RuleType, TypedNode};
use alloc::boxed::Box;
use core::iter::{empty, Empty};
use core::{fmt::Debug, marker::PhantomData, ops::Deref};

macro_rules! impl_debug_with_content_if {
    ($name:ident, ()) => {
        impl<'i> Debug for $name<'i> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($name)).finish()
            }
        }
    };
    ($name:ident, $type:ty) => {
        impl<'i> Debug for $name<'i> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("content", &self.content)
                    .finish()
            }
        }
    };
}
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
        impl<'i> Take for $name<'i> {
            type Taken = $type;
            fn take(self) -> Self::Taken {
                self.content
            }
        }
        impl_debug_with_content_if!($name, $type);
        impl<'i: 'n, 'n, R: RuleType + 'n> Pairs<'i, 'n, R> for $name<'i> {
            type Iter = Empty<&'n (dyn Pair<'i, 'n, R>)>;
            type IntoIter = Empty<Box<dyn Pair<'i, 'n, R> + 'n>>;

            fn iter(&'n self) -> Self::Iter {
                empty()
            }
            fn into_iter(self) -> Self::IntoIter {
                empty()
            }
        }
    };
}

macro_rules! impl_with_span {
    ($name:ident) => {
        impl<'i> From<&'i str> for $name<'i> {
            fn from(content: &'i str) -> Self {
                Self { content }
            }
        }
        impl<'i> Deref for $name<'i> {
            type Target = &'i str;
            fn deref(&self) -> &Self::Target {
                &self.content
            }
        }
        impl<'i> Take for $name<'i> {
            type Taken = &'i str;
            fn take(self) -> Self::Taken {
                self.content
            }
        }
        /*
        impl<'i> Debug for $name<'i> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("content", &self.content)
                    .finish()
            }
        }
        */
        impl<'i: 'n, 'n, R: RuleType + 'n> Pairs<'i, 'n, R> for $name<'i> {
            type Iter = Empty<&'n (dyn Pair<'i, 'n, R>)>;
            type IntoIter = Empty<Box<dyn Pair<'i, 'n, R> + 'n>>;

            fn iter(&'n self) -> Self::Iter {
                empty()
            }
            fn into_iter(self) -> Self::IntoIter {
                empty()
            }
        }
    };
}
macro_rules! impl_generics_with_span {
    ($name:ident, ($($args:tt)*), ($($params:tt)*)) => {
        impl<$($args)*> From<&'i str> for $name<$($params)*> {
            fn from(content: &'i str) -> Self {
                Self { content }
            }
        }
        impl<$($args)*> Deref for $name<$($params)*> {
            type Target = &'i str;
            fn deref(&self) -> &Self::Target {
                &self.content
            }
        }
        impl<$($args)*> Take for $name<$($params)*> {
            type Taken = &'i str;
            fn take(self) -> Self::Taken {
                self.content
            }
        }
        impl<'n, $($args)*, R: RuleType + 'n> Pairs<'i, 'n, R> for $name<$($params)*>
        where 'i: 'n {
            type Iter = Empty<&'n (dyn Pair<'i, 'n, R>)>;
            type IntoIter = Empty<Box<dyn Pair<'i, 'n, R> + 'n>>;

            fn iter(&'n self) -> Self::Iter {
                empty()
            }
            fn into_iter(self) -> Self::IntoIter {
                empty()
            }
        }
    };
}

/// Match any character.
#[derive(Clone, PartialEq)]
pub struct ANY<'i> {
    /// Matched character.
    content: char,
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
    type Inner = char;
    fn deref_once<'n>(node: &'n Self) -> &'n Self::Inner {
        &node.content
    }
}
impl_with_content!(ANY, char);

/// Match the start of input.
#[derive(Clone, PartialEq)]
pub struct SOI<'i> {
    content: (),
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
    type Inner = ();
    fn deref_once<'n>(node: &'n Self) -> &'n Self::Inner {
        &node.content
    }
}
impl_with_content!(SOI, ());

/// Match the end of input.
///
/// [`EOI`] will record its rule if not matched.
#[derive(Clone, PartialEq)]
pub struct EOI<'i> {
    content: (),
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
    type Inner = ();
    fn deref_once<'n>(node: &'n Self) -> &'n Self::Inner {
        &node.content
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
#[derive(Clone, PartialEq)]
pub struct NEWLINE<'i> {
    content: NewLineType,
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
    type Inner = NewLineType;
    fn deref_once<'n>(node: &'n Self) -> &'n Self::Inner {
        &node.content
    }
}
impl_with_content!(NEWLINE, NewLineType);

/// Peek all spans in stack reversely.
/// Will consume input.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub struct PEEK_ALL<'i> {
    /// Pair span.
    content: &'i str,
}
impl<'i, R: RuleType> TypedNode<'i, R> for PEEK_ALL<'i> {
    #[inline]
    fn try_parse_with<const _A: bool>(
        input: Position<'i>,
        stack: &mut Stack<Span<'i>>,
        tracker: &mut Tracker<'i, R>,
    ) -> Result<(Position<'i>, Self), ()> {
        let spans = stack[0..stack.len()].iter().rev();
        let (input, content) = peek_spans::<R>(input, spans, tracker)?;
        let content = content.as_str();
        Ok((input, Self { content }))
    }
    type Inner = &'i str;
    fn deref_once<'n>(node: &'n Self) -> &'n Self::Inner {
        &node.content
    }
}
impl_with_span!(PEEK_ALL);

/// Peek top span in stack.
/// Will consume input.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub struct PEEK<'i> {
    /// Pair span.
    content: &'i str,
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
                true => Ok((input, Self::from(start.span(&input).as_str()))),
                false => Err(()),
            },
            None => {
                tracker.empty_stack(input);
                Err(())
            }
        }
    }
    type Inner = &'i str;
    fn deref_once<'n>(node: &'n Self) -> &'n Self::Inner {
        &node.content
    }
}
impl_with_span!(PEEK);

/// Drop the top of the stack.
/// Fail if there is no span in the stack.
#[derive(Clone, PartialEq)]
pub struct DROP<'i> {
    content: (),
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
    type Inner = ();
    fn deref_once<'n>(node: &'n Self) -> &'n Self::Inner {
        &node.content
    }
}
impl_with_content!(DROP, ());

/// Match and pop the top span of the stack.
#[derive(Debug, Clone, PartialEq)]
pub struct POP<'i> {
    /// Matched span.
    content: &'i str,
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
                true => Ok((input, Self::from(span.as_str()))),
                false => Err(()),
            },
            None => {
                tracker.empty_stack(input);
                Err(())
            }
        }
    }
    type Inner = &'i str;
    fn deref_once<'n>(node: &'n Self) -> &'n Self::Inner {
        &node.content
    }
}
impl_with_span!(POP);

/// Match and pop all spans in the stack in top-to-bottom-order.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub struct POP_ALL<'i> {
    /// Matched span.
    content: &'i str,
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
        Ok((input, Self::from(res.content)))
    }
    type Inner = &'i str;
    fn deref_once<'n>(node: &'n Self) -> &'n Self::Inner {
        &node.content
    }
}
impl_with_span!(POP_ALL);

/// Always fail.
#[derive(Clone, PartialEq)]
pub struct AlwaysFail<'i> {
    content: (),
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
    type Inner = ();
    fn deref_once<'n>(node: &'n Self) -> &'n Self::Inner {
        &node.content
    }
}
impl_with_content!(AlwaysFail, ());

/// Match `[START:END]` in top-to-bottom order of the stack.
#[derive(Debug, Clone, PartialEq)]
pub struct PeekSlice2<'i, const START: i32, const END: i32> {
    content: &'i str,
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
        Ok((input, Self::from(start.span(&input).as_str())))
    }
    type Inner = &'i str;
    fn deref_once<'n>(node: &'n Self) -> &'n Self::Inner {
        &node.content
    }
}
impl_generics_with_span!(PeekSlice2, ('i, const START: i32, const END: i32), ('i, START, END));

/// Match `[START:END]` in top-to-bottom order of the stack.
#[derive(Debug, Clone, PartialEq)]
pub struct PeekSlice1<'i, const START: i32> {
    content: &'i str,
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
        Ok((input, Self::from(start.span(&input).as_str())))
    }
    type Inner = &'i str;
    fn deref_once<'n>(node: &'n Self) -> &'n Self::Inner {
        &node.content
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
