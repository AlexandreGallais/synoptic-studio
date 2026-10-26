//! `@angular-eslint/prefer-output-readonly`: an output, `@Output()`, `output()` or
//! `outputFromObservable()`, is `readonly` (`@angular-eslint/eslint-plugin` 22.6.0, the cases of
//! its tests); each of the plugin's three selectors reports on its own, as there. The added
//! `readonly`, a suggestion, is not carried.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, PropertyDefinition, TSType, TSTypeName};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::classes::called_name;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferOutputReadonly;

/// Whether a property's written type is a reference to a name (`typeName.name`).
fn written_type_is(property: &PropertyDefinition, name: &str) -> bool {
    matches!(property.type_annotation.as_ref().map(|annotation| &annotation.type_annotation),
        Some(TSType::TSTypeReference(reference))
            if matches!(&reference.type_name, TSTypeName::IdentifierReference(type_name)
                if type_name.name == name))
}

/// Whether a property's value calls a function of a name (`value.callee.name`).
fn calls(property: &PropertyDefinition, name: &str) -> bool {
    matches!(&property.value, Some(Expression::CallExpression(call))
        if matches!(&call.callee, Expression::Identifier(callee) if callee.name == name))
}

impl Rule for PreferOutputReadonly {
    fn name(&self) -> &'static str {
        "@angular-eslint/prefer-output-readonly"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::PropertyDefinition(property) = node.kind() else {
            return;
        };
        if property.readonly {
            return;
        }
        let decorated = property
            .decorators
            .iter()
            .filter(|decorator| called_name(decorator) == Some("Output"))
            .map(|_| "@Output");
        let emitter = (written_type_is(property, "OutputEmitterRef") || calls(property, "output"))
            .then_some("OutputEmitterRef");
        let output_ref = (written_type_is(property, "OutputRef")
            || calls(property, "outputFromObservable"))
        .then_some("OutputRef");
        for kind in decorated.chain(emitter).chain(output_ref) {
            findings.push(source.finding(
                self.name(),
                property.key.span().start,
                format!(
                    "Prefer to declare `{kind}` as `readonly` since they are not supposed to be reassigned"
                ),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferOutputReadonly;
    use crate::testing::lint;

    /// The place and kind of each finding on a class of one member.
    fn check(member: &str) -> Vec<(usize, usize, String)> {
        let code = format!("class Test {{\n  {member}\n}}\n");
        lint(&PreferOutputReadonly, "playground/src/app/a.ts", &code)
            .into_iter()
            .map(|each| {
                let kind = each
                    .message
                    .strip_prefix("Prefer to declare `")
                    .and_then(|rest| rest.split('`').next())
                    .unwrap_or_default()
                    .to_owned();
                (each.line, each.column, kind)
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_readonly_outputs_and_other_properties() {
        // The plugin's valid cases.
        for member in [
            "testEmitter = new EventEmitter<string>();",
            "@Output() readonly testEmitter = new EventEmitter<string>();",
            "testEmitter: NotOutputEmitterRef<string>;",
            "readonly testEmitter: OutputEmitterRef<string>;",
            "testEmitter = notOutput();",
            "readonly testEmitter = output();",
            "testEmitter = notOutput<string>();",
            "readonly testEmitter = output<string>();",
            "readonly testEmitter: OutputRef<string>;",
            "readonly testEmitter = outputFromObservable(testObservable);",
        ] {
            assert_eq!(check(member), [], "{member}");
        }
    }

    #[test]
    fn f51_ac2_reports_an_output_not_readonly_at_its_name() {
        // The plugin's invalid cases: `  @Output() ` is 12 columns, the name at 13; else at 3.
        for (member, column, kind) in [
            (
                "@Output() testEmitter = new EventEmitter<string>();",
                13,
                "@Output",
            ),
            (
                "testEmitter: OutputEmitterRef<string>;",
                3,
                "OutputEmitterRef",
            ),
            ("testEmitter = output();", 3, "OutputEmitterRef"),
            ("testEmitter = output<string>();", 3, "OutputEmitterRef"),
            ("testEmitter: OutputRef<string>;", 3, "OutputRef"),
            (
                "testEmitter = outputFromObservable(testObservable);",
                3,
                "OutputRef",
            ),
        ] {
            assert_eq!(check(member), [(2, column, kind.to_owned())], "{member}");
        }
    }
}
