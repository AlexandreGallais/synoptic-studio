//! `@angular-eslint/no-forward-ref`: no `forwardRef()`, the workaround of a circular dependency
//! (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests).

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;
use oxc_semantic::AstNode;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoForwardRef;

impl Rule for NoForwardRef {
    fn name(&self) -> &'static str {
        "@angular-eslint/no-forward-ref"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        if let AstKind::CallExpression(call) = node.kind()
            && matches!(&call.callee, Expression::Identifier(callee) if callee.name == "forwardRef")
        {
            findings.push(source.finding(
                self.name(),
                call.span.start,
                "Avoid using `forwardRef`".into(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoForwardRef;
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&NoForwardRef, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| {
                assert_eq!(each.message, "Avoid using `forwardRef`");
                (each.line, each.column)
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_code_without_forward_ref() {
        for code in [
            "@Component({\n  providers: [MyService],\n})\nclass Test {}\n",
            "const a = ng.forwardRef(() => B);\n",
            "const forwardRef = 1;\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_each_forward_ref_call_at_its_start() {
        // `  providers: [` is 14 columns, the call at 15; `const a = ` 10, at 11.
        assert_eq!(
            check("@Component({\n  providers: [forwardRef(() => A)],\n})\nclass Test {}\n"),
            [(2, 15)]
        );
        assert_eq!(
            check(
                "class Test {\n  constructor(@Inject(forwardRef(() => A)) a: A) {}\n}\nconst a = forwardRef(() => B);\n"
            ),
            [(2, 23), (4, 11)]
        );
    }
}
