//! `functional/no-let`: no `let` (`eslint-plugin-functional` 10.0.1, `docs/rules/no-let.md`),
//! with the project's options: `allowInForLoopInit` and `allowInFunctions` off, so every `let` is
//! reported, in a `for` header and in a function too. No fix, as the plugin's.

use oxc_ast::AstKind;
use oxc_ast::ast::VariableDeclarationKind;
use oxc_semantic::AstNode;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoLet;

/// The plugin's message.
const MESSAGE: &str = "Unexpected let, use const instead.";

impl Rule for NoLet {
    fn name(&self) -> &'static str {
        "functional/no-let"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        if let AstKind::VariableDeclaration(declaration) = node.kind()
            && declaration.kind == VariableDeclarationKind::Let
        {
            findings.push(source.finding(self.name(), declaration.span.start, MESSAGE.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoLet;
    use crate::testing::lint;

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&NoLet, "src/a/b.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_every_let() {
        // The documentation's cases, then `allowInForLoopInit`'s and `allowInFunctions`' own,
        // both off here.
        assert_eq!(places("let x = 5;"), [(1, 1)]);
        assert_eq!(
            places("for (let i = 0; i < array.length; i++) {}"),
            [(1, 6)]
        );
        assert_eq!(places("for (let element of array) {\n}"), [(1, 6)]);
        assert_eq!(
            places("for (let [index, element] of array.entries()) {\n}"),
            [(1, 6)]
        );
        assert_eq!(
            places("function f(): number {\n  let a = 1;\n  return a;\n}"),
            [(2, 3)]
        );
        assert_eq!(places("let a = 1, b = 2;"), [(1, 1)]);
    }

    #[test]
    fn lets_const_and_var_through() {
        for code in [
            "const x = 5;",
            "for (const element of array) {\n}",
            "for (const [index, element] of array.entries()) {\n}",
            "var y = 1;",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
