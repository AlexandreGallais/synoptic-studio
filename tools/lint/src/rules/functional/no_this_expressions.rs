//! `functional/no-this-expressions`: no `this` (`eslint-plugin-functional` 10.0.1,
//! `docs/rules/no-this-expressions.md`), the companion of `no-classes`; a `this` type is not an
//! expression and is left. No fix.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoThisExpressions;

/// The plugin's message.
const MESSAGE: &str = "Unexpected this, use functions not classes.";

impl Rule for NoThisExpressions {
    fn name(&self) -> &'static str {
        "functional/no-this-expressions"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        if let AstKind::ThisExpression(this) = node.kind() {
            findings.push(source.finding(self.name(), this.span.start, MESSAGE.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoThisExpressions;
    use crate::testing::lint;

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&NoThisExpressions, "src/a/b.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_each_this() {
        // The documentation's case, then a method's `this`.
        assert_eq!(places("const foo = this.value + 17;"), [(1, 13)]);
        assert_eq!(
            places("const o = {\n  f() {\n    return this;\n  },\n};"),
            [(3, 12)]
        );
    }

    #[test]
    fn lets_an_object_and_a_this_type_through() {
        for code in [
            "const foo = object.value + 17;",
            "type F = (this: Window) => void;",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
