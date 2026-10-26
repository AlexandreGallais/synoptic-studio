//! `@angular-eslint/template/table-scope`: `scope` only on a `<th>`
//! (`@angular-eslint/eslint-plugin-template` 22.6.0, `src/rules/table-scope.ts`): on any other
//! element of the DOM, its name in lower or upper case, a plain or bound `scope` is reported over
//! itself and fixed by its removal with the space before it.

use super::dom::is_dom_name;
use super::r3::replace;
use super::{Report, Role, Template, TemplateRule, bound3, is_plain_element};

/// The rule.
pub struct TableScope;

impl TemplateRule for TableScope {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/table-scope"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "tableScope",
            "The `scope` attribute should only be on the `<th>` element",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, _) in template.elements() {
            if !is_plain_element(element)
                || !is_dom_name(&element.name)
                || element.name == "th"
                || element.name == "TH"
            {
                continue;
            }
            for each in bound3(element) {
                if each.role == Role::Output || each.name != "scope" {
                    continue;
                }
                let span = each.attribute.span;
                reports.push(Report {
                    fix: Some(replace(span.start - 1, span.end, "")),
                    ..Report::new(span, "tableScope")
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TableScope;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac3_passes_the_plugin_s_cases() {
        let tally = run("table-scope.txt", &[], &|_| Box::new(TableScope));

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 8 valid and 3 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (8, 3, 0));
    }

    #[test]
    fn leaves_a_th_written_in_upper_case() {
        let places = |text: &str| {
            crate::template::lint_with("a.component.html", text, &[&TableScope])
                .iter()
                .map(|finding| (finding.line, finding.column))
                .collect::<Vec<_>>()
        };

        // Our own cases: `<th>` and `<TH>` hold their `scope`; a `<td>` does not, reported at
        // its `scope` after `<td `.
        assert_eq!(
            places("<table><tr><TH scope=\"col\">a</TH></tr></table>"),
            []
        );
        assert_eq!(places("<td scope=\"col\"></td>"), [(1, 5)]);
    }
}
