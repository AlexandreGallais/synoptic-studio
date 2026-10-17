//! `functional/no-loop-statements`: no `for`, `for…in`, `for…of`, `while` nor `do…while`
//! (`eslint-plugin-functional` 10.0.1, `docs/rules/no-loop-statements.md`). No fix.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoLoopStatements;

/// The plugin's message.
const MESSAGE: &str = "Unexpected loop, use map or reduce instead.";

impl Rule for NoLoopStatements {
    fn name(&self) -> &'static str {
        "functional/no-loop-statements"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let start = match node.kind() {
            AstKind::ForStatement(each) => each.span.start,
            AstKind::ForInStatement(each) => each.span.start,
            AstKind::ForOfStatement(each) => each.span.start,
            AstKind::WhileStatement(each) => each.span.start,
            AstKind::DoWhileStatement(each) => each.span.start,
            _ => return,
        };
        findings.push(source.finding(self.name(), start, MESSAGE.to_owned()));
    }
}

#[cfg(test)]
mod tests {
    use super::NoLoopStatements;
    use crate::testing::lint;

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&NoLoopStatements, "src/a/b.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_each_kind_of_loop() {
        // The documentation's case, then each other loop.
        let documented = "const numbers = [1, 2, 3];\nconst double = [];\nfor (let i = 0; i < numbers.length; i++) {\n  double[i] = numbers[i] * 2;\n}";

        assert_eq!(places(documented), [(3, 1)]);
        assert_eq!(
            places(
                "const numbers = [1, 2, 3];\nlet sum = 0;\nfor (const number of numbers) {\n  sum += number;\n}"
            ),
            [(3, 1)]
        );
        assert_eq!(places("for (const k in o) {}"), [(1, 1)]);
        assert_eq!(places("for (const x of xs) {}"), [(1, 1)]);
        assert_eq!(places("while (go()) {}"), [(1, 1)]);
        assert_eq!(places("do {} while (go());"), [(1, 1)]);
        assert_eq!(places("for (;;) {\n  while (a) {}\n}"), [(1, 1), (2, 3)]);
    }

    #[test]
    fn lets_map_and_reduce_through() {
        let code = "const numbers = [1, 2, 3];\nconst double = numbers.map((n) => n * 2);\nconst sum = numbers.reduce((carry, number) => carry + number, 0);";

        assert_eq!(places(code), Vec::<(usize, usize)>::new());
    }
}
