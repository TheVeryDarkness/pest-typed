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

fn main() {
    let s = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/benches/Canada.json"))
        .unwrap();
    let json_file = &s;
    let t = std::time::Instant::now();
    let _ = json_pest::JsonParser::parse(json_pest::Rule::json, json_file).unwrap();
    eprintln!("{}", t.elapsed().as_nanos());
    let _ = json_typed::pairs::json::try_parse(json_file.as_str());
    eprintln!("{}", t.elapsed().as_nanos());
}
