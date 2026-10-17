//! `unicorn/iteration-fallback-style`, in its default `guard` style: an optional loop source is
//! skipped by an `if` guard rather than replaced by an empty `[]` or `{}` fallback
//! (`eslint-plugin-unicorn` 78.0.0). Autofixed for a plain `for…of`, as in `ESLint`.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, LogicalOperator};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use super::syntax::{has_comment, has_side_effect, is_reference, method_named, text, unwrap};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct IterationFallbackStyle;

/// The most common step of indentation in a file: spaces, or a tab when tabs indent more lines
/// or nothing is indented (`getIndentUnit`).
fn indent_unit(text: &str) -> String {
    let mut steps: Vec<(usize, usize)> = Vec::new();
    let (mut tabs, mut spaces, mut previous) = (0, 0, 0);
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        let indent = &line[..line.len() - line.trim_start().len()];
        if indent.starts_with('\t') {
            tabs += 1;
            continue;
        }
        let width = indent.len();
        if width > 0 {
            spaces += 1;
        }
        let step = width.abs_diff(previous);
        if step > 0 {
            match steps.iter_mut().find(|(each, _)| *each == step) {
                Some((_, count)) => *count += 1,
                None => steps.push((step, 1)),
            }
        }
        previous = width;
    }
    let most = steps
        .iter()
        .max_by_key(|(step, count)| (*count, std::cmp::Reverse(*step)));
    match most {
        Some((step, _)) if spaces > tabs => " ".repeat(*step),
        _ => "\t".to_owned(),
    }
}

/// The blank text before a node on its line.
fn indent_before(source: &Source, start: u32) -> String {
    let before = source.text.get(..start as usize).unwrap_or_default();
    let line = before.rsplit('\n').next().unwrap_or_default();
    line[line.trim_end().len()..].to_owned()
}

/// The text with its lines after the first moved from one indentation to another
/// (`reindentText`).
fn reindent(text: &str, from: &str, to: &str) -> String {
    let mut lines = text.split('\n');
    let mut out = lines.next().unwrap_or_default().to_owned();
    for line in lines {
        out.push('\n');
        if line.trim().is_empty() {
            continue;
        }
        match line.strip_prefix(from) {
            Some(rest) => {
                out.push_str(to);
                out.push_str(rest);
            }
            None => out.push_str(line),
        }
    }
    out
}

impl Rule for IterationFallbackStyle {
    fn name(&self) -> &'static str {
        "unicorn/iteration-fallback-style"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let (right, is_for_of, awaits, span) = match node.kind() {
            AstKind::ForOfStatement(statement) => {
                (&statement.right, true, statement.r#await, statement.span)
            }
            AstKind::ForInStatement(statement) => (&statement.right, false, false, statement.span),
            _ => return,
        };
        let object_call = is_for_of
            .then(|| {
                method_named(unwrap(right), &["keys", "values", "entries"]).filter(|method| {
                    method.is_plain()
                        && matches!(method.object, Expression::Identifier(object) if object.name == "Object")
                        && method.call.arguments.len() == 1
                })
            })
            .flatten();
        let object_iteration = !is_for_of || object_call.is_some();
        let source_node = match object_call.and_then(|call| call.arguments()) {
            Some(arguments) => arguments[0],
            None => right,
        };
        let Expression::LogicalExpression(logical) = unwrap(source_node) else {
            return;
        };
        if !matches!(
            logical.operator,
            LogicalOperator::Coalesce | LogicalOperator::Or
        ) {
            return;
        }
        let fallback = unwrap(&logical.right);
        let empty = if object_iteration {
            matches!(fallback, Expression::ObjectExpression(object) if object.properties.is_empty())
        } else {
            matches!(fallback, Expression::ArrayExpression(array) if array.elements.is_empty())
        };
        if !empty {
            return;
        }
        let kind = if matches!(fallback, Expression::ArrayExpression(_)) {
            "array"
        } else {
            "object"
        };
        let message = format!("Prefer an `if` guard over the empty {kind} fallback.");
        let nodes = source.semantic.nodes();
        let wrapped_fallback = !matches!(
            logical.right,
            Expression::ArrayExpression(_) | Expression::ObjectExpression(_)
        );
        let multiline_token = source.text[span.start as usize..span.end as usize]
            .split('\n')
            .count()
            > 1
            && (text(source, span).contains('`'));
        let fixable = is_for_of
            && !awaits
            && !has_comment(source, logical.span)
            && !multiline_token
            && !wrapped_fallback
            && is_reference(&logical.left)
            && !has_side_effect(&logical.left)
            && !matches!(
                nodes.parent_kind(node.id()),
                AstKind::LabeledStatement(_) | AstKind::IfStatement(_)
            );
        if !fixable {
            findings.push(source.finding(self.name(), fallback.span().start, message));
            return;
        }
        let source_text = text(source, logical.left.span());
        let without = format!(
            "{}{source_text}{}",
            text(source, Span::new(span.start, logical.span.start)),
            text(source, Span::new(logical.span.end, span.end)),
        );
        let indent = indent_before(source, span.start);
        let body_indent = format!("{indent}{}", indent_unit(source.text));
        let condition = if logical.operator == LogicalOperator::Coalesce {
            format!("({source_text}) != null")
        } else {
            source_text.to_owned()
        };
        let replacement = format!(
            "if ({condition}) {{\n{body_indent}{}\n{indent}}}",
            reindent(&without, &indent, &body_indent)
        );
        findings.push(source.fixed(
            self.name(),
            fallback.span().start,
            message,
            Fix {
                start: span.start,
                end: span.end,
                text: replacement,
            },
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::{IterationFallbackStyle, indent_unit};
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&IterationFallbackStyle, "playground/a.ts", code))
    }

