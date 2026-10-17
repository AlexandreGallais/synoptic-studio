//! Unicorn's rules that oxlint lacks: arrays, collections and iteration (EN-078), and in their own
//! folders control flow, errors, functions and numbers (EN-079), modules, the platform and
//! promises (EN-081); each one module with the cases of its documentation in
//! `eslint-plugin-unicorn` 78.0.0 as tests, read from the syntax alone. No scope turns one off:
//! they run on every TypeScript file, `no-top-level-side-effects` leaving by itself the files
//! where `ESLint` turned it off.

pub mod consistent_conditional_object_spread;
pub mod control_flow;
pub mod errors;
pub mod functions;
pub mod iteration_fallback_style;
pub mod modules;
pub mod no_array_from_fill;
pub mod no_array_front_mutation;
pub mod no_array_sort_for_min_max;
pub mod no_break_in_nested_loop;
pub mod no_confusing_array_splice;
pub mod no_duplicate_loops;
pub mod no_duplicate_set_values;
pub mod no_impossible_length_comparison;
pub mod no_loop_iterable_mutation;
pub mod no_mismatched_map_key;
pub mod no_negated_array_predicate;
pub mod no_object_methods_with_collections;
pub mod no_return_array_push;
pub mod no_unnecessary_array_flat_map;
pub mod no_unnecessary_splice;
pub mod no_unreadable_for_of_expression;
pub mod no_unused_iterator_helper;
pub mod no_useless_set_construction;
pub mod numbers;
pub mod platform;
pub mod prefer_array_from_map;
pub mod prefer_array_iterable_methods;
pub mod prefer_array_last_methods;
pub mod prefer_array_slice;
pub mod prefer_direct_iteration;
pub mod prefer_flat_math_min_max;
pub mod prefer_group_by;
pub mod prefer_has_check;
pub mod prefer_includes_over_repeated_comparisons;
pub mod prefer_iterable_in_constructor;
pub mod prefer_iterator_helpers;
pub mod prefer_iterator_to_array;
pub mod prefer_iterator_to_array_at_end;
pub mod prefer_map_from_entries;
pub mod prefer_object_iterable_methods;
pub mod prefer_set_methods;
pub mod prefer_simple_sort_comparator;
pub mod prefer_single_array_predicate;
pub mod prefer_while_loop_condition;
pub mod syntax;

use super::Rule;

