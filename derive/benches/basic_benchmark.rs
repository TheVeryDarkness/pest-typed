use criterion::{criterion_group, criterion_main, Criterion};
use pest::Parser;
use pest_typed::ParsableTypedNode;

pub mod json_typed {
    use pest_typed_derive::TypedParser;
    //
    #[derive(TypedParser)]
    #[grammar = "benches/json.pest"]
    #[emit_rule_reference]
    pub struct JsonParser;
}

pub mod json_pest {
    use pest_derive::Parser;
    //
    #[derive(Parser)]
    #[grammar = "benches/json.pest"]
    pub struct JsonParser;
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-example");
    group.sample_size(10);
    let s = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/benches/Canada.json"))
        .unwrap();

    group.bench_function("json_pest", |b| {
        b.iter(|| {
            let json_file = &s;
            let _ = json_pest::JsonParser::parse(json_pest::Rule::json, json_file).unwrap();
        })
    });
    group.bench_function("json_typed", |b| {
        b.iter(|| {
            let json_file = &s;
            let _ = json_typed::pairs::json::try_parse(json_file);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
