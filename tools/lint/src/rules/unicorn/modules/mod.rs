//! Unicorn's rules on modules and the global object that oxlint lacks (EN-081), each one module
//! with the cases of its documentation in `eslint-plugin-unicorn` 78.0.0 as tests, read from the
//! tree alone, as `ESLint` ran them here without type information. They run on every TypeScript
//! file, but `no-top-level-side-effects`, which leaves the files where `ESLint` turned it off.

pub mod globals;
pub mod no_global_object_property_assignment;
pub mod no_top_level_side_effects;
pub mod no_unnecessary_global_this;
pub mod no_useless_re_export;
pub mod prefer_global_number_constants;
pub mod prefer_identifier_import_export_specifiers;
pub mod prefer_json_import;
