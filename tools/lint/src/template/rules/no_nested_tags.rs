//! `@angular-eslint/template/no-nested-tags`: no `<p>` inside a `<p>`, nor `<a>` inside an
//! `<a>`, which browsers split apart and hydration then finds changed
//! (`@angular-eslint/eslint-plugin-template` 22.6.0, `src/rules/no-nested-tags.ts`); an explicit
//! `<ng-template>` between them, not rendered in place, stops the search. Reported over the
//! inner element.

use super::{Report, Template, TemplateRule, is_plain_element};
use crate::template::tags::split_ns_name;
use crate::template::tree::{Element, Node};

/// The rule.
pub struct NoNestedTags;

/// Whether an element is an explicit `<ng-template>` (`isExplicitNgTemplate`,
/// `/^(:svg:)?ng-template$/i`).
fn is_explicit_template(element: &Element) -> bool {
    let name = element.name.to_ascii_lowercase();
    name == "ng-template" || name == ":svg:ng-template"
}

/// Whether an ancestor of an element is an element of its name, case aside, before any
/// explicit `<ng-template>` (`hasAncestorOfSameType`).
fn nested(element: &Element, ancestors: &[&Node]) -> bool {
    for ancestor in ancestors.iter().rev() {
        let Node::Element(parent) = ancestor else {
            continue;
        };
        if is_explicit_template(parent) {
            return false;
        }
        if is_plain_element(parent) && parent.name.eq_ignore_ascii_case(&element.name) {
            return true;
        }
    }
    false
}

impl TemplateRule for NoNestedTags {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/no-nested-tags"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "noNestedTags",
            "<{{tag}}> elements must not be nested! This breaks angular incremental hydration as all browsers will convert \"<{{tag}}>1<{{tag}}>2</{{tag}}>3</{{tag}}>\" into \"<{{tag}}>1</{{tag}}><{{tag}}>2</{{tag}}>3\", creating a DOM mismatch between SSR and Angular",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, ancestors) in template.elements() {
            let name = split_ns_name(&element.name).1;
            let watched = name.eq_ignore_ascii_case("p") || name.eq_ignore_ascii_case("a");
            if watched && element.name.len() == name.len() && nested(element, &ancestors) {
                reports.push(Report {
                    data: vec![("tag", element.name.clone())],
                    ..Report::new(element.span, "noNestedTags")
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoNestedTags;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("no-nested-tags.txt", &[], &|_| Box::new(NoNestedTags));

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 7 valid and 6 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (7, 6, 0));
    }
}
