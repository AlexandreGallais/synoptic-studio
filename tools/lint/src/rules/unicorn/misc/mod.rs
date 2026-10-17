//! Unicorn's miscellaneous rules that oxlint lacks (EN-080): optional chains, compound
//! assignments, checks before a deletion and URLs, each one module with the cases of its
//! documentation in `eslint-plugin-unicorn` 78.0.0 as tests. No scope turns one off: they run on
//! every TypeScript file.

pub mod consistent_optional_chaining;
pub mod no_useless_compound_assignment;
pub mod no_useless_delete_check;
pub mod prefer_https;
