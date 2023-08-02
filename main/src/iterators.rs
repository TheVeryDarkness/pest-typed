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
        AtomicRule, Box, CharRange, Choice, Insens, Negative, NonAtomicRule, Opt, PeekSlice1,
        PeekSlice2, Positive, Push, RepMin, Restorable, Rule, Skip, Str, DROP, NEWLINE, PEEK,
        PEEK_ALL, POP, POP_ALL, SOI, ANY,
    },
    typed_node::RuleStruct,
    NeverFailedTypedNode, RuleWrapper, Span, StringArrayWrapper, StringWrapper, TypedNode,
};
use alloc::{boxed, vec};
use core::iter::{empty, Empty, FlatMap, Iterator};
use pest::RuleType;

pub trait Pairs<'i: 'n, 'n, R: RuleType + 'n> {
    type Iter: Iterator<Item = &'n (dyn RuleStruct<'i, R>)>;
    type IntoIter: Iterator<Item = boxed::Box<dyn RuleStruct<'i, R> + 'n>>;
    fn iter(&'n self) -> Self::Iter;
    fn into_iter(self) -> Self::IntoIter;
}

pub trait Pair<'i: 'n, 'n, R: RuleType + 'n>: Pairs<'i, 'n, R> {
    type Inner: Iterator<Item = &'n (dyn RuleStruct<'i, R>)>;
    type IntoInner: Iterator<Item = boxed::Box<dyn RuleStruct<'i, R> + 'n>>;
    fn inner(&'n self) -> Self::Inner;
    fn into_inner(self) -> Self::IntoInner;
}

macro_rules! impl_empty {
    ($node:ty, $($tt:tt)*) => {
        impl<'i: 'n, 'n, R: RuleType + 'n, $($tt)*> Pairs<'i, 'n, R> for $node {
            type Iter = Empty<&'n (dyn RuleStruct<'i, R>)>;
            type IntoIter = Empty<boxed::Box<dyn RuleStruct<'i, R> + 'n>>;

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

impl<
        'i: 'n,
        'n,
        R: RuleType + 'n,
        T1: TypedNode<'i, R> + Pairs<'i, 'n, R>,
        T2: TypedNode<'i, R> + Pairs<'i, 'n, R>,
    > Pairs<'i, 'n, R> for Choice<'i, R, T1, T2>
{
    type Iter = Either<&'n (dyn RuleStruct<'i, R>), T1::Iter, T2::Iter>;
    type IntoIter = Either<boxed::Box<dyn RuleStruct<'i, R> + 'n>, T1::IntoIter, T2::IntoIter>;

    fn iter(&'n self) -> Self::Iter {
        match self {
            Self::First(first, _) => Either::First(first.iter()),
            Self::Second(second, _) => Either::Second(second.iter()),
        }
    }
    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::First(first, _) => Either::First(first.into_iter()),
            Self::Second(second, _) => Either::Second(second.into_iter()),
        }
    }
}

impl<'i: 'n, 'n, R: RuleType + 'n, T: TypedNode<'i, R> + Pairs<'i, 'n, R>> Pairs<'i, 'n, R>
    for Opt<'i, R, T>
{
    type Iter = Maybe<&'n (dyn RuleStruct<'i, R>), T::Iter>;
    type IntoIter = Maybe<boxed::Box<dyn RuleStruct<'i, R> + 'n>, T::IntoIter>;

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
            type Iter = Empty<&'n (dyn RuleStruct<'i, R>)>;
            type IntoIter = Empty<boxed::Box<dyn RuleStruct<'i, R> + 'n>>;

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
            type Iter = core::iter::Once<&'n dyn RuleStruct<'i, R>>;
            type IntoIter = core::iter::Once<boxed::Box<dyn RuleStruct<'i, R> + 'n>>;

            fn iter(&'n self) -> Self::Iter {
                core::iter::once(self)
            }
            fn into_iter(self) -> Self::IntoIter {
                core::iter::once(boxed::Box::new(self))
            }
        }
        impl<'i: 'n, 'n, R: RuleType + 'n, $($tt)*> RuleStruct<'i, R> for $node {
            fn span(&self) -> Span<'i> { self.span }
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

pub enum Either<Item, T1: Iterator<Item = Item>, T2: Iterator<Item = Item>> {
    First(T1),
    Second(T2),
}

impl<Item, T1: Iterator<Item = Item>, T2: Iterator<Item = Item>> Iterator for Either<Item, T1, T2> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::First(first) => first.next(),
            Self::Second(second) => second.next(),
        }
    }
}

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
