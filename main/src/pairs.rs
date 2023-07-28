// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

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
pub trait Pair<'i, R: RuleType>: Debug {
    fn as_span(&self) -> Span<'i>;
    fn as_rule(&self) -> R;
    fn as_str(&self) -> &'i str {
        self.as_span().as_str()
    }
    fn get_input(&self) -> &'i str {
        self.as_str()
    }
    /// None by default. Override if is a tag.
    fn get_node_tag(&self) -> Option<&str> {
        None
    }
    /// Atomic pairs.
    fn units(&self) -> Vec<&dyn Pair<'i, R>>;
    fn tokens(&self) -> Tokens<'i, R>;
    /// Not a member of [`pest::iterators::Pair`].
    fn find_first_tagged(&self, tag: &'i str) -> Option<&dyn Pair<'i, R>>;
    fn into_inner(self) -> Pairs<'i, R>;
    fn line_col(&self) -> (usize, usize) {
        self.as_span().start_pos().line_col()
    }
}

pub trait Node<'i: 'n, 'n, R: RuleType + 'i> {
    // Returned type. Must be an iterator.
    // type Ret: Iterator<Item = &'n dyn Pair<'i, R>> + 'n;
    fn iterate(&'n self) -> Vec<&'n dyn Pair<'i, R>>;
}

pub struct Pairs<'i, R: RuleType> {
    span: Span<'i>,
    pairs: Vec<Box<dyn Pair<'i, R>>>,
}
impl<'i, R: RuleType> Pairs<'i, R> {
    pub fn as_str(&self) -> &'i str {
        self.span.as_str()
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
    pub fn flatten<'n>(&'n self) -> FlatPairs<'i, 'n, R> {
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
        self.pairs.first().map(|boxed| boxed.as_ref())
    }
}

// Implementations
mod nodes {
    use super::*;
    use crate::{
        predefined_node::{Choice, Seq},
        NeverFailedTypedNode, TypedNode,
    };

    impl<
            'i: 'n,
            'n,
            R: RuleType,
            T1: TypedNode<'i, R> + Node<'i, 'n, R>,
            T2: TypedNode<'i, R> + Node<'i, 'n, R>,
            IGNORED: NeverFailedTypedNode<'i, R>,
        > Node<'i, 'n, R> for Seq<'i, R, T1, T2, IGNORED>
    {
        fn iterate(&'n self) -> Vec<&'n dyn Pair<'i, R>> {
            let mut res = self.first.iterate();
            res.append(&mut self.second.iterate());
            res
        }
    }

    impl<
            'i: 'n,
            'n,
            R: RuleType,
            T1: TypedNode<'i, R> + Node<'i, 'n, R>,
            T2: TypedNode<'i, R> + Node<'i, 'n, R>,
        > Node<'i, 'n, R> for Choice<'i, R, T1, T2>
    {
        fn iterate(&'n self) -> Vec<&'n dyn Pair<'i, R>> {
            match self {
                Self::First(first, _) => first.iterate(),
                Self::Second(second, _) => second.iterate(),
            }
        }
    }
}
