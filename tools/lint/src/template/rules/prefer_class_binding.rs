//! `@angular-eslint/template/prefer-class-binding`: `[class]` and `[class.x]` rather than
//! `[ngClass]` (`@angular-eslint/eslint-plugin-template` 22.6.0,
//! `src/rules/prefer-class-binding.ts`), each bound `ngClass` reported over its source span,
//! unless it is an object literal with a key of several class names, which only `ngClass`
//! reads (`requiresNgClass`).

use oxc_ast::ast::{Expression, ObjectPropertyKind};

use super::{Report, Template, TemplateRule, bound_attributes};
use crate::template::bindings::{Value, read_attribute};

/// The rule.
pub struct PreferClassBinding;

impl TemplateRule for PreferClassBinding {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/prefer-class-binding"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "preferClassBinding",
            "Consider using [class] bindings instead of [ngClass] where applicable.",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, _) in template.elements() {
            for each in bound_attributes(template.allocator, element) {
                if each.name != "ngClass" {
                    continue;
                }
                let requires = !each.microsyntax
                    && read_attribute(template.allocator, &element.name, each.attribute)
                        .is_ok_and(|read| match read.value {
                            Value::Binding(bound) if bound.pipes.is_empty() => {
                                matches!(&bound.expression, Some(Expression::ObjectExpression(object))
                                    if object.properties.iter().any(|property| match property {
                                        ObjectPropertyKind::ObjectProperty(property) => property
                                            .key
                                            .static_name()
                                            .is_some_and(|key| key.trim().contains(char::is_whitespace)),
                                        ObjectPropertyKind::SpreadProperty(_) => false,
                                    }))
                            }
                            _ => false,
                        });
                if !requires {
                    reports.push(Report::new(each.span, "preferClassBinding"));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferClassBinding;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("prefer-class-binding.txt", &[], &|_| {
            Box::new(PreferClassBinding)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 10 valid and 7 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (10, 7, 0));
    }
}
