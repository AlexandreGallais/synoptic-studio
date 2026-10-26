//! `@angular-eslint/template/button-has-type`: every `<button>` has a `type`, plain or bound, of
//! `button`, `submit` or `reset` when written as text (`@angular-eslint/eslint-plugin-template`
//! 22.6.0, `src/rules/button-has-type.ts`): a button without one is reported over the element,
//! unless it carries one of `ignoreWithDirectives` (`has_ignored_directive`); a plain or bound
//! string type outside the three is
//! reported over its attribute. The project runs the plugin's default, no directive ignored.

use oxc_allocator::Allocator;
use oxc_ast::ast::Expression;

use super::{
    Bound3, Report, Role, Template, TemplateRule, bound3, has_ignored_directive, is_plain_element,
};
use crate::template::bindings::{Value, read_attribute};
use crate::template::tree::Element;

/// The rule and its option.
pub struct ButtonHasType {
    /// The directives that leave a button without a type alone.
    pub ignore_with_directives: &'static [&'static str],
}

impl ButtonHasType {
    /// The plugin's default, which the project runs.
    pub const PROJECT: Self = Self {
        ignore_with_directives: &[],
    };
}

/// The types a button takes.
const VALID_BUTTON_TYPES: [&str; 3] = ["button", "submit", "reset"];

/// The type a binding gives when it is a string, `[type]="'button'"`.
fn bound_string(allocator: &Allocator, element: &Element, each: &Bound3) -> Option<String> {
    let read = read_attribute(allocator, &element.name, each.attribute).ok()?;
    match read.value {
        Value::Binding(bound) if bound.pipes.is_empty() => match bound.expression {
            Some(Expression::StringLiteral(string)) => Some(string.value.as_str()?.to_owned()),
            _ => None,
        },
        _ => None,
    }
}

impl TemplateRule for ButtonHasType {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/button-has-type"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("missingType", "Type for <button> is missing"),
            (
                "invalidType",
                "\"{{type}}\" can not be used as a type for <button>",
            ),
        ]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, _) in template.elements() {
            if !element.name.eq_ignore_ascii_case("button") || !is_plain_element(element) {
                continue;
            }
            let attributes = bound3(element);
            let named = |role: Role, name: &'static str| {
                attributes
                    .iter()
                    .filter(move |each| each.role == role && each.name == name)
            };
            let has_type = named(Role::Input, "type")
                .chain(named(Role::Text, "type"))
                .next()
                .is_some();
            let ignored = has_ignored_directive(element, self.ignore_with_directives);
            if !has_type && !ignored {
                reports.push(Report::new(element.span, "missingType"));
            }
            let invalid_text = named(Role::Text, "type")
                .find(|each| !VALID_BUTTON_TYPES.contains(&each.attribute.value.as_str()))
                .map(|each| (each.attribute.span, each.attribute.value.clone()));
            let invalid = invalid_text.or_else(|| {
                named(Role::Input, "type").find_map(|each| {
                    bound_string(template.allocator, element, each)
                        .filter(|value| !VALID_BUTTON_TYPES.contains(&value.as_str()))
                        .map(|value| (each.attribute.span, value))
                })
            });
            if let Some((span, value)) = invalid {
                reports.push(Report {
                    data: vec![("type", value)],
                    ..Report::new(span, "invalidType")
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ButtonHasType;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run(
            "button-has-type.txt",
            &[("INVALID_TYPE_DATA_KEY", "type")],
            &|options| {
                let entries: Vec<String> = options
                    .and_then(|each| each.texts("ignoreWithDirectives"))
                    .unwrap_or_default();
                Box::new(ButtonHasType {
                    ignore_with_directives: Vec::leak(
                        entries
                            .into_iter()
                            .map(|each| &*String::leak(each))
                            .collect(),
                    ),
                })
            },
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 14 valid and 9 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (14, 9, 0));
    }
}
