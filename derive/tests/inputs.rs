use pest_typed::{ParsableTypedNode, Position, Span, Spanned as _};
use pest_typed_derive::TypedParser;

#[allow(dead_code)]
#[derive(TypedParser)]
#[grammar_inline = r#"
skip = @{ (!("\n\r"|"\r"|"\n") ~ ANY)* }

main = @{ "a"* ~ (SOI ~ ANY | "b") ~ EMOJI+ ~ " " ~ ('0'..'9')+ ~ (!("bd"|"bc"|"cd") ~ ANY)* ~ ("b"|"c") ~ (^"c" | ^"d")+  }
"#]
struct Parser;

#[test]
fn string() -> anyhow::Result<()> {
    let input = "ab😀 123xxbccd";

    let main = rules::main::try_parse(input)?;
    assert_eq!(main.span(), Span::new_full(input));

    let input = &input.to_owned();

    let main = rules::main::try_parse(input)?;
    assert_eq!(main.span(), Span::new_full(input));

    Ok(())
}

#[test]
fn position() -> anyhow::Result<()> {
    let input = "aab😃 456yybccd";
    let pos = Position::new(input, 1).unwrap();
    let span = pos.span(&Position::new(input, input.len()).unwrap());

    let main = rules::main::try_parse(pos)?;
    assert_eq!(main.span(), span);

    Ok(())
}

#[test]
fn span() -> anyhow::Result<()> {
    let input = "aab😄 789zzbccddw";
    let span = Span::new(input, 1, input.len() - 2).unwrap();

    let main = rules::main::try_parse(span)?;
    assert_eq!(main.span(), span);

    Ok(())
}

#[test]
fn bad_skip() -> anyhow::Result<()> {
    let input = "<abc>";
    let span = Span::new(input, 1, input.len() - 1).unwrap();
    let main = rules::skip::try_parse(span)?;
    assert_eq!(main.span(), span);

    let input = "<abc";
    let pos = Position::new(input, 1).unwrap();
    let span = Span::new(input, 1, input.len()).unwrap();
    let main = rules::skip::try_parse(pos)?;
    assert_eq!(main.span(), span);

    let input = "abc";
    let span = Span::new_full(input);
    let main = rules::skip::try_parse(input)?;
    assert_eq!(main.span(), span);

    Ok(())
}
