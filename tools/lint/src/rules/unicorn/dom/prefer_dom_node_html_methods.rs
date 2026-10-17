//! `unicorn/prefer-dom-node-html-methods`: `.innerHTML` read is `.getHTML()`
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-dom-node-html-methods.md`, its default
//! options, `checkGetHTML` on and `checkSetHTML` off, as configured here). Autofixed unless a
//! comment is inside or the read is called or tags a template.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::expressions::{has_comment_in, is_written};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferDomNodeHtmlMethods;

/// Whether a read is called, constructed or tags a template, `?.` around it climbed.
fn is_callee_or_tag(source: &Source, node: &AstNode) -> bool {
    let nodes = source.semantic.nodes();
    let mut child = node.id();
    let mut parent = nodes.parent_node(child);
    if matches!(parent.kind(), AstKind::ChainExpression(_)) {
        child = parent.id();
        parent = nodes.parent_node(child);
    }
    let span = nodes.kind(child).span();
    match parent.kind() {
        AstKind::CallExpression(call) => call.callee.span() == span,
        AstKind::NewExpression(call) => call.callee.span() == span,
        AstKind::TaggedTemplateExpression(tagged) => tagged.tag.span() == span,
        _ => false,
    }
}

impl Rule for PreferDomNodeHtmlMethods {
    fn name(&self) -> &'static str {
        "unicorn/prefer-dom-node-html-methods"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::StaticMemberExpression(member) = node.kind() else {
            return;
        };
        if member.property.name != "innerHTML" || is_written(source, node) {
            return;
        }
        let message = "Prefer `.getHTML()` over `.innerHTML`.".to_owned();
        let start = member.property.span.start;
        if has_comment_in(source, member.span) || is_callee_or_tag(source, node) {
            findings.push(source.finding(self.name(), start, message));
            return;
        }
        let fix = Fix {
            start,
            end: member.span.end,
            text: "getHTML()".to_owned(),
        };
        findings.push(source.fixed(self.name(), start, message, fix));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferDomNodeHtmlMethods;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferDomNodeHtmlMethods, "playground/a/b.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn reads_the_html_with_get_html() {
        // The documentation's case.
        assert_eq!(
            check("const html = element.innerHTML;"),
            (vec![(1, 22)], "const html = element.getHTML();".to_owned())
        );
        // `?.`, a member after it; called or commented: reported only.
        assert_eq!(
            check("a?.innerHTML.length;\nb.innerHTML();\nc./* x */innerHTML;"),
            (
                vec![(1, 4), (2, 3), (3, 10)],
                "a?.getHTML().length;\nb.innerHTML();\nc./* x */innerHTML;".to_owned()
            )
        );
    }

    #[test]
    fn leaves_writes() {
        for code in [
            "const html = element.getHTML();",
            "element.innerHTML = userProvidedHTML;",
            "element.innerHTML += x;",
            "delete element.innerHTML;",
            "element['innerHTML'];",
            "element.setHTML(userInput);",
        ] {
            assert_eq!(check(code).0, Vec::new(), "{code}");
        }
    }
}
