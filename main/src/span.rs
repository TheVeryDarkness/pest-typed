// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Copied from pest/pest/src/position.rs (commit ac0aed3eecf435fd93ba575a39704aaa88a375b7)
//! and modified.

use core::fmt::{self, Write};
use core::hash::{Hash, Hasher};
use core::ops::{Bound, Range, RangeBounds};
use core::ptr;
use core::str;

use crate::formatter::FormatOption;
use crate::line_indexer::LineIndexer;
use crate::position;

/// A span over a `&str`. It is created from either [two `Position`s] or from a [`Pair`].
///
/// [two `Position`s]: struct.Position.html#method.span
/// [`Pair`]: ../iterators/struct.Pair.html#method.span
#[derive(Clone, Copy)]
pub struct Span<'i> {
    input: &'i str,
    /// # Safety
    ///
    /// Must be a valid character boundary index into `input`.
    start: usize,
    /// # Safety
    ///
    /// Must be a valid character boundary index into `input`.
    end: usize,
}

impl<'i> Span<'i> {
    /// Create a new `Span` without checking invariants. (Checked with `debug_assertions`.)
    ///
    /// # Safety
    ///
    /// `input[start..end]` must be a valid subslice; that is, said indexing should not panic.
    pub(crate) unsafe fn new_unchecked(input: &'i str, start: usize, end: usize) -> Self {
        debug_assert!(input.get(start..end).is_some());
        Span { input, start, end }
    }

    /// Create a new span that contains the entire input.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest_typed::Span;
    /// let input = "Hello!";
    /// let span = Span::new_full(input);
    /// assert_eq!(span.as_str(), input);
    /// ```
    pub fn new_full(input: &'i str) -> Self {
        Span {
            input,
            start: 0,
            end: input.len(),
        }
    }

    /// Create a new span that points to the end of the input.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest_typed::Span;
    /// let input = "Hello!";
    /// let span = Span::new_at_end(input);
    /// assert_eq!(span.as_str(), "");
    /// ```
    pub fn new_at_end(input: &'i str) -> Self {
        Span {
            input,
            start: input.len(),
            end: input.len(),
        }
    }

    /// Attempts to create a new span. Will return `None` if `input[start..end]` is an invalid index
    /// into `input`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::Span;
    /// let input = "Hello!";
    /// assert_eq!(None, Span::new(input, 100, 0));
    /// assert!(Span::new(input, 0, input.len()).is_some());
    /// ```
    pub fn new(input: &'i str, start: usize, end: usize) -> Option<Self> {
        if input.get(start..end).is_some() {
            Some(Span { input, start, end })
        } else {
            None
        }
    }

    /// Attempts to create a new span based on a sub-range.
    ///
    /// ```
    /// use pest::Span;
    /// let input = "Hello World!";
    /// let world = Span::new(input, 6, input.len()).unwrap();
    /// let orl = world.get(1..=3);
    /// assert!(orl.is_some());
    /// assert_eq!(orl.unwrap().as_str(), "orl");
    /// ```
    ///
    /// # Examples
    pub fn get(&self, range: impl RangeBounds<usize>) -> Option<Self> {
        let start = match range.start_bound() {
            Bound::Included(offset) => *offset,
            Bound::Excluded(offset) => *offset + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(offset) => *offset + 1,
            Bound::Excluded(offset) => *offset,
            Bound::Unbounded => self.as_str().len(),
        };

        self.as_str().get(start..end).map(|_| Span {
            input: self.input,
            start: self.start + start,
            end: self.start + end,
        })
    }

    /// Returns the `Span`'s start byte position as a `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::Position;
    /// let input = "ab";
    /// let start = Position::from_start(input);
    /// let end = start.clone();
    /// let span = start.span(&end);
    ///
    /// assert_eq!(span.start(), 0);
    /// ```
    #[inline]
    pub const fn start(&self) -> usize {
        self.start
    }

    /// Returns the `Span`'s end byte position as a `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::Position;
    /// let input = "ab";
    /// let start = Position::from_start(input);
    /// let end = start.clone();
    /// let span = start.span(&end);
    ///
    /// assert_eq!(span.end(), 0);
    /// ```
    #[inline]
    pub const fn end(&self) -> usize {
        self.end
    }

