// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Simulates [`pest::iterators`].

use crate::typed_node::RuleStruct;
use crate::RuleType;
use alloc::{boxed, collections::VecDeque, vec, vec::Vec};
use core::{iter::once, mem::swap};

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
}

impl<'i: 'n, 'n, R: RuleType + 'n, T: RuleStruct<'i, R> + Pairs<'i, 'n, R> + Pair<'i, 'n, R>>
    PairTree<'i, 'n, R> for T
{
}
