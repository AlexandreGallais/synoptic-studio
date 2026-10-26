//! `@angular-eslint/template/no-autofocus`: no `autofocus` outside a dialog
//! (`@angular-eslint/eslint-plugin-template` 22.6.0, `src/rules/no-autofocus.ts`): on an element
//! of the DOM, its name in lower case, a plain or bound `autofocus` is reported over itself and
//! fixed by its removal with the space before it, unless the element or one above it is a
//! `<dialog>`, any case, where the focus belongs on opening.

use super::dom::is_dom_element;
use super::r3::replace;
use super::{Report, Role, Template, TemplateRule, bound3, is_plain_element};
use crate::template::tree::Node;

/// The rule.
pub struct NoAutofocus;

impl TemplateRule for NoAutofocus {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/no-autofocus"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "noAutofocus",
            "The `autofocus` attribute should not be used, as it reduces usability and accessibility for users",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, ancestors) in template.elements() {
            if !is_plain_element(element) || !is_dom_element(&element.name) {
                continue;
            }
            let in_dialog = element.name.eq_ignore_ascii_case("dialog")
                || ancestors.iter().any(|node| {
                    matches!(node, Node::Element(above) if above.name.eq_ignore_ascii_case("dialog"))
                });
            if in_dialog {
                continue;
            }
            for each in bound3(element) {
                if each.role == Role::Output || each.name != "autofocus" {
                    continue;
                }
                let span = each.attribute.span;
                reports.push(Report {
                    fix: Some(replace(span.start - 1, span.end, "")),
                    ..Report::new(span, "noAutofocus")
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoAutofocus;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac3_passes_the_plugin_s_cases() {
        let tally = run("no-autofocus.txt", &[], &|_| Box::new(NoAutofocus));

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 12 valid and 3 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (12, 3, 0));
    }
}
