//! The rules, one module each with its tests, from the cases of the rule it carries. A rule
//! reads the parsed file once (`check_file`) or each node of the one walk the linter makes over
//! the AST (`check_node`), never parsing again.

pub mod barrel_exports;
pub mod canonical_import_path;
pub mod folder_has_index;
pub mod folder_layout;
pub mod functional;
pub mod immutable_data;
pub mod imports;
pub mod jsdoc;
pub mod kind_in_layer;
pub mod kind_limits;
pub mod kind_naming;
pub mod naming_convention;
pub mod no_conditional_statements;
pub mod no_mixed_types;
pub mod no_non_function_verb_prefix;
pub mod no_restricted_syntax;
pub mod no_return_void;
pub mod one_export_per_file;
pub mod prefer_immutable_types;
pub mod require_atomic_updates;
pub mod require_kind;
pub mod see_references;
pub mod stylistic;
pub mod type_declaration_immutability;
pub mod unicorn;

use oxc_semantic::AstNode;

use crate::finding::Finding;
use crate::source::Source;

/// A rule of the linter.
pub trait Rule: Sync {
    /// Its name, as scopes and directives write it.
    fn name(&self) -> &'static str;

    /// Whether it reads the file's tokens, which the parser then keeps.
    fn needs_tokens(&self) -> bool {
        false
    }

    /// Whether it reads each node.
    fn checks_nodes(&self) -> bool {
        false
    }

    /// Checks the whole file, once.
    fn check_file(&self, _source: &Source, _findings: &mut Vec<Finding>) {}

    /// Checks one node of the walk.
    fn check_node(&self, _node: &AstNode, _source: &Source, _findings: &mut Vec<Finding>) {}
}

/// Every rule of the linter.
pub const ALL: &[&dyn Rule] = &[
    &barrel_exports::BarrelExports,
    &canonical_import_path::CanonicalImportPath,
    &folder_has_index::FolderHasIndex,
    &folder_layout::FolderLayout,
    &immutable_data::ImmutableData,
    &kind_in_layer::KindInLayer,
    &kind_limits::KindLimits,
    &kind_naming::KindNaming,
    &naming_convention::NamingConvention,
    &no_conditional_statements::NoConditionalStatements,
    &no_mixed_types::NoMixedTypes,
    &no_non_function_verb_prefix::NoNonFunctionVerbPrefix,
    &no_restricted_syntax::NoRestrictedSyntax,
    &no_return_void::NoReturnVoid,
    &one_export_per_file::OneExportPerFile,
    &prefer_immutable_types::PreferImmutableTypes,
    &require_atomic_updates::RequireAtomicUpdates,
    &require_kind::RequireKind,
    &see_references::SeeReferences,
    &type_declaration_immutability::TypeDeclarationImmutability,
];

/// The rules of a list of names, in the registry's order, then the other plugins' (EN-084), then
/// Unicorn's, its DOM and web rules, then those of `eslint-plugin-jsdoc`.
#[must_use]
pub fn named(names: &[&str]) -> Vec<&'static dyn Rule> {
    ALL.iter()
        .chain(functional::RULES)
        .chain(imports::RULES)
        .chain(stylistic::RULES)
        .chain(unicorn::RULES)
        .chain(unicorn::dom::RULES)
        .chain(unicorn::web::RULES)
        .chain(jsdoc::RULES)
        .copied()
        .filter(|rule| names.contains(&rule.name()))
        .collect()
}
