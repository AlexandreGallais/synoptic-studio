//! The rules of `@angular-eslint/eslint-plugin` 22.6.0 that catch the framework's bugs (EN-161)
//! and keep each construct in its current form and its names (EN-162), F51: each one module
//! with the cases of the plugin's own tests, read from the tree alone (REF-ANGULAR-ESLINT-SOURCE);
//! on for the playground's Angular app only (`scopes::SCOPES`, its `angular-app` scope).

pub mod bindings;
pub mod classes;
pub mod component_max_inline_declarations;
pub mod component_selector;
pub mod computed_must_return;
pub mod consistent_component_styles;
pub mod contextual_decorator;
pub mod contextual_lifecycle;
pub mod declarations;
pub mod directive_selector;
pub mod inject_at_top;
pub mod metadata;
pub mod no_async_lifecycle_method;
pub mod no_attribute_decorator;
pub mod no_developer_preview;
pub mod no_duplicates_in_metadata_arrays;
pub mod no_empty_lifecycle_method;
pub mod no_experimental;
pub mod no_forward_ref;
pub mod no_implicit_take_until_destroyed;
pub mod no_input_prefix;
pub mod no_input_rename;
pub mod no_inputs_metadata_property;
pub mod no_lifecycle_call;
pub mod no_output_native;
pub mod no_output_on_prefix;
pub mod no_output_rename;
pub mod no_pipe_impure;
pub mod no_queries_metadata_property;
pub mod no_uncalled_signals;
pub mod pipe_prefix;
pub mod prefer_host_metadata_property;
pub mod prefer_inject;
pub mod prefer_on_push_component_change_detection;
pub mod prefer_output_emitter_ref;
pub mod prefer_output_readonly;
pub mod prefer_signal_model;
pub mod prefer_signals;
pub mod prefer_standalone;
pub mod reactive_context_must_read_signal;
pub mod relative_url_prefix;
pub mod require_lifecycle_on_prototype;
pub mod selector_names;
pub mod signals;
pub mod sort_keys_in_type_decorator;
pub mod sort_lifecycle_methods;
pub mod tagged_use;
pub mod use_component_selector;
pub mod use_component_view_encapsulation;
pub mod use_injectable_provided_in;
pub mod use_lifecycle_interface;
pub mod use_pipe_transform_interface;

use super::Rule;

/// The rules, in the order of their names.
pub const RULES: &[&dyn Rule] = &[
    &component_max_inline_declarations::ComponentMaxInlineDeclarations,
    &component_selector::ComponentSelector,
    &computed_must_return::ComputedMustReturn,
    &consistent_component_styles::ConsistentComponentStyles,
    &contextual_decorator::ContextualDecorator,
    &contextual_lifecycle::ContextualLifecycle,
    &directive_selector::DirectiveSelector,
    &inject_at_top::InjectAtTop,
    &no_async_lifecycle_method::NoAsyncLifecycleMethod,
    &no_attribute_decorator::NoAttributeDecorator,
    &no_developer_preview::NoDeveloperPreview,
    &no_duplicates_in_metadata_arrays::NoDuplicatesInMetadataArrays,
    &no_empty_lifecycle_method::NoEmptyLifecycleMethod,
    &no_experimental::NoExperimental,
    &no_forward_ref::NoForwardRef,
    &no_implicit_take_until_destroyed::NoImplicitTakeUntilDestroyed,
    &no_input_prefix::NoInputPrefix,
    &no_input_rename::NoInputRename,
    &no_inputs_metadata_property::NoInputsMetadataProperty,
    &no_lifecycle_call::NoLifecycleCall,
    &no_output_native::NoOutputNative,
    &no_output_on_prefix::NoOutputOnPrefix,
    &no_output_rename::NoOutputRename,
    &no_inputs_metadata_property::NoOutputsMetadataProperty,
    &no_pipe_impure::NoPipeImpure,
    &no_queries_metadata_property::NoQueriesMetadataProperty,
    &no_uncalled_signals::NoUncalledSignals,
    &pipe_prefix::PipePrefix,
    &prefer_host_metadata_property::PreferHostMetadataProperty,
    &prefer_inject::PreferInject,
    &prefer_on_push_component_change_detection::PreferOnPushComponentChangeDetection,
    &prefer_output_emitter_ref::PreferOutputEmitterRef,
    &prefer_output_readonly::PreferOutputReadonly,
    &prefer_signal_model::PreferSignalModel,
    &prefer_signals::PreferSignals,
    &prefer_standalone::PreferStandalone,
    &reactive_context_must_read_signal::ReactiveContextMustReadSignal,
    &relative_url_prefix::RelativeUrlPrefix,
    &require_lifecycle_on_prototype::RequireLifecycleOnPrototype,
    &sort_keys_in_type_decorator::SortKeysInTypeDecorator,
    &sort_lifecycle_methods::SortLifecycleMethods,
    &use_component_selector::UseComponentSelector,
    &use_component_view_encapsulation::UseComponentViewEncapsulation,
    &use_injectable_provided_in::UseInjectableProvidedIn,
    &use_lifecycle_interface::UseLifecycleInterface,
    &use_pipe_transform_interface::UsePipeTransformInterface,
];

