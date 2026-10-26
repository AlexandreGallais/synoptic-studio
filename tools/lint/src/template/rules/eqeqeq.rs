//! `@angular-eslint/template/eqeqeq`: `===` and `!==` rather than `==` and `!=`
//! (`@angular-eslint/eslint-plugin-template` 22.6.0, `src/rules/eqeqeq.ts` and
//! `src/utils/literal-primitive.ts`), reported over the comparison; with `allowNullOrUndefined`,
//! a comparison with `null` or `undefined` is left. A comparison with a string that is not a
//! number is fixed, a `=` added to the operator; any other's fix is a suggestion, not carried.
//! The project runs the plugin's default, `null` and `undefined` compared strictly too.

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryOperator, Expression};
use oxc_span::GetSpan;

use super::{Report, Template, TemplateRule, each_node, placed};
use crate::finding::Fix;
use crate::javascript::is_numeric;

/// The rule and its option.
pub struct Eqeqeq {
    /// Whether a comparison with `null` or `undefined` is left.
    pub allow_null_or_undefined: bool,
}

impl Eqeqeq {
    /// The plugin's default, which the project runs.
    pub const PROJECT: Self = Self {
        allow_null_or_undefined: false,
    };
}

/// Whether an expression is `null` or `undefined`, which Angular reads as literals.
fn is_nil(expression: &Expression) -> bool {
    match expression {
        Expression::NullLiteral(_) => true,
        Expression::Identifier(name) => name.name == "undefined",
        _ => false,
    }
}

/// Whether an expression is a string that is not a number (`isStringNonNumericValue`).
fn is_string_not_number(expression: &Expression) -> bool {
    matches!(expression, Expression::StringLiteral(string) if !is_numeric(&string.value))
}

impl TemplateRule for Eqeqeq {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/eqeqeq"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            (
                "eqeqeq",
                "Expected `{{expectedOperation}}` but received `{{actualOperation}}`",
            ),
            (
                "suggestStrictEquality",
                "Replace `{{actualOperation}}` with `{{expectedOperation}}`",
            ),
        ]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for written in template.expressions() {
            each_node(&written, &mut |node, bound| {
                let AstKind::BinaryExpression(binary) = node else {
                    return;
                };
                let operation = match binary.operator {
                    BinaryOperator::Equality => "==",
                    BinaryOperator::Inequality => "!=",
                    _ => return,
                };
                if self.allow_null_or_undefined && (is_nil(&binary.left) || is_nil(&binary.right)) {
                    return;
                }
                let fixed =
                    is_string_not_number(&binary.left) || is_string_not_number(&binary.right);
                // The operator sits between the operands: a `=` added after it.
                let between = placed(
                    bound,
                    oxc_span::Span::new(binary.left.span().end, binary.right.span().start),
                );
                let gap = template
                    .text
                    .get(between.start as usize..between.end as usize)
                    .unwrap_or_default();
                let fix = gap.find(operation).filter(|_| fixed).map(|at| {
                    let at = between.start + crate::template::lexer::offset(at + operation.len());
                    Fix {
                        start: at,
                        end: at,
                        text: "=".to_owned(),
                    }
                });
                reports.push(Report {
                    data: vec![
                        ("actualOperation", operation.to_owned()),
                        ("expectedOperation", format!("{operation}=")),
                    ],
                    fix,
                    ..Report::new(placed(bound, binary.span), "eqeqeq")
                });
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Eqeqeq;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("eqeqeq.txt", &[], &|options| {
            Box::new(Eqeqeq {
                allow_null_or_undefined: options
                    .and_then(|each| each.flag("allowNullOrUndefined"))
                    .unwrap_or(false),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 7 valid and 13 invalid cases, all run; their 8 suggestions, not carried, unchecked.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (7, 13, 8));
    }

    #[test]
    fn fixes_a_comparison_with_a_string_only_when_it_is_no_number() {
        let fixed = |text: &str| {
            crate::template::lint_text("a.component.html", text)
                .iter()
                .map(|finding| finding.fix.is_some())
                .collect::<Vec<_>>()
        };

        // Our own cases: `'x'` is no number, fixed; `'3'` and `'\u{feff}3'` are, as
        // JavaScript trims U+FEFF, which Rust's `trim` keeps: a suggestion, not carried.
        assert_eq!(fixed("{{ a == 'x' }}"), [true]);
        assert_eq!(fixed("{{ a == '3' }}"), [false]);
        assert_eq!(fixed("{{ a == '\u{feff}3' }}"), [false]);
    }
}
