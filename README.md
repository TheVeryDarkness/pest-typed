# Statically Typed Pest

[Pest](https://pest.rs) provides a elegant and convenient way to implement a parser. However, it's not convenient to consume its output as the syntax tree is not statically tagged.

This motivates me to develop a statically typed version of pest based on current version of pest.

## Usage

See our [documentation](https://docs.rs/pest_typed_derive) for some notes.

## Note

The main drawback of this crate is that it compiles much slower than pest.

So, if you are just to test your grammar, you can use pest in your parsing tests.

Maybe separate your codes into several crates, use a feature in the parser crate to control which one (pest or pest_typed) is used, and use pest when testing whether the parser can parse your test inputs.

## Some Other Crates With Similar Aims

Before working on this project, I've searched for crates with similar aims. They're great, too.

|     name     |                   repository                    |         documentation         |
| :----------: | :---------------------------------------------: | :---------------------------: |
|   pest-ast   | https://github.com/pest-parser/pest_deconstruct |   https://docs.rs/pest-ast/   |
| pest_consume |    https://github.com/Nadrieril/pest_consume    | https://docs.rs/pest_consume/ |
