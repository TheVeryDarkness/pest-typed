// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Test for [`pest_typed::predefined_node`].
#![cfg(test)]

extern crate alloc;

use pest_typed::{
    atomic_rule,
    choices::Choice2,
    compound_atomic_rule,
    iterators::{Pair, PairTree, Pairs},
    non_atomic_rule, normal_rule,
    predefined_node::*,
    rule_eoi,
    sequence::{Seq2, Seq3},
    silent_rule, BoundWrapper, ParsableTypedNode, RefStr, RuleStruct, RuleType, RuleWrapper,
    Storage, StringArrayWrapper, StringWrapper, TypeWrapper,
};
use std::{fmt::Write, ops::Deref, string::String};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Rule {
    Foo,
    RepFoo,
    NotFooBar,
    Life,
    WHITESPACE,
    COMMENT,
    EOI,
    String,
    Quote,
    NestedRep,
    Bar,
}
impl RuleType for Rule {
    fn name(&self) -> &'static str {
        match self {
            Rule::Foo => "Foo",
            Rule::RepFoo => "RepFoo",
            Rule::NotFooBar => "NotFooBar",
            Rule::Life => "Life",
            Rule::WHITESPACE => "WHITESPACE",
            Rule::COMMENT => "COMMENT",
            Rule::EOI => "EOI",
            Rule::String => "String",
            Rule::Quote => "Quote",
            Rule::NestedRep => "NestedRep",
            Rule::Bar => "Bar",
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Foo;
impl StringWrapper for Foo {
    const CONTENT: &'static str = "foo";
}
impl RuleWrapper<Rule> for Foo {
    const RULE: Rule = Rule::Foo;
    type Rule = Rule;
}

atomic_rule!(
    WHITESPACE,
    "White space.",
    Rule,
    Rule::WHITESPACE,
    CharRange::<' ', ' '>
);
compound_atomic_rule!(
    COMMENT,
    "Comment.",
    Rule,
    Rule::COMMENT,
    CharRange::<'\t', '\t'>,
    false
);
normal_rule!(
    StrFoo,
    "String \"Foo\".",
    Rule,
    Rule::Foo,
    Str::<Foo>,
    AtomicRepeat<Choice2<WHITESPACE<C::String>, COMMENT<C::String>>>,
    false
);
rule_eoi!(EOI, Rule);

#[test]
fn string() {
    assert_eq!(
        <StrFoo<&str, 0> as TypeWrapper>::Inner::CONTENT,
        Foo::CONTENT
    );
    StrFoo::try_check("foo").unwrap();
    let s = StrFoo::try_parse("foo").unwrap();
    assert_eq!(s.content.get_content(), "foo");
    assert_eq!(
        format!("{:?}", s),
        "StrFoo { content: Str, span: Span { str: \"foo\", start: 0, end: 3 } }"
    )
}

fn test_range<S: RefStr>() {
    WHITESPACE::try_check(S::from_static(" ")).unwrap();
    let whitespace = WHITESPACE::try_parse(S::from_static(" ")).unwrap();
    assert_eq!(
        format!("{:?}", whitespace),
        "WHITESPACE { span: Span { str: \" \", start: 0, end: 1 } }"
    );

    COMMENT::try_check(S::from_static("\t")).unwrap();
    let comment = COMMENT::try_parse(S::from_static("\t")).unwrap();
    assert_eq!(
        format!("{:?}", comment),
        "COMMENT { content: CharRange { content: '\\t' }, span: Span { str: \"\\t\", start: 0, end: 1 } }"
    );
}

#[test]
fn range() {
    test_range::<&str>();
    #[cfg(feature = "shared-string")]
    test_range::<shared_string::SharedString>();
    #[cfg(feature = "shared-string")]
    test_range::<shared_string::SharedSyncString>();
    #[cfg(feature = "shared-vec")]
    test_range::<shared_vec::RcString>();
    #[cfg(feature = "shared-vec")]
    test_range::<shared_vec::ArcString>();
}

type Ignore<S> = AtomicRepeat<Choice2<WHITESPACE<S>, COMMENT<S>>>;

#[test]
fn ignore() {
    silent_rule!(
        tmp,
        "Temporary rule.",
        Rule,
        Rule::RepFoo,
        Ignore<S>,
        Empty<S>,
        false
    );
    tmp::try_check(" ").unwrap();
    tmp::try_parse(" \t  ").unwrap();
}

fn test_repetition<S: RefStr>() {
    type REP<S> = Rep<StrFoo<S, 0>, Ignore<S>, 1>;
    non_atomic_rule!(
        R,
        "Repetion of [StrFoo].",
        Rule,
        Rule::RepFoo,
        REP<S>,
        Ignore<S>,
        false
    );

    R::try_check(S::from_static("foo")).unwrap();

    let inputs = [
        "foofoofoo",
        "foo foo foo",
        "foo foo\tfoo",
        "foofoofoo",
        "foo foo foo",
        "foo foo\tfoo",
    ];
    for input in inputs {
        R::try_check(S::from_static(input)).unwrap();
    }

    let [rep1, rep2, rep3, ..] = inputs.map(|s| R::try_parse(S::from_static(s)).unwrap());

    let _ = R::try_parse(S::from_static("")).unwrap();
    assert_ne!(rep1, rep2);
    assert_ne!(rep1, rep3);

    let format = |rep: &R<S, 1>| -> String {
        rep.iter_matched()
            .map(|e| e.get_content())
            .collect::<Vec<_>>()
            .concat()
    };
    assert_eq!(format(&rep1), format(&rep2));
    assert_eq!(format(&rep1), format(&rep3));

    for i in rep1.clone().children() {
        assert_eq!(i.rule, Rule::Foo);
    }

    // White spaces and comments aren't filtered out.
    assert_ne!(
        rep1.iter_matched().collect::<Vec<_>>(),
        rep2.iter_matched().collect::<Vec<_>>()
    );
    assert_ne!(
        rep1.iter_matched().collect::<Vec<_>>(),
        rep3.iter_matched().collect::<Vec<_>>()
    );

    assert_eq!(REP::<S>::MIN, 0);
    assert_eq!(rep1.deref().get_min_len(), 0);
    assert_eq!(rep1.deref().get_max_len(), usize::MAX);
    assert_eq!(<R<S, 0> as TypeWrapper>::Inner::MIN, 0);
}

#[test]
fn repetition() {
    test_repetition::<&str>();
    #[cfg(feature = "shared-string")]
    test_repetition::<shared_string::SharedString>();
    #[cfg(feature = "shared-string")]
    test_repetition::<shared_string::SharedSyncString>();
    #[cfg(feature = "shared-vec")]
    test_repetition::<shared_vec::RcString>();
    #[cfg(feature = "shared-vec")]
    test_repetition::<shared_vec::ArcString>();
}

#[test]
fn repetition_at_least_once() {
    type REP<S> = RepOnce<Insens<S, Foo>, Ignore<S>, 1>;
    non_atomic_rule!(
        R,
        "Repetion of [StrFoo].",
        Rule,
        Rule::RepFoo,
        REP<S>,
        Ignore<S>,
        false
    );

    let inputs = ["fooFoofoo", "foo Foo foo", "Foo foo\tfoo", "Foofoofoo"];
    let [rep1, rep2, rep3, rep4] = inputs.map(|s| R::try_parse(s).unwrap());
    assert_ne!(rep1, rep2);
    assert_ne!(rep1, rep3);
    assert_ne!(rep1, rep4);

    let collect = |r: &R<&str, 1>| {
        r.iter_matched()
            .map(|r| r.get_content())
            .collect::<Vec<_>>()
            .concat()
    };
    assert_eq!(collect(&rep1), collect(&rep2));
    assert_eq!(collect(&rep1), collect(&rep3));
    assert_eq!(collect(&rep1), collect(&rep4));

    assert_eq!(rep1.clone().self_or_children().len(), 1);
    assert_eq!(rep1.clone().self_or_children()[0].rule, Rule::RepFoo);

    {
        let mut buf = String::new();
        rep3.iterate_level_order(|p, _depth| writeln!(buf, "{}", p.span.as_str()))
            .unwrap();
        assert_eq!(buf, "Foo foo\tfoo\n \n\t\n");
        assert_eq!(
            rep3.format_as_tree().unwrap(),
            "RepFoo
    WHITESPACE \" \"
    COMMENT \"\\t\"
"
        );
    }

    assert_eq!(REP::<&str>::MIN, 1);
    assert_eq!(rep1.deref().get_min_len(), 1);
    assert_eq!(rep1.deref().get_max_len(), usize::MAX);
    assert_eq!(<R<&str, 0> as TypeWrapper>::Inner::MIN, 1);
}

#[test]
fn skip() {
    #[derive(Clone, Debug, Hash, PartialEq, Eq)]
    struct NewLine;
    impl StringArrayWrapper for NewLine {
        const CONTENT: &'static [&'static str] = &["\r\n", "\n", "\r"];
    }
    compound_atomic_rule!(
        QuotedString,
        "Quoted string.",
        Rule,
        Rule::String,
        Seq2<Skipped<Skip<S, NewLine>, Ignore<S>, 0>, Skipped<NEWLINE, Ignore<S>, 0>>,
        false
    );

    let s1 = QuotedString::<&str, 1>::try_parse("2\r\n").unwrap();
    let s2 = QuotedString::<&str, 1>::try_parse("\r\n").unwrap();
    assert_ne!(s1, s2);

    let new_line = NewLine;
    assert_eq!(new_line.get_content(), NewLine::CONTENT);
}

#[test]
fn skip_char() {
    compound_atomic_rule!(
        three,
        "Skip 3 characters.",
        Rule,
        Rule::Foo,
        SkipChar<S, 3>,
        false
    );
    three::<&str, 1>::try_parse("foo").unwrap();
    three::<&str, 1>::try_parse("foobar").unwrap_err();
}

#[test]
fn positive_predicate() {
    struct A;
    impl StringWrapper for A {
        const CONTENT: &'static str = "a";
    }

    atomic_rule!(
        PositiveAThenA,
        "Match 'a'.",
        Rule,
        Rule::Quote,
        (Positive<Str<A>>, Str<A>)
    );
    PositiveAThenA::try_parse("ab").unwrap_err();
    PositiveAThenA::try_parse("a").unwrap();
    PositiveAThenA::try_parse("aa").unwrap_err();

    atomic_rule!(Quote, "A single `'`.", Rule, Rule::Quote, CharRange<'\'', '\''>);
    compound_atomic_rule!(
        Lifetime,
        "A simplified example of rust lifetime specifier.",
        Rule,
        Rule::Life,
        Seq3<
            Skipped<Quote<S, 0>, Ignore<S>, 0>,
            Skipped<
                Positive<
                    Seq2<Skipped<ANY, Ignore<S>, 0>, Skipped<Negative<Quote<S, 0>>, Ignore<S>, 0>>,
                >,
                Ignore<S>,
                0,
            >,
            Skipped<AtomicRepeat<CharRange<'a', 'z'>>, Ignore<S>, 0>,
        >,
        false
    );

    let l = Lifetime::<&str, 1>::try_parse("'i").unwrap();
    dbg!(&l);
    let (quote, peeked, name) = l.as_ref();
    assert_eq!(quote.span.as_str(), "'");
    let (any, _) = peeked.as_ref();
    assert_eq!(any.content, 'i');
    assert_eq!(name.iter().map(|c| c.content).collect::<String>(), "i");
    assert_eq!(
        name.deref()
            .into_iter()
            .map(|c| c.content)
            .collect::<String>(),
        "i"
    );

    let l = Lifetime::<&str, 1>::try_parse("'input").unwrap();
    let (_, peeked, name) = l.as_ref();
    let (any, _) = peeked.as_ref();
    assert_eq!(any.content, 'i');
    assert_eq!(name.iter().map(|c| c.content).collect::<String>(), "input");
    assert_eq!(
        name.deref()
            .clone()
            .into_iter()
            .map(|c| c.content)
            .collect::<String>(),
        "input"
    );

    Lifetime::<&str, 1>::try_parse("'i'").unwrap_err();
}

#[test]
fn negative_predicate() {
    #[derive(Clone, Debug, Hash, PartialEq, Eq)]
    pub(crate) struct StrFoo;
    impl StringWrapper for StrFoo {
        const CONTENT: &'static str = "foo";
    }
    #[derive(Clone, Debug, Hash, PartialEq, Eq)]
    pub(crate) struct StrBar;
    impl StringWrapper for StrBar {
        const CONTENT: &'static str = "bar";
    }
    compound_atomic_rule!(
        not_foo_bar,
        "Any character except \"foo\" or \"bar\".",
        Rule,
        Rule::NotFooBar,
        AtomicRepeat<(Negative<Choice2<Str<StrFoo>, Str<StrBar>>>, ANY)>,
        false
    );
    let _ = not_foo_bar::<&str, 1>::try_parse("").unwrap();
    let baz = not_foo_bar::<&str, 1>::try_parse("baz").unwrap();
    for i in baz.iter() {
        let (neg, any) = i;
        assert_eq!(
            <Negative<_> as Pairs<&str, Rule>>::self_or_children(neg).len(),
            0
        );
        assert_eq!(<ANY as Pairs<&str, Rule>>::self_or_children(any).len(), 0);
    }
    let _ = not_foo_bar::<&str, 1>::try_parse("Foofoo").unwrap_err();
    let _ = not_foo_bar::<&str, 1>::try_parse("bazfoo").unwrap_err();
}

#[test]
fn peek() {
    compound_atomic_rule!(
        Rep_1_3,
        "Repeat previously matched expression 1 to 3 times",
        Rule,
        Rule::RepFoo,
        Seq2<
            Skipped<Push<Insens<S, Foo>>, Ignore<S>, 0>,
            Skipped<RepeatMinMax<Skipped<PEEK<S>, Ignore<S>, 0>, 1, 3>, Ignore<S>, 0>,
        >,
        false
    );
    let r = Rep_1_3::<&str, 1>::try_parse("foOfoO").unwrap();
    assert_eq!(
        format!("{:#?}", r),
        "Rep_1_3 {
    content: Seq2(
        Push {
            content: Insens {
                content: \"foO\",
            },
        },
        RepeatMinMax {
            content: [
                PEEK {
                    span: Span {
                        str: \"foO\",
                        start: 3,
                        end: 6,
                    },
                },
            ],
        },
    ),
    span: Span {
        str: \"foOfoO\",
        start: 0,
        end: 6,
    },
}"
    );
    let (head, following) = r.as_ref();
    assert_eq!(head.deref().content, "foO");
    assert_eq!(head.get_content(), Foo::CONTENT);
    for i in following.iter_matched() {
        assert_eq!(i.span.as_str(), head.deref().content);
    }

