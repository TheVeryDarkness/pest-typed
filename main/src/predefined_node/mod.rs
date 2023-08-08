// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Predefined tree nodes generics.
//! The generator may use this for convenience.
//! Normally you don't need to reference this module by yourself.

mod builtin;
mod choices;
mod combinator;
mod predicate;
mod rule;
mod sequence;
mod terminal;

use alloc::boxed::Box;
pub use builtin::*;
pub use choices::*;
pub use combinator::*;
pub use predicate::*;
pub use rule::*;
pub use sequence::*;
pub use terminal::*;

use crate::RuleType;
use crate::{
    iterators::{Pair, Pairs},
    typed_node::{RuleStruct, Take},
    NeverFailedTypedNode, RuleWrapper, Span, StringArrayWrapper, StringWrapper, TypedNode,
};
use alloc::{boxed, vec};
use core::{
    iter::{empty, once, Empty, FlatMap, Iterator},
    ops::Deref,
};

macro_rules! impl_empty {
    ($node:ty, $($tt:tt)*) => {
        impl<'i: 'n, 'n, R: RuleType + 'n, $($tt)*> Pairs<'i, 'n, R> for $node {
            type Iter = Empty<&'n (dyn Pair<'i, 'n, R>)>;
            type IntoIter = Empty<Box<dyn Pair<'i, 'n, R> + 'n>>;

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
impl_empty!(PeekSlice2<'i, START, END>, const START: i32, const END: i32);
impl_empty!(PeekSlice1<'i, START>, const START: i32);
impl_forward_inner!(Push);
impl_empty!(Skip<'i, R, Strings>, Strings: StringArrayWrapper);
impl_empty!(CharRange<'i, R, MIN, MAX>, const MIN: char, const MAX: char);
impl_forward_inner!(Ref);
impl_forward_inner!(Positive);
impl_empty!(Negative<'i, R, T>, T: TypedNode<'i, R>);

impl<'i: 'n, 'n, R: RuleType + 'n, T: TypedNode<'i, R> + Pairs<'i, 'n, R>> Pairs<'i, 'n, R>
    for Opt<'i, R, T>
{
    type Iter = Maybe<&'n (dyn Pair<'i, 'n, R>), T::Iter>;
    type IntoIter = Maybe<Box<dyn Pair<'i, 'n, R> + 'n>, T::IntoIter>;

    fn iter(&'n self) -> Self::Iter {
        match self.deref() {
            Some(inner) => Maybe(Some(inner.iter())),
            None => Maybe(None),
        }
    }
    fn into_iter(self) -> Self::IntoIter {
        match self.take() {
            Some(inner) => Maybe(Some(inner.into_iter())),
            None => Maybe(None),
        }
    }
}

macro_rules! impl_easiest {
    ($id: ident) => {
        impl<'i: 'n, 'n, R: RuleType + 'n> Pairs<'i, 'n, R> for $id<'i> {
            type Iter = Empty<&'n (dyn Pair<'i, 'n, R>)>;
            type IntoIter = Empty<Box<dyn Pair<'i, 'n, R> + 'n>>;

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
            type IntoIter = core::iter::Once<Box<dyn Pair<'i, 'n, R> + 'n>>;

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
            fn into_inner(self) -> alloc::vec::IntoIter<Box<(dyn Pair<'i, 'n, R> + 'n)>> {
                vec![].into_iter()
            }
        }
    };
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wrapper::BoundWrapper;
    use crate::{ParsableTypedNode, Storage};
    use crate::{RuleWrapper, StringWrapper};
    use alloc::format;
    use core::ops::Deref;

    macro_rules! make_rules {
        ($($ids:ident,)*) => {
            #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            enum Rule {
                $($ids),*
            }
            mod rule_wrappers {
                $(
                    #[derive(Clone, PartialEq)]
                    pub struct $ids {}
                    impl super::RuleWrapper<super::Rule> for $ids {
                        const RULE:super::Rule = super::Rule::$ids;
                        type Rule = super::Rule;
                    }
                )*
            }
        };
    }

    make_rules! {
        Foo,
        RepFoo,
        WHITESPACE,
        COMMENT,
        EOI,
    }

    #[derive(Clone, PartialEq)]
    struct Foo;
    impl StringWrapper for Foo {
        const CONTENT: &'static str = "foo";
    }
    impl RuleWrapper<Rule> for Foo {
        const RULE: Rule = Rule::Foo;
        type Rule = Rule;
    }

    type WHITESPACE<'i> = AtomicRule<
        'i,
        Rule,
        CharRange<'i, Rule, ' ', ' '>,
        rule_wrappers::WHITESPACE,
        rule_wrappers::EOI,
    >;
    type COMMENT<'i> = AtomicRule<
        'i,
        Rule,
        CharRange<'i, Rule, '\t', '\t'>,
        rule_wrappers::COMMENT,
        rule_wrappers::EOI,
    >;
    type StrFoo<'i> = super::Rule<
        'i,
        Rule,
        Str<'i, Rule, Foo>,
        rule_wrappers::Foo,
        rule_wrappers::EOI,
        Ignore<'i>,
    >;
    #[test]
    fn string() {
        assert_eq!(<StrFoo<'_> as Deref>::Target::CONTENT, Foo::CONTENT);
        let s = StrFoo::parse("foo").unwrap();
        assert_eq!(s.get_content(), "foo");
        assert_eq!(format!("{:?}", s), r#"Rule { rule: Foo, content: Str }"#)
    }
    #[test]
    fn range() {
        let whitespace = WHITESPACE::parse(" ").unwrap();
        assert_eq!(
            format!("{:?}", whitespace),
            r#"AtomicRule { rule: WHITESPACE, content: Range { content: ' ' } }"#
        );
        let comment = COMMENT::parse("\t").unwrap();
        assert_eq!(
            format!("{:?}", comment),
            r#"AtomicRule { rule: COMMENT, content: Range { content: '\t' } }"#
        );
    }
    type Ignore<'i> = Ignored<'i, Rule, COMMENT<'i>, WHITESPACE<'i>>;
    #[test]
    fn ignore() {
        super::Rule::<Rule, Ignore<'_>, rule_wrappers::RepFoo, rule_wrappers::EOI, Ignore<'_>>::parse(
            " \t  ",
        )
        .unwrap();
    }

    type REP<'i> = Rep<'i, Rule, Str<'i, Rule, Foo>, Ignore<'i>>;
    type R<'i> =
        super::Rule<'i, Rule, REP<'i>, rule_wrappers::RepFoo, rule_wrappers::EOI, Ignore<'i>>;
    #[test]
    fn repetition() {
        let rep = R::parse("foofoofoo").unwrap();
        assert_eq!(
            format!("{:?}", rep),
            "Rule { rule: RepFoo, content: RepMin { content: [Str, Str, Str] } }"
        );
        let rep = R::parse("foo foo foo").unwrap();
        assert_eq!(
            format!("{:?}", rep),
            "Rule { rule: RepFoo, content: RepMin { content: [Str, Str, Str] } }"
        );
        let rep = R::parse("foo foo\tfoo").unwrap();
        assert_eq!(
            format!("{:?}", rep),
            "Rule { rule: RepFoo, content: RepMin { content: [Str, Str, Str] } }"
        );
        assert_eq!(REP::MIN, 0);
    }
}
