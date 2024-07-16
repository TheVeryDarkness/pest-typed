use crate::{Position, Span};
use alloc::string::String;
use core::ops::Range;

/// Input.
pub trait Input<'i>: Copy {
    /// Get byte offset.
    fn byte_offset(&self) -> usize;
    /// Get the full input.
    fn input(&self) -> &'i str;

    /// Get line number and column number.
    // fn line_col(&self) -> (usize, usize);
    /// Get current line.
    // fn line_of(&self) -> &'i str;

    /// To position.
    fn as_position(&self) -> Position<'i> {
        Position::new(self.input(), self.byte_offset()).unwrap()
    }
    /// Create a [Span].
    fn span(&self, end: Self) -> Span<'i> {
        self.as_position().span(&end.as_position())
    }

    /// Match a string.
    fn match_string(&mut self, string: &'i str) -> bool;
    /// Match an string insensitively.
    fn match_insensitive(&mut self, string: &'i str) -> bool;
    /// Skip until one of several strings.
    fn skip_until(&mut self, strings: &'i [&'i str]) -> bool;
    /// Skip several characters.
    fn skip(&mut self, n: usize) -> bool;
    /// Match a character in a range.
    fn match_range(&mut self, range: Range<char>) -> bool;
    /// Match a character by a predicate.
    fn match_char_by(&mut self, f: impl FnOnce(char) -> bool) -> bool;
    /// Progress to next character.
    fn next(&mut self) -> Option<char>;

    /// Check if is at the start of the input.
    fn at_start(&self) -> bool;
    /// Check if is at the end of the input.
    fn at_end(&self) -> bool;
}

impl<'i> Input<'i> for Position<'i> {
    fn byte_offset(&self) -> usize {
        self.pos()
    }

    fn input(&self) -> &'i str {
        self.input
    }

    fn match_string(&mut self, s: &'i str) -> bool {
        self.match_string(s)
    }

    fn match_insensitive(&mut self, s: &'i str) -> bool {
        self.match_insensitive(s)
    }

    fn skip_until(&mut self, s: &'i [&'i str]) -> bool {
        self.skip_until(s)
    }

    fn skip(&mut self, n: usize) -> bool {
        self.skip(n)
    }

    fn match_range(&mut self, range: Range<char>) -> bool {
        self.match_range(range)
    }

    fn match_char_by(&mut self, f: impl FnOnce(char) -> bool) -> bool {
        self.match_char_by(f)
    }

    fn next(&mut self) -> Option<char> {
        let c = self.input[self.byte_offset()..].chars().next();
        if let Some(_) = c {
            self.skip(1);
        }
        c
    }

    fn at_start(&self) -> bool {
        self.at_start()
    }

    fn at_end(&self) -> bool {
        self.at_end()
    }
}

/// A part of input.
#[derive(Clone, Copy)]
pub struct SubInput1<'i> {
    input: &'i str,
    start: usize,
    cursor: usize,
}

