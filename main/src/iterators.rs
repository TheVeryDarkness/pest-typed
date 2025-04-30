// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Simulates [`pest::iterators`].

use crate::{
    predefined_node::{
        AlwaysFail, AtomicRepeat, CharRange, Empty, Insens, Negative, PeekSlice1, PeekSlice2,
        Positive, Push, RepeatMin, RepeatMinMax, Skip, Skipped, Str, ANY, DROP, NEWLINE, PEEK,
        PEEK_ALL, POP, POP_ALL, SOI,
    },
    typed_node::{RuleStorage, RuleStruct, Spanned},
    RuleType, Span, StringArrayWrapper, StringWrapper, TypedNode,
};
use alloc::{boxed::Box, collections::VecDeque, string::String, vec::Vec};
use core::{
    iter::{once, Iterator},
    mem::swap,
};
#[cfg(feature = "serde")]
use serde::ser::SerializeStruct;

/// Token.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThinToken<R: RuleType> {
    /// Rule.
    pub rule: R,
    /// Start position.
    pub start: usize,
    /// End position.
    pub end: usize,
    /// Children.
    pub children: Vec<Self>,
}

/// Token.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token<'i, R: RuleType> {
    /// Rule.
    pub rule: R,
    /// Span.
    pub span: Span<'i>,
    /// Children.
    pub children: Vec<Self>,
}

impl<'i, R: RuleType> Token<'i, R> {
    /// To [`ThinToken`].
    pub fn to_thin(&self) -> ThinToken<R> {
        let rule = self.rule;
        let start = self.span.start();
        let end = self.span.end();
        let children = self.children.iter().map(|c| c.to_thin()).collect();
        ThinToken {
            rule,
            start,
            end,
            children,
        }
    }
}

#[cfg(feature = "serde")]
impl<'i, R: RuleType> serde::Serialize for Token<'i, R> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_struct(self.rule.name(), 2)?;

        seq.serialize_field("content", &self.span.as_str())?;
        seq.serialize_field("children", &self.children)?;

        seq.end()
    }
}

/// Simulate [`pest::iterators::Pairs`].
pub trait Pairs<'i, R: RuleType> {
    /// For each inner pair by reference if this is a container, otherwise for self.
    fn for_self_or_each_child(&self, f: &mut impl FnMut(Token<'i, R>));
    /// Collect inner pairs and make them into a [`Vec`].
    fn self_or_children(&self) -> Vec<Token<'i, R>> {
        let mut children = Vec::new();
        self.for_self_or_each_child(&mut |token| children.push(token));
        children
    }
}

/// Simulate [`pest::iterators::Pair`].
pub trait Pair<'i, R: RuleType>: Spanned<'i, R> + RuleStorage<R> {
    /// Iterate over all inner children.
    fn for_each_child(&self, f: impl FnMut(Token<'i, R>));
    /// Collect inner [Token]s and make them into a [`Vec`].
    fn children(&self) -> Vec<Token<'i, R>> {
        let mut children = Vec::new();
        self.for_each_child(|token| children.push(token));
        children
    }
    /// As [Token]s. Call [`Pair::children`] inside.
    fn as_token(&self) -> Token<'i, R> {
        let children = self.children();
        Token::<R> {
            rule: self.rule(),
            span: self.span(),
            children,
        }
    }
    /// As [Token]s. Call [`Pair::children`] inside.
    fn as_thin_token(&self) -> ThinToken<R> {
        self.as_token().to_thin()
    }
}

