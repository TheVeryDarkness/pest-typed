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

criterion_group!(benches, complete_graph_26);
criterion_main!(benches);
