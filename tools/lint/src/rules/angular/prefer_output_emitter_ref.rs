//! `@angular-eslint/prefer-output-emitter-ref`: `output()` rather than the `@Output()` decorator
//! (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests).

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::classes::called_name;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferOutputEmitterRef;

impl Rule for PreferOutputEmitterRef {
    fn name(&self) -> &'static str {
        "@angular-eslint/prefer-output-emitter-ref"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        if let AstKind::Decorator(decorator) = node.kind()
            && called_name(decorator) == Some("Output")
        {
            findings.push(source.finding(
                self.name(),
                decorator.span.start,
                "Use `OutputEmitterRef` via `output()` for Component and Directive outputs rather than the legacy `@Output()` decorator".into(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferOutputEmitterRef;
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&PreferOutputEmitterRef, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_output_functions_and_emitters() {
        // The plugin's valid cases, and `@Output` written without a call.
        for member in [
            "change = outputValue();",
            "change = new EventEmitter();",
            "readonly change = output();",
            "readonly change = output<string>();",
            "@Output readonly change: EventEmitter<number>;",
        ] {
            let code = format!("class Test {{\n  {member}\n}}\n");
            assert_eq!(check(&code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_the_output_decorator_at_its_start() {
        // The plugin's invalid case: the decorator at column 3 of line 2.
        assert_eq!(
            check("class Test {\n  @Output()\n  readonly change: EventEmitter<number>;\n}\n"),
            [(2, 3)]
        );
    }
}
