//! `local/kind-limits`: lines, statements, `McCabe` complexity and depth of a top-level function,
//! bounded by its kind (ADR-0011). Complexity is decision points plus one, nested callbacks
//! included (`REF-NIST-500-235`); depth counts nested blocks as the core `max-depth` rule does;
//! lines are those holding a token, blank and comment-only lines left out.

use oxc_ast::AstKind;
use oxc_ast::ast::{AssignmentOperator, Function, FunctionType};
use oxc_ast_visit::Visit;
use oxc_span::GetSpan;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::settings::KIND_LIMITS;
use crate::source::Source;

/// The rule.
pub struct KindLimits;

/// A function's measures.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Metrics {
    /// Decision points plus one.
    pub complexity: usize,
    /// Most nested blocks.
    pub depth: usize,
    /// Lines holding a token.
    pub lines: usize,
    /// Statements, blocks left out.
    pub statements: usize,
}

/// The walk that counts decision points, statements and nesting below a node.
#[derive(Default)]
struct Measure {
    decisions: usize,
    statements: usize,
    depth: usize,
    deepest: usize,
}

/// Whether a node adds a path: a branch, a loop, `catch`, a `case` with a test, `&&`, `||`, `??`
/// and their assignments.
fn is_decision(kind: &AstKind) -> bool {
    match kind {
        AstKind::CatchClause(_)
        | AstKind::ConditionalExpression(_)
        | AstKind::DoWhileStatement(_)
        | AstKind::ForInStatement(_)
        | AstKind::ForOfStatement(_)
        | AstKind::ForStatement(_)
        | AstKind::IfStatement(_)
        | AstKind::LogicalExpression(_)
        | AstKind::WhileStatement(_) => true,
        AstKind::SwitchCase(case) => case.test.is_some(),
        AstKind::AssignmentExpression(assignment) => matches!(
            assignment.operator,
            AssignmentOperator::LogicalAnd
                | AssignmentOperator::LogicalOr
                | AssignmentOperator::LogicalNullish
        ),
        _ => false,
    }
}

/// Whether a node opens a nesting level, as the core `max-depth` rule counts them.
fn is_nesting(kind: &AstKind) -> bool {
    matches!(
        kind,
        AstKind::DoWhileStatement(_)
            | AstKind::ForInStatement(_)
            | AstKind::ForOfStatement(_)
            | AstKind::ForStatement(_)
            | AstKind::IfStatement(_)
            | AstKind::SwitchStatement(_)
            | AstKind::TryStatement(_)
            | AstKind::WhileStatement(_)
    )
}

/// Whether a node is a statement or a declaration, a block left out: the nodes `ESTree` names
/// `…Statement` or `…Declaration`.
fn is_statement(kind: &AstKind) -> bool {
    match kind {
        AstKind::Function(function) => function.r#type == FunctionType::FunctionDeclaration,
        AstKind::Class(class) => class.is_declaration(),
        AstKind::Directive(_)
        | AstKind::EmptyStatement(_)
        | AstKind::ExpressionStatement(_)
        | AstKind::IfStatement(_)
        | AstKind::DoWhileStatement(_)
        | AstKind::WhileStatement(_)
        | AstKind::ForStatement(_)
        | AstKind::ForInStatement(_)
        | AstKind::ForOfStatement(_)
        | AstKind::ContinueStatement(_)
        | AstKind::BreakStatement(_)
        | AstKind::ReturnStatement(_)
        | AstKind::WithStatement(_)
        | AstKind::SwitchStatement(_)
        | AstKind::LabeledStatement(_)
        | AstKind::ThrowStatement(_)
        | AstKind::TryStatement(_)
        | AstKind::DebuggerStatement(_)
        | AstKind::VariableDeclaration(_)
        | AstKind::TSTypeAliasDeclaration(_)
        | AstKind::TSInterfaceDeclaration(_)
        | AstKind::TSEnumDeclaration(_)
        | AstKind::TSImportEqualsDeclaration(_) => true,
        _ => false,
    }
}

