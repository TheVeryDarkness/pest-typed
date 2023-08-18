// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use pest_typed::{
        atomic_rule, choices::Choice2, compound_atomic_rule, non_atomic_rule, normal_rule,
        predefined_node::*, rule_eoi, sequence::Seq2, silent_rule, BoundWrapper, ParsableTypedNode,
        RuleWrapper, Storage, StringArrayWrapper, StringWrapper, TypeWrapper,
    };
    use std::ops::Deref;

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum Rule {
        Foo,
        RepFoo,
        NotFooBar,
        WHITESPACE,
        COMMENT,
        EOI,
        String,
    }

    #[derive(Clone, PartialEq)]
    pub struct Foo;
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
        "Comment",
        Rule,
        Rule::COMMENT,
        CharRange::<'\t', '\t'>
    );
    normal_rule!(
        StrFoo,
        "String \"Foo\"",
        Rule,
        Rule::Foo,
        Str::<Foo>,
        Ignore::<'i>
    );
    rule_eoi!(EOI, Rule);

    #[test]
    fn string() {
        assert_eq!(<StrFoo<'_> as TypeWrapper>::Inner::CONTENT, Foo::CONTENT);
        let s = StrFoo::parse("foo").unwrap();
        assert_eq!(s.content.get_content(), "foo");
        assert_eq!(
            format!("{:?}", s),
            r#"Rule { name: "StrFoo", content: Str, span: Span { str: "foo", start: 0, end: 3 } }"#
        )
    }

    #[test]
    fn range() {
        let whitespace = WHITESPACE::parse(" ").unwrap();
        assert_eq!(
            format!("{:?}", whitespace),
            "Rule { name: \"WHITESPACE\", span: Span { str: \" \", start: 0, end: 1 } }"
        );
        let comment = COMMENT::parse("\t").unwrap();
        assert_eq!(
            format!("{:?}", comment),
            "Rule { name: \"COMMENT\", content: CharRange { content: '\\t' }, span: Span { str: \"\\t\", start: 0, end: 1 } }"
        );
    }

    type Ignore<'i> = Skipped<COMMENT<'i>, WHITESPACE<'i>>;

    #[test]
    fn ignore() {
        silent_rule!(
            tmp,
            "Temporary rule.",
            Rule,
            Rule::RepFoo,
            Ignore<'i>,
            Ignore<'i>
        );
        tmp::parse(" \t  ").unwrap();
    }

    #[test]
    fn repetition() {
        type REP<'i> = Rep<Str<Foo>, Ignore<'i>>;
        non_atomic_rule!(
            R,
            "Repetion of [StrFoo].",
            Rule,
            Rule::RepFoo,
            REP<'i>,
            Ignore<'i>
        );

        let rep1 = R::parse("foofoofoo").unwrap();
        let rep2 = R::parse("foo foo foo").unwrap();
        let rep3 = R::parse("foo foo\tfoo").unwrap();
        let _ = R::parse("").unwrap();
        assert_ne!(rep1, rep2);
        assert_ne!(rep1, rep3);

        let format = |rep: &R<'_>| -> String {
            rep.iter()
                .map(|e| e.get_content())
                .collect::<Vec<_>>()
                .concat()
        };
        assert_eq!(format(&rep1), format(&rep2));
        assert_eq!(format(&rep1), format(&rep3));

        assert_eq!(REP::MIN, 0);
        assert_eq!(rep1.deref().get_min_len(), 0);
        assert_eq!(rep1.deref().get_max_len(), usize::MAX);
        assert_eq!(<R<'_> as TypeWrapper>::Inner::MIN, 0);
    }

    #[test]
    fn repetition_at_least_once() {
        type REP<'i> = RepOnce<Insens<'i, Foo>, Ignore<'i>>;
        non_atomic_rule!(
            R,
            "Repetion of [StrFoo].",
            Rule,
            Rule::RepFoo,
            REP<'i>,
            Ignore<'i>
        );

        let rep1 = R::parse("fooFoofoo").unwrap();
        let rep2 = R::parse("foo Foo foo").unwrap();
        let rep3 = R::parse("Foo foo\tfoo").unwrap();
        let rep4 = R::parse("Foofoofoo").unwrap();
        assert_ne!(rep1, rep2);
        assert_ne!(rep1, rep3);
        assert_ne!(rep1, rep4);

        let collect = |r: &R<'_>| {
            r.iter()
                .map(|r| r.get_content())
                .collect::<Vec<_>>()
                .concat()
        };
        assert_eq!(collect(&rep1), collect(&rep2));
        assert_eq!(collect(&rep1), collect(&rep3));
        assert_eq!(collect(&rep1), collect(&rep4));

        assert_eq!(REP::MIN, 1);
        assert_eq!(rep1.deref().get_min_len(), 1);
        assert_eq!(rep1.deref().get_max_len(), usize::MAX);
        assert_eq!(<R<'_> as TypeWrapper>::Inner::MIN, 1);
    }

    #[test]
    fn skip() {
        #[derive(Clone, Debug, PartialEq)]
        pub struct NewLine;
        impl StringArrayWrapper for NewLine {
            const CONTENT: &'static [&'static str] = &["\r\n", "\n", "\r"];
        }
        compound_atomic_rule!(
            QuotedString,
            "Quoted string.",
            Rule,
            Rule::String,
            Seq2<Skip<'i, NewLine>, NEWLINE, Ignore<'i>>
        );

        let s1 = QuotedString::parse("2\r\n").unwrap();
        let s2 = QuotedString::parse("\r\n").unwrap();
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
            SkipChar<'i, 3>
        );
        three::parse("foo").unwrap();
        three::parse("foobar").unwrap_err();
    }

    #[test]
    fn negative_predicate() {
        #[derive(Clone, Debug, PartialEq)]
        pub struct StrFoo;
        impl StringWrapper for StrFoo {
            const CONTENT: &'static str = "foo";
        }
        #[derive(Clone, Debug, PartialEq)]
        pub struct StrBar;
        impl StringWrapper for StrBar {
            const CONTENT: &'static str = "bar";
        }
        compound_atomic_rule!(
            not_foo_bar,
            "Any character except \"foo\" or \"bar\".",
            Rule,
            Rule::NotFooBar,
            Rep<Seq2<Negative<Choice2<Str<StrFoo>, Str<StrBar>>>, ANY, Ignore<'i>>, Ignore<'i>>
        );
        not_foo_bar::parse("").unwrap();
        not_foo_bar::parse("baz").unwrap();
        not_foo_bar::parse("Foofoo").unwrap_err();
        not_foo_bar::parse("bazfoo").unwrap_err();
    }
}
