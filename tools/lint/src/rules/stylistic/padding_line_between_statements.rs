//! `@stylistic/padding-line-between-statements`: blank lines between statements where the
//! project's configuration asks one (`@stylistic` 5.10.0,
//! `docs/rules/padding-line-between-statements.md`; the project's entries, moved from
//! `PADDING_LINES` of `eslint/settings/`), the last matching entry deciding: after a `const` or
//! `let` (another one may follow), before a `const` or `let` that follows an expression, after a
//! block-like statement, a multi-line expression, a directive or an import (imports may follow
//! each other), and before a block-like statement, a multi-line expression, a `return` or an
//! export (exports may follow each other).
//! The statements of a program, a block, a function's body, a static block, a module block and a
//! `case`, and the cases of a `switch`, are read as the plugin reads them; a type literal's
//! method signatures, which oxlint's `method-signature-style` forbids, are not. A line of
//! comments between two statements is no blank line. Autofixed: a line break written after the
//! first statement and the comments on its last line.

use std::collections::HashSet;

use oxc_ast::AstKind;
use oxc_ast::ast::{
    ChainElement, Directive, Expression, Statement, SwitchCase, VariableDeclarationKind,
};
use oxc_span::{GetSpan, Span};

use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

use super::comments::lines;

/// The rule.
pub struct PaddingLineBetweenStatements;

/// The plugin's message.
const MESSAGE: &str = "Expected blank line before this statement.";

/// One item of a list the rule reads: a directive, a statement or a `case`.
#[derive(Clone, Copy)]
enum Item<'s, 'a> {
    /// A directive of a prologue.
    Directive(&'s Directive<'a>),
    /// A statement.
    Statement(&'s Statement<'a>),
    /// A clause of a `switch`.
    Case(&'s SwitchCase<'a>),
}

impl Item<'_, '_> {
    /// Its range.
    fn span(&self) -> Span {
        match self {
            Self::Directive(directive) => directive.span,
            Self::Statement(statement) => statement.span(),
            Self::Case(case) => case.span,
        }
    }
}

/// The types of the configuration an item has, as a set of the flags below.
#[derive(Clone, Copy, Default)]
struct Kinds(u16);

impl Kinds {
    /// A `const` declaration.
    const CONSTANT: u16 = 1;
    /// A `let` declaration.
    const VARIABLE: u16 = 1 << 1;
    /// An expression statement.
    const EXPRESSION: u16 = 1 << 2;
    /// An expression statement over several lines.
    const MULTILINE_EXPRESSION: u16 = 1 << 3;
    /// A statement ending with a block's brace, a `do…while` or an IIFE.
    const BLOCK_LIKE: u16 = 1 << 4;
    /// A directive.
    const DIRECTIVE: u16 = 1 << 5;
    /// An `import`.
    const IMPORT: u16 = 1 << 6;
    /// A `return`.
    const RETURN: u16 = 1 << 7;
    /// An export.
    const EXPORT: u16 = 1 << 8;

    /// Whether the item has any of these types.
    fn any(self, kinds: u16) -> bool {
        self.0 & kinds != 0
    }

    /// The set with a type added when a condition holds.
    fn with(self, kind: u16, condition: bool) -> Self {
        if condition { Self(self.0 | kind) } else { self }
    }
}

/// Whether a blank line must separate two items, the configuration read from its last entry.
fn needs_blank_line(prev: Kinds, next: Kinds) -> bool {
    let declares = Kinds::CONSTANT | Kinds::VARIABLE;
    if (prev.any(Kinds::EXPORT) && next.any(Kinds::EXPORT))
        || (prev.any(Kinds::IMPORT) && next.any(Kinds::IMPORT))
    {
        return false;
    }
    if next.any(Kinds::BLOCK_LIKE | Kinds::MULTILINE_EXPRESSION | Kinds::RETURN | Kinds::EXPORT) {
        return true;
    }
    if prev.any(Kinds::BLOCK_LIKE | Kinds::MULTILINE_EXPRESSION | Kinds::DIRECTIVE | Kinds::IMPORT)
    {
        return true;
    }
    if prev.any(Kinds::EXPRESSION) && next.any(declares) {
        return true;
    }
    prev.any(declares) && !next.any(declares)
}