    /// Returns the `Span`'s start `Position`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::Position;
    /// let input = "ab";
    /// let start = Position::from_start(input);
    /// let end = start.clone();
    /// let span = start.clone().span(&end);
    ///
    /// assert_eq!(span.start_pos(), start);
    /// ```
    #[inline]
    pub fn start_pos(&self) -> position::Position<'i> {
        // Span's start position is always a UTF-8 border.
        unsafe { position::Position::new_unchecked(self.input, self.start) }
    }

    /// Returns the `Span`'s end `Position`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::Position;
    /// let input = "ab";
    /// let start = Position::from_start(input);
    /// let end = start.clone();
    /// let span = start.span(&end);
    ///
    /// assert_eq!(span.end_pos(), end);
    /// ```
    #[inline]
    pub fn end_pos(&self) -> position::Position<'i> {
        // Span's end position is always a UTF-8 border.
        unsafe { position::Position::new_unchecked(self.input, self.end) }
    }

    /// Splits the `Span` into a pair of `Position`s.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::Position;
    /// let input = "ab";
    /// let start = Position::from_start(input);
    /// let end = start.clone();
    /// let span = start.clone().span(&end);
    ///
    /// assert_eq!(span.split(), (start, end));
    /// ```
    #[inline]
    pub fn split(self) -> (position::Position<'i>, position::Position<'i>) {
        // Span's start and end positions are always a UTF-8 borders.
        let pos1 = unsafe { position::Position::new_unchecked(self.input, self.start) };
        let pos2 = unsafe { position::Position::new_unchecked(self.input, self.end) };

        (pos1, pos2)
    }

    /// Captures a slice from the `&str` defined by the `Span`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "abc";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input).skip(1).unwrap();
    /// let start_pos = state.position().clone();
    /// state = state.match_string("b").unwrap();
    /// let span = start_pos.span(&state.position().clone());
    /// assert_eq!(span.as_str(), "b");
    /// ```
    #[inline]
    pub fn as_str(&self) -> &'i str {
        // Span's start and end positions are always a UTF-8 borders.
        &self.input[self.start..self.end]
    }

    /// Returns the input string of the `Span`.
    ///
    /// This function returns the input string of the `Span` as a `&str`. This is the source string
    /// from which the `Span` was created. The returned `&str` can be used to examine the contents of
    /// the `Span` or to perform further processing on the string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # use pest::Span;
    ///
    /// // Example: Get input string from a span
    /// let input = "abc\ndef\nghi";
    /// let span = Span::new(input, 1, 7).unwrap();
    /// assert_eq!(span.get_input(), input);
    /// ```
    pub fn get_input(&self) -> &'i str {
        self.input
    }

    /// Iterates over all lines (partially) covered by this span. Yielding a `&str` for each line.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "a\nb\nc";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input).skip(2).unwrap();
    /// let start_pos = state.position().clone();
    /// state = state.match_string("b\nc").unwrap();
    /// let span = start_pos.span(&state.position().clone());
    /// assert_eq!(span.lines().collect::<Vec<_>>(), vec!["b\n", "c"]);
    /// ```
    #[inline]
    pub const fn lines<L: LineIndexer<'i>>(&self, indexer: L) -> Lines<'_, 'i, L> {
        Lines {
            inner: self.lines_span(indexer),
        }
    }

    /// Iterates over all lines (partially) covered by this span. Yielding a `Span` for each line.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest;
    /// # use pest::Span;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {}
    ///
    /// let input = "a\nb\nc";
    /// let mut state: Box<pest::ParserState<'_, Rule>> = pest::ParserState::new(input).skip(2).unwrap();
    /// let start_pos = state.position().clone();
    /// state = state.match_string("b\nc").unwrap();
    /// let span = start_pos.span(&state.position().clone());
    /// assert_eq!(span.lines_span().collect::<Vec<_>>(), vec![Span::new(input, 2, 4).unwrap(), Span::new(input, 4, 5).unwrap()]);
    /// ```
    pub const fn lines_span<L: LineIndexer<'i>>(&self, indexer: L) -> LinesSpan<'_, 'i, L> {
        LinesSpan {
            span: self,
            indexer: indexer,
            pos: self.start,
        }
    }

    /// Skips `n` `char`s from the `Span` and returns `true` if the skip was possible or `false`
    /// otherwise. If the return value is `false`, `start` will not be updated.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn skip(&mut self, n: usize) -> bool {
        let skipped = {
            let mut len = 0;
            // Position's pos is always a UTF-8 border.
            let mut chars = self.input[self.start..self.end].chars();
            for _ in 0..n {
                if let Some(c) = chars.next() {
                    len += c.len_utf8();
                } else {
                    return false;
                }
            }
            len
        };

        self.start += skipped;
        true
    }

    /// Skips until one of the given `strings` is found. If none of the `strings` can be found,
    /// this function will return `false` but its `pos` will *still* be updated.
    #[inline]
    #[allow(dead_code, unexpected_cfgs)]
    pub(crate) fn skip_until(&mut self, strings: &[&str]) -> bool {
        #[cfg(not(feature = "memchr"))]
        {
            self.skip_until_basic(strings)
        }
        #[cfg(feature = "memchr")]
        {
            match strings {
                [] => (),
                [s1] => {
                    if let Some(from) = memchr::memmem::find(
                        &self.input.as_bytes()[self.start..self.end],
                        s1.as_bytes(),
                    ) {
                        self.start += from;
                        return true;
                    }
                }
                [s1, s2] if !s1.is_empty() && !s2.is_empty() => {
                    let b1 = s1.as_bytes()[0];
                    let b2 = s2.as_bytes()[0];
                    let miter = memchr::memchr2_iter(b1, b2, &self.input.as_bytes()[self.pos..]);
                    for from in miter {
                        let start = &self.input[self.pos + from..];
                        if start.starts_with(s1) || start.starts_with(s2) {
                            self.pos += from;
                            return true;
                        }
                    }
                }
                [s1, s2, s3] if !s1.is_empty() && !s2.is_empty() && s3.is_empty() => {
                    let b1 = s1.as_bytes()[0];
                    let b2 = s2.as_bytes()[0];
                    let b3 = s2.as_bytes()[0];
                    let miter =
                        memchr::memchr3_iter(b1, b2, b3, &self.input.as_bytes()[self.pos..]);
                    for from in miter {
                        let start = &self.input[self.pos + from..];
                        if start.starts_with(s1) || start.starts_with(s2) || start.starts_with(s3) {
                            self.pos += from;
                            return true;
                        }
                    }
                }
                _ => {
                    return self.skip_until_basic(strings);
                }
            }
            self.pos = self.input.len();
            false
        }
    }

    #[inline]
    fn skip_until_basic(&mut self, strings: &[&str]) -> bool {
        // TODO: optimize with Aho-Corasick, e.g. https://crates.io/crates/daachorse?
        for from in self.start..self.end {
            let bytes = if let Some(string) = self.input.get(from..) {
                string.as_bytes()
            } else {
                continue;
            };

            for slice in strings.iter() {
                let to = slice.len();
                if Some(slice.as_bytes()) == bytes.get(0..to) {
                    self.start = from;
                    return true;
                }
            }
        }

        self.start = self.end;
        false
    }

    /// Matches the char at the `Position` against a specified character and returns `true` if a match
    /// was made. If no match was made, returns `false`.
    /// `pos` will not be updated in either case.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn match_char(&self, c: char) -> bool {
        matches!(self.input[self.start..self.end].chars().next(), Some(cc) if c == cc)
    }

    /// Matches the char at the `Position` against a filter function and returns `true` if a match
    /// was made. If no match was made, returns `false` and `pos` will not be updated.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn match_char_by<F>(&mut self, f: F) -> bool
    where
        F: FnOnce(char) -> bool,
    {
        if let Some(c) = self.input[self.start..self.end].chars().next() {
            if f(c) {
                self.start += c.len_utf8();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Matches `string` from the `Position` and returns `true` if a match was made or `false`
    /// otherwise. If no match was made, `pos` will not be updated.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn match_string(&mut self, string: &str) -> bool {
        let to = self.start + string.len();

        if self.end < to {
            false
        } else if Some(string.as_bytes()) == self.input.as_bytes().get(self.start..to) {
            self.start = to;
            true
        } else {
            false
        }
    }

    /// Case-insensitively matches `string` from the `Position` and returns `true` if a match was
    /// made or `false` otherwise. If no match was made, `pos` will not be updated.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn match_insensitive(&mut self, string: &str) -> bool {
        let matched = {
            let slice = &self.input[self.start..self.end];
            if let Some(slice) = slice.get(0..string.len()) {
                slice.eq_ignore_ascii_case(string)
            } else {
                false
            }
        };

        if matched {
            self.start += string.len();
            true
        } else {
            false
        }
    }

    /// Matches `char` `range` from the `Position` and returns `true` if a match was made or `false`
    /// otherwise. If no match was made, `pos` will not be updated.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn match_range(&mut self, range: Range<char>) -> bool {
        if let Some(c) = self.input[self.start..self.end].chars().next() {
            if range.start <= c && c <= range.end {
                self.start += c.len_utf8();
                return true;
            }
        }

        false
    }
}

