//! `unicorn/no-unnecessary-global-this`: `globalThis.name` read where `name` is a global the
//! language declares and nothing in scope shadows, written `name` (`eslint-plugin-unicorn`
//! 78.0.0, `docs/rules/no-unnecessary-global-this.md`). Writes, deletes, optional chains, a direct
//! `eval` call and existence checks — under `&&`, `||` or `??`, compared with `null` or
//! `undefined`, read for its truth — are left; a call or a tag is reported without a fix, its
//! `this` changing, and so is a member holding a comment.

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryOperator, Expression};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::globals::{ES_GLOBALS, is_identifier_name, member_name, written};
use crate::expressions::{has_comment_in, is_boolean_context, is_global};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{outer_parent, unwrap};
use crate::source::Source;

/// The rule.
pub struct NoUnnecessaryGlobalThis;

/// The reserved words, which cannot be read as a bare name (`reserved-identifiers`), less
/// `arguments` and `eval`, which can.
const RESERVED: [&str; 46] = [
    "await",
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "enum",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "function",
    "if",
    "import",
    "in",
    "instanceof",
    "new",
    "null",
    "return",
    "super",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "typeof",
    "var",
    "void",
    "while",
    "with",
    "yield",
    "implements",
    "interface",
    "let",
    "package",
    "private",
    "protected",
    "public",
    "static",
];

/// Whether an expression is `null` or the global `undefined`.
fn is_nullish(source: &Source, expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::NullLiteral(_) => true,
        Expression::Identifier(name) => name.name == "undefined" && is_global(source, name),
        _ => false,
    }
}

/// Whether the member is read to know whether the global exists.
fn is_existence_check(source: &Source, node: &AstNode) -> bool {
    let (parent, child) = outer_parent(source, node.id());
    let compared = match parent {
        AstKind::LogicalExpression(_) => return true,
        AstKind::BinaryExpression(binary) => {
            matches!(
                binary.operator,
                BinaryOperator::Equality
                    | BinaryOperator::Inequality
                    | BinaryOperator::StrictEquality
                    | BinaryOperator::StrictInequality
            ) && {
                let child_span = source.semantic.nodes().kind(child).span();
                let other = if binary.left.span() == child_span {
                    &binary.right
                } else {
                    &binary.left
                };
                is_nullish(source, other)
            }
        }
        _ => false,
    };
    compared || is_boolean_context(source, child)
}

/// Whether the member sits in an optional chain, or is called or read optionally.
fn is_optional_use(source: &Source, node: &AstNode, optional: bool) -> bool {
    if optional {
        return true;
    }
    let (parent, child) = outer_parent(source, node.id());
    let child_span = source.semantic.nodes().kind(child).span();
    match parent {
        AstKind::CallExpression(call) => call.optional && call.callee.span() == child_span,
        AstKind::StaticMemberExpression(member) => {
            member.optional && member.object.span() == child_span
        }
        AstKind::ComputedMemberExpression(member) => {
            member.optional && member.object.span() == child_span
        }
        _ => false,
    }
}

/// Whether the member is called or tags a template.
fn is_called(source: &Source, node: &AstNode) -> (bool, bool) {
    let (parent, child) = outer_parent(source, node.id());
    let child_span = source.semantic.nodes().kind(child).span();
    match parent {
        AstKind::CallExpression(call) => (call.callee.span() == child_span, false),
        AstKind::TaggedTemplateExpression(tagged) => (false, tagged.tag.span() == child_span),
        _ => (false, false),
    }
}

impl Rule for NoUnnecessaryGlobalThis {
    fn name(&self) -> &'static str {
        "unicorn/no-unnecessary-global-this"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        if !matches!(
            node.kind(),
            AstKind::StaticMemberExpression(_) | AstKind::ComputedMemberExpression(_)
        ) {
            return;
        }
        let Some((object, name, optional)) = member_name(source, node) else {
            return;
        };
        let Expression::Identifier(object) = unwrap(object) else {
            return;
        };
        if object.name != "globalThis"
            || !is_global(source, object)
            || written(source, node.id()).is_some()
            || is_optional_use(source, node, optional)
            || is_existence_check(source, node)
        {
            return;
        }
        let (called, tagged) = is_called(source, node);
        let usable = is_identifier_name(&name) && !RESERVED.contains(&name.as_str());
        let shadowed = source
            .semantic
            .scoping()
            .find_binding(node.scope_id(), oxc_ast::ast::Ident::from(name.as_str()))
            .is_some();
        if !usable || (name == "eval" && called) || !ES_GLOBALS.contains(&name.as_str()) || shadowed
        {
            return;
        }
        let span = node.kind().span();
        let message = format!("Use `{name}` directly instead of `globalThis.{name}`.");
        if called || tagged || has_comment_in(source, span) {
            findings.push(source.finding(self.name(), span.start, message));
            return;
        }
        let fix = Fix {
            start: span.start,
            end: span.end,
            text: name,
        };
        findings.push(source.fixed(self.name(), span.start, message, fix));
    }
}

#[cfg(test)]
mod tests {
    use super::NoUnnecessaryGlobalThis;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&NoUnnecessaryGlobalThis, "playground/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn reads_a_global_without_global_this() {
        // The documentation's two cases, a computed key and a global value.
        assert_eq!(
            check("globalThis.Array.from(items);"),
            (vec![(1, 1)], "Array.from(items);".to_owned())
        );
        assert_eq!(
            check("globalThis.JSON.stringify(value);").1,
            "JSON.stringify(value);"
        );
        assert_eq!(
            check("const a = globalThis['Math'].PI;").1,
            "const a = Math.PI;"
        );
        assert_eq!(check("const a = globalThis.NaN;").1, "const a = NaN;");
        assert_eq!(
            check("const a = (globalThis.Map as any);").1,
            "const a = (Map as any);"
        );
    }

    #[test]
    fn reports_calls_tags_and_comments_without_a_fix() {
        for code in [
            "globalThis.parseInt('1');",
            "globalThis.String`a`;",
            "const a = globalThis /* b */ .Math;",
        ] {
            assert_eq!(
                check(code),
                (
                    vec![(1, code.find("globalThis").map_or(0, |at| at + 1))],
                    code.to_owned()
                ),
                "{code}"
            );
        }
    }

    #[test]
    fn leaves_unknown_names_writes_chains_checks_and_shadowed_globals() {
        for code in [
            "globalThis.alert?.();",
            "globalThis?.Array;",
            "globalThis.jQuery = jQuery;",
            "globalThis.Array = a;",
            "delete globalThis.Array;",
            "if (globalThis.navigation) {}",
            "if (globalThis.navigation === undefined) {}",
            "const a = globalThis.Map ?? b;",
            "const a = globalThis.Map == null;",
            "const a = !globalThis.Map;",
            "const a = globalThis.location.search;",
            "globalThis.eval('a');",
            "const a = globalThis[name];",
            "const Map = 1;\nconst a = globalThis.Map;",
            "function f(Set: number) { return globalThis.Set; }",
            "const globalThis = {};\nglobalThis.Array;",
            "const a = globalThis['not a name'];",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
        assert_eq!(check("const a = globalThis.eval;").1, "const a = eval;");
    }
}
