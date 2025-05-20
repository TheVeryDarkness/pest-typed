// pest-typed. A statically typed version of pest.
// Copyright (c) 2025 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use alloc::{string::String, vec::Vec};

/// A trait for splitting a string into lines.
///
/// It supports getting the line of a given position in the string,
/// and finding the start and end of the line.
pub trait LineIndexer<'i>: Sized + Copy {
    /// Gets the input string.
    fn input(self) -> &'i str;

    /// Length in bytes of the input string.
    fn len(self) -> usize {
        self.input().len()
    }

    /// Returns the line and column number of this `Position`.
    fn line_col(self, pos: usize) -> (usize, usize);

    /// Returns the entire line of the input that contains the position at `pos`.
    fn line_of(self, pos: usize) -> &'i str;

    /// Returns the start of the line that contains the position at `pos`.
    fn find_line_start(self, pos: usize) -> usize;

    /// Returns the end of the line that contains the position at `pos`.
    fn find_line_end(self, pos: usize) -> usize;
}

impl<'i, T: LineIndexer<'i>> LineIndexer<'i> for &T {
    fn input(self) -> &'i str {
        T::input(*self)
    }

    fn line_col(self, pos: usize) -> (usize, usize) {
        T::line_col(*self, pos)
    }

    fn line_of(self, pos: usize) -> &'i str {
        T::line_of(*self, pos)
    }

    fn find_line_start(self, pos: usize) -> usize {
        T::find_line_start(*self, pos)
    }

    fn find_line_end(self, pos: usize) -> usize {
        T::find_line_end(*self, pos)
    }
}

impl<'i> LineIndexer<'i> for &'i str {
    fn input(self) -> &'i str {
        self
    }

    /// Returns the line and column number of this `Position`.
    ///
    /// This is an O(n) operation, where n is the number of chars in the input.
    /// You better use [`pair.line_col()`](struct.Pair.html#method.line_col) instead.
    #[inline]
    fn line_col(self, pos: usize) -> (usize, usize) {
        if pos > self.len() {
            panic!("position out of bounds");
        }
        let mut pos = pos;
        let slice = &self[..pos];
        let mut chars = slice.chars().peekable();

        let mut line_col = (1, 1);

        while pos != 0 {
            match chars.next() {
                Some('\r') => {
                    if let Some(&'\n') = chars.peek() {
                        chars.next();

                        if pos == 1 {
                            pos -= 1;
                        } else {
                            pos -= 2;
                        }

                        line_col = (line_col.0 + 1, 1);
                    } else {
                        pos -= 1;
                        line_col = (line_col.0, line_col.1 + 1);
                    }
                }
                Some('\n') => {
                    pos -= 1;
                    line_col = (line_col.0 + 1, 1);
                }
                Some(c) => {
                    pos -= c.len_utf8();
                    line_col = (line_col.0, line_col.1 + 1);
                }
                None => unreachable!(),
            }
        }

        line_col
    }

    fn line_of(self, pos: usize) -> &'i str {
        if pos > self.len() {
            panic!("position out of bounds");
        };
        // Safe since start and end can only be valid UTF-8 borders.
        &self[self.find_line_start(pos)..self.find_line_end(pos)]
    }

    fn find_line_start(self, pos: usize) -> usize {
        if self.is_empty() {
            return 0;
        };
        // Position's pos is always a UTF-8 border.
        let start = self
            .char_indices()
            .rev()
            .skip_while(|&(i, _)| i >= pos)
            .find(|&(_, c)| c == '\n');
        match start {
            Some((i, _)) => i + 1,
            None => 0,
        }
    }

    fn find_line_end(self, pos: usize) -> usize {
        if self.is_empty() {
            0
        } else if pos == self.len() - 1 {
            self.len()
        } else {
            // Position's pos is always a UTF-8 border.
            let end = self
                .char_indices()
                .skip_while(|&(i, _)| i < pos)
                .find(|&(_, c)| c == '\n');
            match end {
                Some((i, _)) => i + 1,
                None => self.len(),
            }
        }
    }
}

impl<'i> LineIndexer<'i> for &'i String {
    fn input(self) -> &'i str {
        <&'i str as LineIndexer<'i>>::input(self)
    }

    fn line_col(self, pos: usize) -> (usize, usize) {
        <&'i str as LineIndexer<'i>>::line_col(self, pos)
    }

    fn line_of(self, pos: usize) -> &'i str {
        <&'i str as LineIndexer<'i>>::line_of(self, pos)
    }

    fn find_line_start(self, pos: usize) -> usize {
        <&'i str as LineIndexer<'i>>::find_line_start(self, pos)
    }

    fn find_line_end(self, pos: usize) -> usize {
        <&'i str as LineIndexer<'i>>::find_line_end(self, pos)
    }
}

