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
    choices::Choice2,
    predefined_node::{
        AlwaysFail, CharRange, Empty, Insens, Negative, PeekSlice1, PeekSlice2, Positive, Push,
        RepMin, RepMinMax, Skip, Skipped, Str, ANY, DROP, NEWLINE, PEEK, PEEK_ALL, POP, POP_ALL,
        SOI,
    },
    typed_node::RuleStruct,
    NeverFailedTypedNode, StringArrayWrapper, StringWrapper, TypedNode,
};
use alloc::{boxed, collections::VecDeque, string::String, vec, vec::Vec};
use core::{
    iter::{self, empty, once, Chain, FlatMap, Iterator},
    mem::swap,
};
use pest::RuleType;

/// Token.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token<R: RuleType> {
    /// Rule.
    pub rule: R,
    /// Start position.
    pub start: usize,
    /// End position.
    pub end: usize,
    /// Children.
    pub children: Vec<Self>,
}

/// Simulate [`pest::iterators::Pairs`].
pub trait Pairs<'i: 'n, 'n, R: RuleType + 'n> {
    /// Iterator type that iterate on inner pairs by reference.
    type Iter: Iterator<Item = &'n (dyn Pair<'i, 'n, R>)>;
    /// Iterator type that iterate on inner pairs by value.
    type IntoIter: Iterator<Item = boxed::Box<dyn Pair<'i, 'n, R> + 'n>>;
    /// Iterate on inner pairs by reference. Returns [`Pairs::Iter`].
    fn iter_pairs(&'n self) -> Self::Iter;
    /// Iterate on inner pairs by value. Returns [`Pairs::IntoIter`].
    fn into_iter_pairs(self) -> Self::IntoIter;
}

/// Simulate [`pest::iterators::Pair`].
pub trait Pair<'i: 'n, 'n, R: RuleType + 'n>: RuleStruct<'i, R> {
    /// Collect inner pairs' [`Pairs::Iter`] and make them into a [`vec::IntoIter`].
    fn inner(&'n self) -> vec::IntoIter<&'n (dyn Pair<'i, 'n, R>)>;
    /// Collect inner pairs' [`Pairs::IntoIter`] and make them into a [`vec::IntoIter`].
    fn into_inner(self) -> vec::IntoIter<boxed::Box<dyn Pair<'i, 'n, R> + 'n>>;
    /// As tokens.
    ///
    /// Call [`Pair::inner`] recursively.
    fn as_token_tree(&'n self) -> Token<R> {
        let children = self.inner().map(|p| p.as_token_tree()).collect();
        Token::<R> {
            rule: self.rule(),
            start: self.span().start(),
            end: self.span().end(),
            children,
        }
    }
}

fn iterate_level_order<'i: 'n, 'n, R: RuleType + 'n, E>(
    p: &'n impl Pair<'i, 'n, R>,
    mut f: impl FnMut(
        &'n (dyn Pair<'i, 'n, R>),
        usize,
        &VecDeque<&dyn Pair<'i, 'n, R>>,
    ) -> Result<(), E>,
) -> Result<(), E> {
    let mut queue: VecDeque<&'n (dyn Pair<'i, 'n, R>)> = VecDeque::new();
    let mut next = VecDeque::new();
    queue.push_back(p);
    loop {
        while let Some(p) = queue.pop_front() {
            let mut children = p.inner().collect::<VecDeque<_>>();
            f(p, queue.len(), &children)?;
            next.append(&mut children);
        }
        swap(&mut queue, &mut next);
        if queue.is_empty() {
            return Ok(());
        }
    }
}
/// Pre-order traversal
fn iterate_pre_order<'i: 'n, 'n, R: RuleType + 'n, E>(
    p: &'n impl Pair<'i, 'n, R>,
    mut f: impl FnMut(
        &'n (dyn Pair<'i, 'n, R>),
        usize,
        &VecDeque<&dyn Pair<'i, 'n, R>>,
    ) -> Result<(), E>,
) -> Result<(), E> {
    let mut stack: Vec<VecDeque<&'n (dyn Pair<'i, 'n, R>)>> = Vec::new();

    let root: &'n (dyn Pair<'i, 'n, R>) = p;
    stack.push(VecDeque::<&'n (dyn Pair<'i, 'n, R>)>::from_iter(once(root)));

    loop {
        if let Some(parent) = stack.last_mut() {
            if let Some(first) = parent.pop_front() {
                let children = first.inner().collect::<VecDeque<_>>();
                f(first, stack.len() - 1, &children)?;
                stack.push(children);
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
    p: &'n impl Pair<'i, 'n, R>,
    buf: &mut impl core::fmt::Write,
) -> core::fmt::Result {
    iterate_pre_order(p, |p, depth, children| {
        if children.is_empty() {
            buf.write_fmt(format_args!(
                "{}{:?} {:?}\n",
                &"    ".repeat(depth),
                p.rule(),
                p.span().as_str()
            ))
        } else {
            buf.write_fmt(format_args!("{}{:?}\n", &"    ".repeat(depth), p.rule()))
        }
    })
}

/// A trait to traverse the pair as the root of a tree.
pub trait PairTree<'i: 'n, 'n, R: RuleType + 'n>: Pair<'i, 'n, R> + Sized {
    /// Level order traversal
    fn iterate_level_order<E>(
        &'n self,
        f: impl FnMut(
            &'n (dyn Pair<'i, 'n, R>),
            usize,
            &VecDeque<&dyn Pair<'i, 'n, R>>,
        ) -> Result<(), E>,
    ) -> Result<(), E> {
        iterate_level_order(self, f)
    }
    /// Pre-order traversal
    fn iterate_pre_order<E>(
        &'n self,
        f: impl FnMut(
            &'n (dyn Pair<'i, 'n, R>),
            usize,
            &VecDeque<&dyn Pair<'i, 'n, R>>,
        ) -> Result<(), E>,
    ) -> Result<(), E> {
        iterate_pre_order(self, f)
    }

    /// Write the tree to the `buf`.
    fn write_tree_to(&'n self, buf: &mut impl core::fmt::Write) -> core::fmt::Result {
        write_tree_to(self, buf)
    }

    /// Format as a tree.
    fn format_as_tree(&'n self) -> Result<String, core::fmt::Error> {
        let mut buf = String::new();
        self.write_tree_to(&mut buf)?;
        Ok(buf)
    }
}

impl<'i: 'n, 'n, R: RuleType + 'n, T: RuleStruct<'i, R> + Pairs<'i, 'n, R> + Pair<'i, 'n, R>>
    PairTree<'i, 'n, R> for T
{
}

macro_rules! impl_empty {
    ($node:ty, $($tt:tt)*) => {
        impl<'i: 'n, 'n, R: RuleType + 'n, $($tt)*> Pairs<'i, 'n, R> for $node {
            type Iter = iter::Empty<&'n (dyn Pair<'i, 'n, R>)>;
            type IntoIter = iter::Empty<boxed::Box<dyn Pair<'i, 'n, R> + 'n>>;

            fn iter_pairs(&'n self) -> Self::Iter {
                empty()
            }
            fn into_iter_pairs(self) -> Self::IntoIter {
                empty()
            }
        }
    };
}

macro_rules! impl_forward_inner {
    ($node:ident) => {
        impl<'i: 'n, 'n, R: RuleType + 'n, T: TypedNode<'i, R> + Pairs<'i, 'n, R>> Pairs<'i, 'n, R>
            for $node<T>
        {
            type Iter = T::Iter;
            type IntoIter = T::IntoIter;

            fn iter_pairs(&'n self) -> Self::Iter {
                self.content.iter_pairs()
            }
            fn into_iter_pairs(self) -> Self::IntoIter {
                self.content.into_iter_pairs()
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

impl<'i: 'n, 'n, R: RuleType + 'n, T1: Pairs<'i, 'n, R>, T2: Pairs<'i, 'n, R>> Pairs<'i, 'n, R>
    for (T1, T2)
{
    type Iter = Chain<T1::Iter, T2::Iter>;
    type IntoIter = Chain<T1::IntoIter, T2::IntoIter>;

    fn iter_pairs(&'n self) -> Self::Iter {
        self.0.iter_pairs().chain(self.1.iter_pairs())
    }
    fn into_iter_pairs(self) -> Self::IntoIter {
        self.0.into_iter_pairs().chain(self.1.into_iter_pairs())
    }
}

impl<'i: 'n, 'n, R: RuleType + 'n, T: Pairs<'i, 'n, R> + 'n, const N: usize> Pairs<'i, 'n, R>
    for [T; N]
{
    type Iter = FlatMap<
        core::slice::Iter<'n, T>,
        <T as Pairs<'i, 'n, R>>::Iter,
        fn(&'n T) -> <T as Pairs<'i, 'n, R>>::Iter,
    >;
    type IntoIter = FlatMap<
        core::array::IntoIter<T, N>,
        <T as Pairs<'i, 'n, R>>::IntoIter,
        fn(T) -> <T as Pairs<'i, 'n, R>>::IntoIter,
    >;

    fn iter_pairs(&'n self) -> Self::Iter {
        self.as_slice()
            .iter()
            .flat_map(|i| <T as Pairs<'i, 'n, R>>::iter_pairs(i))
    }
    fn into_iter_pairs(self) -> Self::IntoIter {
        <Self as IntoIterator>::into_iter(self).flat_map(|i| i.into_iter_pairs())
    }
}

impl<'i: 'n, 'n, R: RuleType + 'n, T: TypedNode<'i, R> + Pairs<'i, 'n, R>> Pairs<'i, 'n, R>
    for Option<T>
{
    type Iter = Maybe<&'n (dyn Pair<'i, 'n, R>), T::Iter>;
    type IntoIter = Maybe<boxed::Box<dyn Pair<'i, 'n, R> + 'n>, T::IntoIter>;

    fn iter_pairs(&'n self) -> Self::Iter {
        match &self {
            Some(inner) => Maybe(Some(inner.iter_pairs())),
            None => Maybe(None),
        }
    }
    fn into_iter_pairs(self) -> Self::IntoIter {
        match self {
            Some(inner) => Maybe(Some(inner.into_iter_pairs())),
            None => Maybe(None),
        }
    }
}

impl<
        'i: 'n,
        'n,
        R: RuleType + 'i,
        WHITESPACE: TypedNode<'i, R> + Pairs<'i, 'n, R> + 'n,
        COMMENT: TypedNode<'i, R> + Pairs<'i, 'n, R> + 'n,
    > Pairs<'i, 'n, R> for Skipped<WHITESPACE, COMMENT>
{
    type Iter = FlatMap<
        core::slice::Iter<'n, Choice2<WHITESPACE, COMMENT>>,
        <Choice2<WHITESPACE, COMMENT> as Pairs<'i, 'n, R>>::Iter,
        fn(
            &'n Choice2<WHITESPACE, COMMENT>,
        ) -> <Choice2<WHITESPACE, COMMENT> as Pairs<'i, 'n, R>>::Iter,
    >;
    type IntoIter = FlatMap<
        alloc::vec::IntoIter<Choice2<WHITESPACE, COMMENT>>,
        <Choice2<WHITESPACE, COMMENT> as Pairs<'i, 'n, R>>::IntoIter,
        fn(
            Choice2<WHITESPACE, COMMENT>,
        ) -> <Choice2<WHITESPACE, COMMENT> as Pairs<'i, 'n, R>>::IntoIter,
    >;

    fn iter_pairs(&'n self) -> Self::Iter {
        self.content.iter().flat_map(Pairs::iter_pairs)
    }
    fn into_iter_pairs(self) -> Self::IntoIter {
        self.content.into_iter().flat_map(Pairs::into_iter_pairs)
    }
}

macro_rules! impl_with_vec {
    ($name:ident, $(const $args:ident : $t:ty,)*) => {
        impl<
                'i: 'n,
                'n,
                R: RuleType + 'n,
                T: TypedNode<'i, R> + Pairs<'i, 'n, R> + 'n,
                I: NeverFailedTypedNode<'i, R> + Pairs<'i, 'n, R> + 'n,
                const SKIP: usize,
                $(const $args: $t, )*
            > Pairs<'i, 'n, R> for $name<T, I, SKIP, $($args, )*>
        {
            type Iter = FlatMap<
                core::slice::Iter<'n, ([I; SKIP], T)>,
                Chain<<[I; SKIP] as Pairs<'i,'n,R>>::Iter, T::Iter>,
                fn(&'n ([I; SKIP], T)) -> Chain<<[I; SKIP] as Pairs<'i,'n,R>>::Iter, T::Iter>,
            >;
            type IntoIter = FlatMap<
                vec::IntoIter<([I; SKIP], T)>,
                Chain<<[I; SKIP] as Pairs<'i,'n,R>>::IntoIter, T::IntoIter>,
                fn(([I; SKIP], T)) -> Chain<<[I; SKIP] as Pairs<'i,'n,R>>::IntoIter, T::IntoIter>,
            >;

            fn iter_pairs(&'n self) -> Self::Iter {
                self.content
                    .iter()
                    .flat_map(|(i, e)| i.iter_pairs().chain(e.iter_pairs()))
            }
            fn into_iter_pairs(self) -> Self::IntoIter {
                self.content
                    .into_iter()
                    .flat_map(|(i, e)| i.into_iter_pairs().chain(e.into_iter_pairs()))
            }
        }
    };
}

impl_with_vec!(RepMinMax, const MIN: usize, const MAX: usize, );
impl_with_vec!(RepMin, const MIN: usize,);

macro_rules! impl_without_lifetime {
    ($id: ident) => {
        impl<'i: 'n, 'n, R: RuleType + 'n> Pairs<'i, 'n, R> for $id {
            type Iter = iter::Empty<&'n (dyn Pair<'i, 'n, R>)>;
            type IntoIter = iter::Empty<boxed::Box<dyn Pair<'i, 'n, R> + 'n>>;

            fn iter_pairs(&'n self) -> Self::Iter {
                empty()
            }
            fn into_iter_pairs(self) -> Self::IntoIter {
                empty()
            }
        }
    };
}
macro_rules! impl_with_lifetime {
    ($id: ident) => {
        impl<'i: 'n, 'n, R: RuleType + 'n> Pairs<'i, 'n, R> for $id<'i> {
            type Iter = iter::Empty<&'n (dyn Pair<'i, 'n, R>)>;
            type IntoIter = iter::Empty<boxed::Box<dyn Pair<'i, 'n, R> + 'n>>;

            fn iter_pairs(&'n self) -> Self::Iter {
                empty()
            }
            fn into_iter_pairs(self) -> Self::IntoIter {
                empty()
            }
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
