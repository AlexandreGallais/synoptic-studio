//! `unicorn/prefer-split-limit`: a `split()` whose result is only read up to a known index, by
//! `[i]`, `.at(i)` or a destructuring without rest, passes that many parts as its limit
//! (`eslint-plugin-unicorn` 78.0.0). Autofixed, as in `ESLint`.

use oxc_ast::AstKind;
use oxc_ast::ast::{Argument, ArrayAssignmentTarget, ArrayPattern, CallExpression, Expression};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::known_values::{constant_value, number_text};
use crate::rules::unicorn::syntax::{Value, is_value_unused, method_call};
use crate::source::Source;

/// The rule.
pub struct PreferSplitLimit;

/// The largest limit `split()` takes.
const MAXIMUM_LIMIT: f64 = 4_294_967_295.0;

/// The `split(separator)` call an expression is, with a non-empty string or a regular
/// expression as its only argument.
fn split_without_limit<'b, 'a>(expression: &'b Expression<'a>) -> Option<&'b CallExpression<'a>> {
    let call = match expression {
        Expression::CallExpression(call) => call,
        Expression::ChainExpression(chain) => match &chain.expression {
            oxc_ast::ast::ChainElement::CallExpression(call) => call,
            _ => return None,
        },
        _ => return None,
    };
    let method = method_call(call)?;
    let separator = match call.arguments.as_slice() {
        [Argument::StringLiteral(literal)] => !literal.value.is_empty(),
        [Argument::RegExpLiteral(_)] => true,
        _ => false,
    };
    (method.name == "split" && separator).then_some(call)
}

/// The index an expression always is: a safe non-negative integer below the largest limit.
fn index_of(source: &Source, expression: &Expression) -> Option<f64> {
    let Some(Value::Number(number)) = constant_value(source, expression) else {
        return None;
    };
    let integer = number.trunc().to_bits() == number.to_bits();
    (integer && (0.0..MAXIMUM_LIMIT - 1.0).contains(&number) && number <= 9_007_199_254_740_991.0)
        .then_some(number)
}

/// The fix appending a limit to a `split()` call.
fn append_limit(source: &Source, call: &CallExpression, limit: &str) -> Fix {
    let close = call.span.end - 1;
    let before = source
        .text
        .get(..close as usize)
        .unwrap_or_default()
        .trim_end();
    let text = if before.ends_with(',') {
        format!(" {limit},")
    } else {
        format!(", {limit}")
    };
    Fix {
        start: close,
        end: close,
        text,
    }
}

impl PreferSplitLimit {
    /// Reports a read of a `split()` result up to a limit.
    fn report(
        &self,
        source: &Source,
        at: u32,
        call: &CallExpression,
        limit: &str,
        findings: &mut Vec<Finding>,
    ) {
        findings.push(source.fixed(
            self.name(),
            at,
            "Prefer `String#split()` with a limit.".to_owned(),
            append_limit(source, call, limit),
        ));
    }
}

/// The number of elements of a declaration's array pattern, when its last is no rest.
fn pattern_length(pattern: &ArrayPattern) -> Option<String> {
    (pattern.rest.is_none() && !pattern.elements.is_empty())
        .then(|| pattern.elements.len().to_string())
}

/// The number of elements of an assignment's array pattern, when its last is no rest.
fn target_length(pattern: &ArrayAssignmentTarget) -> Option<String> {
    (pattern.rest.is_none() && !pattern.elements.is_empty())
        .then(|| pattern.elements.len().to_string())
}

