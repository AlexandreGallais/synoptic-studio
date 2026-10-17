//! `functional/no-throw-statements`: no `throw` (`eslint-plugin-functional` 10.0.1,
//! `docs/rules/no-throw-statements.md`), with the project's option `allowToRejectPromises` off,
//! so a `throw` in an `async` function is reported too. No fix.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoThrowStatements;

/// The plugin's message.
const MESSAGE: &str = "Unexpected throw, throwing exceptions is not functional.";

impl Rule for NoThrowStatements {
    fn name(&self) -> &'static str {
        "functional/no-throw-statements"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        if let AstKind::ThrowStatement(throw) = node.kind() {
            findings.push(source.finding(self.name(), throw.span.start, MESSAGE.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoThrowStatements;
    use crate::testing::lint;

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&NoThrowStatements, "src/a/b.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_every_throw() {
        // The documentation's case, then `allowToRejectPromises`' own, off here.
        assert_eq!(
            places("throw new Error(\"Something went wrong.\");"),
            [(1, 1)]
        );
        assert_eq!(
            places(
                "async function divide(x, y) {\n  const [xv, yv] = await Promise.all([x, y]);\n  if (yv === 0) {\n    throw new Error(\"Cannot divide by zero.\");\n  }\n  return xv / yv;\n}"
            ),
            [(4, 5)]
        );
    }

    #[test]
    fn lets_an_error_returned_or_rejected_through() {
        for code in [
            "function divide(x, y) {\n  return y === 0 ? new Error(\"Cannot divide by zero.\") : x / y;\n}",
            "async function divide(x, y) {\n  const [xv, yv] = await Promise.all([x, y]);\n  return yv === 0\n    ? Promise.reject(new Error(\"Cannot divide by zero.\"))\n    : xv / yv;\n}",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