impl fmt::Debug for Span<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Span")
            .field("str", &self.as_str())
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

impl PartialEq for Span<'_> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq::<str>(self.input, other.input)
            && self.start == other.start
            && self.end == other.end
    }
}

impl Eq for Span<'_> {}

impl Hash for Span<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.input as *const str).hash(state);
        self.start.hash(state);
        self.end.hash(state);
    }
}

impl<'i> Span<'i> {
    /// Format span with given option.
    #[inline]
    pub fn display<L, Writer, SF, MF, NF>(
        &self,
        indexer: L,
        f: &mut Writer,
        opt: FormatOption<SF, MF, NF>,
    ) -> fmt::Result
    where
        L: LineIndexer<'i>,
        Writer: Write,
        SF: FnMut(&str, &mut Writer) -> fmt::Result,
        MF: FnMut(&str, &mut Writer) -> fmt::Result,
        NF: FnMut(&str, &mut Writer) -> fmt::Result,
    {
        opt.display_span(self, indexer, f)
    }
}

impl fmt::Display for Span<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let opt = FormatOption::default();
        opt.display_span(self, (), f)
    }
}

/// Merges two spans into one.
///
/// This function merges two spans that are contiguous or overlapping into a single span
/// that covers the entire range of the two input spans. This is useful when you want to
/// aggregate information from multiple spans into a single entity.
///
/// The function checks if the input spans are overlapping or contiguous by comparing their
/// start and end positions. If they are, a new span is created with the minimum start position
/// and the maximum end position of the two input spans.
///
/// If the input spans are neither overlapping nor contiguous, the function returns None,
/// indicating that a merge operation was not possible.
///
/// # Examples
///
/// ```
/// # use pest;
/// # use pest::Span;
/// # use pest::merge_spans;
///
/// // Example 1: Contiguous spans
/// let input = "abc\ndef\nghi";
/// let span1 = Span::new(input, 1, 7).unwrap();
/// let span2 = Span::new(input, 7, 11).unwrap();
/// let merged = merge_spans(&span1, &span2).unwrap();
/// assert_eq!(merged, Span::new(input, 1, 11).unwrap());
///
/// // Example 2: Overlapping spans
/// let input = "abc\ndef\nghi";
/// let span1 = Span::new(input, 1, 7).unwrap();
/// let span2 = Span::new(input, 5, 11).unwrap();
/// let merged = merge_spans(&span1, &span2).unwrap();
/// assert_eq!(merged, Span::new(input, 1, 11).unwrap());
///
/// // Example 3: Non-contiguous spans
/// let input = "abc\ndef\nghi";
/// let span1 = Span::new(input, 1, 7).unwrap();
/// let span2 = Span::new(input, 8, 11).unwrap();
/// let merged = merge_spans(&span1, &span2);
/// assert!(merged.is_none());
/// ```
pub fn merge_spans<'i>(a: &Span<'i>, b: &Span<'i>) -> Option<Span<'i>> {
    if a.end() >= b.start() && a.start() <= b.end() {
        // The spans overlap or are contiguous, so they can be merged.
        Span::new(
            a.get_input(),
            core::cmp::min(a.start(), b.start()),
            core::cmp::max(a.end(), b.end()),
        )
    } else {
        // The spans don't overlap and aren't contiguous, so they can't be merged.
        None
    }
}

