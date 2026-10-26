//! `@angular-eslint/relative-url-prefix`: a component's `templateUrl` and `styleUrls` start with
//! `./` or `../` (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests).

use oxc_ast::AstKind;
use oxc_ast::ast::{ArrayExpressionElement, Expression};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::classes::key_identifier;
use super::metadata::{class_decorator, decorator_properties};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct RelativeUrlPrefix;

/// Whether a URL starts with `./` or `../` and goes on (`/^\.\.?\/.+/`).
fn is_relative(url: &str) -> bool {
    url.strip_prefix("./")
        .or_else(|| url.strip_prefix("../"))
        .and_then(|rest| rest.chars().next())
        .is_some_and(|first| !matches!(first, '\n' | '\r' | '\u{2028}' | '\u{2029}'))
}

/// Whether a value is a string, or a template whose first part, is a relative URL (`isUrlInvalid`
/// negated).
fn is_relative_url(value: &Expression) -> bool {
    match value {
        Expression::StringLiteral(literal) => is_relative(&literal.value),
        Expression::TemplateLiteral(template) => template
            .quasis
            .first()
            .is_some_and(|quasi| is_relative(&quasi.value.raw)),
        _ => false,
    }
}

impl Rule for RelativeUrlPrefix {
    fn name(&self) -> &'static str {
        "@angular-eslint/relative-url-prefix"
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
        let mut wrong: Vec<u32> = Vec::new();
        for property in decorator_properties(decorator) {
            match key_identifier(&property.key) {
                Some("templateUrl") if !is_relative_url(&property.value) => {
                    wrong.push(property.value.span().start);
                }
                Some("styleUrls") => {
                    let Expression::ArrayExpression(array) = &property.value else {
                        continue;
                    };
                    for element in &array.elements {
                        match element {
                            ArrayExpressionElement::Elision(_) => {}
                            ArrayExpressionElement::SpreadElement(spread) => {
                                wrong.push(spread.span.start);
                            }
                            other => {
                                if !other.as_expression().is_some_and(is_relative_url) {
                                    wrong.push(other.span().start);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        for at in wrong {
            findings.push(source.finding(
                self.name(),
                at,
                "The ./ and ../ prefix is standard syntax for relative URLs.".into(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RelativeUrlPrefix;
    use crate::testing::lint;

    /// The place of each finding on a component's metadata, from line 2.
    fn check(metadata: &str) -> Vec<(usize, usize)> {
        let code = format!("@Component({{\n{metadata}}})\nclass Test {{}}\n");
        lint(&RelativeUrlPrefix, "playground/src/app/a.ts", &code)
            .into_iter()
            .map(|each| {
                assert_eq!(
                    each.message,
                    "The ./ and ../ prefix is standard syntax for relative URLs."
                );
                (each.line, each.column)
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_relative_urls() {
        // The plugin's valid cases.
        for metadata in [
            "  styleUrls: [\n    './foo.css',\n    '../bar.css',\n    '../../baz.scss',\n    './../test.css',\n    '.././angular.sass'\n  ]\n",
            "  templateUrl: './foobar.html'\n",
            "  templateUrl: '../foobar.html'\n",
            "  templateUrl: '../../../foobar.html'\n",
            "  templateUrl: './../foobar.html'\n",
            "  templateUrl: '.././foobar.html'\n",
            "  templateUrl: `../foobar.html`,\n  styleUrls: [\n    `.././foobar.css`,\n  ]\n",
            // `styleUrl` is left to `local/component-files`.
            "  styleUrl: 'a.css'\n",
        ] {
            assert_eq!(check(metadata), [], "{metadata}");
        }
    }

    #[test]
    fn f51_ac2_reports_a_url_without_the_prefix_at_its_value() {
        // The plugin's invalid cases: `  styleUrls: ['./foo.css', ` is 27 columns, `'bar.css'` at
        // 28; `  templateUrl: ` is 15, the value at 16.
        assert_eq!(
            check("  styleUrls: ['./foo.css', 'bar.css', '../baz.scss', '../../test.css']\n"),
            [(2, 28)]
        );
        assert_eq!(check("  templateUrl: 'foobar.html'\n"), [(2, 16)]);
        // A value the tree cannot read, and a prefix with nothing after it.
        assert_eq!(check("  templateUrl: url\n"), [(2, 16)]);
        assert_eq!(check("  templateUrl: './'\n"), [(2, 16)]);
    }
}