impl<'a> Visit<'a> for Measure {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        self.decisions += usize::from(is_decision(&kind));
        self.statements += usize::from(is_statement(&kind));
        if is_nesting(&kind) {
            self.depth += 1;
            self.deepest = self.deepest.max(self.depth);
        }
    }

    fn leave_node(&mut self, kind: AstKind<'a>) {
        if is_nesting(&kind) {
            self.depth -= 1;
        }
    }
}

/// The measures of a function: its body walked, its tokens' lines counted.
#[must_use]
pub fn metrics(source: &Source, function: &Function) -> Metrics {
    let mut measure = Measure::default();
    if let Some(body) = &function.body {
        measure.visit_function_body(body);
    }
    let span = function.span();
    let first = source
        .tokens
        .partition_point(|token| token.start() < span.start);
    let mut lines: Vec<usize> = source.tokens[first..]
        .iter()
        .take_while(|token| token.end() <= span.end)
        .flat_map(|token| source.line(token.start())..=source.line(token.end()))
        .collect();
    lines.dedup();

    Metrics {
        complexity: measure.decisions + 1,
        depth: measure.deepest,
        lines: lines.len(),
        statements: measure.statements,
    }
}

impl Rule for KindLimits {
    fn name(&self) -> &'static str {
        "local/kind-limits"
    }

    fn needs_tokens(&self) -> bool {
        true
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for function in source.top_level_functions() {
            let Some(kind) = source.kind(&function) else {
                continue;
            };
            let Some(limit) = KIND_LIMITS.iter().find(|each| each.kind == kind) else {
                continue;
            };
            let measured = metrics(source, function.function);
            let bounds = [
                ("complexity", measured.complexity, limit.max_complexity),
                ("depth", measured.depth, limit.max_depth),
                ("lines", measured.lines, limit.max_lines),
                ("statements", measured.statements, limit.max_statements),
            ];
            for (metric, actual, max) in bounds.into_iter().filter(|(_, actual, max)| actual > max)
            {
                let message = format!(
                    "`{kind}` function {metric} is {actual} (max {max}). Extract named steps (Composed Method)."
                );
                findings.push(source.finding(self.name(), Source::name_offset(&function), message));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{KindLimits, Metrics, metrics};
    use crate::project::Project;
    use crate::settings::REFERENCE_SOURCES;
    use crate::source::Source;
    use crate::testing::{NONE, lint, messages};

    /// The messages on the `math` function of a piece of code, `math`'s limits being 3 for
    /// complexity, 1 for depth, 10 lines and 4 statements.
    fn found(code: &str) -> Vec<String> {
        messages(&lint(&KindLimits, "src/a.ts", code))
    }

    #[test]
    fn counts_decisions_with_logical_operators() {
        // Complexity 4: base 1 + `if` + `&&` + `||`.
        assert_eq!(
            found(
                "/** @kind math */\nfunction f(a: boolean, b: boolean): number {\n  if (a && b || a) return 1;\n  return 0;\n}"
            ),
            ["`math` function complexity is 4 (max 3). Extract named steps (Composed Method)."]
        );
    }

    #[test]
    fn counts_conditionals_nullish_and_cases_but_not_the_default() {
        // Complexity 5: base 1 + `?:` + `??` + `case 1` + `case 2`; the default adds nothing.
        assert_eq!(
            found(
                "/** @kind math */\nfunction f(a?: number): number {\n  switch (a ?? 0) { case 1: return a ? 1 : 2; case 2: default: return 0; }\n}"
            ),
            ["`math` function complexity is 5 (max 3). Extract named steps (Composed Method)."]
        );
    }

    #[test]
    fn counts_catch_loops_and_logical_assignments() {
        // Complexity 4: base 1 + `catch` + `for…of` + `||=`; depth 2: `for…of` inside `try`.
        assert_eq!(
            found(
                "/** @kind math */\nfunction f(o: { a: number }): void {\n  try { for (const x of []) { o.a ||= x; } } catch { }\n}"
            ),
            [
                "`math` function complexity is 4 (max 3). Extract named steps (Composed Method).",
                "`math` function depth is 2 (max 1). Extract named steps (Composed Method).",
            ]
        );
    }

    #[test]
    fn counts_depth_of_nested_blocks() {
        // Depth 2: `if` inside `if`.
        assert_eq!(
            found("/** @kind math */\nfunction f(a: boolean): void {\n  if (a) { if (a) { } }\n}"),
            ["`math` function depth is 2 (max 1). Extract named steps (Composed Method)."]
        );
    }

    #[test]
    fn counts_lines_holding_code_only() {
        // 11 code lines (2, 7 to 16): the blank line and the comment-only lines 3 to 6 are skipped;
        // the template literal's empty line 8 holds its token.
        let code = "/** @kind math */\nfunction f(): string {\n\n  // note\n  /* block\n  */\n  return `a\n\nb`\n    + `c`\n    + `d`\n    + `e`\n    + `f`\n    + `g`\n    + `h`;\n}";
        assert_eq!(
            found(code),
            ["`math` function lines is 11 (max 10). Extract named steps (Composed Method)."]
        );
    }

    #[test]
    fn counts_statements_nested_functions_and_declarations() {
        // 5 statements: two declarations, an inner function, its return, and the return.
        assert_eq!(
            found(
                "/** @kind math */\nfunction f(): number {\n  const a = 1;\n  type T = number;\n  function g(): T { return a; }\n  return g();\n}"
            ),
            ["`math` function statements is 5 (max 4). Extract named steps (Composed Method)."]
        );
    }

    /// The measures of the first top-level function of a piece of code.
    fn measured(code: &str) -> Metrics {
        let allocator = oxc_allocator::Allocator::default();
        let parsed = oxc_parser::Parser::new(&allocator, code, oxc_span::SourceType::ts())
            .with_config(oxc_parser::config::RuntimeParserConfig::new(true))
            .parse();
        let semantic = oxc_semantic::SemanticBuilder::new()
            .build(&parsed.program)
            .semantic;
        let project = Project::new(std::path::Path::new("/project"), REFERENCE_SOURCES);
        let source = Source::new(
            "src/a.ts",
            &parsed.program,
            &semantic,
            &parsed.tokens,
            &project,
        );
        let functions = source.top_level_functions();

        metrics(&source, functions[0].function)
    }

    #[test]
    fn measures_the_cases_of_the_eslint_rule() {
        // Complexity 3: base 1 + `if` + `&&`.
        let both = measured(
            "function f(a: boolean, b: boolean): number {\n  if (a && b) return 1;\n  return 0;\n}",
        );
        // Complexity 4: base 1 + `?:` + `??` + `case 1` (the default case adds nothing).
        let switch = measured(
            "function f(a?: number): number {\n  switch (a ?? 0) { case 1: return a ? 1 : 2; default: return 0; }\n}",
        );
        // Depth 2: `if` inside `if`.
        let nested = measured("function f(a: boolean): void {\n  if (a) { if (a) { } }\n}");
        // 5 code lines: the blank line and the comment-only line are skipped.
        let lines = measured(
            "function f(): number {\n\n  // note\n  const a = 1;\n  const b = 2;\n  return a + b;\n}",
        );

        assert_eq!(both.complexity, 3);
        assert_eq!(switch.complexity, 4);
        assert_eq!(nested.depth, 2);
        assert_eq!(lines.lines, 5);
        // 3 statements: two declarations and a return.
        assert_eq!(lines.statements, 3);
        // A directive is a statement, as `ESTree` writes it.
        assert_eq!(
            measured("function f(): void {\n  \"use strict\";\n}").statements,
            1
        );
        assert_eq!(
            measured("function f(): void {}"),
            Metrics {
                complexity: 1,
                depth: 0,
                lines: 1,
                statements: 0
            }
        );
    }

    #[test]
    fn leaves_a_function_within_limits_or_without_a_known_kind() {
        assert_eq!(
            found(
                "/** @kind math */\nfunction f(a: number): number {\n  return a > 0 ? a : -a;\n}"
            ),
            NONE
        );
        assert_eq!(
            found("/** @kind procedure */\nfunction f(a: boolean): void {\n  if (a) { }\n}"),
            NONE
        );
        assert_eq!(
            found("function f(a: boolean): void {\n  if (a) { if (a) { } }\n}"),
            NONE
        );
        assert_eq!(
            found("/** @kind magic */\nfunction f(a: boolean): void {\n  if (a) { if (a) { } }\n}"),
            NONE
        );
    }
}
