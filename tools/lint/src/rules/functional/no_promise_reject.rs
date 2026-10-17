//! `functional/no-promise-reject`: no rejected promise (`eslint-plugin-functional` 10.0.1,
//! `docs/rules/no-promise-reject.md`): a call of `Promise.reject`, the second parameter of a
//! function given to `new Promise`, and a `throw` in an `async` function that no `try` with a
//! `catch` of the same function holds. `Promise` is read by its name, as the plugin reads it. No
//! fix.

use oxc_ast::AstKind;
use oxc_ast::ast::{Argument, CallExpression, Expression, FormalParameters, NewExpression};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoPromiseReject;

/// The plugin's message.
const MESSAGE: &str = "Unexpected rejection, resolve an error instead.";

/// Whether a call is `Promise.reject(…)`, or `Promise[reject](…)` as the plugin reads it.
fn is_reject_call(call: &CallExpression) -> bool {
    let (object, property) = match &call.callee {
        Expression::StaticMemberExpression(member) => {
            (&member.object, member.property.name.as_str())
        }
        Expression::ComputedMemberExpression(member) => match &member.expression {
            Expression::Identifier(name) => (&member.object, name.name.as_str()),
            _ => return false,
        },
        _ => return false,
    };
    property == "reject" && matches!(object, Expression::Identifier(name) if name.name == "Promise")
}

/// The start of the second of exactly two parameters.
fn second_of_two(parameters: &FormalParameters) -> Option<u32> {
    match (parameters.items.as_slice(), &parameters.rest) {
        ([_, second], None) => Some(second.span.start),
        ([_], Some(rest)) => Some(rest.span.start),
        _ => None,
    }
}

/// The start of the `reject` parameter of the executor given to `new Promise(…)`.
fn executor_reject(call: &NewExpression) -> Option<u32> {
    if !matches!(&call.callee, Expression::Identifier(name) if name.name == "Promise") {
        return None;
    }
    match call.arguments.first()? {
        Argument::ArrowFunctionExpression(arrow) => second_of_two(&arrow.params),
        Argument::FunctionExpression(function) => {
            let parameters = &function.params;
            if function.this_param.is_some() {
                return match (parameters.items.as_slice(), &parameters.rest) {
                    ([only], None) => Some(only.span.start),
                    ([], Some(rest)) => Some(rest.span.start),
                    _ => None,
                };
            }
            second_of_two(parameters)
        }
        _ => None,
    }
}

/// Whether a `throw` rejects a promise: in an `async` function, with no `try` of that function
/// holding it in its block, or the first such `try` without a `catch`.
fn rejects(node: &AstNode, source: &Source) -> bool {
    let nodes = source.semantic.nodes();
    let mut child = node.kind().span();
    let mut caught: Option<bool> = None;

    for ancestor in nodes.ancestors(node.id()) {
        match ancestor.kind() {
            AstKind::Function(function) => return function.r#async && caught != Some(true),
            AstKind::ArrowFunctionExpression(arrow) => {
                return arrow.r#async && caught != Some(true);
            }
            AstKind::TryStatement(statement)
                if caught.is_none() && statement.block.span == child =>
            {
                caught = Some(statement.handler.is_some());
            }
            _ => {}
        }
        child = ancestor.kind().span();
    }
    false
}

impl Rule for NoPromiseReject {
    fn name(&self) -> &'static str {
        "functional/no-promise-reject"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let start = match node.kind() {
            AstKind::CallExpression(call) if is_reject_call(call) => call.span.start,
            AstKind::NewExpression(call) => match executor_reject(call) {
                Some(start) => start,
                None => return,
            },
            AstKind::ThrowStatement(throw) if rejects(node, source) => throw.span.start,
            _ => return,
        };
        findings.push(source.finding(self.name(), start, MESSAGE.to_owned()));
    }
}

#[cfg(test)]
mod tests {
    use super::NoPromiseReject;
    use crate::testing::lint;

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&NoPromiseReject, "src/a/b.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_a_rejection() {
        // The documentation's case, then an executor's `reject` and a `throw` that rejects.
        assert_eq!(
            places(
                "async function divide(x, y) {\n  const [xv, yv] = await Promise.all([x, y]);\n  return yv === 0\n    ? Promise.reject(new Error(\"Cannot divide by zero.\"))\n    : xv / yv;\n}"
            ),
            [(4, 7)]
        );
        assert_eq!(
            places("const p = new Promise((resolve, reject) => reject(1));"),
            [(1, 33)]
        );
        assert_eq!(
            places("const p = new Promise(function (resolve, reject) {});"),
            [(1, 42)]
        );
        assert_eq!(
            places("const p = new Promise((resolve, ...rest) => 1);"),
            [(1, 33)]
        );
        assert_eq!(places("const q = Promise[reject](1);"), [(1, 11)]);
        assert_eq!(
            places("async function f() {\n  throw new Error();\n}"),
            [(2, 3)]
        );
        assert_eq!(
            places(
                "const f = async () => {\n  try {\n    g();\n  } finally {\n    h();\n  }\n  throw 1;\n};"
            ),
            [(7, 3)]
        );
        // A `try` without `catch` decides, though an outer one catches.
        assert_eq!(
            places(
                "async function f() {\n  try {\n    try {\n      throw 1;\n    } finally {}\n  } catch {}\n}"
            ),
            [(4, 7)]
        );
        // A `throw` in a `catch` is not in the `try`'s block.
        assert_eq!(
            places("async function f() {\n  try {\n    g();\n  } catch {\n    throw 1;\n  }\n}"),
            [(5, 5)]
        );
    }

    #[test]
    fn lets_a_resolved_error_and_a_caught_throw_through() {
        // The documentation's correct case, then a resolver alone, a throw in a synchronous
        // function or caught in its own function, and another `reject`.
        for code in [
            "async function divide(x, y) {\n  const [xv, yv] = await Promise.all([x, y]);\n  return yv === 0\n    ? { error: new Error(\"Cannot divide by zero.\") }\n    : { value: xv / yv };\n}",
            "const p = new Promise((resolve) => resolve(1));",
            "function f() {\n  throw new Error();\n}",
            "async function f() {\n  try {\n    throw 1;\n  } catch {}\n}",
            "throw 1;",
            "const r = other.reject(1);",
            "async function f() {\n  const g = () => {\n    throw 1;\n  };\n}",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
