// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Tracker for parsing failures.

use crate::{
    error::{Error, ErrorVariant},
    position::Position,
    RuleType, RuleWrapper,
};
use alloc::{
    borrow::ToOwned,
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::cmp::Ordering;

/// Some special errors that are not matching failures.
pub enum SpecialError {
    /// Peek slice out of bound.
    SliceOutOfBound(i32, Option<i32>),
    /// Repeat too many times.
    RepeatTooManyTimes,
    /// Accessing elements in empty stack, such as Drop or Pop.
    EmptyStack,
}

impl ToString for SpecialError {
    fn to_string(&self) -> String {
        match self {
            SpecialError::SliceOutOfBound(start, end) => match end {
                Some(end) => format!("Peek slice {}..{} out of bound.", start, end),
                None => format!("Peek slice {}.. out of bound.", start),
            },
            SpecialError::RepeatTooManyTimes => "Repeated too many times.".to_owned(),
            SpecialError::EmptyStack => "Nothing to pop or drop.".to_owned(),
        }
    }
}

type Tracked<R> = (Vec<R>, Vec<R>, Vec<SpecialError>);

/// Error tracker.
pub struct Tracker<'i, R: RuleType> {
    position: Position<'i>,
    positive: bool,
    /// upper rule -> (positives, negatives)
    attempts: BTreeMap<Option<R>, Tracked<R>>,
    stack: Vec<(R, Position<'i>, bool)>,
}
impl<'i, R: RuleType> Tracker<'i, R> {
    /// Create an empty tracker for attempts.
    pub fn new(pos: Position<'i>) -> Self {
        Self {
            position: pos,
            positive: true,
            attempts: BTreeMap::new(),
            stack: vec![],
        }
    }
    fn clear(&mut self) {
        self.attempts.clear();
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
    /// Set the tracker to positive during calling `f`.
    pub fn positive_during<Ret>(&mut self, f: impl FnOnce(&mut Self) -> Ret) -> Ret {
        self.during::<Ret, true>(f)
    }
    /// Set the tracker to negative during calling `f`.
    pub fn negative_during<Ret>(&mut self, f: impl FnOnce(&mut Self) -> Ret) -> Ret {
        self.during::<Ret, false>(f)
    }
    fn get_entry<'s>(
        &'s mut self,
        pos: &Position<'_>,
    ) -> &'s mut (Vec<R>, Vec<R>, Vec<SpecialError>) {
        // Find lowest rule with the different position.
        let mut upper = None;
        for (upper_rule, upper_pos, _) in self.stack.iter().rev() {
            if upper_pos != pos {
                upper = Some(*upper_rule);
                break;
            }
        }
        self.attempts.entry(upper).or_default()
    }
    /// Report a repetition that exceeds the limit.
    pub fn repeat_too_many_times(&mut self, pos: Position<'i>) {
        if self.prepare(pos) {
            self.get_entry(&pos)
                .2
                .push(SpecialError::RepeatTooManyTimes);
        }
    }
    /// Reports a stack slice operation that is out of bound.
    pub fn out_of_bound(&mut self, pos: Position<'i>, start: i32, end: Option<i32>) {
        if self.prepare(pos) {
            self.get_entry(&pos)
                .2
                .push(SpecialError::SliceOutOfBound(start, end));
        }
    }
    /// Reports accessing operations on empty stack.
    pub fn empty_stack(&mut self, pos: Position<'i>) {
        if self.prepare(pos) {
            self.get_entry(&pos).2.push(SpecialError::EmptyStack);
        }
    }
    fn same_with_last(vec: &[R], rule: R) -> bool {
        match vec.last() {
            Some(last) => *last == rule,
            None => false,
        }
    }
    #[inline]
    fn record(&mut self, rule: R, pos: Position<'i>, succeeded: bool) {
        if self.prepare(pos) && succeeded != self.positive {
            let positive = self.positive;
            let value = self.get_entry(&pos);
            let vec = if positive { &mut value.0 } else { &mut value.1 };
            if !Self::same_with_last(vec, rule) {
                vec.push(rule);
            }
        }
    }
    /// Record if the result doesn't match the state during calling `f`.
    #[inline]
    pub(crate) fn record_during_with<T, E>(
        &mut self,
        pos: Position<'i>,
        f: impl FnOnce(&mut Self) -> Result<(Position<'i>, T), E>,
        rule: R,
    ) -> Result<(Position<'i>, T), E> {
        if let Some((_, _, has_children)) = self.stack.last_mut() {
            *has_children = true;
        }
        self.stack.push((rule, pos, false));
        let res = f(self);
        let succeeded = res.is_ok();
        let (_r, _pos, has_children) = self.stack.pop().unwrap();
        if !has_children {
            self.record(rule, pos, succeeded);
        }
        res
    }
    /// Record if the result doesn't match the state during calling `f`.
    #[inline]
    pub fn record_during<T: RuleWrapper<R>, E>(
        &mut self,
        pos: Position<'i>,
        f: impl FnOnce(&mut Self) -> Result<(Position<'i>, T), E>,
    ) -> Result<(Position<'i>, T), E> {
        self.record_during_with(pos, f, T::RULE)
    }
    fn collect_to_message(self) -> String {
        let (pos, attempts) = self.finish();
        // "{} | "
        // "{} = "
        let (line, col) = pos.line_col();
        let spacing = format!("{}", line).len() + 3;
        let spacing = "\n".to_owned() + &" ".repeat(spacing);
        // Will not remove trailing CR or LF.
        let line_string = pos.line_of();
        let line_remained_index = line_string
            .char_indices()
            .nth(col.saturating_sub(1))
            .unwrap_or_else(|| (line_string.len(), '\0'))
            .0;
        let line_matched = &line_string[..line_remained_index];

        use core::fmt::Write;
        let mut message = String::new();

        let _ = write!(message, "{}^---", line_matched);

        let mut write_message =
            |(rule, (mut positives, mut negatives, special)): (Option<R>, Tracked<R>)| {
                positives.sort();
                positives.dedup();
                negatives.sort();
                negatives.dedup();
                fn collect_rules<R: RuleType>(vec: Vec<R>) -> String {
                    format!("{:?}", vec)
                }
                let _ = message.write_str(&spacing);
                let _ = match (positives.is_empty(), negatives.is_empty()) {
                    (true, true) => write!(message, "Unknown error (no rule tracked)"),
                    (false, true) => write!(message, "Expected {}", collect_rules(positives)),
                    (true, false) => write!(message, "Unexpected {}", collect_rules(negatives),),
                    (false, false) => write!(
                        message,
                        "Unexpected {}, expected {}",
                        collect_rules(negatives),
                        collect_rules(positives),
                    ),
                };
                if let Some(upper_rule) = rule {
                    let _ = write!(message, ", by {:?}", upper_rule);
                };
                let _ = write!(message, ".");

                for special in special {
                    let _ = message.write_str(&spacing);
                    let _ = write!(message, "{}", special.to_string());
                    if let Some(upper_rule) = rule {
                        let _ = write!(message, " (By {:?})", upper_rule);
                    };
                }
            };
        for attempt in attempts {
            write_message(attempt);
        }
        message
    }
    /// Collect attempts to [`Error<R>`]
    pub fn collect(self) -> Error<R> {
        let pos = self.position;
        match pest::Position::new(pos.input, pos.pos()).ok_or_else(|| {
            Error::new_from_pos(
                ErrorVariant::CustomError {
                    message: format!("Internal error (invalid character index {}).", pos.pos()),
                },
                pest::Position::from_start(pos.input),
            )
        }) {
            Ok(pos) => {
                let message = self.collect_to_message();
                Error::new_from_pos(ErrorVariant::CustomError { message }, pos)
            }
            Err(err) => err,
        }
    }
    /// Finish matching and convert the tracker into recorded information.
    ///
    /// Returned value is:
    ///
    /// - Current position.
    /// - Attempts on current position.
    ///
    /// This information is all you need to generate an [Error].
    pub fn finish(self) -> (Position<'i>, BTreeMap<Option<R>, Tracked<R>>) {
        (self.position, self.attempts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
    enum Rule {
        Program,
        SOI,
        Main,
        Body,
        EOI,
    }
    mod rule_wrappers {
        use super::Rule;
        use crate::RuleWrapper;

        macro_rules! wrap {
            ($name:ident) => {
                #[derive(Clone, PartialEq)]
                pub struct $name;
                impl RuleWrapper<Rule> for $name {
                    const RULE: Rule = Rule::$name;
                    type Rule = Rule;
                }
            };
        }
        wrap!(Program);
        wrap!(SOI);
        wrap!(Main);
        wrap!(Body);
        wrap!(EOI);
    }
    #[test]
    fn negative() -> Result<(), ()> {
        let pos = Position::from_start("abc\ndef\nghi");
        let mut tracker = Tracker::<'_, Rule>::new(pos);
        let _ = tracker.record_during(pos, |tracker| {
            tracker.positive_during(|tracker| {
                tracker.record_during(pos, |_| Ok((pos, rule_wrappers::Main)))
            })?;
            tracker.negative_during(|tracker| {
                tracker.record_during(pos, |_| Ok((pos, rule_wrappers::Main)))
            })?;
            Ok((pos, rule_wrappers::Program))
        })?;

        assert_eq!(
            format!("{}", tracker.collect()),
            r#" --> 1:1
  |
1 | abc
  | ^---
  |
  = ^---
    Unexpected [Main]."#
        );
        Ok(())
    }
    #[test]
    fn positive() -> Result<(), ()> {
        let pos = Position::from_start("abc\ndef\nghi");
        let mut tracker = Tracker::<'_, Rule>::new(pos);
        let _ = tracker.record_during(pos, |tracker| {
            let _ = tracker.positive_during(|tracker| {
                if false {
                    Ok((pos, rule_wrappers::SOI))
                } else {
                    tracker.record_during(pos, |_| Err(()))
                }
            });
            let _ = tracker.negative_during(|tracker| {
                if false {
                    Ok((pos, rule_wrappers::SOI))
                } else {
                    tracker.record_during(pos, |_| Err(()))
                }
            });
            Ok((pos, rule_wrappers::Program))
        })?;

        assert_eq!(
            format!("{}", tracker.collect()),
            r#" --> 1:1
  |
1 | abc
  | ^---
  |
  = ^---
    Expected [SOI]."#
        );
        Ok(())
    }
    #[test]
    fn unicode() -> Result<(), ()> {
        let mut pos = Position::from_start("αβψ\nδεφ\nγηι");
        let mut tracker = Tracker::<'_, Rule>::new(pos);
        let _ = tracker.record_during(pos, |tracker| {
            let suc = pos.match_string("α");
            assert!(suc);
            tracker.positive_during(|tracker| {
                tracker.record_during(pos, |_| Ok((pos, rule_wrappers::Main)))
            })?;
            tracker.negative_during(|tracker| {
                tracker.record_during(pos, |_| Ok((pos, rule_wrappers::Main)))
            })?;
            Ok((pos, rule_wrappers::Program))
        })?;

        assert_eq!(
            format!("{}", tracker.collect()),
            r#" --> 1:2
  |
1 | αβψ
  |  ^---
  |
  = α^---
    Unexpected [Main], by Program."#
        );
        Ok(())
    }
    #[test]
    fn nested() -> Result<(), ()> {
        let mut pos = Position::from_start("αβψ\nδεφ\nγηι");
        let mut tracker = Tracker::<'_, Rule>::new(pos);
        let _ = tracker.record_during(pos, |tracker| {
            let suc = pos.match_string("α");
            assert!(suc);
            tracker.negative_during(|tracker| {
                tracker.record_during(pos, |tracker| {
                    tracker.negative_during(|tracker| {
                        tracker.record_during(pos, |_| Ok((pos, rule_wrappers::Body)))
                    })?;
                    Ok((pos, rule_wrappers::Main))
                })
            })?;
            Ok((pos, rule_wrappers::Program))
        })?;

        assert_eq!(
            format!("{}", tracker.collect()),
            r#" --> 1:2
  |
1 | αβψ
  |  ^---
  |
  = α^---
    Unexpected [Body], by Program."#
        );
        Ok(())
    }
}
