//! `unicorn/prefer-url-href`: a `URL` is stringified by `.href`, not `.toString()` nor
//! `String(…)` (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-url-href.md`). Autofixed for a
//! fresh `new URL(…)` with no comment inside; a `URL` named or annotated is reported only.

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::urls::{is_fresh_url, is_url};
use crate::expressions::{has_comment_in, is_parenthesized};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{method_call, object_text, symbol_of};
use crate::source::Source;

/// The rule.
pub struct PreferUrlHref;

/// The message.
const MESSAGE: &str = "Prefer `URL#href` over stringifying a `URL`.";

impl Rule for PreferUrlHref {
    fn name(&self) -> &'static str {
        "unicorn/prefer-url-href"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let commented = has_comment_in(source, call.span);
        if let Some(method) = method_call(call)
            && method.name == "toString"
            && method.is_plain()
            && call.arguments.is_empty()
        {
            if !is_url(source, method.object) {
                return;
            }
            let fixable = !commented
                && !is_parenthesized(source, call.callee.span())
                && is_fresh_url(source, method.object);
            findings.push(if fixable {
                let fix = Fix {
                    start: method.property.start,
                    end: call.span.end,
                    text: "href".to_owned(),
                };
                source.fixed(self.name(), method.property.start, MESSAGE.to_owned(), fix)
            } else {
                source.finding(self.name(), method.property.start, MESSAGE.to_owned())
            });
            return;
        }
        let is_string = matches!(&call.callee, Expression::Identifier(callee)
            if callee.name == "String" && symbol_of(source, callee).is_none());
        if !is_string || call.optional || call.arguments.len() != 1 {
            return;
        }
        let Some(argument) = call.arguments[0].as_expression() else {
            return;
        };
        if !is_url(source, argument) {
            return;
        }
        let start = call.callee.span().start;
        findings.push(if !commented && is_fresh_url(source, argument) {
            let fix = Fix {
                start: call.span.start,
                end: call.span.end,
                text: format!("{}.href", object_text(source, argument)),
            };
            source.fixed(self.name(), start, MESSAGE.to_owned(), fix)
        } else {
            source.finding(self.name(), start, MESSAGE.to_owned())
        });
    }
}

#[cfg(test)]
mod tests {
    use super::PreferUrlHref;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferUrlHref, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn reads_the_href() {
        // The documentation's three cases.
        assert_eq!(
            check("new URL('https://example.com').toString();\nconst url = new URL('https://example.com');\nString(url);\nfunction stringify(url: URL) {\n\treturn url.toString();\n}"),
            (
                vec![(1, 32), (3, 1), (5, 13)],
                "new URL('https://example.com').href;\nconst url = new URL('https://example.com');\nString(url);\nfunction stringify(url: URL) {\n\treturn url.toString();\n}"
                    .to_owned()
            )
        );
        // `String` of a fresh `URL`, an assertion, a conditional, a function's return type.
        assert_eq!(
            check("String(new URL(a));\n(x as URL).toString();\nString(c ? new URL(a) : new URL(b));\nfunction f(): URL { return u; }\nf().toString();"),
            (
                vec![(1, 1), (2, 12), (3, 1), (5, 5)],
                "new URL(a).href;\n(x as URL).toString();\n(c ? new URL(a) : new URL(b)).href;\nfunction f(): URL { return u; }\nf().toString();"
                    .to_owned()
            )
        );
    }

    #[test]
    fn leaves_other_values() {
        for code in [
            "new URL('https://example.com').href;",
            "url.toString();",
            "String(url);",
            "let u = new URL(a); u.toString();",
            "function f(u: string) { return u.toString(); }",
            "new URL(a)?.toString();",
            "new URL(a).toString(1);",
            "const String = f; String(new URL(a));",
            "class URL {} new URL(a).toString();",
            "new Other(a).toString();",
            "function f(u: URL | string) { return String(u); }",
        ] {
            assert_eq!(check(code).0, Vec::new(), "{code}");
        }
    }
}
