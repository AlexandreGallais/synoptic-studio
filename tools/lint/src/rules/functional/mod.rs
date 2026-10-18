//! `eslint-plugin-functional`'s rules that `ESLint` ran without types (EN-084), each one module
//! with the cases of its documentation in the plugin's 10.0.1 tag as tests, read from the tree
//! alone. On for every TypeScript file; the statement rules (`functional-parameters`,
//! `no-expression-statements`, `no-loop-statements`, `no-throw-statements`, `no-try-statements`)
//! are turned off by the scopes of the imperative shell, the tools and the tests
//! (`scopes::SCOPES`). Not carried: `prefer-property-signatures`, which oxlint's
//! `typescript/method-signature-style` (`property`) already covers; and the `arguments` part of
//! `functional-parameters`, which oxlint's `prefer-rest-params` covers on every file.

pub mod functional_parameters;
pub mod no_classes;
pub mod no_expression_statements;
pub mod no_let;
pub mod no_loop_statements;
pub mod no_promise_reject;
pub mod no_this_expressions;
pub mod no_throw_statements;
pub mod no_try_statements;
pub mod readonly_type;

use super::Rule;

/// The rules, in the order of their names.
pub const RULES: &[&dyn Rule] = &[
    &functional_parameters::FunctionalParameters,
    &no_classes::NoClasses,
    &no_expression_statements::NoExpressionStatements,
    &no_let::NoLet,
    &no_loop_statements::NoLoopStatements,
    &no_promise_reject::NoPromiseReject,
    &no_this_expressions::NoThisExpressions,
    &no_throw_statements::NoThrowStatements,
    &no_try_statements::NoTryStatements,
    &readonly_type::ReadonlyType,
];

/// Their names, on for every TypeScript file before the scopes.
pub const NAMES: &[&str] = &[
    "functional/functional-parameters",
    "functional/no-classes",
    "functional/no-expression-statements",
    "functional/no-let",
    "functional/no-loop-statements",
    "functional/no-promise-reject",
    "functional/no-this-expressions",
    "functional/no-throw-statements",
    "functional/no-try-statements",
    "functional/readonly-type",
];

#[cfg(test)]
mod tests {
    use super::{NAMES, RULES};

    #[test]
    fn lists_each_rule_once_under_its_name() {
        let names: Vec<&str> = RULES.iter().map(|rule| rule.name()).collect();

        assert_eq!(names, NAMES);
        assert!(NAMES.windows(2).all(|pair| pair[0] < pair[1]));
    }
}
