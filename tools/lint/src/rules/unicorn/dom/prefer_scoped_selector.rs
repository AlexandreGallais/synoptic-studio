//! `unicorn/prefer-scoped-selector`: an element's `querySelector(…)` or `querySelectorAll(…)`
//! with a combinator anchors it with `:scope`, or the match may climb out of the element
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-scoped-selector.md`). The document,
//! shadow roots, fragments and parsed documents are left. Reported only: `ESLint` only suggests.

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;
use oxc_semantic::AstNode;

use super::nodes::{is_not_dom_node, static_string};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{method_call, unwrap};
use crate::source::Source;

/// The rule.
pub struct PreferScopedSelector;

/// The top-level branches of a selector list, strings, comments and escapes taken out
/// (`splitSelectorList`).
fn branches(selector: &str) -> Vec<String> {
    let characters: Vec<char> = selector.chars().collect();
    let mut branches = Vec::new();
    let mut current = String::new();
    let mut depth = 0_i32;
    let mut quote: Option<char> = None;
    let mut index = 0;
    while index < characters.len() {
        let character = characters[index];
        if let Some(open) = quote {
            if character == '\\' {
                index += 1;
            } else if character == open {
                quote = None;
            }
        } else if character == '/' && characters.get(index + 1) == Some(&'*') {
            index = (index + 2..characters.len().saturating_sub(1))
                .find(|at| characters[*at] == '*' && characters[*at + 1] == '/')
                .map_or(characters.len(), |end| end + 1);
        } else if matches!(character, '"' | '\'') {
            quote = Some(character);
        } else if character == '\\' {
            index += 1;
            current.push('a');
        } else if character == ',' && depth == 0 {
            branches.push(std::mem::take(&mut current));
        } else {
            match character {
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => depth -= 1,
                _ => {}
            }
            current.push(character);
        }
        index += 1;
    }
    branches.push(current);
    branches
}

/// Whether a branch holds a combinator outside its blocks (`hasCombinator`).
fn has_combinator(branch: &str) -> bool {
    let mut depth = 0_i32;
    let mut stripped = String::new();
    for character in branch.chars() {
        match character {
            '(' | '[' | '{' => {
                if depth == 0 {
                    stripped.push('*');
                }
                depth += 1;
            }
            ')' | ']' | '}' => depth -= 1,
            _ if depth == 0 => stripped.push(character),
            _ => {}
        }
    }
    if stripped.contains(['+', '>', '~']) {
        return true;
    }
    let words = stripped.trim();
    words.contains(char::is_whitespace)
}

/// Whether a branch holds the `:scope` pseudo-class as a whole token.
fn has_scope(branch: &str) -> bool {
    let lower = branch.to_ascii_lowercase();
    lower.match_indices(":scope").any(|(at, _)| {
        !lower[..at].ends_with(':')
            && !lower[at + 6..]
                .chars()
                .next()
                .is_some_and(|next| next.is_ascii_alphanumeric() || matches!(next, '_' | '-'))
    })
}

/// Whether a receiver is no element: the document, a shadow root, a fragment or a parsed
/// document.
fn is_not_element(object: &Expression) -> bool {
    if matches!(object, Expression::Identifier(name) if name.name == "document") {
        return true;
    }
    if let Expression::StaticMemberExpression(member) = object
        && member.property.name == "document"
        && matches!(&member.object, Expression::Identifier(name)
            if matches!(name.name.as_str(), "globalThis" | "window"))
    {
        return true;
    }
    match unwrap(object) {
        Expression::StaticMemberExpression(member) => matches!(
            member.property.name.as_str(),
            "shadowRoot" | "contentDocument" | "ownerDocument"
        ),
        Expression::CallExpression(call) => method_call(call).is_some_and(|method| {
            method.is_plain()
                && matches!(
                    method.name,
                    "attachShadow"
                        | "cloneContents"
                        | "createContextualFragment"
                        | "createDocumentFragment"
                        | "createShadowRoot"
                        | "extractContents"
                        | "getRootNode"
                        | "parseFromString"
                )
        }),
        _ => false,
    }
}

impl Rule for PreferScopedSelector {
    fn name(&self) -> &'static str {
        "unicorn/prefer-scoped-selector"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(method) = method_call(call).filter(|method| {
            matches!(method.name, "querySelector" | "querySelectorAll")
                && call.arguments.len() == 1
                && !call.arguments[0].is_spread()
        }) else {
            return;
        };
        if is_not_element(method.object) || is_not_dom_node(method.object) {
            return;
        }
        let Some(selector) = call.arguments[0].as_expression().and_then(static_string) else {
            return;
        };
        if selector.trim().is_empty()
            || branches(&selector)
                .iter()
                .all(|branch| !has_combinator(branch) || has_scope(branch))
        {
            return;
        }
        findings.push(source.finding(
            self.name(),
            method.property.start,
            "Use `:scope` in element selector queries.".to_owned(),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferScopedSelector;
    use crate::testing::lint;

    /// The places of the findings.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&PreferScopedSelector, "playground/a/b.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_element_queries_with_a_combinator() {
        // The documentation's case, then child, sibling and listed combinators.
        assert_eq!(
            check(
                "element.querySelectorAll('.outer .inner');\na.querySelector('a > b');\na.querySelector(`a+b`);\na.querySelector('a, b ~ c');\na?.querySelector('a b');"
            ),
            [(1, 9), (2, 3), (3, 3), (4, 3), (5, 4)]
        );
    }

    #[test]
    fn leaves_simple_scoped_and_document_queries() {
        for code in [
            "element.querySelectorAll(':scope .outer .inner');",
            "element.querySelector('.a, b');",
            "document.querySelectorAll('.outer .inner');",
            "window.document.querySelector('a b');",
            "element.shadowRoot.querySelectorAll('.outer .inner');",
            "element.getRootNode().querySelector('a b');",
            "element.querySelector('a[title=\"x y\"]');",
            "element.querySelector('a:not(b c)');",
            "element.querySelector('a\\\\ b');",
            "element.querySelector(' a ');",
            "element.querySelector('/* a b */a');",
            "element.querySelector(selector);",
            "element.querySelector(':SCOPE a b');",
            "element.querySelector('   ');",
            "'x'.querySelector('a b');",
        ] {
            assert_eq!(check(code), Vec::new(), "{code}");
        }
        assert_eq!(check("element.querySelector('::scope a b');").len(), 1);
        assert_eq!(check("element.querySelector(':scoped a b');").len(), 1);
    }
}
