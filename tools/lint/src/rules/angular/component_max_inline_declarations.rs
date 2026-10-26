//! `@angular-eslint/component-max-inline-declarations`: at most so many lines of inline template,
//! styles and animations in a component (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its
//! tests), with every limit at 0: a component's template and styles are its own files
//! (NOTE-0036, finding 27), which `local/component-files` checks too.

use oxc_ast::AstKind;
use oxc_ast::ast::{ArrayExpressionElement, Expression};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::classes::key_identifier;
use super::metadata::{class_decorator, decorator_properties};
use crate::expressions::text;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct ComponentMaxInlineDeclarations;

/// The most lines of each inline declaration.
const LIMIT: usize = 0;

/// The lines of a string or a template, trimmed (`getLinesCount`): a string's source text, quotes
/// included; anything else counts none.
fn lines(source: &Source, expression: &Expression) -> usize {
    let written = match expression {
        Expression::TemplateLiteral(template) => template
            .quasis
            .first()
            .map_or("", |quasi| quasi.value.raw.as_str()),
        Expression::StringLiteral(_)
        | Expression::NumericLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_) => text(source, expression.span()),
        _ => return 0,
    };
    written.trim().lines().count().max(1)
}

impl Rule for ComponentMaxInlineDeclarations {
    fn name(&self) -> &'static str {
        "@angular-eslint/component-max-inline-declarations"
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
            let value = &property.value;
            let (kind, count) = match key_identifier(&property.key) {
                Some("template") => ("template", lines(source, value)),
                Some("styles") => (
                    "styles",
                    match value {
                        Expression::ArrayExpression(array) => array
                            .elements
                            .iter()
                            .filter_map(ArrayExpressionElement::as_expression)
                            .map(|element| lines(source, element))
                            .sum(),
                        other => lines(source, other),
                    },
                ),
                Some("animations") => {
                    let Expression::ArrayExpression(array) = value else {
                        continue;
                    };
                    if array.elements.is_empty() {
                        continue;
                    }
                    let span = array.span;
                    let between = source
                        .line(span.end)
                        .saturating_sub(source.line(span.start))
                        .saturating_sub(2);
                    ("animations", between.max(1))
                }
                _ => continue,
            };
            if count > LIMIT {
                findings.push(source.finding(
                    self.name(),
                    value.span().start,
                    format!("`{kind}` has too many lines ({count}). Maximum allowed is {LIMIT}"),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentMaxInlineDeclarations;
    use crate::testing::lint;

    /// The line, column and message of each finding on a component's metadata, from line 2.
    fn check(metadata: &str) -> Vec<(usize, usize, String)> {
        let code = format!("@Component({{\n{metadata}}})\nclass Test {{}}\n");
        lint(
            &ComponentMaxInlineDeclarations,
            "playground/src/app/a.ts",
            &code,
        )
        .into_iter()
        .map(|each| (each.line, each.column, each.message))
        .collect()
    }

    #[test]
    fn f51_ac2_accepts_files_and_values_it_cannot_count() {
        for metadata in [
            "  templateUrl: './a.html',\n  styleUrl: './a.scss',\n",
            "  template: template,\n  styles: [styles],\n",
            "  animations: [],\n",
        ] {
            assert_eq!(check(metadata), [], "{metadata}");
        }
        // A directive's metadata.
        let code = "@Directive({ template: '<a></a>' })\nclass Test {}\n";
        assert_eq!(
            lint(
                &ComponentMaxInlineDeclarations,
                "playground/src/app/a.ts",
                code
            ),
            []
        );
    }

    #[test]
    fn f51_ac2_reports_any_inline_line_at_its_value() {
        // `  template: ` is 12 columns, the value at 13; `  styles: ` 10, at 11; `  animations: `
        // 14, at 15.
        let found = |line: usize, column: usize, kind: &str, count: usize| {
            vec![(
                line,
                column,
                format!("`{kind}` has too many lines ({count}). Maximum allowed is 0"),
            )]
        };
        assert_eq!(check("  template: ''\n"), found(2, 13, "template", 1));
        assert_eq!(
            check("  template: `\n    <div>\n      a\n    </div>\n  `\n"),
            found(2, 13, "template", 3)
        );
        assert_eq!(check("  styles: ':host {}'\n"), found(2, 11, "styles", 1));
        assert_eq!(
            check("  styles: [':host {}', `\n    a {}\n    b {}\n  `]\n"),
            found(2, 11, "styles", 3)
        );
        // The array's lines but its brackets', at least one.
        assert_eq!(
            check("  animations: [trigger('a', [])]\n"),
            found(2, 15, "animations", 1)
        );
        assert_eq!(
            check("  animations: [\n    trigger('a', []),\n    trigger('b', []),\n  ]\n"),
            found(2, 15, "animations", 1)
        );
        assert_eq!(
            check("  animations: [\n    a,\n    b,\n    c,\n  ]\n"),
            found(2, 15, "animations", 2)
        );
    }
}
