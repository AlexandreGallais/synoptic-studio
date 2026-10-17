//! The rules, one module each with its tests, from the cases of the rule it carries. A rule
//! reads the parsed file once (`check_file`) or each node of the one walk the linter makes over
//! the AST (`check_node`), never parsing again.

pub mod kind_in_layer;
pub mod kind_limits;
pub mod kind_naming;
pub mod require_kind;
pub mod see_references;

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
    &kind_in_layer::KindInLayer,
    &kind_limits::KindLimits,
    &kind_naming::KindNaming,
    &require_kind::RequireKind,
    &see_references::SeeReferences,
];

/// The rules of a list of names, in the registry's order.
#[must_use]
pub fn named(names: &[&str]) -> Vec<&'static dyn Rule> {
    ALL.iter()
        .copied()
        .filter(|rule| names.contains(&rule.name()))
        .collect()
}