impl<'i> Input<'i> for SubInput1<'i> {
    fn byte_offset(&self) -> usize {
        self.cursor
    }

    fn input(&self) -> &'i str {
        self.input
    }

    fn match_string(&mut self, s: &'i str) -> bool {
        let end = self.cursor + s.len();
        if Some(s) == self.input.get(self.cursor..end) {
            self.cursor = end;
            true
        } else {
            false
        }
    }

    fn match_insensitive(&mut self, s: &'i str) -> bool {
        let end = self.cursor + s.len();
        if let Some(s_) = self.input.get(self.cursor..end) {
            if s_.eq_ignore_ascii_case(s) {
                self.cursor = end;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn skip_until(&mut self, strings: &'i [&'i str]) -> bool {
        for from in self.cursor..self.input.len() {
            let bytes = if let Some(string) = self.input.get(from..) {
                string.as_bytes()
            } else {
                continue;
            };

            for slice in strings.iter() {
                let to = slice.len();
                if Some(slice.as_bytes()) == bytes.get(0..to) {
                    self.cursor = from;
                    return true;
                }
            }
        }

        self.cursor = self.input.len();
        false
    }

    fn skip(&mut self, n: usize) -> bool {
        let skipped = {
            let mut len = 0;
            // Position's pos is always a UTF-8 border.
            let mut chars = self.input[self.cursor..].chars();
            for _ in 0..n {
                if let Some(c) = chars.next() {
                    len += c.len_utf8();
                } else {
                    return false;
                }
            }
            len
        };

        self.cursor += skipped;
        true
    }

    fn match_range(&mut self, range: Range<char>) -> bool {
        if let Some(c) = self.input[self.cursor..].chars().next() {
            if range.start <= c && c <= range.end {
                self.cursor += c.len_utf8();
                return true;
            }
        }

        false
    }

    fn match_char_by(&mut self, f: impl FnOnce(char) -> bool) -> bool {
        if let Some(c) = self.input[self.cursor..].chars().next() {
            if f(c) {
                self.cursor += c.len_utf8();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn next(&mut self) -> Option<char> {
        let c = self.input[self.cursor..].chars().next();
        if let Some(c) = c {
            self.cursor += c.len_utf8();
        }
        c
    }

    fn at_start(&self) -> bool {
        self.cursor == self.start
    }

    fn at_end(&self) -> bool {
        self.cursor == self.input.len()
    }
}

impl<'i> Input<'i> for SubInput2<'i> {
    fn byte_offset(&self) -> usize {
        self.cursor
    }

    fn input(&self) -> &'i str {
        self.input
    }

    fn match_string(&mut self, string: &'i str) -> bool {
        let to = self.cursor + string.len();

        if self.end < to {
            return false;
        } else if Some(string.as_bytes()) == self.input.as_bytes().get(self.cursor..to) {
            self.cursor = to;
            true
        } else {
            false
        }
    }

    fn match_insensitive(&mut self, string: &'i str) -> bool {
        let matched = {
            let slice = &self.input[self.cursor..self.end];
            if let Some(slice) = slice.get(0..string.len()) {
                slice.eq_ignore_ascii_case(string)
            } else {
                false
            }
        };

        if matched {
            self.cursor += string.len();
            true
        } else {
            false
        }
    }

    fn skip_until(&mut self, strings: &'i [&'i str]) -> bool {
        for from in self.cursor..self.end {
            let bytes = if let Some(string) = self.input.get(from..) {
                string.as_bytes()
            } else {
                continue;
            };

            for slice in strings.iter() {
                let to = slice.len();
                if Some(slice.as_bytes()) == bytes.get(0..to) {
                    self.cursor = from;
                    return true;
                }
            }
        }

        self.cursor = self.end;
        false
    }

    fn skip(&mut self, n: usize) -> bool {
        let skipped = {
            let mut len = 0;
            // Position's pos is always a UTF-8 border.
            let mut chars = self.input[self.cursor..self.end].chars();
            for _ in 0..n {
                if let Some(c) = chars.next() {
                    len += c.len_utf8();
                } else {
                    return false;
                }
            }
            len
        };

        self.cursor += skipped;
        true
    }

    fn match_range(&mut self, range: Range<char>) -> bool {
        if let Some(c) = self.input[self.cursor..self.end].chars().next() {
            if range.start <= c && c <= range.end {
                self.cursor += c.len_utf8();
                return true;
            }
        }

        false
    }

    fn match_char_by(&mut self, f: impl FnOnce(char) -> bool) -> bool {
        if let Some(c) = self.input[self.cursor..self.end].chars().next() {
            if f(c) {
                self.cursor += c.len_utf8();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn next(&mut self) -> Option<char> {
        let c = self.input[self.cursor..self.end].chars().next();
        if let Some(c) = c {
            self.cursor += c.len_utf8();
        }
        c
    }

    fn at_start(&self) -> bool {
        self.cursor == self.start
    }

    fn at_end(&self) -> bool {
        self.cursor == self.end
    }
}

/// A part of input.
#[derive(Clone, Copy)]
pub struct SubInput2<'i> {
    input: &'i str,
    start: usize,
    end: usize,
    cursor: usize,
}

/// Convert to input.
pub trait AsInput<'i> {
    /// Output type.
    type Output: Input<'i>;

    /// Convert to a [Input] type.
    fn as_input(&self) -> Self::Output;
}

impl<'i> AsInput<'i> for &'i String {
    type Output = Position<'i>;

    fn as_input(&self) -> Self::Output {
        Position::from_start(self)
    }
}

impl<'i> AsInput<'i> for &'i str {
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
