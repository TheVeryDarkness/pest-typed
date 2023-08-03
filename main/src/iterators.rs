// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::{
    predefined_node::{
        AtomicRule, Box, CharRange, Insens, Negative, NonAtomicRule, Opt, PeekSlice1, PeekSlice2,
        Positive, Push, RepMin, Restorable, Rule, Skip, Str, ANY, DROP, NEWLINE, PEEK, PEEK_ALL,
        POP, POP_ALL, SOI,
    },
    typed_node::RuleStruct,
    NeverFailedTypedNode, RuleWrapper, Span, StringArrayWrapper, StringWrapper, TypedNode,
};
use alloc::{boxed, collections::VecDeque, vec, vec::Vec};
use core::{
    iter::{empty, once, Empty, FlatMap, Iterator},
    mem::swap,
};
use pest::RuleType;

/// Simulate [`pest::iterators::Pairs`].
pub trait Pairs<'i: 'n, 'n, R: RuleType + 'n> {
    /// Iterator type that iterate on inner pairs by reference.
    type Iter: Iterator<Item = &'n (dyn Pair<'i, 'n, R>)>;
    /// Iterator type that iterate on inner pairs by value.
    type IntoIter: Iterator<Item = boxed::Box<dyn Pair<'i, 'n, R> + 'n>>;
    /// Iterate on inner pairs by reference. Returns [`Pairs::Iter`].
    fn iter(&'n self) -> Self::Iter;
    /// Iterate on inner pairs by value. Returns [`Pairs::IntoIter`].
    fn into_iter(self) -> Self::IntoIter;
}

/// Simulate [`pest::iterators::Pair`].
pub trait Pair<'i: 'n, 'n, R: RuleType + 'n>: RuleStruct<'i, R> {
    /// Collect inner pairs' [`Pairs::Iter`] and make them into a [`vec::IntoIter`].
    fn inner(&'n self) -> vec::IntoIter<&'n (dyn Pair<'i, 'n, R>)>;
    /// Collect inner pairs [`Pairs::IntoIter`] and make them into a [`vec::IntoIter`].
    fn into_inner(self) -> vec::IntoIter<boxed::Box<dyn Pair<'i, 'n, R> + 'n>>;
}

/// A trait to traverse the pair as the root of a tree.
pub trait PairTree<'i: 'n, 'n, R: RuleType + 'n>: Pair<'i, 'n, R> + Sized {
    /// Level order traversal
    fn iterate_level_order(&'n self, mut f: impl FnMut(&'n (dyn Pair<'i, 'n, R>), usize)) {
        let mut queue: VecDeque<&'n (dyn Pair<'i, 'n, R>)> = VecDeque::new();
        let mut next = VecDeque::new();
        queue.push_back(self);
        loop {
            while let Some(p) = queue.pop_front() {
                f(p, queue.len());
                for child in p.inner() {
                    next.push_back(child);
                }
            }
            swap(&mut queue, &mut next);
            if queue.is_empty() {
                break;
            }
        }
    }
    /// Pre-order traversal
    fn iterate_pre_order(&'n self, mut f: impl FnMut(&'n (dyn Pair<'i, 'n, R>), usize)) {
        let mut stack: Vec<VecDeque<&'n (dyn Pair<'i, 'n, R>)>> = Vec::new();

        let root: &'n (dyn Pair<'i, 'n, R>) = self;
        stack.push(VecDeque::<&'n (dyn Pair<'i, 'n, R>)>::from_iter(once(root)));

        loop {
            if let Some(parent) = stack.last_mut() {
                if let Some(first) = parent.pop_front() {
                    f(first, stack.len() - 1);
                    stack.push(first.inner().collect::<VecDeque<_>>());
                } else {
                    stack.pop();
                }
            } else {
                break;
            }
        }
    }
}

impl<'i: 'n, 'n, R: RuleType + 'n, T: RuleStruct<'i, R> + Pairs<'i, 'n, R> + Pair<'i, 'n, R>>
    PairTree<'i, 'n, R> for T
{
}

macro_rules! impl_empty {
    ($node:ty, $($tt:tt)*) => {
        impl<'i: 'n, 'n, R: RuleType + 'n, $($tt)*> Pairs<'i, 'n, R> for $node {
            type Iter = Empty<&'n (dyn Pair<'i, 'n, R>)>;
            type IntoIter = Empty<boxed::Box<dyn Pair<'i, 'n, R> + 'n>>;

            fn iter(&'n self) -> Self::Iter {
                empty()
            }
            fn into_iter(self) -> Self::IntoIter {
                empty()
            }
        }
    };
}

macro_rules! impl_forward_inner {
    ($node:ident) => {
        impl<'i: 'n, 'n, R: RuleType + 'n, T: TypedNode<'i, R> + Pairs<'i, 'n, R>> Pairs<'i, 'n, R>
            for $node<'i, R, T>
        {
            type Iter = T::Iter;
            type IntoIter = T::IntoIter;

            fn iter(&'n self) -> Self::Iter {
                self.content.iter()
            }
            fn into_iter(self) -> Self::IntoIter {
                self.content.into_iter()
            }
        }
    };
}

