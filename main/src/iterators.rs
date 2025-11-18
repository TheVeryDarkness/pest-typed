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
    input::RefStr,
    predefined_node::{
        AlwaysFail, AtomicRepeat, CharRange, Empty, Insens, Negative, PeekSlice1, PeekSlice2,
        Positive, Push, RepeatMin, RepeatMinMax, Skip, Skipped, Str, ANY, DROP, NEWLINE, PEEK,
        PEEK_ALL, POP, POP_ALL, SOI,
    },
    typed_node::{RuleStorage, RuleStruct, Spanned},
    RuleType, Span, StringArrayWrapper, StringWrapper,
};
use alloc::{boxed::Box, collections::VecDeque, string::String, vec::Vec};
use core::{
    iter::{once, Iterator},
    mem::swap,
};
use derive_where::derive_where;
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
#[derive_where(Clone, Debug, Hash, Eq, PartialEq; S: RefStr)]
pub struct Token<S, R: RuleType> {
    /// Rule.
    pub rule: R,
    /// Span.
    pub span: Span<S>,
    /// Children.
    pub children: Vec<Self>,
}

impl<S, R: RuleType> Token<S, R> {
    /// To [`ThinToken`].
    #[inline]
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
impl<Str: RefStr, R: RuleType> serde::Serialize for Token<Str, R> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_struct(self.rule.name(), 3)?;

        seq.serialize_field("type", self.rule.name())?;
        seq.serialize_field("content", self.span.as_str().as_str())?;
        seq.serialize_field("children", &self.children)?;

        seq.end()
    }
}

/// Simulate [`pest::iterators::Pairs`].
pub trait Pairs<S, R: RuleType> {
    /// For each inner pair by reference if this is a container, otherwise for self.
    fn for_self_or_each_child(&self, f: &mut impl FnMut(Token<S, R>));
    /// Collect inner pairs and make them into a [`Vec`].
    #[inline]
    fn self_or_children(&self) -> Vec<Token<S, R>> {
        let mut children = Vec::new();
        self.for_self_or_each_child(&mut |token| children.push(token));
        children
    }
}

/// Simulate [`pest::iterators::Pair`].
pub trait Pair<S: RefStr, R: RuleType>: Spanned<S, R> + RuleStorage<R> {
    /// Iterate over all inner children.
    fn for_each_child(&self, f: impl FnMut(Token<S, R>));
    /// Collect inner [Token]s and make them into a [`Vec`].
    #[inline]
    fn children(&self) -> Vec<Token<S, R>> {
        let mut children = Vec::new();
        self.for_each_child(|token| children.push(token));
        children
    }
    /// As [Token]s. Call [`Pair::children`] inside.
    #[inline]
    fn as_token(&self) -> Token<S, R> {
        let children = self.children();
        Token::<S, R> {
            rule: self.rule(),
            span: self.span(),
            children,
        }
    }
    /// As [Token]s. Call [`Pair::children`] inside.
    #[inline]
    fn as_thin_token(&self) -> ThinToken<R> {
        self.as_token().to_thin()
    }
}

