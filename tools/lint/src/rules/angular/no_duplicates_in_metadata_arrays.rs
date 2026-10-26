//! `@angular-eslint/no-duplicates-in-metadata-arrays`: no entry twice in a metadata array of a
//! module, a component or a directive, a provider Angular would create twice
//! (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests). An identifier and a string are
//! compared by their text, as the plugin does.

use oxc_ast::AstKind;
use oxc_ast::ast::ArrayExpressionElement;
use oxc_semantic::AstNode;

use super::classes::{enclosing_decorators, key_identifier};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoDuplicatesInMetadataArrays;

/// The metadata arrays of a decorator, after Angular's API pages the plugin cites.
fn metadata_arrays(decorator: &str) -> &'static [&'static str] {
    match decorator {
        "NgModule" => &["providers", "declarations", "imports", "exports"],
        "Component" => &["imports", "providers", "viewProviders", "styleUrls"],
        "Directive" => &["providers"],
        _ => &[],
    }
}

impl Rule for NoDuplicatesInMetadataArrays {
    fn name(&self) -> &'static str {
        "@angular-eslint/no-duplicates-in-metadata-arrays"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::ArrayExpression(array) = node.kind() else {
            return;
        };
        let AstKind::ObjectProperty(property) = source.semantic.nodes().parent_kind(node.id())
        else {
            return;
        };
        let Some(key) = key_identifier(&property.key) else {
            return;
        };
        let metadata = enclosing_decorators(source, node.id())
            .iter()
            .any(|decorator| metadata_arrays(decorator).contains(&key));
        if !metadata {
            return;
        }
        let mut seen: Vec<&str> = Vec::new();
        for element in &array.elements {
            let (text, at) = match element {
                ArrayExpressionElement::Identifier(name) => (name.name.as_str(), name.span.start),
                ArrayExpressionElement::StringLiteral(text) => {
                    (text.value.as_str(), text.span.start)
                }
                _ => continue,
            };
            if seen.contains(&text) {
                findings.push(source.finding(
                    self.name(),
                    at,
                    "Entry is duplicated in metadata array".into(),
                ));
            } else {
                seen.push(text);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoDuplicatesInMetadataArrays;
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(
            &NoDuplicatesInMetadataArrays,
            "playground/src/app/a.ts",
            code,
        )
        .into_iter()
        .map(|each| {
            assert_eq!(each.message, "Entry is duplicated in metadata array");
            (each.line, each.column)
        })
        .collect()
    }

    #[test]
    fn f51_ac1_accepts_arrays_of_distinct_entries_and_other_arrays() {
        // The plugin's valid cases, then arrays it does not read.
        for code in [
            "@NgModule({\n  providers: [ProviderA, ProviderB, ProviderC],\n  declarations: [DeclarationA, DeclarationB, DeclarationC],\n  imports: [ImportA, ImportB, ImportC],\n  exports: [ExportA, ExportB, ExportC]\n})\nclass Test {}\n",
            "@Component({\n  imports: [ImportA, ImportB, ImportC],\n  providers: [ProviderA, ProviderB, ProviderC],\n  viewProviders: [ViewProviderA, ViewProviderB, ViewProviderC],\n  styleUrls: ['a.css', 'b.css', 'c.css']\n})\nclass Test {}\n",
            "@Directive({\n  providers: [ProviderA, ProviderB, ProviderC],\n})\nclass Test {}\n",
            // `imports` of a directive, an array outside a decorator, calls and spreads.
            "@Directive({\n  imports: [A, A],\n})\nclass Test {}\n",
            "const imports = { imports: [A, A] };\n",
            "@Component({\n  providers: [f(), f(), ...a, ...a],\n})\nclass Test {}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac1_reports_each_entry_seen_before_at_the_entry() {
        // `  providers: [ProviderA, ProviderB, ` is 36 columns: the second `ProviderA` at 37, the
        // second `ProviderB` 11 further, at 48.
        let in_line = |decorator: &str, key: &str, a: &str, b: &str| {
            format!("@{decorator}({{\n  {key}: [{a}, {b}, {a}, {b}]\n}})\nclass Test {{}}\n")
        };
        let place = |key: &str, a: &str, b: &str| {
            // `  <key>: [` then `a, b, `: the third entry, then the fourth past `a, `.
            let third = 2 + key.len() + 3 + a.len() + 2 + b.len() + 2 + 1;
            vec![(2, third), (2, third + a.len() + 2)]
        };
        for (decorator, key, a, b) in [
            ("NgModule", "providers", "ProviderA", "ProviderB"),
            ("NgModule", "declarations", "DeclarationA", "DeclarationB"),
            ("NgModule", "imports", "ImportA", "ImportB"),
            ("NgModule", "exports", "ExportA", "ExportB"),
            ("Component", "imports", "ImportA", "ImportB"),
            ("Component", "providers", "ProviderA", "ProviderB"),
            (
                "Component",
                "viewProviders",
                "ViewProviderA",
                "ViewProviderB",
            ),
            ("Component", "styleUrls", "'a.css'", "'b.css'"),
            ("Directive", "providers", "ProviderA", "ProviderB"),
        ] {
            let code = in_line(decorator, key, a, b);

            assert_eq!(check(&code), place(key, a, b), "{code}");
        }
        assert_eq!(
            place("providers", "ProviderA", "ProviderB"),
            [(2, 37), (2, 48)]
        );
        // One entry a line: the third, fourth and fifth, at column 5.
        assert_eq!(
            check(
                "@NgModule({\n  imports: [\n    ImportA,\n    ImportB,\n    ImportA,\n    ImportB,\n    ImportA\n  ]\n})\nclass Test {}\n"
            ),
            [(5, 5), (6, 5), (7, 5)]
        );
    }
}
