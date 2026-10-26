//! The rules of `@angular-eslint/eslint-plugin` 22.6.0 that catch the framework's bugs (EN-161,
//! F51), each one module with the cases of the plugin's own tests, read from the tree alone
//! (REF-ANGULAR-ESLINT-SOURCE); on for the playground's Angular app only (`scopes::SCOPES`, its
//! `angular-app` scope).

pub mod classes;
pub mod computed_must_return;
pub mod contextual_lifecycle;
pub mod no_async_lifecycle_method;
pub mod no_duplicates_in_metadata_arrays;
pub mod no_implicit_take_until_destroyed;
pub mod no_lifecycle_call;
pub mod no_pipe_impure;
pub mod no_uncalled_signals;
pub mod reactive_context_must_read_signal;
pub mod require_lifecycle_on_prototype;
pub mod signals;

use super::Rule;

/// The rules, in the order of their names.
pub const RULES: &[&dyn Rule] = &[
    &computed_must_return::ComputedMustReturn,
    &contextual_lifecycle::ContextualLifecycle,
    &no_async_lifecycle_method::NoAsyncLifecycleMethod,
    &no_duplicates_in_metadata_arrays::NoDuplicatesInMetadataArrays,
    &no_implicit_take_until_destroyed::NoImplicitTakeUntilDestroyed,
    &no_lifecycle_call::NoLifecycleCall,
    &no_pipe_impure::NoPipeImpure,
    &no_uncalled_signals::NoUncalledSignals,
    &reactive_context_must_read_signal::ReactiveContextMustReadSignal,
    &require_lifecycle_on_prototype::RequireLifecycleOnPrototype,
];

/// Their names, which the Angular app's scope turns on.
pub const NAMES: &[&str] = &[
    "@angular-eslint/computed-must-return",
    "@angular-eslint/contextual-lifecycle",
    "@angular-eslint/no-async-lifecycle-method",
    "@angular-eslint/no-duplicates-in-metadata-arrays",
    "@angular-eslint/no-implicit-take-until-destroyed",
    "@angular-eslint/no-lifecycle-call",
    "@angular-eslint/no-pipe-impure",
    "@angular-eslint/no-uncalled-signals",
    "@angular-eslint/reactive-context-must-read-signal",
    "@angular-eslint/require-lifecycle-on-prototype",
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
