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

use super::span;
use crate::{formatter::FormatOption, input::RefStr, line_indexer::LineIndexer};
use core::{
    cmp::Ordering,
    fmt::{self, Write},
    hash::{Hash, Hasher},
    str,
};

/// A cursor position in a `&str` which provides useful methods to manually parse that string.
#[derive(Clone, Copy)]
pub struct Position<S> {
    pub(crate) input: S,
    /// # Safety:
    ///
    /// `input[pos..]` must be a valid codepoint boundary (should not panic when indexing thus).
    pub(crate) pos: usize,
}

impl<S: RefStr> Position<S> {
    /// Create a new `Position` without checking invariants. (Checked with `debug_assertions`.)
    ///
    /// # Safety:
    ///
    /// `input[pos..]` must be a valid codepoint boundary (should not panic when indexing thus).
    pub(crate) unsafe fn new_unchecked(input: S, pos: usize) -> Self {
        debug_assert!(input.get(pos..).is_some());

        Self { input, pos }
    }

    /// Attempts to create a new `Position` at the given position. If the specified position is
    /// an invalid index, or the specified position is not a valid UTF8 boundary, then None is
    /// returned.
    ///
    /// # Examples
    /// ```
    /// # use pest::Position;
    /// let cheart = '💖';
    /// let heart = "💖";
    /// assert_eq!(Position::new(heart, 1), None);
    /// assert_ne!(Position::new(heart, cheart.len_utf8()), None);
    /// ```
    pub fn new(input: S, pos: usize) -> Option<Self> {
        input.get(pos..).map(|_| Self { input, pos })
    }

    /// Create a new `Position` at the end of the input.
    pub fn new_at_end(input: S) -> Self {
        Self {
            pos: input.len(),
            input,
        }
    }

    /// Creates a `Position` at the start of a `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::Position;
    /// let start = Position::from_start("");
    /// assert_eq!(start.pos(), 0);
    /// ```
    #[inline]
    pub const fn from_start(input: S) -> Self {
        // Position 0 is always safe because it's always a valid UTF-8 border.
        Self { input, pos: 0 }
    }

    /// Returns the byte position of this `Position` as a `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::Position;
    /// let input = "ab";
    /// let mut start = Position::from_start(input);
    ///
    /// assert_eq!(start.pos(), 0);
    /// ```
    #[inline]
    pub const fn pos(&self) -> usize {
        self.pos
    }

    /// Creates a `Span` from two `Position`s.
    ///
    /// # Panics
    ///
    /// Panics if the positions come from different inputs.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::Position;
    /// let input = "ab";
    /// let start = Position::from_start(input);
    /// let span = start.span(&start.clone());
    ///
    /// assert_eq!(span.start(), 0);
    /// assert_eq!(span.end(), 0);
    /// ```
    #[inline]
    pub fn span(&self, other: &Self) -> span::Span<S> {
        // TODO: maybe a panic if self.pos < other.pos
        if !self.input.ptr_eq(&other.input) {
            panic!("span created from positions from different inputs")
        }
        if self.input.get(self.pos..other.pos).is_none() {
            panic!("span created with positions in wrong order")
        }
        // This is safe because the pos field of a Position should always be a valid str index.
        unsafe { span::Span::new_unchecked(self.input.clone(), self.pos, other.pos) }
    }

    /// Returns the line and column number of this [`Position`] using [LineIndexer::line_col].
    #[inline]
    pub fn line_col(&self, indexer: impl LineIndexer<S>) -> (usize, usize) {
        indexer.line_col(&self.input, self.pos)
    }

    /// Returns the entire line of the input that contains this `Position`.
    #[inline]
    pub fn line_of(&self, indexer: impl LineIndexer<S>) -> S {
        indexer.line_of(&self.input, self.pos)
    }

    pub(crate) fn find_line_start(&self, indexer: impl LineIndexer<S>) -> usize {
        indexer.find_line_start(&self.input, self.pos)
    }

    pub(crate) fn find_line_end(&self, indexer: impl LineIndexer<S>) -> usize {
        indexer.find_line_end(&self.input, self.pos)
    }

    /// Returns `true` when the `Position` points to the start of the input `&str`.
    #[inline]
    #[allow(dead_code)]
    pub(crate) const fn at_start(&self) -> bool {
        self.pos == 0
    }

    /// Returns `true` when the `Position` points to the end of the input `&str`.
    #[inline]
    pub(crate) fn at_end(&self) -> bool {
        self.pos == self.input.len()
    }

