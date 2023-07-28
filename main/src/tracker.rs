// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Tracker for parsing failures.

use core::cmp::Ordering;

use crate::RuleWrapper;

use super::{
    error::{Error, ErrorVariant},
    position::Position,
};
use alloc::{borrow::ToOwned, format, vec, vec::Vec};
use pest::RuleType;

enum SpecialError {
    /// Peek slice out of bound.
    SliceOutOfBound(i32, Option<i32>),
    /// Repeat too many times.
    RepeatTooManyTimes,
    /// Accessing elements in empty stack, such as Drop or Pop.
    EmptyStack,
}

/// Error tracker.
pub struct Tracker<'i, R: RuleType> {
    position: Position<'i>,
    positive: bool,
    positives: Vec<R>,
    negatives: Vec<R>,
    special: Vec<SpecialError>,
}
impl<'i, R: RuleType> Tracker<'i, R> {
    /// Create an empty tracker for attempts.
    pub fn new(pos: Position<'i>) -> Self {
        Self {
            position: pos,
            positive: true,
            positives: vec![],
            negatives: vec![],
            special: vec![],
        }
    }
    fn clear(&mut self) {
        self.positives.clear();
        self.negatives.clear();
        self.special.clear();
    }
    fn prepare(&mut self, pos: Position<'i>) -> bool {
        match pos.cmp(&self.position) {
            Ordering::Less => false,
            Ordering::Equal => true,
            Ordering::Greater => {
                self.clear();
                self.position = pos;
                true
            }
        }
    }
    fn during<Ret, const POSTIVE: bool>(&mut self, f: impl FnOnce(&mut Self) -> Ret) -> Ret {
        let original = self.positive;
        self.positive = POSTIVE;
        let res = f(self);
        self.positive = original;
        res
    }
    pub fn positive_during<Ret>(&mut self, f: impl FnOnce(&mut Self) -> Ret) -> Ret {
        self.during::<Ret, true>(f)
    }
    pub fn negative_during<Ret>(&mut self, f: impl FnOnce(&mut Self) -> Ret) -> Ret {
        self.during::<Ret, false>(f)
    }
    pub fn repeat_too_many_times(&mut self, pos: Position<'i>) {
        if self.prepare(pos) {
            self.special.push(SpecialError::RepeatTooManyTimes);
        }
    }
    pub fn out_of_bound(&mut self, pos: Position<'i>, start: i32, end: Option<i32>) {
        if self.prepare(pos) {
            self.special.push(SpecialError::SliceOutOfBound(start, end));
        }
    }
    pub fn empty_stack(&mut self, pos: Position<'i>) {
        if self.prepare(pos) {
            self.special.push(SpecialError::EmptyStack);
        }
    }
    #[inline]
    fn record(&mut self, rule: R, pos: Position<'i>, succeeded: bool) {
        if self.prepare(pos) {
            if self.positive {
                if !succeeded {
                    self.positives.push(rule);
                }
            } else {
                if succeeded {
                    self.negatives.push(rule);
                }
            }
        }
    }
    #[inline]
    pub fn record_during<Rule: RuleWrapper<R>, T, E>(
        &mut self,
        pos: Position<'i>,
        f: impl FnOnce(&mut Self) -> Result<T, E>,
    ) -> Result<T, E> {
        match f(self) {
            Ok(ok) => {
                self.record(Rule::RULE, pos, true);
                Ok(ok)
            }
            Err(err) => {
                self.record(Rule::RULE, pos, false);
                Err(err)
            }
        }
    }
    /// Collect attempts to `pest::error::Error<R>`
    pub fn collect(self) -> Error<R> {
        let pos = self.position;
        let mut positives = self.positives;
        let mut negatives = self.negatives;
        positives.sort();
        positives.dedup();
        negatives.sort();
        negatives.dedup();
        if !positives.is_empty() || !negatives.is_empty() {
            return Error::new_from_pos(
                ErrorVariant::ParsingError {
                    positives,
                    negatives,
                },
                pos,
            );
        }

        for special in self.special {
            return match special {
                SpecialError::SliceOutOfBound(start, end) => Error::new_from_pos(
                    ErrorVariant::CustomError {
                        message: match end {
                            Some(end) => format!("Peek slice {}..{} out of bound.", start, end),
                            None => format!("Peek slice {}.. out of bound.", start),
                        },
                    },
                    pos,
                ),
                SpecialError::RepeatTooManyTimes => Error::new_from_pos(
                    ErrorVariant::CustomError {
                        message: "Repeated too many times.".to_owned(),
                    },
                    pos,
                ),
                SpecialError::EmptyStack => Error::new_from_pos(
                    ErrorVariant::CustomError {
                        message: "Nothing to pop or drop.".to_owned(),
                    },
                    pos,
                ),
            };
        }

        Error::new_from_pos(
            ErrorVariant::ParsingError {
                positives: vec![],
                negatives: vec![],
            },
            pos,
        )
    }
}