    /// The code after the fixes.
    fn fix(code: &str) -> String {
        fixed(
            code,
            &lint(&IterationFallbackStyle, "playground/a.ts", code),
        )
    }

    #[test]
    fn reports_and_fixes_the_documented_fallbacks() {
        assert_eq!(
            check("for (const item of items || []) {\n\tuse(item);\n}"),
            ["Prefer an `if` guard over the empty array fallback."]
        );
        assert_eq!(
            fix("for (const item of items || []) {\n\tuse(item);\n}"),
            "if (items) {\n\tfor (const item of items) {\n\t\tuse(item);\n\t}\n}"
        );
        assert_eq!(
            check(
                "for (const [key, value] of Object.entries(options.config || {})) {\n\tuse(key, value);\n}"
            ),
            ["Prefer an `if` guard over the empty object fallback."]
        );
        assert_eq!(
            fix(
                "function f() {\n  for (const [key, value] of Object.entries(options.config ?? {})) {\n    use(key, value);\n  }\n}"
            ),
            "function f() {\n  if ((options.config) != null) {\n    for (const [key, value] of Object.entries(options.config)) {\n      use(key, value);\n    }\n  }\n}"
        );
    }

    #[test]
    fn reports_without_a_fix_what_it_cannot_rewrite() {
        for code in [
            "async function f() { for await (const item of items || []) {} }",
            "for (const item of items /* c */ || []) {}",
            "for (const item of f() || []) {}",
            "if (a) for (const item of items || []) {}",
            "for (const key in object || {}) {}",
            "for (const item of items || ([] as T[])) {}",
        ] {
            let findings = lint(&IterationFallbackStyle, "playground/a.ts", code);
            assert_eq!(findings.len(), 1, "{code}");
            assert_eq!(fixed(code, &findings), code, "{code}");
        }
    }

    #[test]
    fn accepts_guards_and_other_sources() {
        for code in [
            "if (items) {\n\tfor (const item of items) {\n\t\tuse(item);\n\t}\n}",
            "for (const item of items || other) {}",
            "for (const item of items && []) {}",
            "for (const item of items || [1]) {}",
            "for (const item of items || {}) {}",
            "for (const key in object || []) {}",
            "for (const [k, v] of Object.entries(x || [])) {}",
            "for (const [k, v] of Object.entries(x, y || {})) {}",
            "for (let i = 0; i < 1; i += 1) {}",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }

    #[test]
    fn finds_the_file_s_indentation_step() {
        assert_eq!(indent_unit("a\n  b\n    c\n  d\n"), "  ");
        assert_eq!(indent_unit("a\n\tb\n\t\tc\n"), "\t");
        assert_eq!(indent_unit("a\nb\n"), "\t");
    }
}