/// Line iterator for Spans, created by [`Span::lines_span()`].
///
/// Iterates all lines that are at least _partially_ covered by the span. Yielding a `Span` for each.
///
/// [`Span::lines_span()`]: struct.Span.html#method.lines_span
pub struct LinesSpan<'s, 'i, L> {
    span: &'s Span<'i>,
    indexer: L,
    pos: usize,
}

impl<'i, L: LineIndexer<'i>> Iterator for LinesSpan<'_, 'i, L> {
    type Item = Span<'i>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos > self.span.end {
            return None;
        }
        let pos = position::Position::new(self.span.input, self.pos)?;
        if pos.at_end() {
            return None;
        }

        let line_start = pos.find_line_start(&self.indexer);
        self.pos = pos.find_line_end(&self.indexer);

        Span::new(self.span.input, line_start, self.pos)
    }
}

/// Line iterator for Spans, created by [`Span::lines()`].
///
/// Iterates all lines that are at least _partially_ covered by the span. Yielding a `&str` for each.
///
/// [`Span::lines()`]: struct.Span.html#method.lines
pub struct Lines<'s, 'i, L> {
    inner: LinesSpan<'s, 'i, L>,
}

impl<'i, L: LineIndexer<'i>> Iterator for Lines<'_, 'i, L> {
    type Item = &'i str;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|span| span.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::borrow::ToOwned;
    use alloc::vec::Vec;

