//! `@angular-eslint/template/prefer-static-string-properties`: a property bound to a string
//! literal is written as a plain attribute (`@angular-eslint/eslint-plugin-template` 22.6.0,
//! `src/rules/prefer-static-string-properties.ts`): `[name]="'value'"` becomes
//! `name="value"`, its quote kept clear of the value's and `&` escaped; animations, `attr.`,
//! `class.` and `style.` bindings, the DOM's property-only APIs (`innerHTML`, `innerText`,
//! `outerHTML`, `textContent`) and the names of `ignore` are left. Reported over the binding.
//! The project runs the plugin's default, no name ignored.

use oxc_ast::ast::Expression;

use super::r3::replace;
use super::{Report, Role, Template, TemplateRule, bound3};
use crate::template::bindings::{AttributeKind, Value, attribute_kind, read_attribute};

/// The rule and its option.
pub struct PreferStaticStringProperties {
    /// The names left alone, beside the property-only APIs.
    pub ignore: &'static [&'static str],
}

impl PreferStaticStringProperties {
    /// The plugin's default, which the project runs.
    pub const PROJECT: Self = Self { ignore: &[] };
}

/// The DOM's properties an attribute does not stand for (`PROPERTY_ONLY_DOM_APIS`).
const PROPERTY_ONLY: [&str; 4] = ["innerHTML", "innerText", "outerHTML", "textContent"];

impl TemplateRule for PreferStaticStringProperties {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/prefer-static-string-properties"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "preferStaticStringProperties",
            "Using a property is more efficient than binding a static string.",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, _) in template.elements() {
            if crate::template::tags::split_ns_name(&element.name).1 == "ng-content" {
                continue;
            }
            for each in bound3(element) {
                // A bound name written inside its delimiters, neither `@` nor `.` in it.
                let bound = matches!(each.kind, AttributeKind::Property | AttributeKind::TwoWay);
                if each.role != Role::Input
                    || !bound
                    || attribute_kind(&each.attribute.name).1.contains(['@', '.'])
                    || PROPERTY_ONLY.contains(&each.name.as_str())
                    || self.ignore.contains(&each.name.as_str())
                {
                    continue;
                }
                let Ok(read) = read_attribute(template.allocator, &element.name, each.attribute)
                else {
                    continue;
                };
                let Value::Binding(binding) = read.value else {
                    continue;
                };
                let Some(Expression::StringLiteral(string)) = binding.expression.as_ref() else {
                    continue;
                };
                if !binding.pipes.is_empty() {
                    continue;
                }
                let quote = if each.attribute.value.trim_start().starts_with('"') {
                    '\''
                } else {
                    '"'
                };
                let escaped = if quote == '"' { "&quot;" } else { "&#39;" };
                let literal = string.value.replace('&', "&amp;").replace(quote, escaped);
                let span = each.attribute.span;
                reports.push(Report {
                    fix: Some(replace(
                        span.start,
                        span.end,
                        &format!("{}={quote}{literal}{quote}", each.name),
                    )),
                    ..Report::new(span, "preferStaticStringProperties")
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferStaticStringProperties;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("prefer-static-string-properties.txt", &[], &|options| {
            let ignore: Vec<&'static str> = options
                .and_then(|each| each.texts("ignore"))
                .unwrap_or_default()
                .into_iter()
                .map(|each| &*String::leak(each))
                .collect();
            Box::new(PreferStaticStringProperties {
                ignore: Vec::leak(ignore),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 18 valid and 7 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (18, 7, 0));
    }
}
