//! Unicorn's rules on the web platform that oxlint lacks (EN-082): URLs, `Blob` and `File`, each
//! one module with the cases of its documentation in `eslint-plugin-unicorn` 78.0.0 as tests,
//! read from the tree alone, as `ESLint` ran them here without type information. `NAMES` run on
//! every TypeScript file, `URL` and `Blob` being Node.js's too; `DOM_NAMES` only where a DOM
//! exists (`scopes::DOM_FILES`).

use crate::rules::Rule;

/// The rules, in the order of their names.
pub const RULES: &[&dyn Rule] = &[];

/// The names of those on for every TypeScript file.
pub const NAMES: &[&str] = &[];

/// The names of those on for the files where a DOM exists.
pub const DOM_NAMES: &[&str] = &[];
