// pest-typed. A statically typed version of pest.
// Copyright (c) 2025 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! A trait and a type for splitting a string into lines.

use crate::input::RefStr;
use alloc::vec::Vec;

/// A trait for splitting a string into lines.
///
/// It supports getting the line of a given position in the string,
/// and finding the start and end of the line.
pub trait LineIndexer<S: RefStr> {
    /// Returns the line and column number of this `Position`.
    fn line_col(&self, input: &S, pos: usize) -> (usize, usize);

    /// Returns the entire line of the input that contains the position at `pos`.
    fn line_of(&self, input: &S, pos: usize) -> S;

    /// Returns the start of the line that contains the position at `pos`.
    fn find_line_start(&self, input: &S, pos: usize) -> usize;

    /// Returns the end of the line that contains the position at `pos`.
    fn find_line_end(&self, input: &S, pos: usize) -> usize;
}

impl<S: RefStr, T: LineIndexer<S>> LineIndexer<S> for &T {
    #[inline(always)]
    fn line_col(&self, input: &S, pos: usize) -> (usize, usize) {
        T::line_col(*self, input, pos)
    }

    #[inline(always)]
    fn line_of(&self, input: &S, pos: usize) -> S {
        T::line_of(*self, input, pos)
    }

    #[inline(always)]
    fn find_line_start(&self, input: &S, pos: usize) -> usize {
        T::find_line_start(*self, input, pos)
    }

    #[inline(always)]
    fn find_line_end(&self, input: &S, pos: usize) -> usize {
        T::find_line_end(*self, input, pos)
    }
}

impl<S: RefStr> LineIndexer<S> for () {
    /// Returns the line and column number of this `Position`.
    ///
    /// This is an O(n) operation, where n is the number of chars in the input.
    /// You better use [`pair.line_col()`](struct.Pair.html#method.line_col) instead.
    #[inline]
    fn line_col(&self, input: &S, pos: usize) -> (usize, usize) {
        let mut pos = pos;
        let slice = &input.get_checked(..pos);
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

    #[inline]
    fn line_of(&self, input: &S, pos: usize) -> S {
        input.get_checked(self.find_line_start(input, pos)..self.find_line_end(input, pos))
    }

    #[inline]
    fn find_line_start(&self, input: &S, pos: usize) -> usize {
        let start = input
            .as_str()
            .char_indices()
            .rev()
            .skip_while(|&(i, _)| i >= pos)
            .find(|&(_, c)| c == '\n');
        match start {
            Some((i, _)) => i + 1,
            None => 0,
        }
    }

    #[inline]
    fn find_line_end(&self, input: &S, pos: usize) -> usize {
        let end = input
            .as_str()
            .char_indices()
            .skip_while(|&(i, _)| i < pos)
            .find(|&(_, c)| c == '\n');
        match end {
            Some((i, _)) => i + 1,
            None => input.len(),
        }
    }
}

/// A cached line indexer that caches the start of each line in the input string.
#[derive(Debug, Clone)]
pub struct CachedLineIndexer {
    /// `line_starts[i + 1]` is the start of the `i`-th line.
    line_starts: Vec<usize>,
}

impl CachedLineIndexer {
    /// Creates a new `CachedLineIndexer` from the input string.
    #[inline]
    pub fn new(input: &str) -> Self {
        let mut line_starts = Vec::new();
        for (i, c) in input.char_indices() {
            if c == '\n' {
                line_starts.push(i + 1);
            }
        }
        Self { line_starts }
    }
    /// Creates a new `CachedLineIndexer` with no lines.
    #[inline]
    pub const fn empty() -> Self {
        Self {
            line_starts: Vec::new(),
        }
    }
}

impl<S: RefStr> LineIndexer<S> for CachedLineIndexer {
    #[inline]
    fn line_col(&self, input: &S, pos: usize) -> (usize, usize) {
        let line = self.line_starts.partition_point(|&start| start <= pos);
        let line_start = line
            .checked_sub(1)
            .map_or_else(|| 0, |line| self.line_starts[line]);
        let col = input.get_checked(line_start..pos).chars().count(); // Still O(n) but faster than the above.
        (line + 1, col + 1)
    }

    #[inline]
    fn line_of(&self, input: &S, pos: usize) -> S {
        let line = self.line_starts.partition_point(|&start| start <= pos);
        let line_start = line
            .checked_sub(1)
            .map_or_else(|| 0, |line| self.line_starts[line]);
        let next_line_start = self
            .line_starts
            .get(line)
            .copied()
            .unwrap_or_else(|| input.len());
        input.get_checked(line_start..next_line_start)
    }

    #[inline]
    fn find_line_start(&self, _: &S, pos: usize) -> usize {
        let line = self.line_starts.partition_point(|&start| start <= pos);
        line.checked_sub(1)
            .map_or_else(|| 0, |line| self.line_starts[line])
    }

    #[inline]
    fn find_line_end(&self, input: &S, pos: usize) -> usize {
        let line = self.line_starts.partition_point(|&start| start <= pos);
        self.line_starts
            .get(line)
            .copied()
            .unwrap_or_else(|| input.len())
    }
}

#[cfg(test)]
mod tests {
    use super::{CachedLineIndexer, LineIndexer};
    use rand::SeedableRng as _;
    use rand_utf8::rand_utf8;

    fn test_line_col<'i, T: LineIndexer<&'i str>>(input: &'i str, line_indexer: T) {
        assert_eq!(line_indexer.line_col(&input, 0), (1, 1));
        assert_eq!(line_indexer.line_col(&input, 5), (1, 6));
        assert_eq!(line_indexer.line_col(&input, 6), (2, 1));
        assert_eq!(line_indexer.line_col(&input, 11), (2, 6));
    }

    fn test_line_of<'i, T: LineIndexer<&'i str>>(input: &'i str, line_indexer: T) {
        assert_eq!(line_indexer.line_of(&input, 0), "Hello\n");
        assert_eq!(line_indexer.line_of(&input, 5), "Hello\n");
        assert_eq!(line_indexer.line_of(&input, 6), "World\n");
        assert_eq!(line_indexer.line_of(&input, 11), "World\n");
    }

    #[test]
    fn tests() {
        let input = "Hello\nWorld\n";
        let line_indexer = CachedLineIndexer::new(input);

        test_line_col(input, ());
        test_line_col(input, &line_indexer);

        test_line_of(input, ());
        test_line_of(input, &line_indexer);

        let mut rng = rand::rngs::SmallRng::seed_from_u64(0);

        let f = |input: &str| {
            let line_indexer = CachedLineIndexer::new(input);
            for (pos, _) in input.char_indices() {
                assert_eq!(line_indexer.line_col(&input, pos), ().line_col(&input, pos));
                assert_eq!(line_indexer.line_of(&input, pos), ().line_of(&input, pos));
                assert_eq!(
                    line_indexer.find_line_start(&input, pos),
                    ().find_line_start(&input, pos)
                );
                assert_eq!(
                    line_indexer.find_line_end(&input, pos),
                    ().find_line_end(&input, pos)
                );
            }
        };

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
            f(input)
        }

        for _ in 0..100 {
            let input = rand_utf8(&mut rng, 1000);
            f(&input);
        }
    }
}
