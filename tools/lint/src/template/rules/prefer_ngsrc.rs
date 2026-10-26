//! `@angular-eslint/template/prefer-ngsrc`: an `<img>` loads through `ngSrc`, never `src`
//! (`@angular-eslint/eslint-plugin-template` 22.6.0, `src/rules/prefer-ngsrc.ts`): its `src`,
//! plain or bound, is reported, as missing its `ngSrc` or doubling it; a `data:` URL without
//! `ngSrc`, plain, a bound string or a sum starting with one, is left. The plugin's
//! replacements are suggestions, not carried.

use oxc_ast::ast::{BinaryOperator, Expression};

use super::{Bound3, Report, Role, Template, TemplateRule, bound3, is_plain_element};
use crate::template::bindings::{Value, read_attribute};
use crate::template::tree::Element;

/// The rule.
pub struct PreferNgsrc;

/// Whether an expression is a string starting with `data:` (`isDataStringPrimitive`).
fn is_data(expression: &Expression) -> bool {
    matches!(expression, Expression::StringLiteral(string) if string.value.display().to_string().trim().starts_with("data:"))
}

/// Whether a `src` holds an inline image (`isSrcBase64Image`).
fn is_inline_image(template: &Template, element: &Element, source: &Bound3) -> bool {
    if source.role == Role::Text {
        return source.attribute.value.trim().starts_with("data:");
    }
    let Ok(read) = read_attribute(template.allocator, &element.name, source.attribute) else {
        return false;
    };
    let Value::Binding(bound) = read.value else {
        return false;
    };
    if !bound.pipes.is_empty() {
        return false;
    }
    match &bound.expression {
        Some(Expression::BinaryExpression(binary)) => {
            binary.operator == BinaryOperator::Addition && is_data(&binary.left)
        }
        Some(expression) => is_data(expression),
        None => false,
    }
}

impl TemplateRule for PreferNgsrc {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/prefer-ngsrc"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            (
                "missingAttribute",
                "The attribute [ngSrc] should be used for img elements instead of [src].",
            ),
            (
                "invalidDoubleSource",
                "Only [ngSrc] should exist on an img element. Delete the [src] attribute.",
            ),
            ("suggestReplaceWithNgSrc", "Replace [src] with [ngSrc]"),
            ("suggestRemoveSrc", "Remove the [src] attribute"),
        ]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, _) in template.elements() {
            if !element.name.eq_ignore_ascii_case("img") || !is_plain_element(element) {
                continue;
            }
            let attributes: Vec<Bound3> = bound3(element)
                .into_iter()
                .filter(|each| each.role != Role::Output)
                .collect();
            let ng_src = attributes.iter().any(|each| each.name == "ngSrc");
            let Some(source) = attributes.iter().find(|each| each.name == "src") else {
                continue;
            };
            if !ng_src && is_inline_image(template, element, source) {
                continue;
            }
            let id = if ng_src {
                "invalidDoubleSource"
            } else {
                "missingAttribute"
            };
            reports.push(Report::new(source.attribute.span, id));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferNgsrc;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("prefer-ngsrc.txt", &[], &|_| Box::new(PreferNgsrc));

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 10 valid and 26 invalid cases, all run; their 26 suggestions, not carried, unchecked.
        assert_eq!(
            (tally.valid, tally.invalid, tally.suggestions),
            (10, 26, 26)
        );
    }
}