    #[test]
    fn get() {
        let input = "abc123abc";
        let span = Span::new(input, 3, input.len()).unwrap();
        assert_eq!(span.as_str(), "123abc");
        assert_eq!(span.input, input);

        let span1 = span.get(..=2);
        assert!(span1.is_some());
        assert_eq!(span1.unwrap().input, input);
        assert_eq!(span1.unwrap().as_str(), "123");

        let span2 = span.get(..);
        assert!(span2.is_some());
        assert_eq!(span2.unwrap().input, input);
        assert_eq!(span2.unwrap().as_str(), "123abc");

        let span3 = span.get(3..);
        assert!(span3.is_some());
        assert_eq!(span3.unwrap().input, input);
        assert_eq!(span3.unwrap().as_str(), "abc");

        let span4 = span.get(0..0);
        assert!(span4.is_some());
        assert_eq!(span4.unwrap().input, input);
        assert_eq!(span4.unwrap().as_str(), "");
    }

    #[test]
    fn get_fails() {
        let input = "abc";
        let span = Span::new(input, 0, input.len()).unwrap();

        let span1 = span.get(0..100);
        assert!(span1.is_none());

        let span2 = span.get(100..200);
        assert!(span2.is_none());
    }

    #[test]
    fn span_comp() {
        let input = "abc\ndef\nghi";
        let span = Span::new(input, 1, 7).unwrap();
        let span2 = Span::new(input, 50, 51);
        assert!(span2.is_none());
        let span3 = Span::new(input, 0, 8).unwrap();
        assert!(span != span3);
    }

    #[test]
    fn split() {
        let input = "a";
        let start = position::Position::from_start(input);
        let mut end = start;

        assert!(end.skip(1));

        let span = start.clone().span(&end.clone());

        assert_eq!(span.split(), (start, end));
    }

    #[test]
    fn lines_mid() {
        let input = "abc\ndef\nghi";
        let span = Span::new(input, 1, 7).unwrap();
        let lines: Vec<_> = span.lines(()).collect();
        let lines_span: Vec<_> = span.lines_span(()).map(|span| span.as_str()).collect();

        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "abc\n".to_owned());
        assert_eq!(lines[1], "def\n".to_owned());
        assert_eq!(lines, lines_span) // Verify parity with lines_span()
    }

    #[test]
    fn lines_eof() {
        let input = "abc\ndef\nghi";
        let span = Span::new(input, 5, 11).unwrap();
        assert!(span.end_pos().at_end());
        assert_eq!(span.end(), 11);
        let lines: Vec<_> = span.lines(()).collect();
        let lines_span: Vec<_> = span.lines_span(()).map(|span| span.as_str()).collect();

        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "def\n".to_owned());
        assert_eq!(lines[1], "ghi".to_owned());
        assert_eq!(lines, lines_span) // Verify parity with lines_span()
    }

    #[test]
    fn lines_span() {
        let input = "abc\ndef\nghi";
        let span = Span::new(input, 1, 7).unwrap();
        let lines_span: Vec<_> = span.lines_span(()).collect();
        let lines: Vec<_> = span.lines(()).collect();

        assert_eq!(lines_span.len(), 2);
        assert_eq!(lines_span[0], Span::new(input, 0, 4).unwrap());
        assert_eq!(lines_span[1], Span::new(input, 4, 8).unwrap());
        assert_eq!(
            lines_span
                .iter()
                .map(|span| span.as_str())
                .collect::<Vec<_>>(),
            lines
        );
    }

    #[test]
    fn get_input_of_span() {
        let input = "abc\ndef\nghi";
        let span = Span::new(input, 1, 7).unwrap();

        assert_eq!(span.get_input(), input);
    }

    #[test]
    fn merge_contiguous() {
        let input = "abc\ndef\nghi";
        let span1 = Span::new(input, 1, 7).unwrap();
        let span2 = Span::new(input, 7, 11).unwrap();
        let merged = merge_spans(&span1, &span2).unwrap();

        assert_eq!(merged, Span::new(input, 1, 11).unwrap());
    }

    #[test]
    fn merge_overlapping() {
        let input = "abc\ndef\nghi";
        let span1 = Span::new(input, 1, 7).unwrap();
        let span2 = Span::new(input, 5, 11).unwrap();
        let merged = merge_spans(&span1, &span2).unwrap();

        assert_eq!(merged, Span::new(input, 1, 11).unwrap());
    }

    #[test]
    fn merge_non_contiguous() {
        let input = "abc\ndef\nghi";
        let span1 = Span::new(input, 1, 7).unwrap();
        let span2 = Span::new(input, 8, 11).unwrap();
        let merged = merge_spans(&span1, &span2);

        assert!(merged.is_none());
    }
}
