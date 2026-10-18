//! `unicorn/no-declarations-before-early-exit`: a `const` declared before a guard — an `if`
//! of which one branch always leaves — and read only after it is moved below the guard
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/no-declarations-before-early-exit.md`). Only the
//! first guard after the declaration counts; a declaration not right before it needs a literal
//! value; a value that awaits, yields or calls a React hook stays, and so does one sharing a name
//! with the guard when either changes anything. Autofixed when the declaration, alone and
//! literal, sits right before the guard with no comment around.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    BindingPattern, Expression, Statement, VariableDeclaration, VariableDeclarationKind,
    VariableDeclarator,
};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use crate::exits::always_exits;
use crate::expressions::{
    has_comment_in, has_side_effect, read_names, statement_names, statements, suspends, text,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoDeclarationsBeforeEarlyExit;

/// The rule's message.
const MESSAGE: &str = "Move this declaration after the early exit.";

/// Whether a statement is a guard: an `if` of which exactly one branch always leaves.
fn is_guard(statement: &Statement) -> bool {
    let Statement::IfStatement(choice) = statement else {
        return false;
    };
    let alternate = choice.alternate.as_ref().is_some_and(always_exits);
    always_exits(&choice.consequent) != alternate
}

/// Whether a value is a literal: none, a literal, or a template without substitution.
fn is_literal(value: Option<&Expression>) -> bool {
    value.is_none_or(|value| match value {
        Expression::TemplateLiteral(template) => template.expressions.is_empty(),
        other => other.is_literal(),
    })
}

/// Whether a name is a React hook's, `useState`.
fn is_hook_name(name: &str) -> bool {
    name.strip_prefix("use")
        .and_then(|rest| rest.chars().next())
        .is_some_and(|first| first.is_ascii_uppercase() || first.is_ascii_digit())
}

/// Whether a value calls a React hook, `useMemo(…)` or `React.useMemo(…)`.
fn calls_hook(value: &Expression) -> bool {
    let Expression::CallExpression(call) = value else {
        return false;
    };
    match &call.callee {
        Expression::Identifier(callee) => is_hook_name(&callee.name),
        Expression::StaticMemberExpression(member) => {
            matches!(&member.object, Expression::Identifier(object)
                if object.name.starts_with(|first: char| first.is_ascii_uppercase()))
                && is_hook_name(&member.property.name)
        }
        _ => false,
    }
}

/// Whether only blanks separate two offsets from a comment, before or after.
fn comment_next_to(source: &Source, start: u32, end: u32) -> bool {
    source.program.comments.iter().any(|comment| {
        (comment.span.end <= start
            && text(source, Span::new(comment.span.end, start))
                .trim()
                .is_empty())
            || (comment.span.start >= end
                && text(source, Span::new(end, comment.span.start))
                    .trim()
                    .is_empty())
    })
}

/// A declaration and the guard after it, in their list.
struct Pair<'s, 'a> {
    declaration: &'s VariableDeclaration<'a>,
    guard: &'s Statement<'a>,
    adjacent: bool,
}

impl NoDeclarationsBeforeEarlyExit {
    /// The finding on one declarator, if it is to move.
    fn check(
        &self,
        pair: &Pair,
        declarator: &VariableDeclarator,
        source: &Source,
    ) -> Option<Finding> {
        let BindingPattern::BindingIdentifier(name) = &declarator.id else {
            return None;
        };
        let value = declarator.init.as_ref();
        if value.is_some_and(|value| calls_hook(value) || suspends(value))
            || (!pair.adjacent && !is_literal(value))
        {
            return None;
        }
        let Statement::IfStatement(choice) = pair.guard else {
            return None;
        };
        if let Some(value) = value {
            let guard_names = statement_names(pair.guard);
            let shared = read_names(value)
                .iter()
                .any(|each| guard_names.contains(each));
            if shared && (has_side_effect(value) || has_side_effect(&choice.test)) {
                return None;
            }
        }
        let symbol = name.symbol_id.get()?;
        let scoping = source.semantic.scoping();
        let nodes = source.semantic.nodes();
        let places: Vec<u32> = scoping
            .get_resolved_references(symbol)
            .filter(|reference| {
                !nodes
                    .ancestor_kinds(reference.node_id())
                    .any(|kind| matches!(kind, AstKind::TSTypeQuery(_)))
            })
            .map(|reference| nodes.kind(reference.node_id()).span().start)
            .collect();
        let guard_end = pair.guard.span().end;
        if places.is_empty() || places.iter().any(|place| *place < guard_end) {
            return None;
        }
        let declaration = pair.declaration.span;
        let fixable = pair.adjacent
            && pair.declaration.declarations.len() == 1
            && is_literal(value)
            && !has_comment_in(source, Span::new(declaration.start, guard_end))
            && !comment_next_to(source, declaration.start, guard_end);
        if !fixable {
            return Some(source.finding(self.name(), name.span.start, MESSAGE.to_owned()));
        }
        let separator = text(source, Span::new(declaration.end, pair.guard.span().start));
        let fix = Fix {
            start: declaration.start,
            end: guard_end,
            text: format!(
                "{}{separator}{}",
                text(source, pair.guard.span()),
                text(source, declaration)
            ),
        };
        Some(source.fixed(self.name(), name.span.start, MESSAGE.to_owned(), fix))
    }
}

impl Rule for NoDeclarationsBeforeEarlyExit {
    fn name(&self) -> &'static str {
        "unicorn/no-declarations-before-early-exit"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        if !matches!(
            node.kind(),
            AstKind::Program(_) | AstKind::BlockStatement(_) | AstKind::FunctionBody(_)
        ) {
            return;
        }
        let Some(list) = statements(node.kind()) else {
            return;
        };
        for (index, statement) in list.iter().enumerate() {
            let Statement::VariableDeclaration(declaration) = statement else {
                continue;
            };
            if declaration.declare
                || !matches!(
                    declaration.kind,
                    VariableDeclarationKind::Const | VariableDeclarationKind::Let
                )
            {
                continue;
            }
            let Some(offset) = list[index + 1..].iter().position(is_guard) else {
                continue;
            };
            let pair = Pair {
                declaration,
                guard: &list[index + 1 + offset],
                adjacent: offset == 0,
            };
            for declarator in &declaration.declarations {
                findings.extend(self.check(&pair, declarator, source));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoDeclarationsBeforeEarlyExit;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&NoDeclarationsBeforeEarlyExit, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn moves_a_declaration_read_only_after_the_guard() {
        // The documentation's first, second and third cases: a literal moved by the fix, a call
        // reported only.
        let code = "function foo(bar) {\n\tconst result = 1;\n\tif (!bar) {\n\t\treturn;\n\t}\n\tconsole.log(result);\n}\n";
        assert_eq!(
            check(code),
            (
                vec![(2, 8)],
                "function foo(bar) {\n\tif (!bar) {\n\t\treturn;\n\t}\n\tconst result = 1;\n\tconsole.log(result);\n}\n".to_owned()
            )
        );
        let thrown = "function foo(bar) {\n\tconst result = 1;\n\tif (!bar) {\n\t\tthrow new Error();\n\t}\n\tconsole.log(result);\n}\n";
        assert_eq!(check(thrown).0, [(2, 8)]);
        let called = "function foo(bar) {\n\tconst result = getResult();\n\tif (!bar) {\n\t\treturn;\n\t}\n\tconsole.log(result);\n}\n";
        assert_eq!(check(called), (vec![(2, 8)], called.to_owned()));
    }

    #[test]
    fn leaves_declarations_read_before_unread_or_unsafe_to_move() {
        for code in [
            "function foo(bar) {\n\tif (!bar) {\n\t\treturn;\n\t}\n\tconst result = 1;\n\tconsole.log(result);\n}",
            "function f(a) { const b = 1; if (a) { return b; } g(b); }",
            "function f(a) { const b = 1; if (a) { return; } }",
            "function f(a) { const b = g(); h(); if (a) { return; } k(b); }",
            "function f(a) { const b = a.pop(); if (a.length) { return; } k(b); }",
            "async function f(a) { const b = await g(); if (a) { return; } k(b); }",
            "function f(a) { const b = useMemo(g); if (a) { return; } k(b); }",
            "function f(a) { const b = React.useState(1); if (a) { return; } k(b); }",
            "function f(a) { const { b } = g(); if (a) { return; } k(b); }",
            "function f(a) { const b = 1; if (a) { return; } else { return; } k(b); }",
            "function f(a) { const b = 1; if (a) { g(); } k(b); }",
            "function f(a) { const b = 1; if (b) { return; } k(b); }",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
    }

    #[test]
    fn reports_without_fix_when_comments_or_other_declarators_stand_around() {
        for code in [
            "function f(a) {\n  // why\n  const b = 1;\n  if (a) { return; }\n  k(b);\n}",
            "function f(a) {\n  const b = 1;\n  if (a) { return; } // why\n  k(b);\n}",
            "function f(a) {\n  const b = 1, c = 2;\n  if (a) { return; }\n  k(b, c);\n}",
        ] {
            let (places, result) = check(code);
            assert!(!places.is_empty(), "{code}");
            assert_eq!(result, code);
        }
        // A literal two statements above the guard is reported, not fixed; a type query is no read.
        let (places, _) =
            check("function f(a) { const b = 1; type T = typeof b; if (a) { throw e; } k(b); }");
        assert_eq!(places, [(1, 23)]);
    }
}
