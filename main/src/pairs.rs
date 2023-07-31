// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#![allow(unused)]
//! Simulate [`pest::iterators::Pair`] and [`pest::iterators::Pairs`] with **pest_typed**.

use crate::{RuleType, Span};
use alloc::{boxed::Box, collections::VecDeque, string::String, vec::Vec};
use core::{fmt::Debug, ops::Deref};
pub use nodes::*;
use pest::Token;

// Traits and default implementations.

#[derive(Clone, Debug)]
pub struct Tokens<'i, R: RuleType> {
    pub(super) tokens: VecDeque<Token<'i, R>>,
}
impl<'i, R: RuleType> Tokens<'i, R> {
    #[allow(unused)]
    pub(crate) fn new(tokens: impl Iterator<Item = Token<'i, R>>) -> Self {
        Self {
            tokens: tokens.collect(),
        }
    }
}
impl<'i, R: RuleType> Iterator for Tokens<'i, R> {
    type Item = Token<'i, R>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.pop_front()
    }
}
impl<'i, R: RuleType> ExactSizeIterator for Tokens<'i, R> {}
impl<'i, R: RuleType> DoubleEndedIterator for Tokens<'i, R> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.tokens.pop_back()
    }
}

#[derive(Clone, Debug)]
pub struct FlatPairs<'i: 'n, 'n, R: RuleType> {
    pub(super) pairs: VecDeque<&'n dyn Pair<'i, R>>,
}
impl<'i: 'n, 'n, R: RuleType> FlatPairs<'i, 'n, R> {
    pub(crate) fn new(pairs: impl Iterator<Item = &'n dyn Pair<'i, R>>) -> Self {
        Self {
            pairs: pairs.collect(),
        }
    }
}
impl<'i: 'n, 'n, R: RuleType> Iterator for FlatPairs<'i, 'n, R> {
    type Item = &'n dyn Pair<'i, R>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pairs.pop_front()
    }
}
impl<'i: 'n, 'n, R: RuleType> ExactSizeIterator for FlatPairs<'i, 'n, R> {}
impl<'i: 'n, 'n, R: RuleType> DoubleEndedIterator for FlatPairs<'i, 'n, R> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.pairs.pop_back()
    }
}

/// A matching pair of [`pest::Token`]s.
pub trait Pair<'i, R: RuleType>: Debug + Node<'i, R> {
    fn as_span(&self) -> Span<'i>;
    fn as_rule(&self) -> R;
    fn as_str(&self) -> &'i str {
        self.as_span().as_str()
    }
    fn get_input(&self) -> &'i str {
        self.as_str()
    }
    fn line_col(&self) -> (usize, usize) {
        self.as_span().start_pos().line_col()
    }
}

/// Nodes inside a rule.
pub trait Node<'i, R: RuleType> {
    /// None by default. Override if is a tag.
    fn get_node_tag(&self) -> Option<&str> {
        None
    }
    /// Atomic pairs.
    fn units<'n>(&'n self) -> Vec<&'n dyn Pair<'i, R>>;
    fn tokens(&self) -> Tokens<'i, R>;
    /// Not a member of [`pest::iterators::Pair`].
    fn find_first_tagged(&self, tag: &'i str) -> Option<&dyn Pair<'i, R>>;
    fn as_inner<'n>(&'n self) -> Pairs<'i, 'n, R>;
}