    Rep_1_3::try_parse("fooFoo").unwrap_err();
    Rep_1_3::try_parse("Foo").unwrap_err();
    Rep_1_3::try_parse("FooFooFooFooFoo").unwrap_err();
    let (pos, _) = Rep_1_3::try_parse_partial("FooFooFooFooFoo").unwrap();
    assert_eq!(pos.pos(), 12);

    {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for input in [
            "foofoo",
            "FooFoo",
            "fOofOo",
            "foOfoO",
            "foofoo",
            "foofoofoo",
        ] {
            set.insert(Rep_1_3::try_parse(input).unwrap());
        }
        assert_eq!(set.len(), 5);
    }
}

#[test]
fn rep() {
    compound_atomic_rule!(
        Rep_0_3,
        "Repeat previously matched expression 0 to 3 times",
        Rule,
        Rule::RepFoo,
        RepeatMin<Skipped<Str<Foo>, Ignore<S>, 0>, 0>,
        false
    );

    let x = Rep_0_3::try_parse("").unwrap();
    assert_eq!(x.ref_inner(), &RepeatMin::default());

    use std::collections::HashSet;

    let mut set = HashSet::new();
    let foos = ["foofoo", "foo", "foofoofoo"];
    for input in foos {
        Rep_0_3::try_check(input).unwrap();
        let r = Rep_0_3::try_parse(input).unwrap();
        for s in r.iter_matched() {
            set.insert(s.clone());
        }
        for s in r.deref().clone().into_iter_matched() {
            set.insert(s);
        }
    }
    assert_eq!(set.len(), 1);

    let mut set = HashSet::new();
    for input in ["foo", "foofoofoo", "foofoo", "foofoofoo"] {
        set.insert(Rep_0_3::try_parse(input).unwrap());
    }
    assert_eq!(set.len(), 3);

    compound_atomic_rule!(
        Rep_2,
        "Repeat \"foo\" 2 times",
        Rule,
        Rule::RepFoo,
        [Insens<S, Foo>; 2],
        false
    );
    for input in ["", "foo", "foofoofoo"] {
        Rep_2::try_check(input).unwrap_err();
        Rep_2::try_parse(input).unwrap_err();
    }
    let arr = Rep_2::try_parse("foofoo").unwrap();
    assert_eq!(
            format!("{:?}", arr),
            "Rep_2 { content: [Insens { content: \"foo\" }, Insens { content: \"foo\" }], span: Span { str: \"foofoo\", start: 0, end: 6 } }"
        );
    let mut set = HashSet::new();
    for input in ["foofoo", "fooFOO", "FooFoo", "foofoo"] {
        set.insert(Rep_2::try_parse(input).unwrap());
    }
    assert_eq!(set.len(), 3);
}

