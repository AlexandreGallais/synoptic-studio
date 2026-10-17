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
pub mod prefer_early_return;
pub mod prefer_else_if;
pub mod prefer_hoisting_branch_code;
pub mod prefer_minimal_ternary;
pub mod prefer_simple_condition_first;
pub mod prefer_simplified_conditions;
pub mod prefer_smaller_scope;
pub mod prefer_switch;

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
    &prefer_early_return::PreferEarlyReturn,
    &prefer_else_if::PreferElseIf,
    &prefer_hoisting_branch_code::PreferHoistingBranchCode,
    &prefer_minimal_ternary::PreferMinimalTernary,
    &prefer_simple_condition_first::PreferSimpleConditionFirst,
    &prefer_simplified_conditions::PreferSimplifiedConditions,
    &prefer_smaller_scope::PreferSmallerScope,
    &prefer_switch::PreferSwitch,
];
