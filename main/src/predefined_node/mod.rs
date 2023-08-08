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

use crate::{
    iterators::{Pair, Pairs},
    typed_node::Take,
    StringArrayWrapper, TypedNode,
};
use crate::{RuleType, StringWrapper};
use core::{
    iter::{empty, Empty, Iterator},
    ops::Deref,
};

/// Choice helper with a next branch.
pub trait NextChoice {
    /// The choice helper that corresponds to the next branch.
    type Next;
}

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

impl_empty!(Str<'i, R, T>, T: StringWrapper);
impl_empty!(Insens<'i, R, T>, T: StringWrapper);
impl_empty!(Skip<'i, R, Strings>, Strings: StringArrayWrapper);
impl_empty!(SkipChar<'i, R, N>, const N: usize);
impl_empty!(CharRange<'i, R, MIN, MAX>, const MIN: char, const MAX: char);
impl_empty!(Positive<'i, R, T>, T: TypedNode<'i, R>);
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
        FooBar,
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

    #[derive(Clone, PartialEq)]
    struct FooBar;
    impl StringArrayWrapper for FooBar {
        const CONTENT: &'static [&'static str] = &["foo", "bar"];
    }
    impl RuleWrapper<Rule> for FooBar {
        const RULE: Rule = Rule::FooBar;
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
        assert_eq!(
            format!("{:?}", s),
            r#"Rule { rule: Foo, content: Str { content: "foo" } }"#
        )
    }
    #[test]
    fn range() {
        let whitespace = WHITESPACE::parse(" ").unwrap();
        assert_eq!(
            format!("{:?}", whitespace),
            r#"AtomicRule { rule: WHITESPACE, content: CharRange { content: ' ' } }"#
        );
        let comment = COMMENT::parse("\t").unwrap();
        assert_eq!(
            format!("{:?}", comment),
            r#"AtomicRule { rule: COMMENT, content: CharRange { content: '\t' } }"#
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
            "Rule { rule: RepFoo, content: RepMin { content: [Str { content: \"foo\" }, Str { content: \"foo\" }, Str { content: \"foo\" }] } }"
        );
        let rep = R::parse("foo foo foo").unwrap();
        assert_eq!(
            format!("{:?}", rep),
            "Rule { rule: RepFoo, content: RepMin { content: [Str { content: \"foo\" }, Str { content: \"foo\" }, Str { content: \"foo\" }] } }"
        );
        let rep = R::parse("foo foo\tfoo").unwrap();
        assert_eq!(
            format!("{:?}", rep),
            "Rule { rule: RepFoo, content: RepMin { content: [Str { content: \"foo\" }, Str { content: \"foo\" }, Str { content: \"foo\" }] } }"
        );
        assert_eq!(REP::MIN, 0);
    }

    fn impl_pairs<T: Pairs<'static, 'static, Rule>>() {}

    #[test]
    fn trait_tests() {
        impl_pairs::<ANY<'static>>();
        impl_pairs::<SOI<'static>>();
        impl_pairs::<EOI<'static>>();
        impl_pairs::<NEWLINE<'static>>();
        impl_pairs::<PEEK_ALL<'static>>();
        impl_pairs::<PEEK<'static>>();
        impl_pairs::<DROP<'static>>();
        impl_pairs::<POP<'static>>();
        impl_pairs::<POP_ALL<'static>>();
        impl_pairs::<PeekSlice2<'static, 0, 0>>();
        impl_pairs::<PeekSlice1<'static, 0>>();

        impl_pairs::<Str<'static, Rule, Foo>>();
        impl_pairs::<Insens<'static, Rule, Foo>>();
        impl_pairs::<Skip<'static, Rule, FooBar>>();
        impl_pairs::<SkipChar<'static, Rule, 0>>();
        impl_pairs::<CharRange<'static, Rule, '0', '0'>>();
    }
}
