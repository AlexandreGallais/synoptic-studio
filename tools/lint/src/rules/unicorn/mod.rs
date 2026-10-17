//! Unicorn's rules on arrays, collections and iteration that oxlint lacks (EN-078), each one
//! module with the cases of its documentation in `eslint-plugin-unicorn` 78.0.0 as tests, read
//! from the syntax alone. No scope turns one off: they run on every TypeScript file.

pub mod no_array_from_fill;
pub mod no_array_front_mutation;
pub mod no_confusing_array_splice;
pub mod no_impossible_length_comparison;
pub mod no_negated_array_predicate;
pub mod no_return_array_push;
pub mod syntax;

use super::Rule;

/// The rules, in the order of their names.
pub const RULES: &[&dyn Rule] = &[
    &no_array_from_fill::NoArrayFromFill,
    &no_array_front_mutation::NoArrayFrontMutation,
    &no_confusing_array_splice::NoConfusingArraySplice,
    &no_impossible_length_comparison::NoImpossibleLengthComparison,
    &no_negated_array_predicate::NoNegatedArrayPredicate,
    &no_return_array_push::NoReturnArrayPush,
];

/// Their names, on for every TypeScript file.
pub const NAMES: &[&str] = &[
    "unicorn/no-array-from-fill",
    "unicorn/no-array-front-mutation",
    "unicorn/no-confusing-array-splice",
    "unicorn/no-impossible-length-comparison",
    "unicorn/no-negated-array-predicate",
    "unicorn/no-return-array-push",
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