fn iterate_level_order<'i, R: RuleType, E>(
    p: &impl Pair<'i, R>,
    mut f: impl FnMut(&Token<'i, R>, usize) -> Result<(), E>,
) -> Result<(), E> {
    let mut queue: VecDeque<Token<'i, R>> = VecDeque::new();
    let mut next = VecDeque::new();
    queue.push_back(p.as_token());
    loop {
        while let Some(p) = queue.pop_front() {
            f(&p, queue.len())?;
            next.extend(p.children);
        }
        swap(&mut queue, &mut next);
        if queue.is_empty() {
            return Ok(());
        }
    }
}
/// Pre-order traversal
fn iterate_pre_order<'i, R: RuleType, E>(
    p: &impl Pair<'i, R>,
    mut f: impl FnMut(&Token<'i, R>, usize) -> Result<(), E>,
) -> Result<(), E> {
    let mut stack: Vec<VecDeque<Token<'i, R>>> = Vec::new();

    let root: Token<'i, R> = p.as_token();
    stack.push(VecDeque::<Token<'i, R>>::from_iter(once(root)));

    loop {
        if let Some(parent) = stack.last_mut() {
            if let Some(first) = parent.pop_front() {
                f(&first, stack.len() - 1)?;
                stack.push(first.children.into());
            } else {
                stack.pop();
            }
        } else {
            return Ok(());
        }
    }
}

/// Write the tree to.
fn write_tree_to<'i: 'n, 'n, R: RuleType + 'n>(
    p: &'n impl Pair<'i, R>,
    buf: &mut impl core::fmt::Write,
) -> core::fmt::Result {
    iterate_pre_order(p, |p, depth| {
        if p.children.is_empty() {
            buf.write_fmt(format_args!(
                "{}{:?} {:?}\n",
                &"    ".repeat(depth),
                p.rule,
                p.span.as_str(),
            ))
        } else {
            buf.write_fmt(format_args!("{}{:?}\n", &"    ".repeat(depth), p.rule))
        }
    })
}

/// A trait to traverse the pair as the root of a tree.
pub trait PairTree<'i, R: RuleType>: Pair<'i, R> + Sized {
    /// Level order traversal
    fn iterate_level_order<E>(
        &self,
        f: impl FnMut(&Token<'i, R>, usize) -> Result<(), E>,
    ) -> Result<(), E> {
        iterate_level_order(self, f)
    }
    /// Pre-order traversal
    fn iterate_pre_order<E>(
        &self,
        f: impl FnMut(&Token<'i, R>, usize) -> Result<(), E>,
    ) -> Result<(), E> {
        iterate_pre_order(self, f)
    }

    /// Write the tree to the `buf`.
    fn write_tree_to(&self, buf: &mut impl core::fmt::Write) -> core::fmt::Result {
        write_tree_to(self, buf)
    }

    /// Format as a tree.
    fn format_as_tree(&self) -> Result<String, core::fmt::Error> {
        let mut buf = String::new();
        self.write_tree_to(&mut buf)?;
        Ok(buf)
    }
}

impl<'i, R: RuleType, T: RuleStruct<'i, R> + Pairs<'i, R> + Pair<'i, R>> PairTree<'i, R> for T {}

macro_rules! impl_empty {
    ($node:ty, $($tt:tt)*) => {
        impl<'i, R: RuleType, $($tt)*> Pairs<'i, R> for $node {
            fn for_self_or_each_child(&self, _f: &mut impl FnMut(Token<'i, R>)) {}
        }
    };
}

macro_rules! impl_forward_inner {
    ($node:ident) => {
        impl<'i, R: RuleType, T: TypedNode<'i, R> + Pairs<'i, R>> Pairs<'i, R> for $node<T> {
            fn for_self_or_each_child(&self, f: &mut impl FnMut(Token<'i, R>)) {
                self.content.for_self_or_each_child(f)
            }
        }
    };
}

impl_empty!(Str<T>, T: StringWrapper);
impl_empty!(Insens<'i, T>, T: StringWrapper);
impl_empty!(PeekSlice2<START, END>, const START: i32, const END: i32);
impl_empty!(PeekSlice1<START>, const START: i32);
impl_forward_inner!(Push);
impl_empty!(Skip<'i, Strings>, Strings: StringArrayWrapper);
impl_empty!(CharRange<MIN, MAX>, const MIN: char, const MAX: char);
impl_empty!(Positive<T>, T: TypedNode<'i, R>);
impl_empty!(Negative<T>, T: TypedNode<'i, R>);

impl<'i, R: RuleType, T1: Pairs<'i, R>, T2: Pairs<'i, R>> Pairs<'i, R> for (T1, T2) {
    fn for_self_or_each_child(&self, f: &mut impl FnMut(Token<'i, R>)) {
        self.0.for_self_or_each_child(f);
        self.1.for_self_or_each_child(f);
    }
}

impl<'i, R: RuleType, T: Pairs<'i, R>, const N: usize> Pairs<'i, R> for [T; N] {
    fn for_self_or_each_child(&self, f: &mut impl FnMut(Token<'i, R>)) {
        self.as_slice()
            .iter()
            .for_each(|n| n.for_self_or_each_child(f))
    }
}

impl<'i, R: RuleType, T: TypedNode<'i, R> + Pairs<'i, R>> Pairs<'i, R> for Box<T> {
    fn for_self_or_each_child(&self, f: &mut impl FnMut(Token<'i, R>)) {
        self.as_ref().for_self_or_each_child(f)
    }
}

impl<'i, R: RuleType, T: TypedNode<'i, R> + Pairs<'i, R>> Pairs<'i, R> for Option<T> {
    fn for_self_or_each_child(&self, f: &mut impl FnMut(Token<'i, R>)) {
        if let Some(node) = self {
            node.for_self_or_each_child(f)
        }
    }
}

impl<'i, R: RuleType, T: Pairs<'i, R>, Skip: Pairs<'i, R>, const SKIP: usize> Pairs<'i, R>
    for Skipped<T, Skip, SKIP>
{
    fn for_self_or_each_child(&self, f: &mut impl FnMut(Token<'i, R>)) {
        self.skipped.for_self_or_each_child(f);
        self.matched.for_self_or_each_child(f);
    }
}

macro_rules! impl_with_vec {
    ($name:ident, $(const $args:ident : $t:ty,)*) => {
        impl<
                'i,
                R: RuleType,
                T: Pairs<'i, R>,
                $(const $args: $t, )*
            > Pairs<'i, R> for $name<T, $($args, )*>
        {
            fn for_self_or_each_child(&self, f: &mut impl FnMut(Token<'i, R>)) {
                self.content.iter().for_each(|n| n.for_self_or_each_child(f));
            }
        }
    };
}

impl_with_vec!(RepeatMinMax, const MIN: usize, const MAX: usize,);
impl_with_vec!(RepeatMin, const MIN: usize,);
impl_with_vec!(AtomicRepeat,);

macro_rules! impl_without_lifetime {
    ($id: ident) => {
        impl<'i, R: RuleType> Pairs<'i, R> for $id {
            fn for_self_or_each_child(&self, _f: &mut impl FnMut(Token<'i, R>)) {}
        }
    };
}
macro_rules! impl_with_lifetime {
    ($id: ident) => {
        impl<'i, R: RuleType> Pairs<'i, R> for $id<'i> {
            fn for_self_or_each_child(&self, _f: &mut impl FnMut(Token<'i, R>)) {}
        }
    };
}

impl_without_lifetime!(ANY);
impl_without_lifetime!(SOI);
impl_without_lifetime!(NEWLINE);
impl_with_lifetime!(PEEK);
impl_with_lifetime!(PEEK_ALL);
impl_with_lifetime!(POP);
impl_with_lifetime!(POP_ALL);
impl_without_lifetime!(DROP);

impl_with_lifetime!(AlwaysFail);
impl_with_lifetime!(Empty);

/// An iterator that maybe contain another iterator.
pub struct Maybe<Item, T: Iterator<Item = Item>>(Option<T>);
impl<Item, T: Iterator<Item = Item>> Iterator for Maybe<Item, T> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            Some(inner) => inner.next(),
            None => None,
        }
    }
}