/// The rules, in the order of their names.
pub const RULES: &[&dyn Rule] = &[
    &consistent_conditional_object_spread::ConsistentConditionalObjectSpread,
    &iteration_fallback_style::IterationFallbackStyle,
    &no_array_from_fill::NoArrayFromFill,
    &no_array_front_mutation::NoArrayFrontMutation,
    &no_array_sort_for_min_max::NoArraySortForMinMax,
    &no_break_in_nested_loop::NoBreakInNestedLoop,
    &no_confusing_array_splice::NoConfusingArraySplice,
    &control_flow::no_declarations_before_early_exit::NoDeclarationsBeforeEarlyExit,
    &control_flow::no_duplicate_if_branches::NoDuplicateIfBranches,
    &control_flow::no_duplicate_logical_operands::NoDuplicateLogicalOperands,
    &no_duplicate_loops::NoDuplicateLoops,
    &no_duplicate_set_values::NoDuplicateSetValues,
    &errors::no_error_property_assignment::NoErrorPropertyAssignment,
    &modules::no_global_object_property_assignment::NoGlobalObjectPropertyAssignment,
    &no_impossible_length_comparison::NoImpossibleLengthComparison,
    &no_loop_iterable_mutation::NoLoopIterableMutation,
    &no_mismatched_map_key::NoMismatchedMapKey,
    &no_negated_array_predicate::NoNegatedArrayPredicate,
    &numbers::no_negated_comparison::NoNegatedComparison,
    &no_object_methods_with_collections::NoObjectMethodsWithCollections,
    &numbers::no_redundant_comparison::NoRedundantComparison,
    &functions::no_repeated_call_wrappers::NoRepeatedCallWrappers,
    &no_return_array_push::NoReturnArrayPush,
    &numbers::no_subtraction_comparison::NoSubtractionComparison,
    &modules::no_top_level_side_effects::NoTopLevelSideEffects,
    &functions::no_uncalled_method::NoUncalledMethod,
    &no_unnecessary_array_flat_map::NoUnnecessaryArrayFlatMap,
    &functions::no_unnecessary_default_arguments::NoUnnecessaryDefaultArguments,
    &modules::no_unnecessary_global_this::NoUnnecessaryGlobalThis,
    &functions::no_unnecessary_parameters::NoUnnecessaryParameters,
    &no_unnecessary_splice::NoUnnecessarySplice,
    &no_unreadable_for_of_expression::NoUnreadableForOfExpression,
    &platform::no_unsafe_buffer_conversion::NoUnsafeBufferConversion,
    &platform::no_unsafe_json_serialization::NoUnsafeJsonSerialization,
    &no_unused_iterator_helper::NoUnusedIteratorHelper,
    &control_flow::no_useless_boolean_cast::NoUselessBooleanCast,
    &control_flow::no_useless_else::NoUselessElse,
    &control_flow::no_useless_logical_operand::NoUselessLogicalOperand,
    &modules::no_useless_re_export::NoUselessReExport,
    &functions::no_useless_recursion::NoUselessRecursion,
    &no_useless_set_construction::NoUselessSetConstruction,
    &errors::prefer_aggregate_error::PreferAggregateError,
    &prefer_array_from_map::PreferArrayFromMap,
    &prefer_array_iterable_methods::PreferArrayIterableMethods,
    &prefer_array_last_methods::PreferArrayLastMethods,
    &prefer_array_slice::PreferArraySlice,
    &functions::prefer_block_statement_over_iife::PreferBlockStatementOverIife,
    &control_flow::prefer_boolean_return::PreferBooleanReturn,
    &control_flow::prefer_combined_guards::PreferCombinedGuards,
    &prefer_direct_iteration::PreferDirectIteration,
    &control_flow::prefer_early_return::PreferEarlyReturn,
    &control_flow::prefer_else_if::PreferElseIf,
    &errors::prefer_error_is_error::PreferErrorIsError,
    &prefer_flat_math_min_max::PreferFlatMathMinMax,
    &modules::prefer_global_number_constants::PreferGlobalNumberConstants,
    &prefer_group_by::PreferGroupBy,
    &prefer_has_check::PreferHasCheck,
    &control_flow::prefer_hoisting_branch_code::PreferHoistingBranchCode,
    &modules::prefer_identifier_import_export_specifiers::PreferIdentifierImportExportSpecifiers,
    &prefer_includes_over_repeated_comparisons::PreferIncludesOverRepeatedComparisons,
    &prefer_iterable_in_constructor::PreferIterableInConstructor,
    &prefer_iterator_helpers::PreferIteratorHelpers,
    &prefer_iterator_to_array::PreferIteratorToArray,
    &prefer_iterator_to_array_at_end::PreferIteratorToArrayAtEnd,
    &modules::prefer_json_import::PreferJsonImport,
    &prefer_map_from_entries::PreferMapFromEntries,
    &numbers::prefer_math_abs::PreferMathAbs,
    &control_flow::prefer_minimal_ternary::PreferMinimalTernary,
    &numbers::prefer_number_is_safe_integer::PreferNumberIsSafeInteger,
    &prefer_object_iterable_methods::PreferObjectIterableMethods,
    &prefer_set_methods::PreferSetMethods,
    &functions::prefer_short_arrow_method::PreferShortArrowMethod,
    &control_flow::prefer_simple_condition_first::PreferSimpleConditionFirst,
    &prefer_simple_sort_comparator::PreferSimpleSortComparator,
    &control_flow::prefer_simplified_conditions::PreferSimplifiedConditions,
    &prefer_single_array_predicate::PreferSingleArrayPredicate,
    &control_flow::prefer_smaller_scope::PreferSmallerScope,
    &control_flow::prefer_switch::PreferSwitch,
    &numbers::prefer_unary_minus::PreferUnaryMinus,
    &prefer_while_loop_condition::PreferWhileLoopCondition,
    &errors::try_complexity::TryComplexity,
];

