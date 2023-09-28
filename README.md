# Statically Typed Pest

[Pest](https://pest.rs) provides a elegant and convenient way to implement a parser. However, it's not convenient to consume its output as the syntax tree is not statically tagged.

This motivates me to develop a statically typed version of pest based on current version of pest.

## Usage

## Note

The main drawback of this crate is that it compiles much slower that pest.

So, if you are to test your grammar, you can use pest in your parsing tests.

## Some Other Crates With Similar Aims

|     name     |                   repository                    |         documentation         |
| :----------: | :---------------------------------------------: | :---------------------------: |
| pest_consume |    https://github.com/Nadrieril/pest_consume    | https://docs.rs/pest_consume/ |
|   pest-ast   | https://github.com/pest-parser/pest_deconstruct |   https://docs.rs/pest-ast/   |