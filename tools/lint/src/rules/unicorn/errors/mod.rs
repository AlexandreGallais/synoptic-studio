//! Unicorn's rules on errors that oxlint lacks (EN-079), each one module with the cases of its
//! documentation in `eslint-plugin-unicorn` 78.0.0 as tests, read from the tree alone, as
//! `ESLint` ran them here without type information. No scope turns one off: they run on every
//! TypeScript file, `throw` and `try` being allowed in the shell, the tests and the tools.

pub mod no_error_property_assignment;
pub mod prefer_aggregate_error;
pub mod prefer_error_is_error;
pub mod try_complexity;
