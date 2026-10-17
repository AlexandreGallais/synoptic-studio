//! Unicorn's rules on the DOM that oxlint lacks (EN-082), each one module with the cases of its
//! documentation in `eslint-plugin-unicorn` 78.0.0 as tests, read from the tree alone, as
//! `ESLint` ran them here without type information. They run only where a DOM exists
//! (`scopes::DOM_FILES`): the library's core never touches it (ADR-0005, ADR-0025).

pub mod better_dom_traversing;
pub mod css_properties;
pub mod events;
pub mod no_incorrect_query_selector;
pub mod no_invalid_dom_token;
pub mod no_invalid_style_set_property;
pub mod no_late_current_target_access;
pub mod no_late_event_control;
pub mod no_prevent_default_in_passive_listener;
pub mod no_selector_as_dom_name;
pub mod nodes;

use crate::rules::Rule;

/// The rules, in the order of their names.
pub const RULES: &[&dyn Rule] = &[
    &better_dom_traversing::BetterDomTraversing,
    &no_incorrect_query_selector::NoIncorrectQuerySelector,
    &no_invalid_dom_token::NoInvalidDomToken,
    &no_invalid_style_set_property::NoInvalidStyleSetProperty,
    &no_late_current_target_access::NoLateCurrentTargetAccess,
    &no_late_event_control::NoLateEventControl,
    &no_prevent_default_in_passive_listener::NoPreventDefaultInPassiveListener,
    &no_selector_as_dom_name::NoSelectorAsDomName,
];

/// Their names, on for the files where a DOM exists.
pub const NAMES: &[&str] = &[
    "unicorn/better-dom-traversing",
    "unicorn/no-incorrect-query-selector",
    "unicorn/no-invalid-dom-token",
    "unicorn/no-invalid-style-set-property",
    "unicorn/no-late-current-target-access",
    "unicorn/no-late-event-control",
    "unicorn/no-prevent-default-in-passive-listener",
    "unicorn/no-selector-as-dom-name",
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