/// Whether an expression statement calls a function written in place.
fn is_iife(expression: &Expression) -> bool {
    let unwrapped = match expression {
        Expression::UnaryExpression(unary) => &unary.argument,
        other => other,
    };
    let callee = match unwrapped {
        Expression::CallExpression(call) => &call.callee,
        Expression::ChainExpression(chain) => match &chain.expression {
            ChainElement::CallExpression(call) => &call.callee,
            _ => return false,
        },
        _ => return false,
    };
    let mut callee = callee;
    while let Expression::SequenceExpression(sequence) = callee {
        let Some(last) = sequence.expressions.last() else {
            return false;
        };
        callee = last;
    }
    matches!(
        callee,
        Expression::FunctionExpression(_) | Expression::ArrowFunctionExpression(_)
    )
}

/// Whether the last token of a range, a `;` left out, is the brace closing a block, a function's
/// body or a `switch`.
fn ends_with_block(source: &Source, span: Span, block_ends: &HashSet<u32>) -> bool {
    let text = &source.text[span.start as usize..span.end as usize];
    let trimmed = text.trim_end();
    let trimmed = trimmed.strip_suffix(';').unwrap_or(trimmed).trim_end();

    trimmed.ends_with('}')
        && block_ends.contains(&(span.start + u32::try_from(trimmed.len()).unwrap_or(0)))
}

/// The statement a label holds, at any depth.
fn unlabeled<'s, 'a>(statement: &'s Statement<'a>) -> &'s Statement<'a> {
    match statement {
        Statement::LabeledStatement(labeled) => unlabeled(&labeled.body),
        other => other,
    }
}

/// The types of an item.
fn kinds(source: &Source, item: Item, block_ends: &HashSet<u32>) -> Kinds {
    let span = item.span();
    let block_like = ends_with_block(source, span, block_ends);
    let statement = match item {
        Item::Directive(_) => return Kinds(Kinds::DIRECTIVE),
        Item::Case(_) => return Kinds::default().with(Kinds::BLOCK_LIKE, block_like),
        Item::Statement(statement) => unlabeled(statement),
    };
    let declared = |kind: VariableDeclarationKind| matches!(statement, Statement::VariableDeclaration(each) if each.kind == kind && !each.declare);
    let expression = matches!(statement, Statement::ExpressionStatement(_));
    let inner = statement.span();
    let multiline = source.line(inner.start) != source.line(inner.end);

    Kinds::default()
        .with(Kinds::CONSTANT, declared(VariableDeclarationKind::Const))
        .with(Kinds::VARIABLE, declared(VariableDeclarationKind::Let))
        .with(Kinds::EXPRESSION, expression)
        .with(Kinds::MULTILINE_EXPRESSION, expression && multiline)
        .with(
            Kinds::BLOCK_LIKE,
            block_like
                || ends_with_block(source, inner, block_ends)
                || matches!(statement, Statement::DoWhileStatement(each) if matches!(each.body, Statement::BlockStatement(_)))
                || matches!(statement, Statement::ExpressionStatement(each) if is_iife(&each.expression)),
        )
        .with(Kinds::IMPORT, matches!(statement, Statement::ImportDeclaration(_)))
        .with(Kinds::RETURN, matches!(statement, Statement::ReturnStatement(_)))
        .with(
            Kinds::EXPORT,
            matches!(
                statement,
                Statement::ExportAllDeclaration(_)
                    | Statement::ExportDefaultDeclaration(_)
                    | Statement::ExportDeclaration(_)
                    | Statement::ExportNamedDeclaration(_)
                    | Statement::ExportFromDeclaration(_)
            ),
        )
}

/// The number of line breaks in a text.
fn line_breaks(text: &str) -> usize {
    lines(text).len() - 1
}

