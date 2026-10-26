//! `@angular-eslint/template/click-events-have-key-events`: a `(click)` comes with a key event
//! (`@angular-eslint/eslint-plugin-template` 22.6.0, `src/rules/click-events-have-key-events.ts`):
//! an element of the DOM, its name in lower case, with a `(click)` and no `(keyup)`, `(keydown)`
//! nor `(keypress)`, with or without a key, is reported over itself, unless it carries one of
//! `ignoreWithDirectives`, has the role `none`, `presentation` or `button`, is hidden from a
//! screen reader or is interactive by its semantics. With `requireKeyCode`, the key event must
//! name a key (`(keydown.enter)`); with `allowedKeyCodes`, one of those keys, any case. The
//! project runs the plugin's defaults.

use super::a11y::{
    attribute_value, is_hidden_from_screen_reader, is_inherently_interactive_element,
    is_presentation_role,
};
use super::dom::is_dom_element;
use super::{
    Report, Role, Template, TemplateRule, bound3, has_ignored_directive, is_plain_element,
};

/// The rule and its options.
pub struct ClickEventsHaveKeyEvents {
    /// The directives that leave an element alone.
    pub ignore_with_directives: &'static [&'static str],
    /// Whether the key event must name a key.
    pub require_key_code: bool,
    /// The keys a key event must name, when any.
    pub allowed_key_codes: &'static [&'static str],
}

impl ClickEventsHaveKeyEvents {
    /// The plugin's defaults, which the project runs.
    pub const PROJECT: Self = Self {
        ignore_with_directives: &[],
        require_key_code: false,
        allowed_key_codes: &[],
    };
}

/// The key events (`KEY_EVENTS`).
const KEY_EVENTS: [&str; 3] = ["keyup", "keydown", "keypress"];

/// Whether an output is a key event, with or without a key (`isKeyEvent`).
fn is_key_event(name: &str) -> bool {
    KEY_EVENTS.iter().any(|event| {
        name == *event
            || name
                .strip_prefix(event)
                .is_some_and(|rest| rest.starts_with('.'))
    })
}

/// The key a key event names, its last segment in lower case, when it has one (`getKeyCode`).
fn key_code(name: &str) -> Option<String> {
    let (_, last) = name.rsplit_once('.')?;
    (!last.is_empty()).then(|| last.to_lowercase())
}

impl TemplateRule for ClickEventsHaveKeyEvents {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/click-events-have-key-events"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            (
                "clickEventsHaveKeyEvents",
                "click must be accompanied by either keyup, keydown or keypress event for accessibility.",
            ),
            (
                "clickEventsHaveKeyCode",
                "click must be accompanied by a keyup, keydown or keypress event that specifies a key (e.g. `(keydown.enter)`) for accessibility.",
            ),
            (
                "clickEventsHaveAllowedKeyCode",
                "click must be accompanied by a keyup, keydown or keypress event for one of the allowed keys ({{allowedKeyCodes}}) for accessibility.",
            ),
        ]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let allocator = template.allocator;
        let allowed: Vec<String> = self
            .allowed_key_codes
            .iter()
            .map(|each| each.to_lowercase())
            .collect();
        for (element, ancestors) in template.elements() {
            if !is_plain_element(element)
                || !is_dom_element(&element.name.to_lowercase())
                || has_ignored_directive(element, self.ignore_with_directives)
                || is_presentation_role(allocator, element)
                || is_hidden_from_screen_reader(allocator, element, &ancestors)
                || is_inherently_interactive_element(allocator, element)
                || attribute_value(allocator, element, "role").text() == Some("button")
            {
                continue;
            }
            let (mut has_click, mut has_key_event, mut satisfied) = (false, false, false);
            for output in bound3(element)
                .iter()
                .filter(|each| each.role == Role::Output)
            {
                has_click = has_click || output.name == "click";
                if !is_key_event(&output.name) {
                    continue;
                }
                has_key_event = true;
                let key = key_code(&output.name);
                satisfied = satisfied
                    || if !allowed.is_empty() {
                        key.is_some_and(|key| allowed.contains(&key))
                    } else if self.require_key_code {
                        key.is_some()
                    } else {
                        true
                    };
            }
            if !has_click || satisfied {
                continue;
            }
            let report = if !has_key_event {
                Report::new(element.span, "clickEventsHaveKeyEvents")
            } else if !allowed.is_empty() {
                Report {
                    data: vec![("allowedKeyCodes", allowed.join(", "))],
                    ..Report::new(element.span, "clickEventsHaveAllowedKeyCode")
                }
            } else {
                Report::new(element.span, "clickEventsHaveKeyCode")
            };
            reports.push(report);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ClickEventsHaveKeyEvents, key_code};
    use crate::template::rules::tester::run;

    /// Strings kept for the test's life.
    fn leaked(texts: Vec<String>) -> &'static [&'static str] {
        Vec::leak(texts.into_iter().map(|each| &*String::leak(each)).collect())
    }

    #[test]
    fn f52_ac3_passes_the_plugin_s_cases() {
        let tally = run("click-events-have-key-events.txt", &[], &|options| {
            let texts =
                |key: &str| leaked(options.and_then(|each| each.texts(key)).unwrap_or_default());
            Box::new(ClickEventsHaveKeyEvents {
                ignore_with_directives: texts("ignoreWithDirectives"),
                require_key_code: options
                    .and_then(|each| each.flag("requireKeyCode"))
                    .unwrap_or(false),
                allowed_key_codes: texts("allowedKeyCodes"),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 11 valid and 18 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (11, 18, 0));
    }

    #[test]
    fn reads_the_key_of_a_key_event() {
        // Our own cases, after `getKeyCode`.
        assert_eq!(key_code("keydown.shift.Enter").as_deref(), Some("enter"));
        assert_eq!(key_code("keydown"), None);
        assert_eq!(key_code("keydown."), None);
    }
}