/// Their names, which the Angular app's scope turns on.
pub const NAMES: &[&str] = &[
    "@angular-eslint/component-max-inline-declarations",
    "@angular-eslint/component-selector",
    "@angular-eslint/computed-must-return",
    "@angular-eslint/consistent-component-styles",
    "@angular-eslint/contextual-decorator",
    "@angular-eslint/contextual-lifecycle",
    "@angular-eslint/directive-selector",
    "@angular-eslint/inject-at-top",
    "@angular-eslint/no-async-lifecycle-method",
    "@angular-eslint/no-attribute-decorator",
    "@angular-eslint/no-developer-preview",
    "@angular-eslint/no-duplicates-in-metadata-arrays",
    "@angular-eslint/no-empty-lifecycle-method",
    "@angular-eslint/no-experimental",
    "@angular-eslint/no-forward-ref",
    "@angular-eslint/no-implicit-take-until-destroyed",
    "@angular-eslint/no-input-prefix",
    "@angular-eslint/no-input-rename",
    "@angular-eslint/no-inputs-metadata-property",
    "@angular-eslint/no-lifecycle-call",
    "@angular-eslint/no-output-native",
    "@angular-eslint/no-output-on-prefix",
    "@angular-eslint/no-output-rename",
    "@angular-eslint/no-outputs-metadata-property",
    "@angular-eslint/no-pipe-impure",
    "@angular-eslint/no-queries-metadata-property",
    "@angular-eslint/no-uncalled-signals",
    "@angular-eslint/pipe-prefix",
    "@angular-eslint/prefer-host-metadata-property",
    "@angular-eslint/prefer-inject",
    "@angular-eslint/prefer-on-push-component-change-detection",
    "@angular-eslint/prefer-output-emitter-ref",
    "@angular-eslint/prefer-output-readonly",
    "@angular-eslint/prefer-signal-model",
    "@angular-eslint/prefer-signals",
    "@angular-eslint/prefer-standalone",
    "@angular-eslint/reactive-context-must-read-signal",
    "@angular-eslint/relative-url-prefix",
    "@angular-eslint/require-lifecycle-on-prototype",
    "@angular-eslint/sort-keys-in-type-decorator",
    "@angular-eslint/sort-lifecycle-methods",
    "@angular-eslint/use-component-selector",
    "@angular-eslint/use-component-view-encapsulation",
    "@angular-eslint/use-injectable-provided-in",
    "@angular-eslint/use-lifecycle-interface",
    "@angular-eslint/use-pipe-transform-interface",
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
