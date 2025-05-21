use crate::{Position, Span};
use alloc::string::String;
use core::{ops::Range, str::Chars};
use derive_where::derive_where;

/// Input with span information.
///
/// # Safety
///
/// [`byte_offset()`](Input::byte_offset) must be in the range of [`input()`](Input::input).
pub unsafe trait Input<'i>: Copy {
    /// Get byte offset.
    fn byte_offset(&self) -> usize;
    /// Get the full input.
    fn input(&self) -> &'i str;

    /// Get unconsumed characters.
    fn get(&self) -> &'i str;
    /// Get characters.
    fn chars(&self) -> Chars<'i> {
        self.get().chars()
    }

    // /// Get line number and column number.
    // fn line_col(&self) -> (usize, usize);
    // /// Get current line.
    // fn line_of(&self) -> &'i str;

    /// To position.
    fn as_position(&self) -> Position<'i> {
        unsafe { Position::new_unchecked(self.input(), self.byte_offset()) }
    }
    /// Create a [Span].
    fn span(&self, end: Self) -> Span<'i> {
        self.as_position().span(&end.as_position())
    }

    /// Match a string.
    fn match_string(&mut self, string: &'i str) -> bool {
        let res = self.get().starts_with(string);
        if res {
            unsafe { *self.cursor() += string.len() };
        }
        res
    }
    /// Match an string insensitively.
    fn match_insensitive(&mut self, string: &'i str) -> bool {
        let len = string.len();
        if let Some(prefix) = self.get().get(..len) {
            if prefix.eq_ignore_ascii_case(string) {
                unsafe { *self.cursor() += len };
                return true;
            }
        }
        false
    }
    /// Skip until one of several strings.
    fn skip_until(&mut self, strings: &'i [&'i str]) -> bool {
        for from in self.byte_offset()..self.end() {
            let bytes = if let Some(string) = self.input().get(from..) {
                string.as_bytes()
            } else {
                continue;
            };

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
    fn skip(&mut self, n: usize) -> bool {
        let skipped = {
            let mut len = 0;
            let mut chars = self.chars();
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
    fn match_range(&mut self, range: Range<char>) -> bool {
        if let Some(c) = self.chars().next() {
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
    fn match_char_by(&mut self, f: impl FnOnce(char) -> bool) -> bool {
        if let Some(c) = self.chars().next() {
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
    fn next(&mut self) -> Option<char> {
        let c = self.chars().next();
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
    fn at_start(&self) -> bool {
        self.byte_offset() == self.start()
    }
    /// Check if is at the end of the input.
    fn at_end(&self) -> bool {
        self.byte_offset() == self.end()
    }
}

unsafe impl<'i> Input<'i> for Position<'i> {
    fn byte_offset(&self) -> usize {
        self.pos
    }

    fn input(&self) -> &'i str {
        self.input
    }

    fn get(&self) -> &'i str {
        if cfg!(debug_assertions) {
            &self.input()[self.pos..]
        } else {
            unsafe { self.input().get_unchecked(self.pos..) }
        }
    }

    fn next(&mut self) -> Option<char> {
        let c = self.chars().next();
        if c.is_some() {
            self.skip(1);
        }
        c
    }

    unsafe fn cursor(&mut self) -> &mut usize {
        &mut self.pos
    }

    fn start(&self) -> usize {
        0
    }
    fn end(&self) -> usize {
        self.input().len()
    }
}

/// A part of input.
#[derive_where(Clone, Copy)]
pub struct SubInput1<'i, S: ?Sized = str> {
    input: &'i S,
    start: usize,
    cursor: usize,
}

/// A part of input.
#[derive_where(Clone, Copy)]
pub struct SubInput2<'i, S: ?Sized = str> {
    input: &'i S,
    start: usize,
    end: usize,
    cursor: usize,
}

impl<'i> AsInput<'i> for SubInput1<'i> {
    type Output = Self;

    fn as_input(&self) -> Self::Output {
        *self
    }
}

impl<'i> AsInput<'i> for SubInput2<'i> {
    type Output = Self;

    fn as_input(&self) -> Self::Output {
        *self
    }
}

unsafe impl<'i> Input<'i> for SubInput1<'i> {
    fn byte_offset(&self) -> usize {
        self.cursor
    }

    fn input(&self) -> &'i str {
        self.input
    }

    fn get(&self) -> &'i str {
        if cfg!(debug_assertions) {
            &self.input()[self.cursor..]
        } else {
            unsafe { self.input().get_unchecked(self.cursor..) }
        }
    }

    unsafe fn cursor(&mut self) -> &mut usize {
        &mut self.cursor
    }

    fn start(&self) -> usize {
        self.start
    }
    fn end(&self) -> usize {
        self.input().len()
    }
}

unsafe impl<'i> Input<'i> for SubInput2<'i> {
    fn byte_offset(&self) -> usize {
        self.cursor
    }

    fn input(&self) -> &'i str {
        self.input
    }

    fn get(&self) -> &'i str {
        if cfg!(debug_assertions) {
            &self.input()[self.cursor..self.end]
        } else {
            unsafe { self.input().get_unchecked(self.cursor..self.end) }
        }
    }

    unsafe fn cursor(&mut self) -> &mut usize {
        &mut self.cursor
    }

    fn start(&self) -> usize {
        self.start
    }
    fn end(&self) -> usize {
        self.end
    }
}

/// Convert to input.
pub trait AsInput<'i> {
    /// Output type.
    type Output: Input<'i>;

    /// Convert to a [Input] type.
    fn as_input(&self) -> Self::Output;
}

impl<'i> AsInput<'i> for &'i str {
    type Output = Position<'i>;

    fn as_input(&self) -> Self::Output {
        Position::from_start(self)
    }
}

impl<'i> AsInput<'i> for &'i String {
    type Output = Position<'i>;

    fn as_input(&self) -> Self::Output {
        Position::from_start(self)
    }
}

impl<'i> AsInput<'i> for Position<'i> {
    type Output = SubInput1<'i>;

    fn as_input(&self) -> Self::Output {
        let input = self.input;
        let start = self.pos();
        let cursor = start;
        SubInput1 {
            input,
            start,
            cursor,
        }
    }
}

impl<'i> AsInput<'i> for Span<'i> {
    type Output = SubInput2<'i>;

    fn as_input(&self) -> Self::Output {
        let input = self.get_input();
        let start = self.start();
        let end = self.end();
        let cursor = start;
        SubInput2 {
            input,
            start,
            end,
            cursor,
        }
    }
}
