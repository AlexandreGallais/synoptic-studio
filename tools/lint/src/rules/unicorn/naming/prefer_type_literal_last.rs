//! `unicorn/prefer-type-literal-last`: an inline object type comes after the other members of
//! its union, `null` and `undefined` last of all (`eslint-plugin-unicorn` 78.0.0). Autofixed,
//! as in `ESLint`, unless a comment sits between the members.

use oxc_ast::AstKind;
use oxc_ast::ast::TSType;
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::syntax::text;
use crate::source::Source;

/// The rule.
pub struct PreferTypeLiteralLast;

/// A member without the parentheses around it.
fn unwrapped<'b, 'a>(member: &'b TSType<'a>) -> &'b TSType<'a> {
    match member {
        TSType::TSParenthesizedType(inner) => unwrapped(&inner.type_annotation),
        other => other,
    }
}

/// Whether a member is an inline object type.
fn is_literal(member: &TSType) -> bool {
    matches!(unwrapped(member), TSType::TSTypeLiteral(_))
}

/// Whether a member is `null` or `undefined`.
fn is_nullish(member: &TSType) -> bool {
    matches!(
        unwrapped(member),
        TSType::TSNullKeyword(_) | TSType::TSUndefinedKeyword(_)
    )
}

/// Whether a member is itself a union or an intersection.
fn is_composite(member: &TSType) -> bool {
    matches!(
        unwrapped(member),
        TSType::TSUnionType(_) | TSType::TSIntersectionType(_)
    )
}

/// Whether a comment sits around the union's members, outside them.
fn has_loose_comment(source: &Source, span: Span, members: &[TSType]) -> bool {
    let before = source.text.get(..span.start as usize).unwrap_or_default();
    let region_start = before.trim_end().len();
    let after = source.text.get(span.end as usize..).unwrap_or_default();
    let region_end = span.end as usize + (after.len() - after.trim_start().len());
    source.program.comments.iter().any(|comment| {
        comment.span.start as usize >= region_start
            && comment.span.end as usize <= region_end
            && !members.iter().any(|member| {
                let inner = member.span();
                comment.span.start >= inner.start && comment.span.end <= inner.end
            })
    })
}

impl Rule for PreferTypeLiteralLast {
    fn name(&self) -> &'static str {
        "unicorn/prefer-type-literal-last"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::TSUnionType(union) = node.kind() else {
            return;
        };
        let mut first_literal: Option<&TSType> = None;
        let mut reported: Option<&TSType> = None;
        for member in &union.types {
            if is_literal(member) {
                first_literal = first_literal.or(Some(member));
                continue;
            }
            if is_nullish(member) || is_composite(member) {
                continue;
            }
            if first_literal.is_some() {
                reported = first_literal;
                break;
            }
        }
        let Some(literal) = reported else {
            return;
        };
        let message = "Move type literals to the end of this union.".to_owned();
        if has_loose_comment(source, union.span, &union.types) {
            findings.push(source.finding(self.name(), literal.span().start, message));
            return;
        }
        let others = union
            .types
            .iter()
            .filter(|member| !is_literal(member) && !is_nullish(member));
        let literals = union.types.iter().filter(|member| is_literal(member));
        let nullish = union.types.iter().filter(|member| is_nullish(member));
        let replacement: Vec<&str> = others
            .chain(literals)
            .chain(nullish)
            .map(|member| text(source, member.span()))
            .collect();
        findings.push(source.fixed(
            self.name(),
            literal.span().start,
            message,
            Fix {
                start: union.span.start,
                end: union.span.end,
                text: replacement.join(" | "),
            },
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferTypeLiteralLast;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferTypeLiteralLast, "playground/a.ts", code))
    }

    /// The code once fixed.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&PreferTypeLiteralLast, "playground/a.ts", code))
    }

    #[test]
    fn fixes_the_documented_unions() {
        let code = "type ElementUnion = {\n\tfoo: string;\n} | Other;";
        assert_eq!(
            check(code),
            ["Move type literals to the end of this union."]
        );
        assert_eq!(
            fix(code),
            "type ElementUnion = Other | {\n\tfoo: string;\n};"
        );
        assert_eq!(
            fix("type A = undefined | {a: 1} | B | null | {b: 2};"),
            "type A = B | {a: 1} | {b: 2} | undefined | null;"
        );
        let commented = "type A = {a: 1} /* first */ | B;";
        assert_eq!(fix(commented), commented);
        assert_eq!(check(commented).len(), 1);
    }

    #[test]
    fn accepts_the_documented_orders() {
        for code in [
            "type ElementUnion = Other | {\n\tfoo: string;\n};",
            "type ElementUnion = Other | {\n\tfoo: string;\n} | undefined;",
            "type A = {a: 1} | null;",
            "type A = {a: 1} | (B & C);",
            "type A = B | C;",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