    /// Skips `n` `char`s from the `Position` and returns `true` if the skip was possible or `false`
    /// otherwise. If the return value is `false`, `pos` will not be updated.
    #[inline]
    pub(crate) fn skip(&mut self, n: usize) -> bool {
        let skipped = {
            let mut len = 0;
            let input = unsafe { self.input.get_range_unchecked(self.pos..) };
            // Position's pos is always a UTF-8 border.
            let mut chars = input.chars();
            for _ in 0..n {
                if let Some(c) = chars.next() {
                    len += c.len_utf8();
                } else {
                    return false;
                }
            }
            len
        };

        self.pos += skipped;
        true
    }

    /// Goes back `n` `char`s from the `Position` and returns `true` if the skip was possible or `false`
    /// otherwise. If the return value is `false`, `pos` will not be updated.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn skip_back(&mut self, n: usize) -> bool {
        let skipped = {
            let mut len = 0;
            let input = unsafe { self.input.get_range_unchecked(..self.pos) };
            // Position's pos is always a UTF-8 border.
            let mut chars = input.chars().rev();
            for _ in 0..n {
                if let Some(c) = chars.next() {
                    len += c.len_utf8();
                } else {
                    return false;
                }
            }
            len
        };

        self.pos -= skipped;
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
                    if let Some(from) =
                        memchr::memmem::find(&self.input.as_bytes()[self.pos..], s1.as_bytes())
                    {
                        self.pos += from;
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
        for from in self.pos..self.input.len() {
            let Some(string) = self.input.get(from..) else {
                continue;
            };
            let bytes = string.as_str().as_bytes();

            for slice in strings.iter() {
                let to = <str>::len(slice);
                if Some(slice.as_bytes()) == bytes.get(0..to) {
                    self.pos = from;
                    return true;
                }
            }
        }

        self.pos = self.input.len();
        false
    }

    /// Matches the char at the `Position` against a specified character and returns `true` if a match
    /// was made. If no match was made, returns `false`.
    /// `pos` will not be updated in either case.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn match_char(&self, c: char) -> bool {
        let input = unsafe { self.input.get_range_unchecked(self.pos..) };
        matches!(input.chars().next(), Some(cc) if c == cc)
    }

    /// Matches the char at the `Position` against a filter function and returns `true` if a match
    /// was made. If no match was made, returns `false` and `pos` will not be updated.
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn match_char_by<F>(&mut self, f: F) -> bool
    where
        F: FnOnce(char) -> bool,
    {
        let input = unsafe { self.input.get_range_unchecked(self.pos..) };
        if let Some(c) = input.chars().next() {
            if f(c) {
                self.pos += c.len_utf8();
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl<S> fmt::Debug for Position<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Position").field("pos", &self.pos).finish()
    }
}

impl<S: RefStr> Position<S> {
    /// Format position with given option.
    #[inline]
    pub fn display<L, Writer, SF, MF, NF>(
        &self,
        indexer: L,
        f: &mut Writer,
        opt: FormatOption<SF, MF, NF>,
    ) -> fmt::Result
    where
        L: LineIndexer<S>,
        Writer: Write,
        SF: FnMut(&str, &mut Writer) -> fmt::Result,
        MF: FnMut(&str, &mut Writer) -> fmt::Result,
        NF: FnMut(&str, &mut Writer) -> fmt::Result,
    {
        opt.display_position(self, indexer, f)
    }
}

impl<S: RefStr> fmt::Display for Position<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        FormatOption::default().display_position(self, (), f)
    }
}

impl<S: RefStr> PartialEq for Position<S> {
    fn eq(&self, other: &Self) -> bool {
        self.input.ptr_eq(&other.input) && self.pos == other.pos
    }
}

impl<S: RefStr> Eq for Position<S> {}

impl<S: RefStr> PartialOrd for Position<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: RefStr> Ord for Position<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        debug_assert_eq!(
            self.input, other.input,
            "cannot compare positions from different strings"
        );
        self.pos.cmp(&other.pos)
    }
}

impl<S: RefStr> Hash for Position<S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.input.ptr_hash(state);
        self.pos.hash(state);
    }
}

#[expect(clippy::fallible_impl_from)]
impl<'i> From<Position<&'i str>> for pest::Position<'i> {
    #[inline]
    fn from(pos: Position<&'i str>) -> Self {
        //FIXME: eliminate the check
        pest::Position::new(pos.input, pos.pos).unwrap()
    }
}