/// The ranges of the comments between two offsets.
fn comments_between(source: &Source, start: u32, end: u32) -> Vec<Span> {
    let comments = &source.program.comments;
    let first = comments.partition_point(|comment| comment.span.start < start);

    comments[first..]
        .iter()
        .take_while(|comment| comment.span.end <= end)
        .map(|comment| comment.span)
        .collect()
}

/// Whether a blank line lies between two items: two line breaks between the first item, the
/// comments and the second item, at one place.
fn has_blank_line(source: &Source, prev: Span, next: Span) -> bool {
    let mut cursor = prev.end;

    for comment in comments_between(source, prev.end, next.start)
        .into_iter()
        .chain(std::iter::once(Span::new(next.start, next.start)))
    {
        if line_breaks(&source.text[cursor as usize..comment.start as usize]) >= 2 {
            return true;
        }
        cursor = comment.end;
    }
    false
}

/// The fix: a line break after the first item and the comments that end its line, two when the
/// second item starts on that line.
fn fix(source: &Source, prev: Span, next: Span) -> Fix {
    let mut cursor = prev.end;
    let mut following = next.start;

    for comment in comments_between(source, prev.end, next.start) {
        if line_breaks(&source.text[cursor as usize..comment.start as usize]) > 0 {
            following = comment.start;
            break;
        }
        cursor = comment.end;
    }
    let same_line = line_breaks(&source.text[cursor as usize..following as usize]) == 0;

    Fix {
        start: cursor,
        end: cursor,
        text: if same_line { "\n\n" } else { "\n" }.to_owned(),
    }
}

/// The items of each list the rule reads.
fn lists<'s, 'a>(source: &'s Source<'s, 'a>) -> Vec<Vec<Item<'s, 'a>>> {
    let statements = |directives: &'s [Directive<'a>], body: &'s [Statement<'a>]| {
        directives
            .iter()
            .map(Item::Directive)
            .chain(
                body.iter()
                    .filter(|each| !matches!(each, Statement::TSExportAssignment(_)))
                    .map(Item::Statement),
            )
            .collect::<Vec<_>>()
    };

    source
        .semantic
        .nodes()
        .iter()
        .filter_map(|node| match node.kind() {
            AstKind::Program(program) => Some(statements(&program.directives, &program.body)),
            AstKind::BlockStatement(block) => Some(statements(&[], &block.body)),
            AstKind::FunctionBody(body) => Some(statements(&body.directives, &body.statements)),
            AstKind::StaticBlock(block) => Some(statements(&[], &block.body)),
            AstKind::TSModuleBlock(block) => Some(statements(&block.directives, &block.body)),
            AstKind::SwitchCase(case) => Some(statements(&[], &case.consequent)),
            AstKind::SwitchStatement(switch) => Some(switch.cases.iter().map(Item::Case).collect()),
            _ => None,
        })
        .collect()
}

impl Rule for PaddingLineBetweenStatements {
    fn name(&self) -> &'static str {
        "@stylistic/padding-line-between-statements"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let block_ends: HashSet<u32> = source
            .semantic
            .nodes()
            .iter()
            .filter_map(|node| match node.kind() {
                AstKind::BlockStatement(block) => Some(block.span.end),
                AstKind::FunctionBody(body) => Some(body.span.end),
                AstKind::SwitchStatement(switch) => Some(switch.span.end),
                _ => None,
            })
            .collect();

