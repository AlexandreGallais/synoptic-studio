//! `unicorn/prefer-smaller-scope`: a `const` holding a primitive literal or a template without
//! substitution, declared right before an `if` and read only inside one braced branch of it,
//! moves into that branch (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-smaller-scope.md`).
//! The plugin's other case, a `let` declared bare and assigned once in a nested block, cannot
//! occur here: `functional/no-let` forbids `let` in every file. Autofixed unless the declaration
//! has a written type or a comment stands by it.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    BindingPattern, BlockStatement, Expression, IfStatement, Statement, VariableDeclarationKind,
};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use crate::expressions::{has_comment_in, siblings, text};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferSmallerScope;

/// Whether a value is a primitive literal, a regular expression aside, or a template without
/// substitution.
fn is_primitive_literal(value: &Expression) -> bool {
    match value {
        Expression::RegExpLiteral(_) => false,
        Expression::TemplateLiteral(template) => template.expressions.is_empty(),
        other => other.is_literal(),
    }
}

/// Whether a comment touches a range: right before it, right after it, or inside.
fn comment_by(source: &Source, span: Span) -> bool {
    has_comment_in(source, span)
        || source.program.comments.iter().any(|comment| {
            (comment.span.end <= span.start
                && text(source, Span::new(comment.span.end, span.start))
                    .trim()
                    .is_empty())
                || (comment.span.start >= span.end
                    && text(source, Span::new(span.end, comment.span.start))
                        .trim()
                        .is_empty())
        })
}

impl Rule for PreferSmallerScope {
    fn name(&self) -> &'static str {
        "unicorn/prefer-smaller-scope"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::VariableDeclaration(declaration) = node.kind() else {
            return;
        };
        let in_block = matches!(
            source.semantic.nodes().parent_kind(node.id()),
            AstKind::Program(_) | AstKind::BlockStatement(_) | AstKind::FunctionBody(_)
        );
        if !in_block
            || declaration.declare
            || declaration.kind == VariableDeclarationKind::Var
            || declaration.declarations.len() != 1
        {
            return;
        }
        let declarator = &declaration.declarations[0];
        let (BindingPattern::BindingIdentifier(name), Some(value)) =
            (&declarator.id, &declarator.init)
        else {
            return;
        };
        if !is_primitive_literal(value) {
            return;
        }
        let Some((list, index)) = siblings(source, node) else {
            return;
        };
        let Some(Statement::IfStatement(next)) = list.get(index + 1) else {
            return;
        };
        let Some(symbol) = name.symbol_id.get() else {
            return;
        };
        let Some(block) = reading_branch(source, symbol, next) else {
            return;
        };
        let message = format!("Move `{}` into the block where it is used.", name.name);
        let Some(first) = block.body.first() else {
            return;
        };
        let typed = declarator.type_annotation.is_some();
        let before_first = source.program.comments.iter().any(|comment| {
            comment.span.end <= first.span().start
                && comment.span.start > block.span.start
                && text(source, Span::new(comment.span.end, first.span().start))
                    .trim()
                    .is_empty()
        });
        if typed || comment_by(source, declaration.span) || before_first {
            findings.push(source.finding(self.name(), name.span.start, message));
            return;
        }
        let declared = text(source, declaration.span);
        let semicolon = if declared.ends_with(';') { "" } else { ";" };
        let separator = text(source, Span::new(block.span.start + 1, first.span().start));
        let separator = if separator.is_empty() { " " } else { separator };
        let mut edits = vec![
            (declaration.span.start, next.span.start, String::new()),
            (
                first.span().start,
                first.span().start,
                format!("{declared}{semicolon}{separator}"),
            ),
        ];
        let replaced = Span::new(declaration.span.start, next.span.end);
        let fix = Fix {
            start: replaced.start,
            end: replaced.end,
            text: crate::expressions::edited(source, replaced, &mut edits),
        };
        findings.push(source.fixed(self.name(), name.span.start, message, fix));
    }
}

/// The braced branch of an `if` holding every read of a variable, none of them inside a
/// function, a class or a static block of it.
fn reading_branch<'a>(
    source: &Source<'_, 'a>,
    symbol: oxc_semantic::SymbolId,
    next: &'a IfStatement<'a>,
) -> Option<&'a BlockStatement<'a>> {
    let nodes = source.semantic.nodes();
    let reads: Vec<oxc_semantic::NodeId> = source
        .semantic
        .scoping()
        .get_resolved_references(symbol)
        .map(oxc_semantic::Reference::node_id)
        .collect();
    if reads.is_empty() {
        return None;
    }
    let inside = |block: &BlockStatement, read: oxc_semantic::NodeId| {
        let at = nodes.kind(read).span();
        at.start >= block.span.start
            && at.end <= block.span.end
            && !nodes
                .ancestor_kinds(read)
                .take_while(|kind| kind.span() != block.span)
                .any(|kind| {
                    matches!(
                        kind,
                        AstKind::Function(_)
                            | AstKind::ArrowFunctionExpression(_)
                            | AstKind::Class(_)
                            | AstKind::StaticBlock(_)
                    )
                })
    };
    std::iter::once(&next.consequent)
        .chain(next.alternate.as_ref())
        .filter_map(|branch| match branch {
            Statement::BlockStatement(block) => Some(block.as_ref()),
            _ => None,
        })
        .find(|block| reads.iter().all(|read| inside(block, *read)))
}

#[cfg(test)]
mod tests {
    use super::PreferSmallerScope;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferSmallerScope, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn moves_a_literal_into_the_branch_reading_it() {
        // The documentation's third case; its first two need `let`, which no file can hold.
        assert_eq!(
            check("function foo(bar) {\n\tconst result = 1;\n\tif (bar) {\n\t\tconsole.log(result);\n\t}\n}"),
            (
                vec![(2, 8)],
                "function foo(bar) {\n\tif (bar) {\n\t\tconst result = 1;\n\t\tconsole.log(result);\n\t}\n}".to_owned()
            )
        );
        // The `else` branch, a template, a missing semicolon.
        assert_eq!(
            check("const a = `x`\nif (b) { g(); } else { h(a); }").1,
            "if (b) { g(); } else { const a = `x`; h(a); }"
        );
    }

    #[test]
    fn leaves_wider_reads_and_other_values() {
        for code in [
            "function foo(bar) {\n\tif (bar) {\n\t\tconst result = 1;\n\t\tconsole.log(result);\n\t}\n}",
            "const a = 1; if (b) { g(a); } h(a);",
            "const a = 1; if (b) { g(a); } else { h(a); }",
            "const a = g(); if (b) { h(a); }",
            "const a = /x/; if (b) { h(a); }",
            "const a = -1; if (b) { h(a); }",
            "const a = 1; g(); if (b) { h(a); }",
            "const a = 1; if (b) h(a);",
            "const a = 1; if (b) { const f = () => a; f(); }",
            "const a = 1; if (b) {}",
            "const a = 1; if (b) { g(); }",
            "const a = 1, c = 2; if (b) { h(a, c); }",
            "const { a } = o; if (b) { h(a); }",
            "const a = 1; if (b) { h(a); }\nexport { a };",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
    }

    #[test]
    fn reports_without_fix_when_typed_or_commented() {
        for code in [
            "const a: number = 1;\nif (b) { h(a); }",
            "// why\nconst a = 1;\nif (b) { h(a); }",
            "const a = 1;\nif (b) {\n  // first\n  h(a);\n}",
        ] {
            let (places, result) = check(code);
            assert_eq!(places.len(), 1, "{code}");
            assert_eq!(result, code);
        }
    }
}