impl<S: RefStr> Position<S> {
    /// Convert to [`pest::Position`].
    #[inline]
    pub fn as_pest_position(&self) -> pest::Position<'_> {
        //FIXME: eliminate the check
        pest::Position::new(self.input.as_str(), self.pos).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_col() {
        let input = "a\rb\nc\r\nd嗨";

        assert_eq!(Position::new(input, 0).unwrap().line_col(()), (1, 1));
        assert_eq!(Position::new(input, 1).unwrap().line_col(()), (1, 2));
        assert_eq!(Position::new(input, 2).unwrap().line_col(()), (1, 3));
        assert_eq!(Position::new(input, 3).unwrap().line_col(()), (1, 4));
        assert_eq!(Position::new(input, 4).unwrap().line_col(()), (2, 1));
        assert_eq!(Position::new(input, 5).unwrap().line_col(()), (2, 2));
        assert_eq!(Position::new(input, 6).unwrap().line_col(()), (2, 3));
        assert_eq!(Position::new(input, 7).unwrap().line_col(()), (3, 1));
        assert_eq!(Position::new(input, 8).unwrap().line_col(()), (3, 2));
        assert_eq!(Position::new(input, 11).unwrap().line_col(()), (3, 3));
        let input = "abcd嗨";
        assert_eq!(Position::new(input, 7).unwrap().line_col(()), (1, 6));
    }

    #[test]
    fn line_of() {
        let input = "a\rb\nc\r\nd嗨";

        assert_eq!(Position::new(input, 0).unwrap().line_of(()), "a\rb\n");
        assert_eq!(Position::new(input, 1).unwrap().line_of(()), "a\rb\n");
        assert_eq!(Position::new(input, 2).unwrap().line_of(()), "a\rb\n");
        assert_eq!(Position::new(input, 3).unwrap().line_of(()), "a\rb\n");
        assert_eq!(Position::new(input, 4).unwrap().line_of(()), "c\r\n");
        assert_eq!(Position::new(input, 5).unwrap().line_of(()), "c\r\n");
        assert_eq!(Position::new(input, 6).unwrap().line_of(()), "c\r\n");
        assert_eq!(Position::new(input, 7).unwrap().line_of(()), "d嗨");
        assert_eq!(Position::new(input, 8).unwrap().line_of(()), "d嗨");
        assert_eq!(Position::new(input, 11).unwrap().line_of(()), "d嗨");
    }

    #[test]
    fn line_of_empty() {
        let input = "";

        assert_eq!(Position::new(input, 0).unwrap().line_of(()), "");
    }

    #[test]
    fn line_of_new_line() {
        let input = "\n";

        assert_eq!(Position::new(input, 0).unwrap().line_of(()), "\n");
    }

    #[test]
    fn line_of_between_new_line() {
        let input = "\n\n";

        assert_eq!(Position::new(input, 1).unwrap().line_of(()), "\n");
    }

    fn measure_skip(input: &str, pos: usize, n: usize) -> Option<usize> {
        let mut p = Position::new(input, pos).unwrap();
        if p.skip(n) {
            Some(p.pos - pos)
        } else {
            None
        }
    }

    #[test]
    fn skip_empty() {
        let input = "";

        assert_eq!(measure_skip(input, 0, 0), Some(0));
        assert_eq!(measure_skip(input, 0, 1), None);
    }

    #[test]
    fn skip() {
        let input = "d嗨";

        assert_eq!(measure_skip(input, 0, 0), Some(0));
        assert_eq!(measure_skip(input, 0, 1), Some(1));
        assert_eq!(measure_skip(input, 1, 1), Some(3));
    }

    #[test]
    fn skip_until() {
        let input = "ab ac";
        let pos = Position::from_start(input);

        let mut test_pos = pos;
        test_pos.skip_until(&["a", "b"]);
        assert_eq!(test_pos.pos(), 0);

        test_pos = pos;
        test_pos.skip_until(&["b"]);
        assert_eq!(test_pos.pos(), 1);

        test_pos = pos;
        test_pos.skip_until(&["ab"]);
        assert_eq!(test_pos.pos(), 0);

        test_pos = pos;
        test_pos.skip_until(&["ac", "z"]);
        assert_eq!(test_pos.pos(), 3);

        test_pos = pos;
        assert!(!test_pos.skip_until(&["z"]));
        assert_eq!(test_pos.pos(), 5);
    }

    #[test]
    fn cmp() {
        let input = "a";
        let start = Position::from_start(input);
        let mut end = start;

        assert!(end.skip(1));
        let result = start.cmp(&end);

        assert_eq!(result, Ordering::Less);
    }

    #[test]
    #[should_panic]
    fn cmp_panic() {
        let input1 = "a";
        let input2 = "b";
        let pos1 = Position::from_start(input1);
        let pos2 = Position::from_start(input2);

        let _ = pos1.cmp(&pos2);
    }

    #[test]
    #[cfg(feature = "std")]
    fn hash() {
        use std::collections::HashSet;

        let input = "a";
        let start = Position::from_start(input);
        let mut positions = HashSet::new();

        positions.insert(start);
    }
}
