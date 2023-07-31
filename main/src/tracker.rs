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
use alloc::{
    borrow::{Cow, ToOwned},
    collections::BTreeMap,
    format,
    string::String,
    vec,
    vec::Vec,
};
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
    /// upper_rule -> (positives, negatives)
    attempts: BTreeMap<R, (Vec<R>, Vec<R>)>,
    special: Vec<SpecialError>,
    stack: Vec<(R, bool)>,
}
impl<'i, R: RuleType> Tracker<'i, R> {
    /// Create an empty tracker for attempts.
    pub fn new(pos: Position<'i>) -> Self {
        Self {
            position: pos,
            positive: true,
            attempts: BTreeMap::new(),
            special: vec![],
            stack: vec![],
        }
    }
    fn clear(&mut self) {
        self.attempts.clear();
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
    fn same_with_last(vec: &Vec<R>, rule: R) -> bool {
        match vec.last() {
            Some(last) => *last == rule,
            None => false,
        }
    }
    #[inline]
    fn record(&mut self, rule: R, upper_rule: R, pos: Position<'i>, succeeded: bool) {
        if self.prepare(pos) {
            match self.positive {
                true => {
                    if !succeeded {
                        let (vec, _) = self.attempts.entry(upper_rule).or_default();
                        if !Self::same_with_last(vec, rule) {
                            vec.push(rule);
                        }
                    }
                }
                false => {
                    if succeeded {
                        let (_, vec) = self.attempts.entry(upper_rule).or_default();
                        if !Self::same_with_last(vec, rule) {
                            vec.push(rule);
                        }
                    }
                }
            }
        }
    }
    #[inline]
    pub fn record_during<T: RuleWrapper<R>, E>(
        &mut self,
        pos: Position<'i>,
        f: impl FnOnce(&mut Self) -> Result<(Position<'i>, T), E>,
    ) -> Result<(Position<'i>, T), E> {
        if let Some((_, has_children)) = self.stack.last_mut() {
            *has_children = true;
        }
        self.stack.push((T::RULE, false));
        let res = f(self);
        let succeeded = match &res {
            Ok(_) => true,
            Err(_) => false,
        };
        let (rule, has_children) = self.stack.pop().unwrap();
        if !has_children {
            if let Some((upper, _)) = self.stack.last() {
                self.record(rule, *upper, pos, succeeded);
            }
        }
        res
    }
    /// Collect attempts to [`Error<R>`].
    pub fn collect(self) -> Error<R> {
        let pos = self.position;
        let mut attempts = self.attempts;
        for (_, (positives, negatives)) in attempts.iter_mut() {
            positives.sort();
            positives.dedup();
            negatives.sort();
            negatives.dedup();
        }
        fn collect_rules<R: RuleType>(vec: &Vec<R>) -> String {
            format!("{:?}", vec)
        }
        fn collect_attempts<R: RuleType>(
            upper_rule: &R,
            positives: &Vec<R>,
            negatives: &Vec<R>,
        ) -> Cow<'static, str> {
            match (positives.is_empty(), negatives.is_empty()) {
                (true, true) => Cow::Borrowed("Unknown error (no rule tracked)."),
                (false, true) => Cow::Owned(format!(
                    "Expected {}, by {:?}",
                    collect_rules(positives),
                    upper_rule,
                )),
                (true, false) => Cow::Owned(format!(
                    "Unexpected {}, by {:?}",
                    collect_rules(negatives),
                    upper_rule,
                )),
                (false, false) => Cow::Owned(format!(
                    "Unexpected {}, expected {}, by {:?}",
                    collect_rules(negatives),
                    collect_rules(positives),
                    upper_rule,
                )),
            }
        }
        if !attempts.is_empty() {
            let spacing = format!("{}", self.position.line_col().0).len() + 2;
            let spacing = "\n".to_owned() + &" ".repeat(spacing);
            // The four spaces after the `\n` is to aligh the lines, as there is a `  = ` in the first line.
            let message = attempts
                .iter()
                .map(|(upper_rule, (positives, negatives))| {
                    collect_attempts(upper_rule, positives, negatives)
                })
                .collect::<Vec<_>>()
                .join(spacing.as_str());
            return Error::new_from_pos(ErrorVariant::CustomError { message }, pos);
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
