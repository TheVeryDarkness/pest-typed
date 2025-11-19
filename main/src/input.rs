//! Abstract input and cursor types.
//!
//!! This module defines the [`Str`], [`Input`] and [`Cursor`] traits, which are used to abstract over
//! different input types for the parser.
//!
//! - [`Str`]: A trait for string-like types that can be used as input.
//! - [`Input`]: A trait for types that may have extra span information, such as [`Position`] and [`Span`].
//! - [`Cursor`]: A trait for types that can traverse the input, such as [`Position`], [`PositionCursor`] and [`SpanCursor`].
use crate::{Position, Span};
use alloc::string::String;
use core::{
    fmt,
    hash::Hash,
    ops::{Range, RangeBounds},
    ptr,
    str::Chars,
};

/// Cursor with span information.
///
/// # Safety
///
/// [`byte_offset()`](Cursor::byte_offset) must be in the range of [`input()`](Cursor::input).
pub unsafe trait Cursor: Sized + Clone {
    /// Underlying string type, such as `&str`, `&String`, or your own string type.
    type String: RefStr;

    /// Get byte offset.
    fn byte_offset(&self) -> usize;
    /// Get the full input.
    fn input(&self) -> Self::String;

    /// Get unconsumed string.
    fn get(&self) -> Self::String;
    // /// Get unconsumed characters.
    // fn chars(&self) -> Chars<'_> {
    //     self.get().chars()
    // }

    // /// Get line number and column number.
    // fn line_col(&self) -> (usize, usize);
    // /// Get current line.
    // fn line_of(&self) -> &'i str;

    /// To position.
    #[inline]
    fn as_position(&self) -> Position<Self::String> {
        unsafe { Position::new_unchecked(self.input(), self.byte_offset()) }
    }
    /// Create a [Span].
    #[inline]
    fn span(&self, end: &Self) -> Span<Self::String> {
        self.as_position().span(&end.as_position())
    }