impl Rule for PreferSplitLimit {
    fn name(&self) -> &'static str {
        "unicorn/prefer-split-limit"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        match node.kind() {
            AstKind::ComputedMemberExpression(member) => {
                let written = match source.semantic.nodes().parent_kind(node.id()) {
                    AstKind::AssignmentExpression(assignment) => {
                        assignment.left.span() == member.span
                    }
                    AstKind::UpdateExpression(_)
                    | AstKind::ArrayAssignmentTarget(_)
                    | AstKind::AssignmentTargetRest(_) => true,
                    AstKind::AssignmentTargetWithDefault(target) => {
                        target.binding.span() == member.span
                    }
                    AstKind::AssignmentTargetPropertyProperty(property) => {
                        property.binding.span() == member.span
                    }
                    AstKind::ForOfStatement(statement) => statement.left.span() == member.span,
                    AstKind::ForInStatement(statement) => statement.left.span() == member.span,
                    AstKind::UnaryExpression(unary) => {
                        unary.operator == oxc_ast::ast::UnaryOperator::Delete
                    }
                    _ => false,
                };
                if written {
                    return;
                }
                let Some(call) = split_without_limit(&member.object) else {
                    return;
                };
                if let Some(index) = index_of(source, &member.expression) {
                    self.report(
                        source,
                        member.span.start,
                        call,
                        &number_text(index + 1.0),
                        findings,
                    );
                }
            }
            AstKind::CallExpression(at) => {
                let Some(method) = method_call(at) else {
                    return;
                };
                if method.name != "at" || at.arguments.len() != 1 {
                    return;
                }
                let Some(call) = split_without_limit(method.object) else {
                    return;
                };
                let index = at.arguments[0]
                    .as_expression()
                    .and_then(|argument| index_of(source, argument));
                if let Some(index) = index {
                    self.report(
                        source,
                        at.span.start,
                        call,
                        &number_text(index + 1.0),
                        findings,
                    );
                }
            }
            AstKind::VariableDeclarator(declarator) => {
                let oxc_ast::ast::BindingPattern::ArrayPattern(pattern) = &declarator.id else {
                    return;
                };
                let (Some(length), Some(call)) = (
                    pattern_length(pattern),
                    declarator.init.as_ref().and_then(split_without_limit),
                ) else {
                    return;
                };
                self.report(source, pattern.span.start, call, &length, findings);
            }
            AstKind::AssignmentExpression(assignment) => {
                let oxc_ast::ast::AssignmentTarget::ArrayAssignmentTarget(pattern) =
                    &assignment.left
                else {
                    return;
                };
                if !is_value_unused(source, node.id()) {
                    return;
                }
                let (Some(length), Some(call)) = (
                    target_length(pattern),
                    split_without_limit(&assignment.right),
                ) else {
                    return;
                };
                self.report(source, pattern.span.start, call, &length, findings);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferSplitLimit;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferSplitLimit, "playground/a.ts", code))
    }

    /// The code once fixed.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&PreferSplitLimit, "playground/a.ts", code))
    }

    #[test]
    fn fixes_the_documented_reads() {
        let code = "const protocol = url.split(':')[0];";
        assert_eq!(check(code), ["Prefer `String#split()` with a limit."]);
        assert_eq!(fix(code), "const protocol = url.split(':', 1)[0];");
        assert_eq!(
            fix("const [, filename] = path.split('/');"),
            "const [, filename] = path.split('/', 2);"
        );
        assert_eq!(
            fix("const part = csvLine.split(',')[3];"),
            "const part = csvLine.split(',', 4)[3];"
        );
        assert_eq!(
            fix("const a = s.split(/,/).at(1);"),
            "const a = s.split(/,/, 2).at(1);"
        );
        assert_eq!(fix("[a, b] = s.split(',',);"), "[a, b] = s.split(',', 2,);");
        assert_eq!(
            fix("const a = s?.split(',')[1 + 1];"),
            "const a = s?.split(',', 3)[1 + 1];"
        );
    }

    #[test]
    fn accepts_the_documented_full_splits() {
        for code in [
            "const lastPart = string.split('/').at(-1);",
            "const parts = string.split(dynamicSeparator)[unknownIndex];",
            "const [firstPart, ...remainingParts] = string.split('/');",
            "const a = string.split('')[0];",
            "const a = string.split('/', 2)[0];",
            "const a = string.split('/').length;",
            "string.split('/')[0] = 1;",
            "const x = ([a] = s.split(','));",
            "const [] = s.split(',');",
            "const a = string.split('/')[1.5];",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
