//! `@angular-eslint/template/no-positive-tabindex`: no positive `tabindex`
//! (`@angular-eslint/eslint-plugin-template` 22.6.0, `src/rules/no-positive-tabindex.ts`): a
//! `tabindex` bound to a literal on an element of the DOM, its name in lower or upper case, or a
//! plain `tabindex` on any element, is reported over its value when JavaScript reads that value
//! above 0 (`[value>0]` of the plugin's selector: `true` is 1, a text its number). The
//! replacements by `-1` and `0` are suggestions, not carried.

use super::dom::is_dom_name;
use super::r3::{Literal, Ng, of_bound};
use super::{Report, Role, Template, TemplateRule, bound3, is_plain_element};
use crate::javascript::to_number;
use crate::template::bindings::{Value, read_attribute};

/// The rule.
pub struct NoPositiveTabindex;

/// Whether a bound literal is above 0 for JavaScript; a binding of no literal has no `value`.
fn is_positive(value: &Ng) -> bool {
    match value {
        Ng::Literal(Literal::Number(number)) => *number > 0.0,
        Ng::Literal(Literal::Text(text)) => to_number(text) > 0.0,
        Ng::Literal(Literal::Bool(value)) => *value,
        _ => false,
    }
}

impl TemplateRule for NoPositiveTabindex {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/no-positive-tabindex"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            (
                "noPositiveTabindex",
                "The `tabindex` attribute should not be positive",
            ),
            (
                "suggestNonNegativeTabindex",
                "Use `tabindex=\"{{tabindex}}\"`",
            ),
        ]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, _) in template.elements() {
            if !is_plain_element(element) {
                continue;
            }
            for each in bound3(element) {
                if each.name != "tabindex" {
                    continue;
                }
                let positive = match each.role {
                    Role::Text => to_number(&each.attribute.value) > 0.0,
                    Role::Input if is_dom_name(&element.name) => {
                        match read_attribute(template.allocator, &element.name, each.attribute)
                            .map(|read| read.value)
                        {
                            Ok(Value::Binding(bound)) => is_positive(&of_bound(&bound)),
                            _ => false,
                        }
                    }
                    _ => false,
                };
                if let (true, Some(value)) = (positive, each.attribute.value_span) {
                    reports.push(Report::new(value, "noPositiveTabindex"));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoPositiveTabindex;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac3_passes_the_plugin_s_cases() {
        let tally = run("no-positive-tabindex.txt", &[], &|_| {
            Box::new(NoPositiveTabindex)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 11 valid and 3 invalid cases, all run; 6 suggestions, not carried.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (11, 3, 6));
    }

    #[test]
    fn reports_a_positive_text_bound_as_a_literal() {
        let places = |text: &str| {
            crate::template::lint_with("a.component.html", text, &[&NoPositiveTabindex])
                .iter()
                .map(|finding| (finding.line, finding.column))
                .collect::<Vec<_>>()
        };

        // Our own cases: the text `"1"` reads 1, above 0, reported at its value after
        // `<div [tabindex]="`, 17 bytes; `"0"` and `"-1"` are not above 0.
        assert_eq!(places("<div [tabindex]=\"'1'\"></div>"), [(1, 18)]);
        assert_eq!(places("<div [tabindex]=\"'0'\"></div>"), []);
        assert_eq!(places("<div [tabindex]=\"'-1'\"></div>"), []);
    }
}