/// Their names, on for every TypeScript file.
pub const NAMES: &[&str] = &[
    "unicorn/consistent-conditional-object-spread",
    "unicorn/iteration-fallback-style",
    "unicorn/no-array-from-fill",
    "unicorn/no-array-front-mutation",
    "unicorn/no-array-sort-for-min-max",
    "unicorn/no-break-in-nested-loop",
    "unicorn/no-confusing-array-splice",
    "unicorn/no-declarations-before-early-exit",
    "unicorn/no-duplicate-if-branches",
    "unicorn/no-duplicate-logical-operands",
    "unicorn/no-duplicate-loops",
    "unicorn/no-duplicate-set-values",
    "unicorn/no-error-property-assignment",
    "unicorn/no-global-object-property-assignment",
    "unicorn/no-impossible-length-comparison",
    "unicorn/no-loop-iterable-mutation",
    "unicorn/no-mismatched-map-key",
    "unicorn/no-negated-array-predicate",
    "unicorn/no-negated-comparison",
    "unicorn/no-object-methods-with-collections",
    "unicorn/no-redundant-comparison",
    "unicorn/no-repeated-call-wrappers",
    "unicorn/no-return-array-push",
    "unicorn/no-subtraction-comparison",
    "unicorn/no-top-level-side-effects",
    "unicorn/no-uncalled-method",
    "unicorn/no-unnecessary-array-flat-map",
    "unicorn/no-unnecessary-default-arguments",
    "unicorn/no-unnecessary-global-this",
    "unicorn/no-unnecessary-parameters",
    "unicorn/no-unnecessary-splice",
    "unicorn/no-unreadable-for-of-expression",
    "unicorn/no-unsafe-buffer-conversion",
    "unicorn/no-unsafe-json-serialization",
    "unicorn/no-unused-iterator-helper",
    "unicorn/no-useless-boolean-cast",
    "unicorn/no-useless-else",
    "unicorn/no-useless-logical-operand",
    "unicorn/no-useless-re-export",
    "unicorn/no-useless-recursion",
    "unicorn/no-useless-set-construction",
    "unicorn/prefer-aggregate-error",
    "unicorn/prefer-array-from-map",
    "unicorn/prefer-array-iterable-methods",
    "unicorn/prefer-array-last-methods",
    "unicorn/prefer-array-slice",
    "unicorn/prefer-block-statement-over-iife",
    "unicorn/prefer-boolean-return",
    "unicorn/prefer-combined-guards",
    "unicorn/prefer-direct-iteration",
    "unicorn/prefer-early-return",
    "unicorn/prefer-else-if",
    "unicorn/prefer-error-is-error",
    "unicorn/prefer-flat-math-min-max",
    "unicorn/prefer-global-number-constants",
    "unicorn/prefer-group-by",
    "unicorn/prefer-has-check",
    "unicorn/prefer-hoisting-branch-code",
    "unicorn/prefer-identifier-import-export-specifiers",
    "unicorn/prefer-includes-over-repeated-comparisons",
    "unicorn/prefer-iterable-in-constructor",
    "unicorn/prefer-iterator-helpers",
    "unicorn/prefer-iterator-to-array",
    "unicorn/prefer-iterator-to-array-at-end",
    "unicorn/prefer-json-import",
    "unicorn/prefer-map-from-entries",
    "unicorn/prefer-math-abs",
    "unicorn/prefer-minimal-ternary",
    "unicorn/prefer-number-is-safe-integer",
    "unicorn/prefer-object-iterable-methods",
    "unicorn/prefer-set-methods",
    "unicorn/prefer-short-arrow-method",
    "unicorn/prefer-simple-condition-first",
    "unicorn/prefer-simple-sort-comparator",
    "unicorn/prefer-simplified-conditions",
    "unicorn/prefer-single-array-predicate",
    "unicorn/prefer-smaller-scope",
    "unicorn/prefer-switch",
    "unicorn/prefer-unary-minus",
    "unicorn/prefer-while-loop-condition",
    "unicorn/try-complexity",
];

#[cfg(test)]
mod tests {
    use super::{NAMES, RULES};

    #[test]
    fn names_each_rule_once_in_order() {
        let names: Vec<&str> = RULES.iter().map(|rule| rule.name()).collect();
        let mut sorted = names.clone();
        sorted.sort_unstable();
        sorted.dedup();

        assert_eq!(names, NAMES);
        assert_eq!(sorted, NAMES);
    }
}
