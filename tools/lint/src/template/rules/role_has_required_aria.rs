//! `@angular-eslint/template/role-has-required-aria`: an element given a role has the ARIA
//! properties the role requires (`@angular-eslint/eslint-plugin-template` 22.6.0,
//! `src/rules/role-has-required-aria.ts`): on an element of the DOM, its name in lower or upper
//! case, a plain `role` whose role of `aria-query` requires properties the element lacks among its
//! plain attributes and inputs is reported over the `role` attribute, unless the element has that
//! role by its semantics (`<input type="checkbox" role="switch">`). The removal of the role is a
//! suggestion, not carried.

use super::a11y::{attributes_and_inputs, is_semantic_role_element};
use super::aria::role;
use super::dom::is_dom_name;
use super::{Report, Role, Template, TemplateRule, is_plain_element};

/// The rule.
pub struct RoleHasRequiredAria;

impl TemplateRule for RoleHasRequiredAria {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/role-has-required-aria"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            (
                "roleHasRequiredAria",
                "The {{element}} with role=\"{{role}}\" does not have required ARIA properties: {{missingProps}}",
            ),
            ("suggestRemoveRole", "Remove role `{{role}}`"),
        ]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, _) in template.elements() {
            if !is_plain_element(element) || !is_dom_name(&element.name) {
                continue;
            }
            let attributes = attributes_and_inputs(element);
            for each in attributes
                .iter()
                .filter(|each| each.role == Role::Text && each.name == "role")
            {
                let name = &each.attribute.value;
                let required = role(name).map_or(&[][..], |found| found.required_props);
                if required.is_empty() || is_semantic_role_element(&element.name, name, &attributes)
                {
                    continue;
                }
                let missing: Vec<&str> = required
                    .iter()
                    .filter(|prop| !attributes.iter().any(|each| each.name == **prop))
                    .copied()
                    .collect();
                if !missing.is_empty() {
                    reports.push(Report {
                        data: vec![
                            ("element", element.name.clone()),
                            ("role", name.clone()),
                            ("missingProps", missing.join(", ")),
                        ],
                        ..Report::new(each.attribute.span, "roleHasRequiredAria")
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RoleHasRequiredAria;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac3_passes_the_plugin_s_cases() {
        let tally = run("role-has-required-aria.txt", &[], &|_| {
            Box::new(RoleHasRequiredAria)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 7 valid and 2 invalid cases, all run; 2 suggestions, not carried.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (7, 2, 2));
    }
}