#[test]
fn nested_rep() {
    #[derive(Clone, Hash, PartialEq, Eq)]
    struct Bar1;
    impl StringWrapper for Bar1 {
        const CONTENT: &'static str = "a";
    }
    impl RuleWrapper<Rule> for Bar1 {
        const RULE: Rule = Rule::Bar;
        type Rule = Rule;
    }
    #[derive(Clone, Hash, PartialEq, Eq)]
    struct Bar2;
    impl StringWrapper for Bar2 {
        const CONTENT: &'static str = "b";
    }
    impl RuleWrapper<Rule> for Bar2 {
        const RULE: Rule = Rule::Bar;
        type Rule = Rule;
    }

    compound_atomic_rule!(
        Nested_Rep,
        "Match \"a\"* ~ (SOI ~ ANY | \"b\")",
        Rule,
        Rule::NestedRep,
        (
            RepeatMin<Skipped<Str<Bar1>, Ignore<S>, 0>, 0>,
            Choice2<(SOI, ANY), Str<Bar2>>
        ),
        false
    );

    let x = Nested_Rep::try_parse("x").unwrap();
    assert_eq!(
        format!("{:?}", x),
        "Nested_Rep { content: (RepeatMin { content: [] }, Choice2 { _0: (SOI, ANY { content: 'x' }) }), span: Span { str: \"x\", start: 0, end: 1 } }"
    );

    let x = Nested_Rep::try_parse("ab").unwrap();
    assert_eq!(
        format!("{:?}", x),
        "Nested_Rep { content: (RepeatMin { content: [Str] }, Choice2 { _1: Str }), span: Span { str: \"ab\", start: 0, end: 2 } }"
    );
}
