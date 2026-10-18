//! `functional/no-try-statements`: no `try` with a `catch` or a `finally`
//! (`eslint-plugin-functional` 10.0.1, `docs/rules/no-try-statements.md`), with the project's
//! options `allowCatch` and `allowFinally` off: the `catch` message wins when both are written.
//! No fix.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoTryStatements;

/// The plugin's message on a `try` with a `catch`.
const CATCH: &str = "Unexpected try-catch, this pattern is not functional.";
/// The plugin's message on a `try` with a `finally` only.
const FINALLY: &str = "Unexpected try-finally, this pattern is not functional.";

impl Rule for NoTryStatements {
    fn name(&self) -> &'static str {
        "functional/no-try-statements"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::TryStatement(statement) = node.kind() else {
            return;
        };
        let message = if statement.handler.is_some() {
            CATCH
        } else {
            FINALLY
        };
        findings.push(source.finding(self.name(), statement.span.start, message.to_owned()));
    }
}

#[cfg(test)]
mod tests {
    use super::{CATCH, FINALLY, NoTryStatements};
    use crate::testing::{lint, messages};

    /// The messages of the findings.
    fn found(code: &str) -> Vec<String> {
        messages(&lint(&NoTryStatements, "src/a/b.ts", code))
    }

    #[test]
    fn reports_a_catch_and_a_finally() {
        // The documentation's cases, then both clauses at once.
        assert_eq!(
            found(
                "try {\n  doSomethingThatMightGoWrong(); // <-- Might throw an exception.\n} catch (error) {\n  // Handle error.\n}"
            ),
            [CATCH]
        );
        assert_eq!(
            found("try {\n  doSomethingThatMightGoWrong();\n} finally {\n  // Clean up.\n}"),
            [FINALLY]
        );
        assert_eq!(found("try {\n  a();\n} catch {\n} finally {\n}"), [CATCH]);
    }

    #[test]
    fn lets_code_without_try_through() {
        let code = "doSomethingThatMightGoWrong() // <-- Returns a Promise\n  .catch((error) => {\n    // Handle error.\n  });";

        assert_eq!(found(code), Vec::<String>::new());
    }
}
