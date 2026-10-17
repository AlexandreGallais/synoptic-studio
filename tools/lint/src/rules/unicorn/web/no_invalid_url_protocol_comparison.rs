//! `unicorn/no-invalid-url-protocol-comparison`: `URL#protocol` is a lowercase scheme ending with
//! `:`, so a comparison with `'https'` or `'HTTPS:'` never matches; the string is rewritten in
//! equalities, `switch` cases, inline arrays' `includes()`, `indexOf()` and `lastIndexOf()`,
//! and `new Set([…]).has()` (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/no-invalid-url-protocol-comparison.md`). Autofixed, but for a case whose
//! rewrite would repeat another case.

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryOperator, Expression};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::urls::is_url;
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::dom::nodes::{static_property_name, static_string};
use crate::rules::unicorn::syntax::{method_call, unwrap};
use crate::source::Source;

/// The rule.
pub struct NoInvalidUrlProtocolComparison;

/// The protocol a scheme string stands for, lowercase and ending with `:`.
fn normalized(expression: &Expression) -> Option<String> {
    let value = static_string(unwrap(expression))?;
    let scheme = value.strip_suffix(':').unwrap_or(&value);
    let mut characters = scheme.chars();
    let valid = characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic())
        && characters.all(|each| each.is_ascii_alphanumeric() || matches!(each, '+' | '-' | '.'));
    valid.then(|| format!("{}:", scheme.to_ascii_lowercase()))
}

/// Whether an expression reads `protocol` of a `URL`.
fn is_protocol(source: &Source, expression: &Expression) -> bool {
    let mut expression = unwrap(expression);
    if let Expression::ChainExpression(chain) = expression {
        let Some(member) = chain.expression.as_member_expression() else {
            return false;
        };
        return member
            .static_property_name()
            .is_some_and(|name| name == "protocol")
            && is_url(source, member.object());
    }
    expression = unwrap(expression);
    static_property_name(expression).as_deref() == Some("protocol")
        && match expression {
            Expression::StaticMemberExpression(member) => is_url(source, &member.object),
            Expression::ComputedMemberExpression(member) => is_url(source, &member.object),
            _ => false,
        }
}

/// The finding of a protocol string that is not one, with its fix when allowed.
fn problem(
    source: &Source,
    rule: &'static str,
    expression: &Expression,
    fixable: bool,
) -> Option<(Finding, String)> {
    let expression = unwrap(expression);
    let protocol = normalized(expression)?;
    if static_string(expression).as_deref() == Some(protocol.as_str()) {
        return None;
    }
    let span = expression.span();
    let message =
        format!("`URL#protocol` values are lowercase and end with `:`. Compare with `{protocol}`.");
    let finding = if fixable {
        let fix = Fix {
            start: span.start + 1,
            end: span.end - 1,
            text: protocol.clone(),
        };
        source.fixed(rule, span.start, message, fix)
    } else {
        source.finding(rule, span.start, message)
    };
    Some((finding, protocol))
}

