//! `@angular-eslint/template/interactive-supports-focus`: an element with a `(click)` or a key
//! event can take the focus (`@angular-eslint/eslint-plugin-template` 22.6.0,
//! `src/rules/interactive-supports-focus.ts`): an element of the DOM, its name in lower case,
//! outside `allowList`, with a `(click)` or an output starting with `keyup`, `keydown` or
//! `keypress`, is reported over itself unless disabled, hidden from a screen reader, of the role
//! `none` or `presentation`, given a `tabindex`, interactive by its semantics, of a
//! non-interactive role, or content editable. The project runs the plugin's default, `<form>`
//! allowed for its events bubbling up from its controls.

use super::a11y::{
    attributes_and_inputs, is_content_editable, is_disabled_element, is_hidden_from_screen_reader,
    is_inherently_interactive_element, is_non_interactive_role, is_presentation_role,
};
use super::dom::is_dom_element;
use super::{Report, Role, Template, TemplateRule, bound3, is_plain_element};

/// The rule and its option.
pub struct InteractiveSupportsFocus {
    /// The elements left alone, by their names in lower case.
    pub allow_list: &'static [&'static str],
}

impl InteractiveSupportsFocus {
    /// The plugin's default, which the project runs.
    pub const PROJECT: Self = Self {
        allow_list: &["form"],
    };
}

/// Whether an output handles an interaction.
fn is_interactive_output(name: &str) -> bool {
    name == "click"
        || name.starts_with("keyup")
        || name.starts_with("keydown")
        || name.starts_with("keypress")
}

impl TemplateRule for InteractiveSupportsFocus {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/interactive-supports-focus"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "interactiveSupportsFocus",
            "Elements with interaction handlers must be focusable.",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let allocator = template.allocator;
        for (element, ancestors) in template.elements() {
            let name = element.name.to_lowercase();
            if !is_plain_element(element)
                || !is_dom_element(&name)
                || self.allow_list.contains(&name.as_str())
            {
                continue;
            }
            let interactive = bound3(element)
                .iter()
                .any(|each| each.role == Role::Output && is_interactive_output(&each.name));
            if !interactive
                || is_disabled_element(allocator, element)
                || is_hidden_from_screen_reader(allocator, element, &ancestors)
                || is_presentation_role(allocator, element)
            {
                continue;
            }
            let tab_index = attributes_and_inputs(element)
                .iter()
                .any(|each| each.name == "tabindex");
            if !tab_index
                && !is_inherently_interactive_element(allocator, element)
                && !is_non_interactive_role(allocator, element)
                && !is_content_editable(allocator, element)
            {
                reports.push(Report::new(element.span, "interactiveSupportsFocus"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InteractiveSupportsFocus;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac3_passes_the_plugin_s_cases() {
        let tally = run("interactive-supports-focus.txt", &[], &|options| {
            let list = options.and_then(|each| each.texts("allowList"));
            Box::new(list.map_or(InteractiveSupportsFocus::PROJECT, |list| {
                InteractiveSupportsFocus {
                    allow_list: Vec::leak(
                        list.into_iter().map(|each| &*String::leak(each)).collect(),
                    ),
                }
            }))
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 20 valid and 14 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (20, 14, 0));
    }
}
