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
    line_indexer::LineIndexer,
    position::Position,
    Input, RuleType, RuleWrapper,
};
use alloc::{borrow::ToOwned, collections::BTreeMap, format, string::String, vec, vec::Vec};
use core::{
    borrow::Borrow,
    cmp::Ordering,
    fmt::{self, Display},
    marker::PhantomData,
};

/// Some special errors that are not matching failures.
pub enum SpecialError {
    /// Peek slice out of bound.
    SliceOutOfBound(i32, Option<i32>),
    /// Repeat too many times.
    RepeatTooManyTimes,
    /// Accessing elements in empty stack, such as Drop or Pop.
    EmptyStack,
}

impl Display for SpecialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SliceOutOfBound(start, end) => match end {
                Some(end) => write!(f, "Peek slice {}..{} out of bound.", start, end),
                None => write!(f, "Peek slice {}.. out of bound.", start),
            },
            Self::RepeatTooManyTimes => f.write_str("Repeated too many times."),
            Self::EmptyStack => f.write_str("Nothing to pop or drop."),
        }
    }
}

type Tracked<R> = (Vec<R>, Vec<R>, Vec<SpecialError>);

/// Error tracker.
pub struct Tracker<'i, R: RuleType, S: ?Sized = str> {
    position: Position<'i, S>,
    positive: bool,
    /// upper rule -> (positives, negatives)
    attempts: BTreeMap<Option<R>, Tracked<R>>,
    stack: Vec<(R, usize, bool)>,
    phantom: PhantomData<&'i ()>,
}
impl<'i, R: RuleType, S: ?Sized + Borrow<str>> Tracker<'i, R, S> {
    /// Create an empty tracker for attempts.
    pub fn new(pos: impl Input<'i, S>) -> Self {
        Self {
            position: pos.as_position(),
            positive: true,
            attempts: BTreeMap::new(),
            stack: vec![],
            phantom: PhantomData,
        }
    }
    fn clear(&mut self) {
        self.attempts.clear();
    }
    fn prepare(&mut self, pos: impl Input<'i, S>) -> bool {
        debug_assert_eq!(pos.input(), self.position.input());
        let pos = pos.as_position();
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
        pos: impl Input<'i, S>,
    ) -> &'s mut (Vec<R>, Vec<R>, Vec<SpecialError>) {
        // Find lowest rule with the different position.
        let mut upper = None;
        let pos = &pos.byte_offset();
        for (upper_rule, upper_pos, _) in self.stack.iter().rev() {
            if upper_pos != pos {
                upper = Some(*upper_rule);
                break;
            }
        }
        self.attempts.entry(upper).or_default()
    }
    /// Report a repetition that exceeds the limit.
    pub fn repeat_too_many_times(&mut self, pos: impl Input<'i, S>) {
        if self.prepare(pos) {
            self.get_entry(pos).2.push(SpecialError::RepeatTooManyTimes);
        }
    }
    /// Reports a stack slice operation that is out of bound.
    pub fn out_of_bound(&mut self, pos: impl Input<'i, S>, start: i32, end: Option<i32>) {
        if self.prepare(pos) {
            self.get_entry(pos)
                .2
                .push(SpecialError::SliceOutOfBound(start, end));
        }
    }
    /// Reports accessing operations on empty stack.
    pub fn empty_stack(&mut self, pos: impl Input<'i, S>) {
        if self.prepare(pos) {
            self.get_entry(pos).2.push(SpecialError::EmptyStack);
        }
    }
    fn same_with_last(vec: &[R], rule: R) -> bool {
        match vec.last() {
            Some(last) => *last == rule,
            None => false,
        }
    }
    #[inline]
    fn record(&mut self, rule: R, pos: impl Input<'i, S>, succeeded: bool) {
        if self.prepare(pos) && succeeded != self.positive {
            let positive = self.positive;
            let value = self.get_entry(pos);
            let vec = if positive { &mut value.0 } else { &mut value.1 };
            if !Self::same_with_last(vec, rule) {
                vec.push(rule);
            }
        }
    }
    /// Record if the result doesn't match the state during calling `f`.
    #[inline]
    pub fn record_during_with<Ret, I: Input<'i, S>>(
        &mut self,
        pos: I,
        f: impl FnOnce(&mut Self) -> Option<Ret>,
        rule: R,
    ) -> Option<Ret> {
        if let Some((_, _, has_children)) = self.stack.last_mut() {
            *has_children = true;
        }
        debug_assert_eq!(pos.input(), self.position.input());
        self.stack.push((rule, pos.byte_offset(), false));
        let res = f(self);
        let succeeded = res.is_some();
        let (_r, _pos, has_children) = self.stack.pop().unwrap();
        if !has_children {
            self.record(rule, pos, succeeded);
        }
        res
    }
    /// Record if the result doesn't match the state during calling `f`.
    #[inline]
    pub fn record_during<T: RuleWrapper<R>, I: Input<'i, S>>(
        &mut self,
        pos: I,
        f: impl FnOnce(&mut Self) -> Option<(I, T)>,
    ) -> Option<(I, T)> {
        self.record_during_with(pos, f, T::RULE)
    }
    fn collect_to_message(self) -> String
    where
        &'i S: LineIndexer<'i>,
    {
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
            .unwrap_or((line_string.len(), '\0'))
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
                    let _ = write!(message, "{}", special);
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
    pub fn collect(self) -> Error<R>
    where
        &'i S: LineIndexer<'i>,
    {
        let pos = self.position;
        match Position::new(pos.input, pos.pos()).ok_or_else(|| {
            Error::new_from_pos(
                ErrorVariant::CustomError {
                    message: format!("Internal error (invalid character index {}).", pos.pos()),
                },
                pest::Position::from_start(pos.input.borrow()),
            )
        }) {
            Ok(pos) => {
                let message = self.collect_to_message();
                Error::new_from_pos(ErrorVariant::CustomError { message }, pos.into())
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
    pub fn finish(self) -> (Position<'i, S>, BTreeMap<Option<R>, Tracked<R>>) {
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
    impl RuleType for Rule {
        fn name(&self) -> &'static str {
            match self {
                Rule::Program => "Program",
                Rule::SOI => "SOI",
                Rule::Main => "Main",
                Rule::Body => "Body",
                Rule::EOI => "EOI",
            }
        }
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
        let _ = tracker
            .record_during(pos, |tracker| {
                tracker.positive_during(|tracker| {
                    tracker.record_during(pos, |_| Some((pos, rule_wrappers::Main)))
                })?;
                tracker.negative_during(|tracker| {
                    tracker.record_during(pos, |_| Some((pos, rule_wrappers::Main)))
                })?;
                Some((pos, rule_wrappers::Program))
            })
            .ok_or(())?;

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
        let _ = tracker
            .record_during(pos, |tracker| {
                let _ = tracker.positive_during(|tracker| {
                    if false {
                        Some((pos, rule_wrappers::SOI))
                    } else {
                        tracker.record_during(pos, |_| None)
                    }
                });
                let _ = tracker.negative_during(|tracker| {
                    if false {
                        Some((pos, rule_wrappers::SOI))
                    } else {
                        tracker.record_during(pos, |_| None)
                    }
                });
                Some((pos, rule_wrappers::Program))
            })
            .ok_or(())?;

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
        let _ = tracker
            .record_during(pos, |tracker| {
                let suc = pos.match_string("α");
                assert!(suc);
                tracker.positive_during(|tracker| {
                    tracker.record_during(pos, |_| Some((pos, rule_wrappers::Main)))
                })?;
                tracker.negative_during(|tracker| {
                    tracker.record_during(pos, |_| Some((pos, rule_wrappers::Main)))
                })?;
                Some((pos, rule_wrappers::Program))
            })
            .ok_or(())?;

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
        let _ = tracker
            .record_during(pos, |tracker| {
                let suc = pos.match_string("α");
                assert!(suc);
                tracker.negative_during(|tracker| {
                    tracker.record_during(pos, |tracker| {
                        tracker.negative_during(|tracker| {
                            tracker.record_during(pos, |_| Some((pos, rule_wrappers::Body)))
                        })?;
                        Some((pos, rule_wrappers::Main))
                    })
                })?;
                Some((pos, rule_wrappers::Program))
            })
            .ok_or(())?;

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
