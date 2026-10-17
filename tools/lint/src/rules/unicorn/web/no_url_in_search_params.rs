//! `unicorn/no-url-in-search-params`: `new URLSearchParams(…)` parses a query string, not a URL,
//! so a full URL string, a `URL`, its `href` or the location's `href` given to it is a mistake
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/no-url-in-search-params.md`). Reported only:
//! `ESLint` only suggests.

use oxc_ast::AstKind;
use oxc_ast::ast::{Argument, Expression};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::annotations::{Class, Targets, class_of_expression};
use crate::rules::unicorn::syntax::{Value, static_value, unwrap};
use crate::source::Source;

/// The rule.
pub struct NoUrlInSearchParams;

/// How a `URL` is read (`createBuiltinTypeCheckers({name: 'URL'})`).
const URLS: Targets = Targets {
    names: &["URL"],
    non_targets: &[],
    constructors: &["URL"],
    imports: &[("node:url", &["URL"]), ("url", &["URL"])],
    allow_nullish_in_mixed_union: false,
    mixed_union: Class::Unknown,
    prefer_definitions: true,
};

/// Whether a value is a `URL`.
fn is_url(source: &Source, node: &AstNode, expression: &Expression) -> bool {
    class_of_expression(source, node.scope_id(), expression, &URLS, 0) == Class::Target
}

/// Whether a value is the browser's location: `location`, or read from `window`, `globalThis`,
/// `document` or `self`.
fn is_location(expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::Identifier(name) => name.name == "location",
        Expression::StaticMemberExpression(member) => {
            !member.optional
                && member.property.name == "location"
                && matches!(&member.object, Expression::Identifier(object)
                    if matches!(object.name.as_str(), "window" | "globalThis" | "document" | "self"))
        }
        _ => false,
    }
}

/// Whether a string is a full URL a `URL` would parse: a scheme, `//`, and for the special
/// schemes but `file` a host.
fn is_full_url(value: &str) -> bool {
    let value = value.trim();
    let Some((scheme, rest)) = value.split_once("://") else {
        return false;
    };
    let mut characters = scheme.chars();
    let valid = characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic())
        && characters.all(|each| each.is_ascii_alphanumeric() || matches!(each, '+' | '-' | '.'));
    let special = matches!(
        scheme.to_ascii_lowercase().as_str(),
        "http" | "https" | "ws" | "wss" | "ftp"
    );
    let host = rest.split(['/', '?', '#']).next().unwrap_or_default();
    valid && (!special || !host.is_empty()) && !value.contains(char::is_whitespace)
}

impl Rule for NoUrlInSearchParams {
    fn name(&self) -> &'static str {
        "unicorn/no-url-in-search-params"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::NewExpression(new) = node.kind() else {
            return;
        };
        let constructor =
            matches!(&new.callee, Expression::Identifier(name) if name.name == "URLSearchParams");
        if !constructor || new.arguments.len() != 1 || new.arguments.iter().any(Argument::is_spread)
        {
            return;
        }
        let Some(argument) = new.arguments[0].as_expression() else {
            return;
        };
        let full = is_url(source, node, argument)
            || match unwrap(argument) {
                Expression::StaticMemberExpression(member)
                    if member.property.name == "href" && !member.optional =>
                {
                    is_url(source, node, &member.object) || is_location(&member.object)
                }
                _ => {
                    matches!(static_value(source, argument), Some(Value::Text(text)) if is_full_url(&text))
                }
            };
        if full {
            findings.push(source.finding(
                self.name(),
                argument.span().start,
                "`URLSearchParams` does not parse full URLs.".to_owned(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoUrlInSearchParams;
    use crate::testing::lint;

    /// The places of the findings.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&NoUrlInSearchParams, "scripts/a.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_full_urls() {
        // The documentation's cases.
        assert_eq!(
            check(
                "const parameters = new URLSearchParams('https://example.com/?query=hello');\nconst url = new URL(input);\nconst a = new URLSearchParams(url.href);\nconst b = new URLSearchParams(url);\nconst c = new URLSearchParams(window.location.href);"
            ),
            [(1, 40), (3, 31), (4, 31), (5, 31)]
        );
        // An annotation, a constant, `location`, a URL built inline.
        assert_eq!(
            check(
                "function f(u: URL) { return new URLSearchParams(u); }\nconst s = 'foo://x';\nnew URLSearchParams(s);\nnew URLSearchParams(location.href);\nnew URLSearchParams(new URL(a));"
            ),
            [(1, 49), (3, 21), (4, 21), (5, 21)]
        );
    }

    #[test]
    fn leaves_query_strings() {
        for code in [
            "const parameters = new URL('https://example.com/?query=hello').searchParams;",
            "const url = new URL(input); new URLSearchParams(url.search);",
            "new URLSearchParams(window.location.search);",
            "new URLSearchParams('a=1&b=2');",
            "new URLSearchParams('https://');",
            "new URLSearchParams(other.href);",
            "new URLSearchParams(query);",
            "new URLSearchParams(a, b);",
            "function f(u: string) { return new URLSearchParams(u); }",
        ] {
            assert_eq!(check(code), Vec::new(), "{code}");
        }
    }
}
