//! Unicorn's rules on objects, properties and destructuring that oxlint lacks (EN-080), each one
//! module with the cases of its documentation in `eslint-plugin-unicorn` 78.0.0 as tests, read
//! from the syntax alone, as `ESLint` ran them here without type information. No scope turns
//! one off: they run on every TypeScript file.

pub mod consistent_destructuring;
pub mod no_computed_property_existence_check;
pub mod no_invalid_property_descriptor;
pub mod no_invalid_well_known_symbol_methods;
pub mod no_unreadable_new_expression;
pub mod no_unreadable_object_destructuring;
pub mod no_unsafe_property_key;
pub mod no_unused_builtin_method_return;
pub mod no_unused_properties;
pub mod prefer_object_define_properties;
pub mod prefer_object_destructuring_defaults;
pub mod prefer_single_object_destructuring;
