//! Unicorn's rules on conditions, branches and scopes that oxlint lacks (EN-079), each one
//! module with the cases of its documentation in `eslint-plugin-unicorn` 78.0.0 as tests, read
//! from the tree alone, as `ESLint` ran them here without type information. No scope turns one
//! off: they run on every TypeScript file. `no-unnecessary-nested-ternary` is not carried:
//! `no-nested-ternary` (oxlint) already forbids every nested conditional expression.

pub mod no_declarations_before_early_exit;
pub mod no_duplicate_if_branches;
pub mod no_duplicate_logical_operands;
pub mod no_useless_boolean_cast;
pub mod no_useless_else;
pub mod no_useless_logical_operand;
pub mod prefer_boolean_return;
pub mod prefer_combined_guards;

use super::Rule;

/// The rules, in the order of their names.
pub const RULES: &[&dyn Rule] = &[
    &no_declarations_before_early_exit::NoDeclarationsBeforeEarlyExit,
    &no_duplicate_if_branches::NoDuplicateIfBranches,
    &no_duplicate_logical_operands::NoDuplicateLogicalOperands,
    &no_useless_boolean_cast::NoUselessBooleanCast,
    &no_useless_else::NoUselessElse,
    &no_useless_logical_operand::NoUselessLogicalOperand,
    &prefer_boolean_return::PreferBooleanReturn,
    &prefer_combined_guards::PreferCombinedGuards,
];
