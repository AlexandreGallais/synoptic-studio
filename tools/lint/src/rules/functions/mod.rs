//! Unicorn's rules on functions that oxlint lacks (EN-079), each one module with the cases of
//! its documentation in `eslint-plugin-unicorn` 78.0.0 as tests, read from the tree alone, as
//! `ESLint` ran them here without type information. No scope turns one off: they run on every
//! TypeScript file. Two are not carried: `consistent-function-style`, whose every role defaults
//! to `ignore`, so that with no option, as configured here, it reports nothing (`func-style`,
//! `prefer-arrow-callback` and `object-shorthand` in oxlint hold the project's choices); and
//! `isolated-functions`, which checks only functions given to `makeSynchronous`, `workerize`,
//! `page.evaluate`, `browser.execute` or `executeScript`, or tagged `@isolated`, none of which
//! this code holds or can import (`dependencies` stays empty).

pub mod calls;
pub mod no_repeated_call_wrappers;
pub mod no_uncalled_method;
pub mod no_unnecessary_default_arguments;
pub mod no_unnecessary_parameters;
pub mod no_useless_recursion;
pub mod prefer_block_statement_over_iife;
pub mod prefer_short_arrow_method;
pub mod primitives;

use super::Rule;

/// The rules, in the order of their names.
pub const RULES: &[&dyn Rule] = &[
    &no_repeated_call_wrappers::NoRepeatedCallWrappers,
    &no_uncalled_method::NoUncalledMethod,
    &no_unnecessary_default_arguments::NoUnnecessaryDefaultArguments,
    &no_unnecessary_parameters::NoUnnecessaryParameters,
    &no_useless_recursion::NoUselessRecursion,
    &prefer_block_statement_over_iife::PreferBlockStatementOverIife,
    &prefer_short_arrow_method::PreferShortArrowMethod,
];
