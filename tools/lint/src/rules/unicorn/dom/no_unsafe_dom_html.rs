//! `unicorn/no-unsafe-dom-html`: the DOM APIs that parse HTML into the page are injection
//! sinks — writing `innerHTML`, `outerHTML` or `srcdoc`, `insertAdjacentHTML()`,
//! `setHTMLUnsafe()`, `createContextualFragment()`, `setAttribute('srcdoc', …)`,
//! `Document.parseHTMLUnsafe()`, `document.write()` and `document.writeln()`
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/no-unsafe-dom-html.md`).

use oxc_ast::AstKind;
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::nodes::{is_global_path, property_start, static_property_name, static_string};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoUnsafeDomHtml;

/// The message.
const MESSAGE: &str = "Do not use unsafe DOM HTML APIs.";

/// Whether a member is written: assigned or destructured into (`isHtmlSetter`).
fn is_setter(source: &Source, node: &AstNode) -> bool {
    let span = node.kind().span();
    match source.semantic.nodes().parent_kind(node.id()) {
        AstKind::AssignmentExpression(assignment) => assignment.left.span() == span,
        AstKind::ForOfStatement(statement) => statement.left.span() == span,
        AstKind::ForInStatement(statement) => statement.left.span() == span,
        AstKind::AssignmentTargetWithDefault(target) => target.binding.span() == span,
        AstKind::ArrayAssignmentTarget(_)
        | AstKind::AssignmentTargetPropertyProperty(_)
        | AstKind::AssignmentTargetRest(_) => true,
        _ => false,
    }
}

impl Rule for NoUnsafeDomHtml {
    fn name(&self) -> &'static str {
        "unicorn/no-unsafe-dom-html"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let found = match node.kind() {
            AstKind::StaticMemberExpression(member) => (matches!(
                member.property.name.as_str(),
                "innerHTML" | "outerHTML" | "srcdoc"
            ) && is_setter(source, node))
            .then_some(member.property.span.start),
            AstKind::ComputedMemberExpression(member) => (static_string(&member.expression)
                .is_some_and(|name| matches!(name.as_str(), "innerHTML" | "outerHTML" | "srcdoc"))
                && is_setter(source, node))
            .then(|| member.expression.span().start),
            AstKind::CallExpression(call) => {
                let method = static_property_name(&call.callee);
                let html = method.as_deref().is_some_and(|name| {
                    matches!(
                        name,
                        "createContextualFragment" | "insertAdjacentHTML" | "setHTMLUnsafe"
                    )
                });
                let srcdoc = method.as_deref() == Some("setAttribute")
                    && call.arguments.len() >= 2
                    && call.arguments[0]
                        .as_expression()
                        .and_then(static_string)
                        .is_some_and(|name| name.eq_ignore_ascii_case("srcdoc"));
                if html || srcdoc {
                    property_start(&call.callee)
                } else if is_global_path(source, &call.callee, &["Document", "parseHTMLUnsafe"])
                    || is_global_path(source, &call.callee, &["document", "write"])
                    || is_global_path(source, &call.callee, &["document", "writeln"])
                {
                    Some(call.callee.span().start)
                } else {
                    None
                }
            }
            _ => None,
        };
        if let Some(offset) = found {
            findings.push(source.finding(self.name(), offset, MESSAGE.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoUnsafeDomHtml;
    use crate::testing::lint;

    /// The places of the findings.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&NoUnsafeDomHtml, "playground/a/b.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_the_html_sinks() {
        // The documentation's ten cases.
        assert_eq!(
            check(
                "element.innerHTML = html;\nelement.outerHTML = html;\niframe.srcdoc = html;\nelement.insertAdjacentHTML('beforeend', html);\nelement.setHTMLUnsafe(html);\nDocument.parseHTMLUnsafe(html);\nrange.createContextualFragment(html);\niframe.setAttribute('srcdoc', html);\ndocument.write(html);\ndocument.writeln(html);"
            ),
            [
                (1, 9),
                (2, 9),
                (3, 8),
                (4, 9),
                (5, 9),
                (6, 1),
                (7, 7),
                (8, 8),
                (9, 1),
                (10, 1)
            ]
        );
        // Computed names, `+=`, destructuring, `?.` and `window.`.
        assert_eq!(
            check(
                "el['innerHTML'] += x;\n[el.outerHTML] = list;\n({ a: el.srcdoc } = o);\nel?.insertAdjacentHTML('a', b);\nwindow.document.write(x);\nf.setAttribute('SRCDOC', x);\nconst w = document.write; w(1);"
            ),
            [(1, 4), (2, 5), (3, 10), (4, 5), (5, 1), (6, 3), (7, 27)]
        );
    }

    #[test]
    fn leaves_safe_apis_and_reads() {
        for code in [
            "element.setHTML(html);",
            "Document.parseHTML(html);",
            "element.textContent = text;",
            "element.insertAdjacentText('beforeend', text);",
            "const html = element.innerHTML;",
            "element.innerHTML++;",
            "iframe.setAttribute('srcdoc');",
            "const document = x; document.write(html);",
            "other.write(html);",
        ] {
            assert_eq!(check(code), Vec::new(), "{code}");
        }
    }
}
