//! Unicorn's rules on numbers, arithmetic and comparisons that oxlint lacks (EN-079), each one
//! module with the cases of its documentation in `eslint-plugin-unicorn` 78.0.0 as tests, read
//! from the tree alone, as `ESLint` ran them here without type information. No scope turns one
//! off: they run on every TypeScript file.

pub mod no_negated_comparison;
pub mod no_redundant_comparison;
pub mod no_subtraction_comparison;
pub mod prefer_math_abs;
pub mod prefer_number_is_safe_integer;
pub mod prefer_unary_minus;