    /// Match a string.
    #[inline]
    fn match_string(&mut self, string: &str) -> bool {
        let res = self.get().starts_with(string);
        if res {
            unsafe { *self.cursor() += string.len() };
        }
        res
    }
    /// Match an string insensitively.
    #[inline]
    fn match_insensitive(&mut self, string: &str) -> bool {
        let res = self.get().starts_with_insensitive(string);
        if res {
            unsafe { *self.cursor() += string.len() };
        }
        res
    }
    /// Skip until one of several strings.
    #[inline]
    fn skip_until(&mut self, strings: &[&str]) -> bool {
        for from in self.byte_offset()..self.end() {
            let Some(string) = self.input().get(from..) else {
                continue;
            };
            let bytes = string.as_str().as_bytes();

            for slice in strings.iter() {
                let to = slice.len();
                if Some(slice.as_bytes()) == bytes.get(0..to) {
                    unsafe { *self.cursor() = from };
                    return true;
                }
            }
        }

        unsafe { *self.cursor() = self.end() };
        false
    }
    /// Skip several characters.
    #[inline]
    fn skip(&mut self, n: usize) -> bool {
        let skipped = {
            let mut len = 0;
            let unconsumed = self.get();
            let mut chars = unconsumed.chars();
            for _ in 0..n {
                if let Some(c) = chars.next() {
                    len += c.len_utf8();
                } else {
                    return false;
                }
            }
            len
        };

        unsafe { *self.cursor() += skipped };
        true
    }
    /// Match a character in a range.
    #[inline]
    fn match_range(&mut self, range: Range<char>) -> bool {
        if let Some(c) = self.get().chars().next() {
            if range.start <= c && c <= range.end {
                unsafe { *self.cursor() += c.len_utf8() };
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    /// Match a character by a predicate.
    #[inline]
    fn match_char_by(&mut self, f: impl FnOnce(char) -> bool) -> bool {
        if let Some(c) = self.get().chars().next() {
            if f(c) {
                unsafe { *self.cursor() += c.len_utf8() };
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    /// Progress to next character.
    #[inline]
    fn advance_char(&mut self) -> Option<char> {
        let c = self.get().chars().next();
        if let Some(c) = c {
            unsafe { *self.cursor() += c.len_utf8() };
        }
        c
    }

    /// Get the cursor, which is the current byte offset.
    ///
    /// # Safety
    ///
    /// The cursor must be in the range of [`input()`](Input::input).
    unsafe fn cursor(&mut self) -> &mut usize;

    /// Get the start of the input.
    fn start(&self) -> usize;
    /// Get the end of the input.
    fn end(&self) -> usize;

    /// Check if is at the start of the input.
    #[inline]
    fn at_start(&self) -> bool {
        self.byte_offset() == self.start()
    }
    /// Check if is at the end of the input.
    #[inline]
    fn at_end(&self) -> bool {
        self.byte_offset() == self.end()
    }
}

unsafe impl<S: RefStr> Cursor for Position<S> {
    type String = S;

    #[inline(always)]
    fn byte_offset(&self) -> usize {
        self.pos
    }

    #[inline(always)]
    fn input(&self) -> S {
        self.input.clone()
    }

    #[inline]
    fn get(&self) -> S {
        unsafe { self.input().get_range_unchecked(self.pos..) }
    }

    #[inline]
    fn advance_char(&mut self) -> Option<char> {
        let c = self.get().chars().next();
        if c.is_some() {
            self.skip(1);
        }
        c
    }

    #[inline(always)]
    unsafe fn cursor(&mut self) -> &mut usize {
        &mut self.pos
    }

    #[inline(always)]
    fn start(&self) -> usize {
        0
    }
    #[inline(always)]
    fn end(&self) -> usize {
        self.input().len()
    }
}

/// A part of input.
///
/// A cursor that is used to traverse a [Position] input, A.K.A. a string slice starting from a certain [`Position`].
#[derive(Clone, Copy)]
pub struct PositionCursor<I> {
    input: I,
    start: usize,
    cursor: usize,
}

/// A part of input.
///
/// A cursor that is used to traverse a [Span] input, A.K.A. a string slice within a certain [`Span`].
#[derive(Clone, Copy)]
pub struct SpanCursor<I> {
    input: I,
    start: usize,
    end: usize,
    cursor: usize,
}

// impl<I> Input for SubInput1<I> {
//     type Cursor = Self;

//     fn as_cursor(&self) -> Self::Cursor {
//         self.clone()
//     }
// }

// impl<I> Input for SubInput2<I> {
//     type Cursor = Self;

//     fn as_cursor(&self) -> Self::Cursor {
//         self.clone()
//     }
// }

unsafe impl<S: RefStr> Cursor for PositionCursor<S> {
    type String = S;

    #[inline(always)]
    fn byte_offset(&self) -> usize {
        self.cursor
    }

    #[inline(always)]
    fn input(&self) -> S {
        self.input.clone()
    }

    #[inline]
    fn get(&self) -> S {
        unsafe { self.input.get_range_unchecked(self.cursor..) }
    }

    #[inline(always)]
    unsafe fn cursor(&mut self) -> &mut usize {
        &mut self.cursor
    }

    #[inline(always)]
    fn start(&self) -> usize {
        self.start
    }
    #[inline(always)]
    fn end(&self) -> usize {
        self.input().len()
    }
}

unsafe impl<S: RefStr> Cursor for SpanCursor<S> {
    type String = S;

    #[inline(always)]
    fn byte_offset(&self) -> usize {
        self.cursor
    }

    #[inline(always)]
    fn input(&self) -> S {
        self.input.clone()
    }

    #[inline]
    fn get(&self) -> S {
        unsafe { self.input.get_range_unchecked(self.cursor..self.end) }
    }

    #[inline(always)]
    unsafe fn cursor(&mut self) -> &mut usize {
        &mut self.cursor
    }

    #[inline(always)]
    fn start(&self) -> usize {
        self.start
    }
    #[inline(always)]
    fn end(&self) -> usize {
        self.end
    }
}

/// Parser input.
///
/// Should be implemented for types that can be used as parser input, such as `&str` and `&String`.
///
/// Must be [`Clone`], and should be cheap to clone.
pub trait Input: Clone {
    /// Cursor type.
    type Cursor: Cursor;
    /// String type.
    type String: RefStr;

    /// Convert to a [Input] type.
    fn as_cursor(&self) -> Self::Cursor;
}

/// A reference to a [`String`](alloc::string::String)-like type.
///
/// # Safety
///
/// This trait is `unsafe` because incorrect implementations may lead to memory safety issues.
pub unsafe trait RefStr: Clone + Hash + PartialEq + Eq + fmt::Debug {
    /// Create from a static string.
    fn from_static(s: &'static str) -> Self;
    /// Get length in bytes.
    fn len(&self) -> usize;
    /// Check if is empty.
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Convert to a string.
    fn as_str(&self) -> &str;
    /// Get a substring.
    ///
    /// # Safety
    ///
    /// The range must be in the bounds of the string and aligned to UTF-8 character boundaries.
    unsafe fn get_range_unchecked(&self, range: impl RangeBounds<usize>) -> Self;
    /// Get a substring.
    fn get(&self, range: impl RangeBounds<usize>) -> Option<Self>;
    /// Get a substring.
    fn get_checked(&self, range: impl RangeBounds<usize>) -> Self;
    /// Check if starts with a string.
    fn starts_with(&self, string: &str) -> bool;
    /// Check if starts with a string insensitively.
    fn starts_with_insensitive(&self, string: &str) -> bool;
    /// Get characters iterator.
    fn chars(&self) -> Chars<'_>;
    /// Check if two references point to the same string.
    fn ptr_eq(&self, other: &Self) -> bool;
    /// Hash the pointer of the string.
    fn ptr_hash<H: core::hash::Hasher>(&self, state: &mut H);
}

impl<S: RefStr> Input for S {
    type Cursor = Position<Self>;
    type String = Self;

    #[inline]
    fn as_cursor(&self) -> Self::Cursor {
        Position::from_start(self.clone())
    }
}

impl<'i> Input for &'i String {
    type Cursor = Position<&'i str>;
    type String = &'i str;

    #[inline]
    fn as_cursor(&self) -> Self::Cursor {
        Position::from_start(self)
    }
}

unsafe impl RefStr for &str {
    #[inline(always)]
    fn from_static(s: &'static str) -> Self {
        s
    }

    #[inline(always)]
    fn len(&self) -> usize {
        str::len(self)
    }

    #[inline(always)]
    fn as_str(&self) -> &str {
        self
    }

    #[inline(always)]
    unsafe fn get_range_unchecked(&self, range: impl RangeBounds<usize>) -> Self {
        match (range.start_bound(), range.end_bound()) {
            (core::ops::Bound::Included(&start), core::ops::Bound::Included(&end)) => {
                self.get_unchecked(start..=end)
            }
            (core::ops::Bound::Included(&start), core::ops::Bound::Excluded(&end)) => {
                self.get_unchecked(start..end)
            }
            (core::ops::Bound::Included(&start), core::ops::Bound::Unbounded) => {
                self.get_unchecked(start..)
            }
            (core::ops::Bound::Excluded(&start), core::ops::Bound::Included(&end)) => {
                self.get_unchecked(start + 1..=end)
            }
            (core::ops::Bound::Excluded(&start), core::ops::Bound::Excluded(&end)) => {
                self.get_unchecked(start + 1..end)
            }
            (core::ops::Bound::Excluded(&start), core::ops::Bound::Unbounded) => {
                self.get_unchecked(start + 1..)
            }
            (core::ops::Bound::Unbounded, core::ops::Bound::Included(&end)) => {
                self.get_unchecked(..=end)
            }
            (core::ops::Bound::Unbounded, core::ops::Bound::Excluded(&end)) => {
                self.get_unchecked(..end)
            }
            (core::ops::Bound::Unbounded, core::ops::Bound::Unbounded) => self,
        }
    }

    #[inline(always)]
    fn get(&self, range: impl RangeBounds<usize>) -> Option<Self> {
        match (range.start_bound(), range.end_bound()) {
            (core::ops::Bound::Included(&start), core::ops::Bound::Included(&end)) => {
                str::get(self, start..=end)
            }
            (core::ops::Bound::Included(&start), core::ops::Bound::Excluded(&end)) => {
                str::get(self, start..end)
            }
            (core::ops::Bound::Included(&start), core::ops::Bound::Unbounded) => {
                str::get(self, start..)
            }
            (core::ops::Bound::Excluded(&start), core::ops::Bound::Included(&end)) => {
                str::get(self, start + 1..=end)
            }
            (core::ops::Bound::Excluded(&start), core::ops::Bound::Excluded(&end)) => {
                str::get(self, start + 1..end)
            }
            (core::ops::Bound::Excluded(&start), core::ops::Bound::Unbounded) => {
                str::get(self, start + 1..)
            }
            (core::ops::Bound::Unbounded, core::ops::Bound::Included(&end)) => {
                str::get(self, ..=end)
            }
            (core::ops::Bound::Unbounded, core::ops::Bound::Excluded(&end)) => {
                str::get(self, ..end)
            }
            (core::ops::Bound::Unbounded, core::ops::Bound::Unbounded) => Some(self),
        }
    }

    #[inline(always)]
    fn get_checked(&self, range: impl RangeBounds<usize>) -> Self {
        match (range.start_bound(), range.end_bound()) {
            (core::ops::Bound::Included(&start), core::ops::Bound::Included(&end)) => {
                &self[start..=end]
            }
            (core::ops::Bound::Included(&start), core::ops::Bound::Excluded(&end)) => {
                &self[start..end]
            }
            (core::ops::Bound::Included(&start), core::ops::Bound::Unbounded) => &self[start..],
            (core::ops::Bound::Excluded(&start), core::ops::Bound::Included(&end)) => {
                &self[start + 1..=end]
            }
            (core::ops::Bound::Excluded(&start), core::ops::Bound::Excluded(&end)) => {
                &self[start + 1..end]
            }
            (core::ops::Bound::Excluded(&start), core::ops::Bound::Unbounded) => &self[start + 1..],
            (core::ops::Bound::Unbounded, core::ops::Bound::Included(&end)) => &self[..=end],
            (core::ops::Bound::Unbounded, core::ops::Bound::Excluded(&end)) => &self[..end],
            (core::ops::Bound::Unbounded, core::ops::Bound::Unbounded) => self,
        }
    }

    #[inline(always)]
    fn starts_with(&self, string: &str) -> bool {
        str::starts_with(self, string)
    }

    #[inline]
    fn starts_with_insensitive(&self, string: &str) -> bool {
        self.get(0..string.len())
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case(string))
    }

    #[inline(always)]
    fn chars(&self) -> Chars<'_> {
        str::chars(self)
    }

    #[inline(always)]
    fn ptr_eq(&self, other: &Self) -> bool {
        ptr::eq::<str>(*self, *other)
    }

    #[inline(always)]
    fn ptr_hash<H: core::hash::Hasher>(&self, state: &mut H) {
        ptr::hash::<str, H>(*self, state);
    }
}

#[cfg(feature = "shared-string")]
mod impl_shared_string {
    use super::*;
    use crate::RefStr;
    use shared_string::{RefCounter, SharedGenString};

    unsafe impl<R: RefCounter> RefStr for SharedGenString<R> {
        #[inline(always)]
        fn from_static(s: &'static str) -> Self {
            unsafe { Self::from_utf8_unchecked(s.as_bytes().to_vec()) }
        }

        #[inline(always)]
        fn len(&self) -> usize {
            str::len(self)
        }

        #[inline(always)]
        fn as_str(&self) -> &str {
            self
        }

        #[inline(always)]
        unsafe fn get_range_unchecked(&self, range: impl RangeBounds<usize>) -> Self {
            // `SharedGenString` does not expose an unchecked get method, so we use the checked one.
            self.idx(range)
        }

        #[inline(always)]
        fn get(&self, range: impl RangeBounds<usize>) -> Option<Self> {
            self.get(range)
        }

        #[inline(always)]
        fn get_checked(&self, range: impl RangeBounds<usize>) -> Self {
            self.get(range)
                .unwrap_or_else(|| panic!("Range out of bounds"))
        }

        #[inline(always)]
        fn starts_with(&self, string: &str) -> bool {
            str::starts_with(self, string)
        }

        #[inline]
        fn starts_with_insensitive(&self, string: &str) -> bool {
            self.get(0..string.len())
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case(string))
        }

        #[inline(always)]
        fn chars(&self) -> Chars<'_> {
            str::chars(self)
        }

        #[inline(always)]
        fn ptr_eq(&self, other: &Self) -> bool {
            ptr::eq::<str>(self.as_full_str(), other.as_full_str())
        }

        #[inline(always)]
        fn ptr_hash<H: core::hash::Hasher>(&self, state: &mut H) {
            ptr::hash::<str, H>(self.as_full_str(), state);
        }
    }
}

#[cfg(feature = "shared-vec")]
mod impl_shared_vec {
    use super::*;
    use crate::RefStr;
    use shared_vec::{Counter, String};
    use std::borrow::ToOwned;

    unsafe impl<C: Counter<usize>> RefStr for String<C> {
        #[inline(always)]
        fn from_static(s: &'static str) -> Self {
            Self::from_boxed_str(s.to_owned().into_boxed_str())
        }

        #[inline(always)]
        fn len(&self) -> usize {
            str::len(self)
        }

        #[inline(always)]
        fn as_str(&self) -> &str {
            self
        }

        #[inline(always)]
        unsafe fn get_range_unchecked(&self, range: impl RangeBounds<usize>) -> Self {
            // `SharedGenString` does not expose an unchecked get method, so we use the checked one.
            self.idx(range)
        }

        #[inline(always)]
        fn get(&self, range: impl RangeBounds<usize>) -> Option<Self> {
            self.get(range)
        }

        #[inline(always)]
        fn get_checked(&self, range: impl RangeBounds<usize>) -> Self {
            self.get(range)
                .unwrap_or_else(|| panic!("Range out of bounds"))
        }

        #[inline(always)]
        fn starts_with(&self, string: &str) -> bool {
            str::starts_with(self, string)
        }

        #[inline]
        fn starts_with_insensitive(&self, string: &str) -> bool {
            self.get(0..string.len())
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case(string))
        }

        #[inline(always)]
        fn chars(&self) -> Chars<'_> {
            str::chars(self)
        }

        #[inline(always)]
        fn ptr_eq(&self, other: &Self) -> bool {
            ptr::eq::<str>(self.as_original_str(), other.as_original_str())
        }

        #[inline(always)]
        fn ptr_hash<H: core::hash::Hasher>(&self, state: &mut H) {
            ptr::hash::<str, H>(self.as_original_str(), state);
        }
    }
}

impl<S: RefStr> Input for Position<S> {
    type Cursor = PositionCursor<S>;
    type String = S;

