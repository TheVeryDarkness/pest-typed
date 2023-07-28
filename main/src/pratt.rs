// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! For Pratt Parser. See [`pest::pratt_parser`]

use crate::{predefined_node::*, NeverFailedTypedNode, RuleType, TypedNode};

/// Node counter.
pub trait NodeCounter {
    const CHOICES: usize = 1;
    const SEQUENCE: usize = 1;
}

impl<'i, R: RuleType, T1: TypedNode<'i, R> + NodeCounter, T2: TypedNode<'i, R> + NodeCounter>
    NodeCounter for Choice<'i, R, T1, T2>
{
    const CHOICES: usize = 1 + T2::CHOICES;
}

impl<
        'i,
        R: RuleType,
        T1: TypedNode<'i, R> + NodeCounter,
        T2: TypedNode<'i, R> + NodeCounter,
        IGNORED: NeverFailedTypedNode<'i, R>,
    > NodeCounter for Seq<'i, R, T1, T2, IGNORED>
{
    const SEQUENCE: usize = 1 + T2::SEQUENCE;
}

/// Pratt parser for those nodes with prefix, infix and postfix.
pub trait PrattPrefixInfixPostfix<
    'i,
    R: RuleType,
    Prefix: TypedNode<'i, R>,
    InFix: TypedNode<'i, R>,
    Postfix: TypedNode<'i, R>,
>
{
}
