//! # Seq2
//! A library/program to parse and transform a string of comma separated number to a vector of the said numbers.
//! Strings can be used to specify:
//! - A single number, negative or positive (eg. `"1"`, `"-1"`)
//! - A range of numbers (similar to rust's range syntax) (eg. `"{1..3, s:2}"`, `"{-1..=-10, m:*3}"`)
//! - Basic arithmetic operations (eg. `"(1 + 2 - 3)"`, `"(-2^3 - (3 * 100 / 20))"`)
//!
//! > Note: The library does not support floating point numbers.
//!
//! ## Syntax
//! ### Single numbers
//! Single number are can be any positive or negative number that can fit i64
//!
//! ### Range of numbers
//! A range specification must be encapsulated in squiggly braces `{}` and must adhere the following syntax:
//! - For exclusive ranges:
//!    - `{<START>..<END>}`
//!    - `{<START>..<END>, s:<STEP>}`
//!    - `{<START>..<END>, m:<MUTATION>}`
//!    - `{<START>..<END>, s:<STEP>, m:<MUTATION>}`
//! - For inclusive ranges:
//!    - `{<START>..=<END>}`
//!    - `{<START>..=<END>, s:<STEP>}`
//!    - `{<START>..=<END>, m:<MUTATION>}`
//!    - `{<START>..=<END>, s:<STEP>, m:<MUTATION>}`
//!
//! #### `<START>`, `<END>`:
//! Any positive or negative number that can fit i64.
//! If the `END` is smaller than the `START`, the parser will assume
//! you wish to decrement the numbers.
//!
//! i.e.
//!   - `{3..=1}` will be parsed to `3, 2, 1`
//!   - `{-3..=-6}` will be parsed to `-3, -4, -5, -6`
//!
//! #### `s:<STEP>` (_Optional argument_):
//! The increment or decrement between each number in the range.
//! Value must be prefixed with `s:`.
//! If no `STEP` is  specified, the default `step` is 1 or -1.
//! `STEP` must respect the `START` and `END` of the range.
//! Meaning if the `START` is smaller than the `END`, the `STEP` must be positive
//! and if the `START` is greater than the `END`, the `STEP` must be negative.
//!
//! Additionally, the final output vector cannot exceed the `END`. In case the final `STEP`
//! would exceed the `END`, the closet number to the `END` will be used as the final number.
//!
//! i.e.
//!   - `{1..=5, s:2}` will be parsed to `1, 3, 5`
//!   - `{5..=0, s:-2}` will be parsed to `5, 3, 1` (-1 is trimmed as it exceeds the `END`)
//!
//! #### `m:<MUTATION>` (_Optional argument_):
//! The mutation (an arithmetic operation) to be applied to each number in the range.
//! Value must be prefixed with `m:`.
//! If not specified, not mutations will be applied.
//!
//! The `MUTATION` is applied after each `STEP` increment/decrement of the range
//! and must written as an arithmetic operation that assumes the number to be mutated
//! will be on the lhs of the operation.
//!
//! i.e.
//!   - `{1..=5, m:+2}` will be parsed to `3, 5, 7`
//!   - `{5..=1, s:-2, m:-2}` will be parsed to `3, 1, -1`
//!   - `{5..=0, s:-2, m:-2}`
//!
//! ### Basic arithmetic operations
//! Basic arithmetic operations can be applied to any number or range of numbers.
//! The operations must be encapsulated in parenthesis `()`.
//! The operations can be any combination of the following:
//! - Addition `+`
//! - Subtraction `-`
//! - Multiplication `*`
//! - Division `/`
//! - Exponentiation `^`
//! > Note: Any floating point number will be truncated to an integer.
//!
//! The operations can be applied set the `START` or `END` of a number range.
//!
//! i.e.
//!   - `"(1 + 2 - 3)"` will be parsed to `0`
//!   - `"(-2^3 - (3 * 100 / 20))"` will be parsed to `-23`
//!   - `"{(1 - (10 ^ 2))..-108, s:3, m:*-1}"` will be parsed to `99, 102, 105`
//!     > **Breakdown of the above example:**
//!     > 1. `1 - (10 ^ 2)` will be calculated to `-99` (range start)
//!     > 2. From `-99`, the number will decrement as specified by the step `s:3`
//!          and then mutated by `m:*-1`. (`-99*-1`, `-102*-1`, etc.)
//!     > 3. Stops generating new numbers once `-108` is reached.
//!
//! ## Chaining all the syntaxes
//! All the syntaxes can be chained together to create complex number vectors.
//! The parser will parse the string from left to right and apply the operations in the order they are found.
//!
//! i.e.
//!   - `"-1, -2, -3, {1..=3, s:2, m:+2}, (200 ^ 2 + 1)"` will be parsed to `-1, -2, -3, 3, 5, 7, 400001`

pub mod errors;
pub mod lexer;
pub(crate) mod parser;
pub(crate) mod tokens;
