//! `unicorn/prefer-global-number-constants`: `Number.NaN`, `Number.POSITIVE_INFINITY` and
//! `Number.NEGATIVE_INFINITY` read from the global `Number`, written `NaN`, `Infinity` and
//! `-Infinity` (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-global-number-constants.md`).
//! `Number` is followed as `ESLint`'s `ReferenceTracker` follows it: named, read from
//! `globalThis`, optionally or not, and destructured. Left when the replacement is shadowed or
//! the member written. The first two are autofixed unless a comment is inside; `-Infinity` is
//! only reported, as the plugin does, and so is a destructured property, whose fix by the plugin
//! would drop the local name.

use oxc_ast::AstKind;
use oxc_ast::ast::{BindingPattern, Expression, PropertyKey};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use super::globals::{member_name, written};
use crate::expressions::{has_comment_in, is_global};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::syntax::unwrap;
use crate::source::Source;

/// The rule.
pub struct PreferGlobalNumberConstants;

/// Whether an expression is the global `Number`, named or read from `globalThis`.
fn is_number(source: &Source, expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::Identifier(name) => name.name == "Number" && is_global(source, name),
        Expression::StaticMemberExpression(member) => {
            member.property.name == "Number"
                && matches!(unwrap(&member.object), Expression::Identifier(object)
                    if object.name == "globalThis" && is_global(source, object))
        }
        _ => false,
    }
}

/// The global a property of `Number` is better written as.
fn replacement(property: &str) -> Option<&'static str> {
    match property {
        "NaN" => Some("NaN"),
        "POSITIVE_INFINITY" => Some("Infinity"),
        "NEGATIVE_INFINITY" => Some("-Infinity"),
        _ => None,
    }
}

impl PreferGlobalNumberConstants {
    /// The finding on a property of `Number` read at a range, unless its global is shadowed;
    /// fixed when asked.
    fn report(
        &self,
        source: &Source,
        node: &AstNode,
        (property, span): (&str, Span),
        fixable: bool,
    ) -> Option<Finding> {
        let replacement = replacement(property)?;
        let global_name = if replacement == "NaN" {
            "NaN"
        } else {
            "Infinity"
        };
        let shadowed = source
            .semantic
            .scoping()
            .find_binding(node.scope_id(), oxc_ast::ast::Ident::from(global_name))
            .is_some();
        if shadowed {
            return None;
        }
        let message = format!("Prefer `{replacement}` over `Number.{property}`.");
        if !fixable || property == "NEGATIVE_INFINITY" || has_comment_in(source, span) {
            return Some(source.finding(self.name(), span.start, message));
        }
        let fix = Fix {
            start: span.start,
            end: span.end,
            text: replacement.to_owned(),
        };
        Some(source.fixed(self.name(), span.start, message, fix))
    }
}

impl Rule for PreferGlobalNumberConstants {
    fn name(&self) -> &'static str {
        "unicorn/prefer-global-number-constants"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        match node.kind() {
            AstKind::StaticMemberExpression(_) | AstKind::ComputedMemberExpression(_) => {
                let Some((object, property, _)) = member_name(source, node) else {
                    return;
                };
                if is_number(source, object) && written(source, node.id()).is_none() {
                    let span = node.kind().span();
                    findings.extend(self.report(source, node, (&property, span), true));
                }
            }
            AstKind::VariableDeclarator(declarator) => {
                let (BindingPattern::ObjectPattern(pattern), Some(init)) =
                    (&declarator.id, &declarator.init)
                else {
                    return;
                };
                if !is_number(source, init) {
                    return;
                }
                for property in &pattern.properties {
                    let name = match &property.key {
                        PropertyKey::StaticIdentifier(key) if !property.computed => {
                            key.name.as_str()
                        }
                        PropertyKey::StringLiteral(key) => match key.value.as_str() {
                            Some(name) => name,
                            None => continue,
                        },
                        _ => continue,
                    };
                    findings.extend(self.report(source, node, (name, property.span), false));
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferGlobalNumberConstants;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferGlobalNumberConstants, "src/a/b.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn writes_the_global_constants() {
        // The documentation's three cases.
        assert_eq!(
            check("const foo = Number.NaN;"),
            (vec![(1, 13)], "const foo = NaN;".to_owned())
        );
        assert_eq!(
            check("const foo = Number.POSITIVE_INFINITY;").1,
            "const foo = Infinity;"
        );
        let negative = "const foo = Number.NEGATIVE_INFINITY;";
        assert_eq!(check(negative), (vec![(1, 13)], negative.to_owned()));
        assert_eq!(check("const foo = Number['NaN'];").1, "const foo = NaN;");
        assert_eq!(
            check("const foo = globalThis.Number.NaN;").1,
            "const foo = NaN;"
        );
        let commented = "const foo = Number /* a */ .NaN;";
        assert_eq!(check(commented), (vec![(1, 13)], commented.to_owned()));
        assert_eq!(check("const foo = Number?.NaN;").1, "const foo = NaN;");
        let destructured = "const { NaN: a, POSITIVE_INFINITY } = Number;";
        assert_eq!(
            check(destructured),
            (vec![(1, 9), (1, 17)], destructured.to_owned())
        );
    }

    #[test]
    fn leaves_other_properties_shadows_writes_and_locals() {
        for code in [
            "const foo = Number.MAX_VALUE;",
            "const NaN = 1;\nconst foo = Number.NaN;",
            "function f(Infinity: number) { return Number.POSITIVE_INFINITY; }",
            "Number.NaN = 1;",
            "const Number = { NaN: 1 };\nconst foo = Number.NaN;",
            "const foo = other.NaN;",
            "const foo = window.Number.NaN;",
            "const { MAX_VALUE } = Number;",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
