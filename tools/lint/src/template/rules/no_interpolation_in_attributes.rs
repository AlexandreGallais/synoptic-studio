//! `@angular-eslint/template/no-interpolation-in-attributes`: a property binding rather than
//! an interpolation in an attribute's value (`@angular-eslint/eslint-plugin-template` 22.6.0,
//! `src/rules/no-interpolation-in-attributes.ts`): each attribute whose value interpolates is
//! reported over its value; one that is only interpolations is fixed into `[name]="…"`. With
//! `allowSubstringInterpolation`, a value mixing text and interpolations is left. The project
//! runs the plugin's default, every interpolation reported.

use super::r3::{replace, slice};
use super::{Report, Role, Template, TemplateRule, bound3};
use crate::template::lexer::TokenType;

/// The rule and its option.
pub struct NoInterpolationInAttributes {
    /// Whether a value mixing text and interpolations is left.
    pub allow_substring_interpolation: bool,
}

impl NoInterpolationInAttributes {
    /// The plugin's default, which the project runs.
    pub const PROJECT: Self = Self {
        allow_substring_interpolation: false,
    };
}

impl TemplateRule for NoInterpolationInAttributes {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/no-interpolation-in-attributes"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "noInterpolationInAttributes",
            "Use property binding [attribute]=\"value\" instead of interpolation {{ value }} for an attribute.",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, _) in template.elements() {
            for each in bound3(element) {
                let attribute = each.attribute;
                let interpolated = each.role == Role::Input
                    && attribute.value_tokens.iter().any(|token| {
                        token.kind == TokenType::AttrValueInterpolation && token.parts.len() == 3
                    });
                let Some(value) = attribute.value_span.filter(|_| interpolated) else {
                    continue;
                };
                // The strings between the interpolations, all empty for a full one.
                let full = attribute.value_tokens.iter().all(|token| {
                    token.kind == TokenType::AttrValueInterpolation
                        || token.parts.iter().all(String::is_empty)
                });
                if self.allow_substring_interpolation && !full {
                    continue;
                }
                let fix = full.then(|| {
                    let name = slice(
                        template.text,
                        attribute.key_span.start,
                        attribute.key_span.end,
                    )
                    .trim();
                    let expression = slice(template.text, value.start + 2, value.end - 2).trim();
                    replace(
                        attribute.span.start,
                        attribute.span.end,
                        &format!("[{name}]=\"{expression}\""),
                    )
                });
                reports.push(Report {
                    fix,
                    ..Report::new(value, "noInterpolationInAttributes")
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoInterpolationInAttributes;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("no-interpolation-in-attributes.txt", &[], &|options| {
            Box::new(NoInterpolationInAttributes {
                allow_substring_interpolation: options
                    .and_then(|each| each.flag("allowSubstringInterpolation"))
                    .unwrap_or(false),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 6 valid and 5 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (6, 5, 0));
    }
}
