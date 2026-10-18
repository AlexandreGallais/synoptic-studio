//! `unicorn/prefer-math-constants`: a decimal written by hand that truncates or rounds one of
//! `Math`'s numeric constants, `3.14` for `Math.PI`, is reported (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/prefer-math-constants.md`). A literal needs as many decimals as the plugin asks of
//! each constant, two for `PI`, three for `E`, four for the others, so that `1.41` or `0.69` pass;
//! a property key and a literal type, where `Math.PI` cannot be written, are left. The constant
//! is not the literal's value, so the plugin only suggests it: no fix.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::super::syntax::{parent, text};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferMathConstants;

/// `Math`'s numeric constants, their values, and the decimals a literal needs to be read as one
/// (the plugin's `mathConstants`, in its order).
const CONSTANTS: [(&str, f64, usize); 8] = [
    ("PI", std::f64::consts::PI, 2),
    ("E", std::f64::consts::E, 3),
    ("LN2", std::f64::consts::LN_2, 4),
    ("LN10", std::f64::consts::LN_10, 4),
    ("LOG2E", std::f64::consts::LOG2_E, 4),
    ("LOG10E", std::f64::consts::LOG10_E, 4),
    ("SQRT2", std::f64::consts::SQRT_2, 4),
    ("SQRT1_2", std::f64::consts::FRAC_1_SQRT_2, 4),
];

/// A literal written as a plain decimal, `.5` or `3.14`, its separators removed: its value with
/// a leading `0` and its count of decimals.
fn plain_decimal(raw: &str) -> Option<(String, usize)> {
    let written: String = raw.chars().filter(|each| *each != '_').collect();
    let (whole, fraction) = written.split_once('.')?;
    let digits = |part: &str| part.chars().all(|each| each.is_ascii_digit());
    if fraction.is_empty() || !digits(whole) || !digits(fraction) {
        return None;
    }
    let whole = if whole.is_empty() { "0" } else { whole };
    Some((format!("{whole}.{fraction}"), fraction.len()))
}

/// The constant a decimal approximates: one whose text starts with it, or which rounds to it at
/// its number of decimals.
fn approximated(value: &str, decimals: usize) -> Option<&'static str> {
    CONSTANTS
        .iter()
        .find(|(_, exact, minimum)| {
            let exact_text = exact.to_string();
            decimals >= *minimum
                && (exact_text.starts_with(value)
                    || (value.len() < exact_text.len() && format!("{exact:.decimals$}") == value))
        })
        .map(|(name, _, _)| *name)
}

/// Whether a literal stands where `Math.…` cannot be written: a key that is not computed, or a
/// literal type, signed or not.
fn is_non_replaceable(source: &Source, node: &AstNode) -> bool {
    let span = node.kind().span();
    let is_key = |key: &oxc_ast::ast::PropertyKey, computed: bool| !computed && key.span() == span;
    match parent(source, node.id()) {
        AstKind::ObjectProperty(property) => is_key(&property.key, property.computed),
        AstKind::PropertyDefinition(property) => is_key(&property.key, property.computed),
        AstKind::MethodDefinition(method) => is_key(&method.key, method.computed),
        AstKind::AccessorProperty(property) => is_key(&property.key, property.computed),
        AstKind::TSPropertySignature(property) => is_key(&property.key, property.computed),
        AstKind::TSMethodSignature(method) => is_key(&method.key, method.computed),
        AstKind::TSLiteralType(_) => true,
        AstKind::UnaryExpression(_) => {
            let unary = source.semantic.nodes().parent_id(node.id());
            matches!(parent(source, unary), AstKind::TSLiteralType(_))
        }
        _ => false,
    }
}

impl Rule for PreferMathConstants {
    fn name(&self) -> &'static str {
        "unicorn/prefer-math-constants"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::NumericLiteral(literal) = node.kind() else {
            return;
        };
        let raw = text(source, literal.span);
        let Some(name) = plain_decimal(raw)
            .and_then(|(value, decimals)| approximated(&value, decimals))
            .filter(|_| !is_non_replaceable(source, node))
        else {
            return;
        };
        findings.push(source.finding(
            self.name(),
            literal.span.start,
            format!("Prefer `Math.{name}` over the approximate value `{raw}`."),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferMathConstants;
    use crate::testing::{lint, messages};

    /// The messages of the findings.
    fn found(code: &str) -> Vec<String> {
        messages(&lint(&PreferMathConstants, "scripts/a.ts", code))
    }

    #[test]
    fn reports_the_documented_approximations() {
        // The documentation's three examples.
        assert_eq!(
            found("const area = 3.14 * radius ** 2;"),
            ["Prefer `Math.PI` over the approximate value `3.14`."]
        );
        assert_eq!(
            found("const growth = amount * 2.718;"),
            ["Prefer `Math.E` over the approximate value `2.718`."]
        );
        assert_eq!(
            found("const diagonal = side * 1.4142;"),
            ["Prefer `Math.SQRT2` over the approximate value `1.4142`."]
        );
    }

    #[test]
    fn reports_roundings_truncations_and_every_constant() {
        // 3.1416 rounds π (3.14159…); .6931 truncates ln 2; separators are read through.
        for (code, name) in [
            ("f(3.1416);", "PI"),
            ("f(3.141_59);", "PI"),
            ("f(.6931);", "LN2"),
            ("f(2.3026);", "LN10"),
            ("f(1.4427);", "LOG2E"),
            ("f(0.4343);", "LOG10E"),
            ("f(0.7071);", "SQRT1_2"),
            ("f(-3.14);", "PI"),
            ("const o = { [3.14]: 1 };", "PI"),
        ] {
            assert_eq!(found(code).len(), 1, "{code}");
            assert!(found(code)[0].contains(name), "{code}");
        }
    }

    #[test]
    fn leaves_short_decimals_other_forms_keys_and_literal_types() {
        for code in [
            "f(3.1);",
            "f(1.41);",
            "f(0.69);",
            "f(2.71);",
            "f(3.15);",
            "f(3);",
            "f(314e-2);",
            "f(3.);",
            "const o = { 3.14: 1 };",
            "type T = { 3.14: string };",
            "type P = 3.14;",
            "type N = -3.14;",
            "const v = x as 3.14;",
        ] {
            assert_eq!(found(code), Vec::<String>::new(), "{code}");
        }
    }
}
