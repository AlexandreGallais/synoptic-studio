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

/// Whether a text is a number to JavaScript (`isNumeric`: `parseFloat` gives a number and
/// `Number` a finite one): blank aside, an integer in hexadecimal, octal or binary, or a
/// decimal with an optional sign, fraction and exponent, whitespace around.
fn is_numeric(text: &str) -> bool {
    let text = text.trim();
    let radix = [
        ("0x", 16),
        ("0X", 16),
        ("0o", 8),
        ("0O", 8),
        ("0b", 2),
        ("0B", 2),
    ];
    for (prefix, base) in radix {
        if let Some(digits) = text.strip_prefix(prefix) {
            return !digits.is_empty() && digits.chars().all(|each| each.is_digit(base));
        }
    }
    let unsigned = text.strip_prefix(['+', '-']).unwrap_or(text);
    let (mantissa, exponent) = unsigned
        .split_once(['e', 'E'])
        .map_or((unsigned, None), |(mantissa, exponent)| {
            (mantissa, Some(exponent))
        });
    let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let digits = |part: &str| part.chars().all(|each| each.is_ascii_digit());
    let mantissa_ok =
        (!whole.is_empty() || !fraction.is_empty()) && digits(whole) && digits(fraction);
    let exponent_ok = exponent.is_none_or(|exponent| {
        let exponent = exponent.strip_prefix(['+', '-']).unwrap_or(exponent);
        !exponent.is_empty() && digits(exponent)
    });
    mantissa_ok && exponent_ok && unsigned.parse::<f64>().is_ok_and(f64::is_finite)
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
    use super::{Eqeqeq, is_numeric};
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
    fn tells_a_number_as_javascript_does() {
        // Our own cases: `Number` and `parseFloat` both give a finite number.
        for numeric in ["3", " 3 ", "-1.5e3", ".5", "5.", "0x1F", "0b10"] {
            assert!(is_numeric(numeric), "{numeric}");
        }
        for text in [
            "", " ", "null", "3abc", "Infinity", "1e", "0x", "inf", "1_0", "+",
        ] {
            assert!(!is_numeric(text), "{text}");
        }
    }
}
