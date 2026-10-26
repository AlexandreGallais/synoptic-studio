//! `@angular-eslint/no-inputs-metadata-property` and `no-outputs-metadata-property`: a
//! component's or directive's inputs and outputs are declared on its members, never in its
//! `inputs` and `outputs` metadata (`@angular-eslint/eslint-plugin` 22.6.0, the cases of their
//! tests); a host directive's, nested under `hostDirectives`, is left, as there.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::metadata::{class_decorator_of, decorator_properties, names_metadata};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule on `inputs`.
pub struct NoInputsMetadataProperty;

/// The rule on `outputs`.
pub struct NoOutputsMetadataProperty;

/// Reports the metadata property of a key on a component or a directive.
fn check(
    rule: &'static str,
    key: &str,
    message: &str,
    node: &AstNode,
    source: &Source,
    findings: &mut Vec<Finding>,
) {
    let AstKind::Class(class) = node.kind() else {
        return;
    };
    let Some((decorator, _)) = class_decorator_of(class, &["Component", "Directive"]) else {
        return;
    };
    for property in decorator_properties(decorator) {
        if names_metadata(property, key) {
            findings.push(source.finding(rule, property.span.start, message.into()));
        }
    }
}

impl Rule for NoInputsMetadataProperty {
    fn name(&self) -> &'static str {
        "@angular-eslint/no-inputs-metadata-property"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        check(
            self.name(),
            "inputs",
            "Use `@Input` rather than the `inputs` metadata property",
            node,
            source,
            findings,
        );
    }
}

impl Rule for NoOutputsMetadataProperty {
    fn name(&self) -> &'static str {
        "@angular-eslint/no-outputs-metadata-property"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        check(
            self.name(),
            "outputs",
            "Use `@Output` rather than the `outputs` metadata property",
            node,
            source,
            findings,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{NoInputsMetadataProperty, NoOutputsMetadataProperty};
    use crate::rules::Rule;
    use crate::testing::lint;

    /// The place of each finding of a rule on a piece of code.
    fn check(rule: &dyn Rule, code: &str) -> Vec<(usize, usize)> {
        lint(rule, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_bindings_on_members_and_host_directives() {
        // The plugins' valid cases.
        for (rule, key) in [
            (&NoInputsMetadataProperty as &dyn Rule, "inputs"),
            (&NoOutputsMetadataProperty, "outputs"),
        ] {
            for code in [
                "class Test {}\n".to_owned(),
                format!("@Page({{\n  {key}: ['a'],\n}})\nclass Test {{}}\n"),
                format!("@Injectable({{\n  {key}: ['a'],\n}})\nclass Test {{}}\n"),
                format!(
                    "@Component({{\n  hostDirectives: [{{\n    directive: CdkMenuItem,\n    {key}: ['a: b'],\n  }}]\n}})\nclass Test {{}}\n"
                ),
                format!("@Directive({{\n  ['{key}2']: ['a'],\n}})\nclass Test {{}}\n"),
            ] {
                assert_eq!(check(rule, &code), [], "{code}");
            }
        }
    }

    #[test]
    fn f51_ac2_reports_the_metadata_property_at_its_start() {
        for (rule, key, message) in [
            (
                &NoInputsMetadataProperty as &dyn Rule,
                "inputs",
                "Use `@Input` rather than the `inputs` metadata property",
            ),
            (
                &NoOutputsMetadataProperty,
                "outputs",
                "Use `@Output` rather than the `outputs` metadata property",
            ),
        ] {
            for written in [key.to_owned(), format!("'{key}'"), format!("[`{key}`]")] {
                let code = format!("@Component({{\n  {written}: ['a'],\n}})\nclass Test {{}}\n");
                let found = lint(rule, "playground/src/app/a.ts", &code);

                assert_eq!(check(rule, &code), [(2, 3)], "{code}");
                assert_eq!(found[0].message, message);
            }
            let code = format!("@Directive({{\n  selector: 'a',\n  {key},\n}})\nclass Test {{}}\n");
            assert_eq!(check(rule, &code), [(3, 3)], "{code}");
        }
    }
}
