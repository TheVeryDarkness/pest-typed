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
    fn match_string(&mut self, s: &'i str) -> bool;
    /// Match an string insensitively.
    fn match_insensitive(&mut self, s: &'i str) -> bool;
    /// Skip until one of several strings.
    fn skip_until(&mut self, s: &'i [&'i str]) -> bool;
    /// Skip several characters.
    fn skip(&mut self, n: usize) -> bool;
    /// Match a character in a range.
    fn match_range(&mut self, range: Range<char>) -> bool;
    /// Match a character by a predicate.
    fn match_char_by(&mut self, f: impl FnOnce(char) -> bool) -> bool;

    /// Check if is at the start of the input.
    fn at_start(&self) -> bool {
        self.byte_offset() == 0
    }
    /// Check if is at the end of the input.
    fn at_end(&self) -> bool {
        self.byte_offset() == self.input().chars().count()
    }
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
}

impl<'i> Input<'i> for Span<'i> {
    fn byte_offset(&self) -> usize {
        self.start()
    }

    fn input(&self) -> &'i str {
        self.get_input()
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
    type Output = Position<'i>;

    fn as_input(&self) -> Self::Output {
        *self
    }
}
impl<'i> AsInput<'i> for Span<'i> {
    type Output = Span<'i>;

    fn as_input(&self) -> Self::Output {
        *self
    }
}
