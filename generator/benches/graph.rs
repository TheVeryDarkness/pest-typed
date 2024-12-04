use anyhow::Result;
use criterion::{criterion_group, criterion_main, Criterion};
use pest_typed_generator::derive_typed_parser;
use quote::quote;
use std::fmt::Write;

fn complete_graph_26(c: &mut Criterion) {
    (|| -> Result<()> {
        let mut s = String::new();
        for c in 'a'..='z' {
            write!(s, "{} = {{ \"{}\"", c, c)?;
            for d in 'a'..='z' {
                write!(s, " ~ {}?", d)?;
            }
            write!(s, " }}")?;
        }
        c.bench_function("complete_graph_26", |c| {
            c.iter(|| {
                let _output = derive_typed_parser(
                    quote! {
                        #[grammar_inline = #s]
                        #[pest_optimizer = false]
                        struct x;
                    },
                    false,
                    false,
                );
            });
        });
        c.bench_function("complete_graph_26 optimized", |c| {
            c.iter(|| {
                let _output = derive_typed_parser(
                    quote! {
                        #[grammar_inline = #s]
                        struct x;
                    },
                    false,
                    false,
                );
            });
        });

        Ok(())
    })()
    .unwrap()
}

fn circle_260(c: &mut Criterion) {
    (|| -> Result<()> {
        let mut s = String::new();
        let chars: [char; 26] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        for i in 0..260 {
            let c0 = chars[i % 26];
            let i0 = i / 26;
            let i_ = (i + 1) % 260;
            let c1 = chars[i_ % 26];
            let i1 = i_ / 26;
            write!(s, "{}{} = {{ \"{}\" ~ {}{}? }}", c0, i0, c0, c1, i1)?;
        }
        c.bench_function("circle_260", |c| {
            c.iter(|| {
                let _output = derive_typed_parser(
                    quote! {
                        #[grammar_inline = #s]
                        #[pest_optimizer = false]
                        struct x;
                    },
                    false,
                    false,
                );
            });
        });
        c.bench_function("circle_260 optimized", |c| {
            c.iter(|| {
                let _output = derive_typed_parser(
                    quote! {
                        #[grammar_inline = #s]
                        struct x;
                    },
                    false,
                    false,
                );
            });
        });

        Ok(())
    })()
    .unwrap()
}

criterion_group!(benches, complete_graph_26, circle_260);
criterion_main!(benches);
