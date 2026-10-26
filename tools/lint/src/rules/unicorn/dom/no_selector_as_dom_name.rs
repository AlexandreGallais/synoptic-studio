//! `unicorn/no-selector-as-dom-name`: `classList`'s methods and `getElementsByClassName()` take
//! class names, `getElementById()` an ID, never a selector starting with `.` or `#`
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/no-selector-as-dom-name.md`). The prefix the API
//! expects is removed from a simple name or from `` `.${name}` ``; other cases are reported only.

use oxc_ast::AstKind;
use oxc_ast::ast::{Argument, Expression};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;
use oxc_str::JSStr;

use super::nodes::{is_not_dom_node, static_string};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{method_call, unwrap};
use crate::source::Source;

/// The rule.
pub struct NoSelectorAsDomName;

/// The message.
const MESSAGE: &str = "Do not use selector syntax in DOM names.";

/// The arguments of a call that are DOM names, with the selector prefix their API would read.
fn names<'b, 'a>(
    expression_call: &'b oxc_ast::ast::CallExpression<'a>,
) -> Vec<(&'b Argument<'a>, char)> {
    let Some(method) = method_call(expression_call) else {
        return Vec::new();
    };
    let arguments = &expression_call.arguments;
    let class_list = matches!(method.object, Expression::StaticMemberExpression(member)
        if member.property.name == "classList" && !is_not_dom_node(unwrap(&member.object)));
    let taken = match method.name {
        "add" | "remove" if class_list => arguments.len(),
        "replace" if class_list => {
            if arguments.first().is_some_and(Argument::is_spread) {
                0
            } else {
                2
            }
        }
        "contains" | "toggle" if class_list => 1,
        "getElementById" | "getElementsByClassName"
            if arguments.len() == 1 && !is_not_dom_node(unwrap(method.object)) =>
        {
            let prefix = if method.name == "getElementById" {
                '#'
            } else {
                '.'
            };
            return arguments
                .iter()
                .map(|argument| (argument, prefix))
                .collect();
        }
        _ => 0,
    };
    arguments
        .iter()
        .take(taken)
        .map(|argument| (argument, '.'))
        .collect()
}

/// Whether a name is a simple selector: a prefix, then a CSS-like name.
fn is_simple(value: &str) -> bool {
    let rest = &value[1..];
    let rest = rest.strip_prefix('-').unwrap_or(rest);
    let mut characters = rest.chars();
    characters
        .next()
        .is_some_and(|first| first == '_' || first.is_ascii_alphabetic())
        && characters.all(|each| each == '_' || each == '-' || each.is_ascii_alphanumeric())
}

impl Rule for NoSelectorAsDomName {
    fn name(&self) -> &'static str {
        "unicorn/no-selector-as-dom-name"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        for (argument, prefix) in names(call) {
            let Some(expression) = argument.as_expression() else {
                continue;
            };
            let name = unwrap(expression);
            let value = static_string(name).or_else(|| match name {
                Expression::TemplateLiteral(template) => template
                    .quasis
                    .first()
                    .and_then(|quasi| quasi.value.cooked)
                    .and_then(|cooked| cooked.as_str().map(str::to_owned)),
                _ => None,
            });
            let Some(value) = value.filter(|value| value.starts_with(['.', '#'])) else {
                continue;
            };
            let start = name.span().start;
            let fixable = value.starts_with(prefix)
                && source.text[start as usize + 1..].starts_with(prefix)
                && match name {
                    Expression::TemplateLiteral(template) if !template.expressions.is_empty() => {
                        template.expressions.len() == 1
                            && template.quasis[0]
                                .value
                                .cooked
                                .is_some_and(|cooked| cooked == prefix.to_string().as_str())
                            && template.quasis[1].value.cooked.is_some_and(JSStr::is_empty)
                    }
                    _ => is_simple(&value),
                };
            let finding = if fixable {
                let fix = Fix {
                    start: start + 1,
                    end: start + 2,
                    text: String::new(),
                };
                source.fixed(self.name(), start, MESSAGE.to_owned(), fix)
            } else {
                source.finding(self.name(), start, MESSAGE.to_owned())
            };
            findings.push(finding);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoSelectorAsDomName;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&NoSelectorAsDomName, "playground/a/b.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn removes_the_prefix_of_names_given_as_selectors() {
        // The documentation's seven cases.
        assert_eq!(
            check("element.classList.add('.active');\nelement.classList.remove('.hidden');\nelement.classList.contains('.selected');\nelement.classList.toggle('.expanded');\nelement.classList.replace('.old', '.new');\ndocument.getElementsByClassName('.item');\ndocument.getElementById('#app');"),
            (
                vec![(1, 23), (2, 26), (3, 28), (4, 26), (5, 27), (5, 35), (6, 33), (7, 25)],
                "element.classList.add('active');\nelement.classList.remove('hidden');\nelement.classList.contains('selected');\nelement.classList.toggle('expanded');\nelement.classList.replace('old', 'new');\ndocument.getElementsByClassName('item');\ndocument.getElementById('app');"
                    .to_owned()
            )
        );
        // A template, the wrong prefix, a complex selector: reported, fixed only when simple.
        assert_eq!(
            check("el.classList.add(`.${name}`, `.${a}-${b}`);\ndocument.getElementById('.app');\nel.classList.toggle('.a .b');"),
            (
                vec![(1, 18), (1, 30), (2, 25), (3, 21)],
                "el.classList.add(`${name}`, `.${a}-${b}`);\ndocument.getElementById('.app');\nel.classList.toggle('.a .b');"
                    .to_owned()
            )
        );
    }

    #[test]
    fn leaves_names() {
        for code in [
            "element.classList.add('active');",
            "element.classList.replace('old', 'new');",
            "document.getElementById('app');",
            "document.querySelector('#app');",
            "element.classList.toggle('a', '.b');",
            "element.list.add('.a');",
            "[].classList.add('.a');",
            "document.getElementById('#a', 1);",
        ] {
            assert_eq!(check(code).0, Vec::new(), "{code}");
        }
    }
}
