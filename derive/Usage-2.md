- Implement `pest_typed::TypedParser` for dervied struct.
- Generate several modules:

  - `pairs`, which contains the definitions of some generated types. Each type corresponds to a rule.
  - `constant_wrappers` and `rule_wrappers`, which contain some wrappers that are passed to predefined generics of statically typed nodes.
  - `unicode`, which contains the definitions of Unicode-related built-in rules.

  Only the first one is expected to be referenced by the user.

## Attributes

Attributes:

- Provide grammar file (see [pest.rs](https://pest.rs) and [its documents](https://docs.rs/pest/latest/pest/) for more information):
  - `grammar`: specify grammar file path.
  - `grammar`: provide grammars in an inline string.
- Generation option:
  - `emit_rule_reference`: emit [accesser functions](#accesser-functions) for those rules referenced by current rule.
  - `emit_tagged_node_reference`: emit [accesser functions](#accesser-functions) for those tagged nodes referenced by current rule.
    Only takes effect when node tags are enabled (currently controlled by feature **grammar-extras** of [pest](https://docs.rs/pest/latest/pest/).).
  - `do_not_emit_span`: never emit field `span` for rules.

## Accesser functions

An accesser function is a function generated to access the referenced rules or tagged nodes.

Accesser function is named with the same name of the referenced rule or tag.