impl Rule for NoInvalidUrlProtocolComparison {
    fn name(&self) -> &'static str {
        "unicorn/no-invalid-url-protocol-comparison"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        match node.kind() {
            AstKind::BinaryExpression(binary) => {
                if !matches!(
                    binary.operator,
                    BinaryOperator::Equality
                        | BinaryOperator::StrictEquality
                        | BinaryOperator::Inequality
                        | BinaryOperator::StrictInequality
                ) {
                    return;
                }
                let other = if is_protocol(source, &binary.left) {
                    &binary.right
                } else if is_protocol(source, &binary.right) {
                    &binary.left
                } else {
                    return;
                };
                if let Some((finding, _)) = problem(source, self.name(), other, true) {
                    findings.push(finding);
                }
            }
            AstKind::SwitchStatement(statement) => {
                if !is_protocol(source, &statement.discriminant) {
                    return;
                }
                let protocols: Vec<Option<String>> = statement
                    .cases
                    .iter()
                    .map(|case| case.test.as_ref().and_then(normalized))
                    .collect();
                for (index, case) in statement.cases.iter().enumerate() {
                    let Some(test) = &case.test else {
                        continue;
                    };
                    let Some(protocol) = normalized(unwrap(test)) else {
                        continue;
                    };
                    let repeated = protocols.iter().enumerate().any(|(other, each)| {
                        other != index && each.as_deref() == Some(protocol.as_str())
                    });
                    if let Some((finding, _)) = problem(source, self.name(), test, !repeated) {
                        findings.push(finding);
                    }
                }
            }
            AstKind::CallExpression(call) => {
                let Some(method) = method_call(call) else {
                    return;
                };
                let array = match method.name {
                    "includes" | "indexOf" | "lastIndexOf"
                        if (1..=2).contains(&call.arguments.len()) =>
                    {
                        unwrap(method.object)
                    }
                    "has" if call.arguments.len() == 1 => match unwrap(method.object) {
                        Expression::NewExpression(set)
                            if set.arguments.len() == 1
                                && matches!(&set.callee, Expression::Identifier(name) if name.name == "Set") =>
                        {
                            match set.arguments[0].as_expression() {
                                Some(values) => unwrap(values),
                                None => return,
                            }
                        }
                        _ => return,
                    },
                    _ => return,
                };
                let Expression::ArrayExpression(array) = array else {
                    return;
                };
                if call.arguments.iter().any(oxc_ast::ast::Argument::is_spread)
                    || !call.arguments[0]
                        .as_expression()
                        .is_some_and(|value| is_protocol(source, value))
                {
                    return;
                }
                for element in &array.elements {
                    if let Some(value) = element.as_expression()
                        && let Some((finding, _)) = problem(source, self.name(), value, true)
                    {
                        findings.push(finding);
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoInvalidUrlProtocolComparison;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&NoInvalidUrlProtocolComparison, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn rewrites_protocols_as_url_gives_them() {
        // The documentation's cases.
        assert_eq!(
            check("const url = new URL('https://example.com');\nurl.protocol === 'https';\nurl.protocol !== 'HTTP:';\nswitch (url.protocol) {\n\tcase 'https': {\n\t\tbreak;\n\t}\n}\n['http', 'HTTPS:'].includes(url.protocol);\n['https'].indexOf(url.protocol) !== -1;\nnew Set(['http', 'https']).has(url.protocol);"),
            (
                vec![(2, 18), (3, 18), (5, 7), (9, 2), (9, 10), (10, 2), (11, 10), (11, 18)],
                "const url = new URL('https://example.com');\nurl.protocol === 'https:';\nurl.protocol !== 'http:';\nswitch (url.protocol) {\n\tcase 'https:': {\n\t\tbreak;\n\t}\n}\n['http:', 'https:'].includes(url.protocol);\n['https:'].indexOf(url.protocol) !== -1;\nnew Set(['http:', 'https:']).has(url.protocol);"
                    .to_owned()
            )
        );
        // A case whose rewrite repeats another is reported only; a custom scheme, a template.
        assert_eq!(
            check("function f(u: URL) {\n  switch (u.protocol) { case 'http': break; case 'http:': break; }\n  return u['protocol'] == `web+demo` || 'X' === u?.protocol;\n}"),
            (
                vec![(2, 30), (3, 27), (3, 41)],
                "function f(u: URL) {\n  switch (u.protocol) { case 'http': break; case 'http:': break; }\n  return u['protocol'] == `web+demo:` || 'x:' === u?.protocol;\n}"
                    .to_owned()
            )
        );
    }

    #[test]
    fn leaves_valid_and_unknown_comparisons() {
        for code in [
            "const url = new URL('https://example.com');\nurl.protocol === 'https:';\n['http:', 'https:'].includes(url.protocol);",
            "const url = new URL('http://example.com');\nurl.protocol = 'https';",
            "other.protocol === 'https';",
            "const url = new URL(a); url.protocol.startsWith('http');",
            "const url = new URL(a); url.protocol === '';",
            "const url = new URL(a); url.protocol === '1http';",
            "const url = new URL(a); url.protocol === `${a}`;",
            "const url = new URL(a); list.includes(url.protocol);",
            "const url = new URL(a); url.protocol < 'https';",
        ] {
            assert_eq!(check(code).0, Vec::new(), "{code}");
        }
    }
}
