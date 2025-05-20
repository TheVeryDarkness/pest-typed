#![recursion_limit = "1024"]

use criterion::{criterion_group, criterion_main, Criterion};
use pest::Parser;
use pest_derive::Parser;
use pest_typed::{Spanned, TypedParser};
use pest_typed_derive::TypedParser;
use std::iter::once;

macro_rules! case {
    ($name:ident, $grammar:literal, $input:expr,) => {
        mod $name {
            use super::*;

            mod pest_typed {
                use super::*;
                #[derive(TypedParser)]
                #[grammar_inline = $grammar]
                #[pest_optimizer = false]
                pub struct Parser;
            }

            mod pest_typed_optimized {
                use super::*;
                #[derive(TypedParser)]
                #[grammar_inline = $grammar]
                #[pest_optimizer = true]
                pub struct Parser;
            }

            mod pest {
                use super::*;
                #[derive(Parser)]
                #[grammar_inline = $grammar]
                pub struct Parser;
            }

            pub fn bench(c: &mut Criterion) {
                let mut group = c.benchmark_group(stringify!($name));
                group.sample_size(10);
                let s: String = $input;
                println!("Input string has {} characters.", s.len());
                group.bench_function("pest-typed-parse", |b| {
                    b.iter(|| {
                        let pair = pest_typed::Parser::try_parse::<pest_typed::rules::$name>(&s)
                            .unwrap_or_else(|err| panic!("{}", err));
                        assert_eq!(pair.span().as_str().len(), s.len());
                    })
                });
                group.bench_function("pest-typed-check", |b| {
                    b.iter(|| {
                        let _pair = pest_typed::Parser::try_check::<pest_typed::rules::$name>(&s)
                            .unwrap_or_else(|err| panic!("{}", err));
                    })
                });
                group.bench_function("pest-typed-optimized-parse", |b| {
                    b.iter(|| {
                        let pair = pest_typed_optimized::Parser::try_parse::<
                            pest_typed_optimized::rules::$name,
                        >(&s)
                        .unwrap_or_else(|err| panic!("{}", err));
                        assert_eq!(pair.span().as_str().len(), s.len());
                    })
                });
                group.bench_function("pest-typed-optimized-check", |b| {
                    b.iter(|| {
                        let _pair = pest_typed_optimized::Parser::try_check::<
                            pest_typed_optimized::rules::$name,
                        >(&s)
                        .unwrap_or_else(|err| panic!("{}", err));
                    })
                });
                group.bench_function("pest-parse", |b| {
                    b.iter(|| {
                        let mut pair = pest::Parser::parse(pest::Rule::$name, &s)
                            .unwrap_or_else(|err| panic!("{}", err));
                        assert_eq!(pair.next().unwrap().as_span().as_str().len(), s.len());
                    })
                });
                // group.bench_function("check", |b| {
                //     b.iter(|| {
                //         let _ = pest_typed::rules::$name::check(s).unwrap();
                //     })
                // });
            }
        }
    };
}

case!(
    string_array,
    r#"string_array = { "0123456789"+ }"#,
    "0123456789".repeat(100000),
);

case!(
    char_range_array,
    r#"char_range_array = { ('0'..'9')+ }"#,
    "0123456789".repeat(100000),
);

case!(
    average_choices_array,
    r#"average_choices_array = { ("0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9")+ }"#,
    "0123456789".repeat(100000),
);

case!(
    best_choices_array,
    r#"best_choices_array = { ("0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9")+ }"#,
    "0".repeat(1000000),
);

case!(
    worst_choices_array,
    r#"worst_choices_array = { ("0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9")+ }"#,
    "9".repeat(1000000),
);

case!(
    table,
    r#"
WHITESPACE = _{ " " | "\t" }
cell       = @{ ('0'..'9' | 'a'..'z' | 'A'..'Z')+ }
table      =  { (cell ~ ("," ~ cell)* ~ ","? ~ NEWLINE)+ }"#,
    ('a'..'z')
        .flat_map(|c| once(c)
            .chain(", ".chars())
            .cycle()
            .take(30)
            .chain("\n".chars()))
        .cycle()
        .take(31 * 10000)
        .collect(),
);

case!(
    compact_table,
    r#"
compact_cell       = @{ ('0'..'9' | 'a'..'z' | 'A'..'Z')+ }
compact_table      =  { (compact_cell ~ ("," ~ compact_cell)* ~ ","? ~ NEWLINE)+ }"#,
    ('a'..'z')
        .flat_map(|c| once(c)
            .chain(",".chars())
            .cycle()
            .take(30)
            .chain("\n".chars()))
        .cycle()
        .take(31 * 10000)
        .collect(),
);

criterion_group!(
    benches,
    string_array::bench,
    char_range_array::bench,
    average_choices_array::bench,
    best_choices_array::bench,
    worst_choices_array::bench,
    table::bench,
    compact_table::bench,
);
criterion_main!(benches);