pub struct Pairs<'i: 'n, 'n, R: RuleType> {
    pairs: Vec<&'n dyn Pair<'i, R>>,
}
impl<'i: 'n, 'n, R: RuleType> Pairs<'i, 'n, R> {
    pub fn as_span(&self) -> Span<'i> {
        let start = self.pairs.first().expect("Pairs can't be empty.");
        let end = self.pairs.last().expect("Pairs can't be empty.");
        start.as_span().start_pos().span(&end.as_span().end_pos())
    }
    pub fn as_str(&self) -> &'i str {
        self.as_span().as_str()
    }
    pub fn get_input(&self) -> &'i str {
        self.as_str()
    }
    pub fn concat(&self) -> String {
        self.pairs
            .iter()
            .map(|pair| pair.as_str())
            .collect::<Vec<_>>()
            .concat()
    }
    pub fn flatten(&'n self) -> FlatPairs<'i, 'n, R> {
        let iter = self.pairs.iter().flat_map(|pair| pair.units());
        FlatPairs::new(iter)
    }
    pub fn find_first_tagged(&self, tag: &'i str) -> Option<&dyn Pair<'i, R>> {
        for pair in self.pairs.iter() {
            if let Some(first_tagged) = pair.deref().find_first_tagged(tag) {
                return Some(first_tagged);
            }
        }
        None
    }
    pub fn peek(&self) -> Option<&dyn Pair<'i, R>> {
        self.pairs.first().copied()
    }
}

// Implementations
mod nodes {
    use super::*;
    use crate::{
        predefined_node::{AtomicRule, Choice, Seq},
        NeverFailedTypedNode, RuleWrapper, TypedNode,
    };
    use alloc::vec;

    impl<
            'i,
            R: RuleType,
            T1: TypedNode<'i, R> + Node<'i, R>,
            T2: TypedNode<'i, R> + Node<'i, R>,
            IGNORED: NeverFailedTypedNode<'i, R>,
        > Node<'i, R> for Seq<'i, R, T1, T2, IGNORED>
    {
        fn units<'n>(&'n self) -> Vec<&'n dyn Pair<'i, R>> {
            let mut res = self.first.units();
            res.append(&mut self.second.units());
            res
        }

        fn tokens(&self) -> Tokens<'i, R> {
            todo!()
        }

        fn find_first_tagged(&self, tag: &'i str) -> Option<&dyn Pair<'i, R>> {
            if let Some(tagged) = self.first.find_first_tagged(tag) {
                Some(tagged)
            } else if let Some(tagged) = self.second.find_first_tagged(tag) {
                Some(tagged)
            } else {
                None
            }
        }

        fn as_inner<'n>(&'n self) -> Pairs<'i, 'n, R> {
            todo!()
        }
    }

    impl<
            'i,
            R: RuleType,
            T1: TypedNode<'i, R> + Node<'i, R>,
            T2: TypedNode<'i, R> + Node<'i, R>,
        > Node<'i, R> for Choice<'i, R, T1, T2>
    {
        fn units<'n>(&'n self) -> Vec<&'n dyn Pair<'i, R>> {
            match self {
                Self::First(first, _) => first.units(),
                Self::Second(second, _) => second.units(),
            }
        }

        fn tokens(&self) -> Tokens<'i, R> {
            todo!()
        }

        fn find_first_tagged(&self, tag: &'i str) -> Option<&dyn Pair<'i, R>> {
            todo!()
        }

        fn as_inner<'n>(&'n self) -> Pairs<'i, 'n, R> {
            todo!()
        }
    }

    impl<
            'i,
            R: RuleType,
            T: TypedNode<'i, R> + Node<'i, R>,
            RULE: RuleWrapper<R>,
            _EOI: RuleWrapper<R>,
        > Node<'i, R> for AtomicRule<'i, R, T, RULE, _EOI>
    {
        fn units<'n>(&'n self) -> Vec<&'n dyn Pair<'i, R>> {
            vec![self]
        }

        fn tokens(&self) -> Tokens<'i, R> {
            todo!()
        }

        fn find_first_tagged(&self, tag: &'i str) -> Option<&dyn Pair<'i, R>> {
            todo!()
        }

        fn as_inner<'n>(&'n self) -> Pairs<'i, 'n, R> {
            todo!()
        }
    }
    impl<
            'i,
            R: RuleType,
            T: TypedNode<'i, R> + Node<'i, R>,
            RULE: RuleWrapper<R>,
            _EOI: RuleWrapper<R>,
        > Pair<'i, R> for AtomicRule<'i, R, T, RULE, _EOI>
    {
        fn as_span(&self) -> Span<'i> {
            self.span
        }

        fn as_rule(&self) -> R {
            RULE::RULE
        }
    }
}