impl_empty!(Str<'i, R, T>, T: StringWrapper);
impl_empty!(Insens<'i, R, T>, T: StringWrapper);
impl_empty!(PeekSlice2<'i, R, START, END>, const START: i32, const END: i32);
impl_empty!(PeekSlice1<'i, R, START>, const START: i32);
impl_forward_inner!(Push);
impl_empty!(Skip<'i, R, Strings>, Strings: StringArrayWrapper);
impl_empty!(CharRange<'i, R, MIN, MAX>, const MIN: char, const MAX: char);
impl_forward_inner!(Box);
impl_forward_inner!(Positive);
impl_empty!(Negative<'i, R, T>, T: TypedNode<'i, R>);
impl_forward_inner!(Restorable);

impl<'i: 'n, 'n, R: RuleType + 'n, T: TypedNode<'i, R> + Pairs<'i, 'n, R>> Pairs<'i, 'n, R>
    for Opt<'i, R, T>
{
    type Iter = Maybe<&'n (dyn Pair<'i, 'n, R>), T::Iter>;
    type IntoIter = Maybe<boxed::Box<dyn Pair<'i, 'n, R> + 'n>, T::IntoIter>;

    fn iter(&'n self) -> Self::Iter {
        match &self.content {
            Some(inner) => Maybe(Some(inner.iter())),
            None => Maybe(None),
        }
    }
    fn into_iter(self) -> Self::IntoIter {
        match self.content {
            Some(inner) => Maybe(Some(inner.into_iter())),
            None => Maybe(None),
        }
    }
}

impl<
        'i: 'n,
        'n,
        R: RuleType + 'n,
        T: TypedNode<'i, R> + Pairs<'i, 'n, R> + 'n,
        IGNORED: NeverFailedTypedNode<'i, R>,
        const MIN: usize,
    > Pairs<'i, 'n, R> for RepMin<'i, R, T, IGNORED, MIN>
{
    type Iter = FlatMap<core::slice::Iter<'n, T>, T::Iter, fn(&'n T) -> T::Iter>;
    type IntoIter = FlatMap<vec::IntoIter<T>, T::IntoIter, fn(T) -> T::IntoIter>;

    fn iter(&'n self) -> Self::Iter {
        self.content.iter().flat_map(|e: &'n T| e.iter())
    }
    fn into_iter(self) -> Self::IntoIter {
        self.content.into_iter().flat_map(|e| e.into_iter())
    }
}

macro_rules! impl_easiest {
    ($id: ident) => {
        impl<'i: 'n, 'n, R: RuleType + 'n> Pairs<'i, 'n, R> for $id<'i> {
            type Iter = Empty<&'n (dyn Pair<'i, 'n, R>)>;
            type IntoIter = Empty<boxed::Box<dyn Pair<'i, 'n, R> + 'n>>;

            fn iter(&'n self) -> Self::Iter {
                empty()
            }
            fn into_iter(self) -> Self::IntoIter {
                empty()
            }
        }
    };
}

impl_easiest!(ANY);

impl_easiest!(SOI);
impl_easiest!(NEWLINE);
impl_easiest!(PEEK);
impl_easiest!(PEEK_ALL);
impl_easiest!(POP);
impl_easiest!(POP_ALL);
impl_easiest!(DROP);

macro_rules! impl_self {
    ($node:ty, $($tt:tt)*) => {
        impl<'i: 'n, 'n, R: RuleType + 'n, $($tt)*> Pairs<'i, 'n, R> for $node {
            type Iter = core::iter::Once<&'n dyn Pair<'i, 'n, R>>;
            type IntoIter = core::iter::Once<boxed::Box<dyn Pair<'i, 'n, R> + 'n>>;

            fn iter(&'n self) -> Self::Iter {
                once(self)
            }
            fn into_iter(self) -> Self::IntoIter {
                once(boxed::Box::new(self))
            }
        }
        impl<'i: 'n, 'n, R: RuleType + 'n, $($tt)*> RuleStruct<'i, R> for $node {
            fn span(&self) -> Span<'i> { self.span }
        }
        impl<'i: 'n, 'n, R: RuleType + 'n, $($tt)*> Pair<'i, 'n, R> for $node {
            fn inner(&'n self) -> alloc::vec::IntoIter<&'n (dyn Pair<'i, 'n, R> + 'n)> {
                vec![].into_iter()
            }
            fn into_inner(self) -> alloc::vec::IntoIter<alloc::boxed::Box<(dyn Pair<'i, 'n, R> + 'n)>> {
                vec![].into_iter()
            }
        }
    };
}

impl_self!(
    AtomicRule<'i, R, T, RULE, _EOI>,
    T: TypedNode<'i, R> + 'n,
    RULE: RuleWrapper<R>,
    _EOI: RuleWrapper<R>
);
impl_self!(
    Rule<'i, R, T, RULE, _EOI, IGNORED>,
    T: TypedNode<'i, R> + 'n,
    RULE: RuleWrapper<R>,
    _EOI: RuleWrapper<R>,
    IGNORED: NeverFailedTypedNode<'i, R>,
);
impl_self!(
    NonAtomicRule<'i, R, T, RULE, _EOI, IGNORED>,
    T: TypedNode<'i, R>,
    RULE: RuleWrapper<R>,
    _EOI: RuleWrapper<R>,
    IGNORED: NeverFailedTypedNode<'i, R>,
);

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
