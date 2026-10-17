//! Unicorn's rules on the platform's APIs that oxlint lacks (EN-081), each one module with the
//! cases of its documentation in `eslint-plugin-unicorn` 78.0.0 as tests, read from the tree
//! alone, as `ESLint` ran them here without type information. No scope turns one off: they run
//! on every TypeScript file.

pub mod known_types;
pub mod no_unsafe_buffer_conversion;
pub mod no_unsafe_json_serialization;