/// A cached line indexer that caches the start of each line in the input string.
pub struct CachedLineIndexer<'i> {
    input: &'i str,
    /// `line_starts[i + 1]` is the start of the `i`-th line.
    line_starts: Vec<usize>,
}

impl<'i> CachedLineIndexer<'i> {
    /// Creates a new `CachedLineIndexer` from the input string.
    pub fn new(input: &'i str) -> Self {
        let mut line_starts = Vec::new();
        for (i, c) in input.char_indices() {
            if c == '\n' {
                line_starts.push(i + 1);
            }
        }
        Self { input, line_starts }
    }
}

impl<'i> LineIndexer<'i> for &CachedLineIndexer<'i> {
    fn input(self) -> &'i str {
        self.input
    }

    fn line_col(self, pos: usize) -> (usize, usize) {
        if pos > self.input.len() {
            panic!("position out of bounds");
        }
        let line = self.line_starts.partition_point(|&start| start <= pos);
        let line_start = line
            .checked_sub(1)
            .map_or_else(|| 0, |line| self.line_starts[line]);
        let col = self.input[line_start..pos].chars().count(); // Still O(n) but faster than the above.
        (line + 1, col + 1)
    }

    fn line_of(self, pos: usize) -> &'i str {
        if pos > self.input.len() {
            panic!("position out of bounds");
        };
        let line = self.line_starts.partition_point(|&start| start <= pos);
        let line_start = line
            .checked_sub(1)
            .map_or_else(|| 0, |line| self.line_starts[line]);
        let next_line_start = self
            .line_starts
            .get(line)
            .copied()
            .unwrap_or(self.input.len());
        &self.input[line_start..next_line_start]
    }

    fn find_line_start(self, pos: usize) -> usize {
        if self.input.is_empty() {
            return 0;
        };
        let line = self.line_starts.partition_point(|&start| start <= pos);
        line.checked_sub(1)
            .map_or_else(|| 0, |line| self.line_starts[line])
    }

    fn find_line_end(self, pos: usize) -> usize {
        let line = self.line_starts.partition_point(|&start| start <= pos);
        self.line_starts
            .get(line)
            .copied()
            .unwrap_or(self.input.len())
    }
}

#[cfg(test)]
mod tests {
    use super::{CachedLineIndexer, LineIndexer};

    fn test_line_col<'i, T: LineIndexer<'i>>(line_indexer: T) {
        assert_eq!(line_indexer.line_col(0), (1, 1));
        assert_eq!(line_indexer.line_col(5), (1, 6));
        assert_eq!(line_indexer.line_col(6), (2, 1));
        assert_eq!(line_indexer.line_col(11), (2, 6));
    }

    fn test_line_of<'i, T: LineIndexer<'i>>(line_indexer: T) {
        assert_eq!(line_indexer.line_of(0), "Hello\n");
        assert_eq!(line_indexer.line_of(5), "Hello\n");
        assert_eq!(line_indexer.line_of(6), "World\n");
        assert_eq!(line_indexer.line_of(11), "World\n");
    }

    #[test]
    fn tests() {
        let input = "Hello\nWorld\n";
        let line_indexer = CachedLineIndexer::new(input);

        test_line_col(input);
        test_line_col(&line_indexer);

        test_line_of(input);
        test_line_of(&line_indexer);

        for input in [
            "",
            "\n",
            "\n\n",
            "\r\n",
            "a\r\nb\r\n",
            "a\rb\r\n",
            "\r\n\r\n",
            "a\n\n",
            "a\nb\nc\n",
            "🦀🦀",
            "🦀\n🦀\n",
        ] {
            let line_indexer = CachedLineIndexer::new(input);
            for (pos, _) in input.char_indices() {
                assert_eq!(line_indexer.line_col(pos), input.line_col(pos));
                assert_eq!(line_indexer.line_of(pos), input.line_of(pos));
                assert_eq!(
                    line_indexer.find_line_start(pos),
                    input.find_line_start(pos)
                );
                assert_eq!(line_indexer.find_line_end(pos), input.find_line_end(pos));
            }
        }
    }
}
