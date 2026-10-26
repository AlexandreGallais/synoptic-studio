//! `@angular-eslint/component-max-inline-declarations`: at most so many lines of inline template,
//! styles and animations in a component (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its
//! tests, with the counts they make at our limits), with every limit at 0: a component's template
//! and styles are its own files (NOTE-0036, finding 27), which `local/component-files` checks too.
//! The tests marked as ours are written here.

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

    /// The finding expected on a value at line 2, at a column, of a kind and a count of lines.
    fn found(column: usize, kind: &str, count: usize) -> Vec<(usize, usize, String)> {
        vec![(
            2,
            column,
            format!("`{kind}` has too many lines ({count}). Maximum allowed is 0"),
        )]
    }

    #[test]
    fn f51_ac2_reports_the_plugin_s_valid_cases_but_the_files_at_a_limit_of_0() {
        // The plugin's 6 valid cases, valid under its limits of 3, 3 and 15 lines; at our limit of
        // 0, each counts 1 line but the files' one. `  template: ` is 12 columns, the value at 13;
        // `  styles: ` 10, at 11; `  animations: ` 14, at 15.
        assert_eq!(
            check("  template: '<div>just one line template</div>'\n"),
            found(13, "template", 1)
        );
        assert_eq!(
            check("  styles: ['div { display: none; }']\n"),
            found(11, "styles", 1)
        );
        assert_eq!(
            check("  styles: 'div { display: none; }'\n"),
            found(11, "styles", 1)
        );
        // One line: 2 − 2 − 2 is under 0, at least 1.
        assert_eq!(
            check("  animations: [state('void', style({opacity: 0, transform: 'scale(1, 0)'}))]\n"),
            found(15, "animations", 1)
        );
        assert_eq!(
            check("  styleUrls: ['./foobar.scss'],\n  templateUrl: './foobar.html',\n"),
            []
        );
        // Lines 2 to 4: 4 − 2 − 2 = 0, at least 1.
        assert_eq!(
            check(
                "  animations: [\n    state('void', style({opacity: 0, transform: 'scale(1, 0)'}))\n  ],\n  templateUrl: './foobar.html',\n"
            ),
            found(15, "animations", 1)
        );
    }

    #[test]
    fn f51_ac2_reports_the_plugin_s_invalid_cases_with_their_counts() {
        // The plugin's 9 invalid cases, its options left out: its annotation lines (`~`) stay in
        // its code as blank lines, kept here as empty lines, which the animations count.
        // A template of 4 lines once trimmed, the blank line among the trimmed ones.
        assert_eq!(
            check(
                "  template: `\n\n    <div>first line</div>\n    <div>second line</div>\n    <div>third line</div>\n    <div>fourth line</div>\n  `\n\n"
            ),
            found(13, "template", 4)
        );
        assert_eq!(
            check("  template: '<div>first line</div>'\n\n"),
            found(13, "template", 1)
        );
        // `div {`, `display: block;`, `height: 40px;`, `}`: 4 lines, in an array or alone.
        assert_eq!(
            check(
                "  styles: [\n\n    `\n      div {\n        display: block;\n        height: 40px;\n      }\n    `\n  ]\n\n"
            ),
            found(11, "styles", 4)
        );
        assert_eq!(
            check(
                "  styles: `\n\n    div {\n      display: block;\n      height: 40px;\n    }\n  `\n\n"
            ),
            found(11, "styles", 4)
        );
        // Two templates of 3 lines each: 6.
        assert_eq!(
            check(
                "  styles: [\n\n    `\n      div {\n        display: block;\n      }\n    `,\n    `\n      span {\n        width: 30px;\n      }\n    `\n  ]\n\n"
            ),
            found(11, "styles", 6)
        );
        assert_eq!(
            check("  styles: ['div { display: none; }']\n\n"),
            found(11, "styles", 1)
        );
        // `[{` on line 2, the blank line 3, 16 lines of triggers on 4 to 19, `}]` on 20:
        // 20 − 2 − 2 = 16.
        assert_eq!(
            check(concat!(
                "  animations: [{\n",
                "\n",
                "    transformPanelWrap: trigger('transformPanelWrap', [\n",
                "      transition('* => void', query('@transformPanel', [animateChild()], {optional: true})),\n",
                "    ]),\n",
                "    transformPanel: trigger('transformPanel', [\n",
                "      state('void', style({\n",
                "        transform: 'scaleY(0.8)',\n",
                "        minWidth: '100%',\n",
                "        opacity: 0\n",
                "      })),\n",
                "      state('showing', style({\n",
                "        opacity: 1,\n",
                "        minWidth: 'calc(100% + 32px)',\n",
                "        transform: 'scaleY(1)'\n",
                "      })),\n",
                "      state('next', style({height: '0px', visibility: 'hidden'}))\n",
                "    ])\n",
                "  }]\n",
                "\n",
            )),
            found(15, "animations", 16)
        );
        // `[` on line 2, the blank line 3, 18 lines of triggers on 4 to 21, `]` on 22:
        // 22 − 2 − 2 = 18.
        assert_eq!(
            check(concat!(
                "  animations: [\n",
                "\n",
                "    trigger('dialogContainer', [\n",
                "      transition('* => void', query('@transformPanel', [animateChild()], {optional: true}))\n",
                "    ]),\n",
                "    trigger('transformPanel', [\n",
                "      state('void', style({\n",
                "        transform: 'scaleY(0.8)',\n",
                "        minWidth: '100%',\n",
                "        opacity: 0\n",
                "      })),\n",
                "      state('showing', style({\n",
                "        opacity: 1,\n",
                "        minWidth: 'calc(100% + 32px)',\n",
                "        transform: 'scaleY(1)'\n",
                "      }))\n",
                "    ]),\n",
                "    trigger('transformPanel', [\n",
                "      state('void', style({opacity: 0, transform: 'scale(1, 0)'}))\n",
                "    ])\n",
                "  ]\n",
                "\n",
            )),
            found(15, "animations", 18)
        );
        // `[{` on line 2, the blank line 3, 3 lines on 4 to 6, `}]` on 7: 7 − 2 − 2 = 3.
        assert_eq!(
            check(
                "  animations: [{\n\n    transformPanel: trigger('transformPanel', [\n      state('void', style({opacity: 0, transform: 'scale(1, 0)'}))\n    ])\n  }]\n\n"
            ),
            found(15, "animations", 3)
        );
    }

    #[test]
    fn f51_ac2_counts_an_empty_inline_template_or_style_as_1_line() {
        // Our cases: an empty template literal trims to no line, which the plugin's `split`
        // counts as 1.
        assert_eq!(check("  template: ``\n"), found(13, "template", 1));
        assert_eq!(check("  styles: [``]\n"), found(11, "styles", 1));
        assert_eq!(check("  styles: ``\n"), found(11, "styles", 1));
    }

    #[test]
    fn f51_ac2_accepts_files_and_values_it_cannot_count() {
        // Our cases.
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
        // Our cases. `  template: ` is 12 columns, the value at 13; `  styles: ` 10, at 11; `  animations: `
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