        for list in lists(source) {
            for pair in list.windows(2) {
                let (prev, next) = (pair[0], pair[1]);
                let (before, after) = (prev.span(), next.span());
                if needs_blank_line(
                    kinds(source, prev, &block_ends),
                    kinds(source, next, &block_ends),
                ) && !has_blank_line(source, before, after)
                {
                    findings.push(source.fixed(
                        self.name(),
                        after.start,
                        MESSAGE.to_owned(),
                        fix(source, before, after),
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PaddingLineBetweenStatements;
    use crate::testing::{fixed, lint};

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        let mut found: Vec<(usize, usize)> =
            lint(&PaddingLineBetweenStatements, "src/a/b.ts", code)
                .iter()
                .map(|each| (each.line, each.column))
                .collect();
        found.sort_unstable();
        found
    }

    /// The code with the findings' fixes applied.
    fn fix(code: &str) -> String {
        fixed(
            code,
            &lint(&PaddingLineBetweenStatements, "src/a/b.ts", code),
        )
    }

    #[test]
    fn asks_a_blank_line_before_a_return_and_after_declarations() {
        // The documentation's cases that the project's configuration shares: a `return` after
        // a statement, a call after a `const` or a `let`, a statement after a directive.
        assert_eq!(
            places("function foo() {\n    bar();\n    return;\n}"),
            [(3, 5)]
        );
        assert_eq!(
            places(
                "function foo2() {\n    let a = 0;\n    bar();\n}\nfunction foo3() {\n    const a = 0;\n    bar();\n}"
            ),
            [(3, 5), (5, 1), (7, 5)]
        );
        assert_eq!(places("\"use strict\";\nfoo();"), [(2, 1)]);
        assert_eq!(
            fix("function foo() {\n    bar();\n    return;\n}"),
            "function foo() {\n    bar();\n\n    return;\n}"
        );
    }

    #[test]
    fn reads_blocks_expressions_imports_and_exports() {
        // A declaration after an expression; a multi-line expression; an import, then code; a
        // block-like statement on either side; an export after a statement.
        assert_eq!(places("f();\nconst a = 1;\n\ng(a);"), [(2, 1)]);
        assert_eq!(places("f();\ng(\n  1,\n);\nh();"), [(2, 1), (5, 1)]);
        assert_eq!(
            places("import a from \"a\";\nimport b from \"b\";\nf(a, b);"),
            [(3, 1)]
        );
        assert_eq!(places("if (a) {\n}\nf();\nif (b) {\n}"), [(3, 1), (4, 1)]);
        assert_eq!(
            places("const f = () => {\n};\n\ng();"),
            Vec::<(usize, usize)>::new()
        );
        assert_eq!(places("f();\nconst g = () => {\n};"), [(2, 1)]);
        assert_eq!(
            places("f();\nexport const a = 1;\nexport const b = 2;"),
            [(2, 1)]
        );
        assert_eq!(places("do {\n} while (a);\nf();"), [(3, 1)]);
        // A label is read through: here an expression, then a declaration.
        assert_eq!(places("x: f();\nconst a = 1;"), [(2, 1)]);
        assert_eq!(places("(() => {\n})();\nf();"), [(3, 1)]);
        assert_eq!(
            places("switch (a) {\n  case 1: {\n    f();\n  }\n  case 2:\n    g();\n}"),
            [(5, 3)]
        );
        // An object's brace is no block: a one-line call follows freely.
        assert_eq!(places("f({});\ng();"), Vec::<(usize, usize)>::new());
    }

    #[test]
    fn counts_no_comment_line_as_blank_and_fixes_after_a_trailing_comment() {
        assert_eq!(places("const a = 1;\n// note\nf(a);"), [(3, 1)]);
        assert_eq!(
            places("const a = 1;\n// note\n\nf(a);"),
            Vec::<(usize, usize)>::new()
        );
        assert_eq!(
            places("const a = 1;\n\n// note\nf(a);"),
            Vec::<(usize, usize)>::new()
        );
        assert_eq!(
            fix("const a = 1; // x\nf(a);"),
            "const a = 1; // x\n\nf(a);"
        );
        assert_eq!(
            fix("const a = 1;\n// note\nf(a);"),
            "const a = 1;\n\n// note\nf(a);"
        );
        assert_eq!(fix("const a = 1; f(a);"), "const a = 1;\n\n f(a);");
    }

    #[test]
    fn lets_declarations_imports_and_exports_follow_each_other() {
        for code in [
            "const a = 1;\nlet b = 2;\nconst c = 3;",
            "import a from \"a\";\nimport b from \"b\";",
            "export const a = 1;\nexport const b = 2;",
            "f();\ng();",
            "const a = 1;\n\nf(a);\ng();",
            "function f() {\n  return 1;\n}",
            "label: for (;;) {\n}",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