fn iterate_level_order<S: RefStr, R: RuleType, E>(
    p: &impl Pair<S, R>,
    mut f: impl FnMut(&Token<S, R>, usize) -> Result<(), E>,
) -> Result<(), E> {
    let mut queue: VecDeque<Token<S, R>> = VecDeque::new();
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
fn iterate_pre_order<S: RefStr, R: RuleType, E>(
    p: &impl Pair<S, R>,
    mut f: impl FnMut(&Token<S, R>, usize) -> Result<(), E>,
) -> Result<(), E> {
    let mut stack: Vec<VecDeque<Token<S, R>>> = Vec::new();

    let root: Token<S, R> = p.as_token();
    stack.push(VecDeque::<Token<S, R>>::from_iter(once(root)));

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
fn write_tree_to<'n, S: RefStr, R: RuleType + 'n>(
    p: &'n impl Pair<S, R>,
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
pub trait PairTree<S: RefStr, R: RuleType>: Pair<S, R> + Sized {
    /// Level order traversal
    #[inline]
    fn iterate_level_order<E>(
        &self,
        f: impl FnMut(&Token<S, R>, usize) -> Result<(), E>,
    ) -> Result<(), E> {
        iterate_level_order(self, f)
    }
    /// Pre-order traversal
    #[inline]
    fn iterate_pre_order<E>(
        &self,
        f: impl FnMut(&Token<S, R>, usize) -> Result<(), E>,
    ) -> Result<(), E> {
        iterate_pre_order(self, f)
    }

    /// Write the tree to the `buf`.
    #[inline]
    fn write_tree_to(&self, buf: &mut impl core::fmt::Write) -> core::fmt::Result {
        write_tree_to(self, buf)
    }

    /// Format as a tree.
    #[inline]
    fn format_as_tree(&self) -> Result<String, core::fmt::Error> {
        let mut buf = String::new();
        self.write_tree_to(&mut buf)?;
        Ok(buf)
    }
}

impl<S: RefStr, R: RuleType, T: RuleStruct<S, R> + Pairs<S, R> + Pair<S, R>> PairTree<S, R> for T {}

macro_rules! impl_empty {
    ($node:ty $(, $($tt:tt)*)?) => {
        impl<S: RefStr, R: RuleType $(, $($tt)*)?> Pairs<S, R> for $node {
            #[inline(always)]
            fn for_self_or_each_child(&self, _f: &mut impl FnMut(Token<S, R>)) {}
        }
    };
}

macro_rules! impl_forward_inner {
    ($node:ident) => {
        impl<S: RefStr, R: RuleType, T: Pairs<S, R>> Pairs<S, R> for $node<T> {
            #[inline(always)]
            fn for_self_or_each_child(&self, f: &mut impl FnMut(Token<S, R>)) {
                self.content.for_self_or_each_child(f)
            }
        }
    };
}

impl_empty!(Str<T>, T: StringWrapper);
impl_empty!(Insens<S, T>, T: StringWrapper);
impl_empty!(PeekSlice2<START, END>, const START: i32, const END: i32);
impl_empty!(PeekSlice1<START>, const START: i32);
impl_forward_inner!(Push);
impl_empty!(Skip<S, Strings>, Strings: StringArrayWrapper);
impl_empty!(CharRange<MIN, MAX>, const MIN: char, const MAX: char);
impl_empty!(Positive<T>, T);
impl_empty!(Negative<T>, T);

impl<S: RefStr, R: RuleType, T1: Pairs<S, R>, T2: Pairs<S, R>> Pairs<S, R> for (T1, T2) {
    #[inline]
    fn for_self_or_each_child(&self, f: &mut impl FnMut(Token<S, R>)) {
        self.0.for_self_or_each_child(f);
        self.1.for_self_or_each_child(f);
    }
}

impl<S, R: RuleType, T: Pairs<S, R>, const N: usize> Pairs<S, R> for [T; N] {
    #[inline]
    fn for_self_or_each_child(&self, f: &mut impl FnMut(Token<S, R>)) {
        self.as_slice()
            .iter()
            .for_each(|n| n.for_self_or_each_child(f))
    }
}

impl<S, R: RuleType, T: Pairs<S, R>> Pairs<S, R> for Box<T> {
    #[inline]
    fn for_self_or_each_child(&self, f: &mut impl FnMut(Token<S, R>)) {
        self.as_ref().for_self_or_each_child(f)
    }
}

impl<S, R: RuleType, T: Pairs<S, R>> Pairs<S, R> for Option<T> {
    #[inline]
    fn for_self_or_each_child(&self, f: &mut impl FnMut(Token<S, R>)) {
        if let Some(node) = self {
            node.for_self_or_each_child(f)
        }
    }
}

impl<S, R: RuleType, T: Pairs<S, R>, Skip: Pairs<S, R>, const SKIP: usize> Pairs<S, R>
    for Skipped<T, Skip, SKIP>
{
    #[inline]
    fn for_self_or_each_child(&self, f: &mut impl FnMut(Token<S, R>)) {
        self.skipped.for_self_or_each_child(f);
        self.matched.for_self_or_each_child(f);
    }
}

macro_rules! impl_with_vec {
    ($name:ident, $(const $args:ident : $t:ty,)*) => {
        impl<
                S,
                R: RuleType,
                T: Pairs<S, R>,
                $(const $args: $t, )*
            > Pairs<S, R> for $name<T, $($args, )*>
        {
            #[inline]
            fn for_self_or_each_child(&self, f: &mut impl FnMut(Token<S, R>)) {
                self.content.iter().for_each(|n| n.for_self_or_each_child(f));
            }
        }
    };
}

impl_with_vec!(RepeatMinMax, const MIN: usize, const MAX: usize,);
impl_with_vec!(RepeatMin, const MIN: usize,);
impl_with_vec!(AtomicRepeat,);

impl_empty!(ANY);
impl_empty!(SOI);
impl_empty!(NEWLINE);
impl_empty!(PEEK<S>);
impl_empty!(PEEK_ALL<S>);
impl_empty!(POP<S>);
impl_empty!(POP_ALL<S>);
impl_empty!(DROP);

impl_empty!(AlwaysFail<S>);
impl_empty!(Empty<S>);

/// An iterator that maybe contain another iterator.
pub struct Maybe<Item, T: Iterator<Item = Item>>(Option<T>);
impl<Item, T: Iterator<Item = Item>> Iterator for Maybe<Item, T> {
    type Item = Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            Some(inner) => inner.next(),
            None => None,
        }
    }
}
