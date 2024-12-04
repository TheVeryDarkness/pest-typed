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
  - `grammar_inline`: provide grammars in an inline string.
- Generation option:

  |             Attribute name              | Default value |                                                                                                                    Meaning                                                                                                                    |
  | :-------------------------------------: | :-----------: | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: |
  |          `emit_rule_reference`          |     false     |                                                                            Emit [getter functions](#getter-functions) for those rules referenced by current rule.                                                                             |
  |      `emit_tagged_node_reference`       |     false     | Emit [getter functions](#getter-functions) for those tagged nodes referenced by current rule. Only takes effect when node tags are enabled (currently controlled by feature **grammar-extras** of [pest](https://docs.rs/pest/latest/pest/)). |
  |           `do_not_emit_span`            |     false     |                                                                                                      Never emit field `span` for rules.                                                                                                       |
  |     `truncate_getter_at_node_tag`     |     true      |                                                            Generated [getter functions](#getter-functions) won't contain those referenced rules or tags inside a tagged node.                                                             |
  | `simulate_pair_api` (Currently ignored) |     false     |                                                                                               Generate implementation of Pair for rule structs                                                                                                |
  |          `box_only_if_needed`           |     false     |                                                                                                 Wrap rule content in `Box` only if necessary.                                                                                                 |
  |              `no_warnings`              |     false     |                                                                                                        Suppress warnings in generator.                                                                                                        |

  box_only_if_needed
  See [pest_typed_derive](https://docs.rs/pest_typed_derive/latest/pest_typed_derive/) for more examples.

- Processing option:
  - `no_warnings`: do not show warnings.

## Getter functions

An getter function is a function generated to access the referenced rules or tagged nodes.

Getter function is named with the same name of the referenced rule or tag.