    #[inline]
    fn as_cursor(&self) -> Self::Cursor {
        let input = self.input.clone();
        let start = self.pos();
        let cursor = start;
        PositionCursor {
            input,
            start,
            cursor,
        }
    }
}

impl<S: RefStr> Input for Span<S> {
    type Cursor = SpanCursor<S>;
    type String = S;

    #[inline]
    fn as_cursor(&self) -> Self::Cursor {
        let input = self.get_input();
        let start = self.start();
        let end = self.end();
        let cursor = start;
        SpanCursor {
            input,
            start,
            end,
            cursor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_match_string() {
        let input = "hello, world!";
        let span = Span::new(input, 7, 12).unwrap();
        let mut cursor = span.as_cursor();

        assert_eq!(cursor.byte_offset(), 7);
        assert_eq!(cursor.get(), "world");

        assert!(!cursor.at_end());
        assert!(cursor.match_string("world"));
        assert_eq!(cursor.byte_offset(), 12);
        assert_eq!(cursor.get(), "");

        assert!(cursor.at_end());
    }

    #[test]
    fn span_match_string_insensitive() {
        let input = "hello, world!";
        let span = Span::new(input, 7, 12).unwrap();
        let mut cursor = span.as_cursor();

        assert_eq!(cursor.byte_offset(), 7);
        assert_eq!(cursor.get(), "world");

        assert!(!cursor.at_end());
        assert!(cursor.match_insensitive("WORLD"));
        assert_eq!(cursor.byte_offset(), 12);
        assert_eq!(cursor.get(), "");

        assert!(cursor.at_end());
    }

    #[test]
    fn span_skip_until() {
        let input = "abcde12345xyz";
        let span = Span::new_full(input);
        let mut cursor = span.as_cursor();

        assert_eq!(cursor.byte_offset(), 0);
        assert_eq!(cursor.get(), "abcde12345xyz");

        assert!(!cursor.at_end());
        assert!(cursor.skip_until(&["123", "xyz"]));
        assert_eq!(cursor.byte_offset(), 5);
        assert_eq!(cursor.get(), "12345xyz");

        assert!(!cursor.at_end());
        assert!(cursor.skip_until(&["xyz"]));
        assert_eq!(cursor.byte_offset(), 10);
        assert_eq!(cursor.get(), "xyz");

        assert!(!cursor.at_end());
        assert!(cursor.skip_until(&["notfound", "also_notfound", "xyz"]));
        assert_eq!(cursor.byte_offset(), 10);
        assert_eq!(cursor.get(), "xyz");

        assert!(!cursor.at_end());
        assert!(cursor.match_string("xyz"));
        assert_eq!(cursor.byte_offset(), 13);
        assert_eq!(cursor.get(), "");

        assert!(cursor.at_end());
    }

    #[test]
    fn position_match_string() {
        let input = "hello, world!";
        let span = Position::new(input, 7).unwrap();
        let mut cursor = span.as_cursor();

        assert_eq!(cursor.byte_offset(), 7);
        assert_eq!(cursor.get(), "world!");

        assert!(!cursor.at_end());
        assert!(cursor.match_string("world"));
        assert_eq!(cursor.byte_offset(), 12);
        assert_eq!(cursor.get(), "!");

        assert!(!cursor.at_end());
        cursor.match_string("!");
        assert_eq!(cursor.byte_offset(), 13);
        assert_eq!(cursor.get(), "");
        assert!(cursor.at_end());
    }

    #[test]
    fn position_match_string_insensitive() {
        let input = "hello, world!";
        let span = Position::new(input, 7).unwrap();
        let mut cursor = span.as_cursor();

        assert_eq!(cursor.byte_offset(), 7);
        assert_eq!(cursor.get(), "world!");

        assert!(!cursor.at_end());
        assert!(cursor.match_insensitive("WORLD"));
        assert_eq!(cursor.byte_offset(), 12);
        assert_eq!(cursor.get(), "!");

        assert!(!cursor.at_end());
        cursor.match_insensitive("!");
        assert_eq!(cursor.byte_offset(), 13);
        assert_eq!(cursor.get(), "");
        assert!(cursor.at_end());
    }

    #[test]
    fn position_skip_until() {
        let input = "abcde12345xyz";
        let span = Position::new(input, 0).unwrap();
        let mut cursor = span.as_cursor();

        assert_eq!(cursor.byte_offset(), 0);
        assert_eq!(cursor.get(), "abcde12345xyz");

        assert!(!cursor.at_end());
        assert!(cursor.skip_until(&["123", "xyz"]));
        assert_eq!(cursor.byte_offset(), 5);
        assert_eq!(cursor.get(), "12345xyz");

        assert!(!cursor.at_end());
        assert!(cursor.skip_until(&["xyz"]));
        assert_eq!(cursor.byte_offset(), 10);
        assert_eq!(cursor.get(), "xyz");

        assert!(!cursor.at_end());
        assert!(cursor.skip_until(&["notfound", "also_notfound", "xyz"]));
        assert_eq!(cursor.byte_offset(), 10);
        assert_eq!(cursor.get(), "xyz");

        assert!(!cursor.at_end());
        assert!(cursor.match_string("xyz"));
        assert_eq!(cursor.byte_offset(), 13);
        assert_eq!(cursor.get(), "");

        assert!(cursor.at_end());
    }
}
