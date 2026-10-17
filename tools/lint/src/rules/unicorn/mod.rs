//! Unicorn's rules on arrays, collections and iteration that oxlint lacks (EN-078), each one
//! module with the cases of its documentation in `eslint-plugin-unicorn` 78.0.0 as tests, read
//! from the syntax alone. No scope turns one off: they run on every TypeScript file.

pub mod consistent_conditional_object_spread;
pub mod no_array_from_fill;
pub mod no_array_front_mutation;
pub mod no_confusing_array_splice;
pub mod no_duplicate_set_values;
pub mod no_impossible_length_comparison;
pub mod no_mismatched_map_key;
pub mod no_negated_array_predicate;
pub mod no_object_methods_with_collections;
pub mod no_return_array_push;
pub mod no_unnecessary_array_flat_map;
pub mod no_unnecessary_splice;
pub mod no_useless_set_construction;
pub mod prefer_array_from_map;
pub mod prefer_array_last_methods;
pub mod prefer_array_slice;
pub mod prefer_flat_math_min_max;
pub mod prefer_simple_sort_comparator;
pub mod prefer_single_array_predicate;
pub mod syntax;

use super::Rule;

/// The rules, in the order of their names.
pub const RULES: &[&dyn Rule] = &[
    &consistent_conditional_object_spread::ConsistentConditionalObjectSpread,
    &no_array_from_fill::NoArrayFromFill,
    &no_array_front_mutation::NoArrayFrontMutation,
    &no_confusing_array_splice::NoConfusingArraySplice,
    &no_duplicate_set_values::NoDuplicateSetValues,
    &no_impossible_length_comparison::NoImpossibleLengthComparison,
    &no_mismatched_map_key::NoMismatchedMapKey,
    &no_negated_array_predicate::NoNegatedArrayPredicate,
    &no_object_methods_with_collections::NoObjectMethodsWithCollections,
    &no_return_array_push::NoReturnArrayPush,
    &no_unnecessary_array_flat_map::NoUnnecessaryArrayFlatMap,
    &no_unnecessary_splice::NoUnnecessarySplice,
    &no_useless_set_construction::NoUselessSetConstruction,
    &prefer_array_from_map::PreferArrayFromMap,
    &prefer_array_last_methods::PreferArrayLastMethods,
    &prefer_array_slice::PreferArraySlice,
    &prefer_flat_math_min_max::PreferFlatMathMinMax,
    &prefer_simple_sort_comparator::PreferSimpleSortComparator,
    &prefer_single_array_predicate::PreferSingleArrayPredicate,
];

/// Their names, on for every TypeScript file.
pub const NAMES: &[&str] = &[
    "unicorn/consistent-conditional-object-spread",
    "unicorn/no-array-from-fill",
    "unicorn/no-array-front-mutation",
    "unicorn/no-confusing-array-splice",
    "unicorn/no-duplicate-set-values",
    "unicorn/no-impossible-length-comparison",
    "unicorn/no-mismatched-map-key",
    "unicorn/no-negated-array-predicate",
    "unicorn/no-object-methods-with-collections",
    "unicorn/no-return-array-push",
    "unicorn/no-unnecessary-array-flat-map",
    "unicorn/no-unnecessary-splice",
    "unicorn/no-useless-set-construction",
    "unicorn/prefer-array-from-map",
    "unicorn/prefer-array-last-methods",
    "unicorn/prefer-array-slice",
    "unicorn/prefer-flat-math-min-max",
    "unicorn/prefer-simple-sort-comparator",
    "unicorn/prefer-single-array-predicate",
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
