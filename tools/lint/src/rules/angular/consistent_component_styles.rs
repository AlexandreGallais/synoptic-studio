//! `@angular-eslint/consistent-component-styles`: one stylesheet is `styleUrl`, one inline style
//! a string, never an array of one (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its
//! tests), in its default `string` mode. The plugin's `:has(Literal, TemplateLiteral)` is read as
//! the one element being a string or a template. Its rewrites are not carried.

use oxc_ast::AstKind;
use oxc_ast::ast::{ArrayExpressionElement, Expression};
use oxc_semantic::AstNode;

use super::metadata::{class_decorator, decorator_properties, names_metadata};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct ConsistentComponentStyles;

/// Whether a value is an array of one string or template.
fn is_single_text_array(value: &Expression) -> bool {
    matches!(value, Expression::ArrayExpression(array)
        if array.elements.len() == 1
            && matches!(array.elements[0],
                ArrayExpressionElement::StringLiteral(_) | ArrayExpressionElement::TemplateLiteral(_)))
}

impl Rule for ConsistentComponentStyles {
    fn name(&self) -> &'static str {
        "@angular-eslint/consistent-component-styles"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        let Some(decorator) = class_decorator(class, "Component") else {
            return;
        };
        for property in decorator_properties(decorator) {
            if !is_single_text_array(&property.value) {
                continue;
            }
            if names_metadata(property, "styles") {
                let Expression::ArrayExpression(array) = &property.value else {
                    continue;
                };
                findings.push(source.finding(
                    self.name(),
                    array.span.start,
                    "Use a `string` instead of a `string[]` for the `styles` property".into(),
                ));
            } else if names_metadata(property, "styleUrls") {
                findings.push(source.finding(
                    self.name(),
                    property.span.start,
                    "Use `styleUrl` instead of `styleUrls` for a single stylesheet".into(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConsistentComponentStyles;
    use crate::testing::lint;

    /// The place and message of each finding on a component's metadata, from line 2.
    fn check(metadata: &str) -> Vec<(usize, usize, String)> {
        let code = format!("@Component({{\n{metadata}}})\nclass Test {{}}\n");
        lint(&ConsistentComponentStyles, "playground/src/app/a.ts", &code)
            .into_iter()
            .map(|each| (each.line, each.column, each.message))
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_one_string_one_url_and_several_of_either() {
        // The plugin's valid cases in its default mode.
        for metadata in [
            "  styles: ':host { display: block; }',\n",
            "  styles: `\n    :host { display: block; }\n  `,\n",
            "  selector: 'my-test',\n  imports: [CommonModule],\n  styles: `:host { display: block; }`,\n  providers: [FooService]\n",
            "  styles: [\n    ':host { display: block; }',\n    `.foo { color: red; }`\n  ],\n",
            "  styleUrl: `./test.component.css`,\n",
            "  styleUrls: [\n    '../shared.css',\n    `./test.component.css`\n  ],\n",
        ] {
            assert_eq!(check(metadata), [], "{metadata}");
        }
    }

    #[test]
    fn f51_ac2_reports_an_array_of_one_style_or_one_url() {
        let styles = "Use a `string` instead of a `string[]` for the `styles` property".to_owned();
        let url = "Use `styleUrl` instead of `styleUrls` for a single stylesheet".to_owned();
        // The plugin's invalid cases: `  styles: ` is 10 columns, the array at 11; the
        // `styleUrls` property at 3.
        for (metadata, found) in [
            (
                "  styles: [':host { display: block; }']\n",
                (2, 11, styles.clone()),
            ),
            (
                "  standalone: true,\n  imports: [MatButtonModule],\n  styles: [':host { display: block; }'],\n  providers: []\n",
                (4, 11, styles.clone()),
            ),
            (
                "  styleUrls: ['./test.component.css']\n",
                (2, 3, url.clone()),
            ),
            (
                "  imports: [MatButtonModule],\n  styleUrls: ['./test.component.css'],\n  providers: []\n",
                (3, 3, url.clone()),
            ),
            (
                "  styles: [`:host\\t{ display: ${type}; }`]\n",
                (2, 11, styles.clone()),
            ),
            (
                "  styles: [\":host{ display: block; }\"]\n",
                (2, 11, styles),
            ),
            ("  styleUrls: [`./test.component.css`]\n", (2, 3, url)),
        ] {
            assert_eq!(check(metadata), [found], "{metadata}");
        }
    }
}
