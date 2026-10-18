//! `unicorn/no-mismatched-map-key`: no access to a `Map` by another key than the one `has()`
//! just checked, in the branch that check guards, a copy-and-paste slip (`eslint-plugin-unicorn`
//! 78.0.0).

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, LogicalOperator, Statement, UnaryOperator};
use oxc_semantic::{AstNode, NodeId};
use oxc_span::GetSpan;

use super::syntax::{
    Kind, MethodCall, descendants, has_call, is_function_or_class, is_reference, kind_of,
    method_call, path_up, root_identifier, same_binding, same_reference_binding, single_statement,
    static_value, text, unwrap,
};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoMismatchedMapKey;

/// Its message.
const MESSAGE: &str = "Do not check a Map key before accessing a different key.";

/// A plain `map.has(key)` call, through TypeScript's wrappers.
fn has_call_of<'b, 'a>(expression: &'b Expression<'a>) -> Option<MethodCall<'b, 'a>> {
    let Expression::CallExpression(call) = unwrap(expression) else {
        return None;
    };
    method_call(call).filter(|method| {
        method.name == "has"
            && method.is_plain()
            && matches!(method.arguments().as_deref(), Some([_]))
    })
}

/// The `has()` call a test makes, negated or not.
fn guard_call<'b, 'a>(expression: &'b Expression<'a>) -> Option<MethodCall<'b, 'a>> {
    match unwrap(expression) {
        Expression::UnaryExpression(unary) if unary.operator == UnaryOperator::LogicalNot => {
            has_call_of(&unary.argument)
        }
        other => has_call_of(other),
    }
}

/// The last operand of an `&&` chain, the one that admits the guarded code.
fn guard_test<'b, 'a>(mut test: &'b Expression<'a>) -> &'b Expression<'a> {
    while let Expression::LogicalExpression(logical) = test {
        if logical.operator != LogicalOperator::And {
            break;
        }
        test = &logical.right;
    }
    test
}

/// Whether two keys are known to differ: different static values, or different references,
/// neither calling anything.
fn different_keys(source: &Source, left: &Expression, right: &Expression) -> bool {
    if has_call(left) || has_call(right) {
        return false;
    }
    if let (Some(a), Some(b)) = (static_value(source, left), static_value(source, right)) {
        return !a.same_value_zero(&b);
    }
    is_reference(left) && is_reference(right) && !same_reference_binding(source, left, right)
}

/// Whether a node is an `if` or a ternary whose test checks the same map again.
fn is_nested_guard(source: &Source, node: &AstNode, receiver: &Expression) -> bool {
    let test = match node.kind() {
        AstKind::IfStatement(statement) => &statement.test,
        AstKind::ConditionalExpression(conditional) => &conditional.test,
        _ => return false,
    };
    let test_id = test.node_id();
    std::iter::once(source.semantic.nodes().get_node(test_id))
        .chain(descendants(source, test_id))
        .any(|each| match each.kind() {
            AstKind::CallExpression(call) => method_call(call).is_some_and(|method| {
                method.name == "has"
                    && method.is_plain()
                    && same_reference_binding(source, receiver, method.object)
            }),
            _ => false,
        })
}

/// Whether a node of a branch is out of the guard's reach: in a function or a class, or under
/// another check of the same map.
fn is_out_of_reach(source: &Source, node: NodeId, branch: NodeId, receiver: &Expression) -> bool {
    path_up(source, node, branch)
        .any(|each| is_function_or_class(&each.kind()) || is_nested_guard(source, each, receiver))
}

/// Whether a node writes the map's receiver or its root binding.
fn writes_receiver(source: &Source, node: &AstNode, receiver: &Expression) -> bool {
    let receiver_text = text(source, unwrap(receiver).span());
    match node.kind() {
        AstKind::IdentifierReference(identifier) => {
            let write = identifier.reference_id.get().is_some_and(|reference| {
                source
                    .semantic
                    .scoping()
                    .get_reference(reference)
                    .is_write()
            });
            write
                && root_identifier(receiver)
                    .is_some_and(|root| same_binding(source, root, identifier))
        }
        AstKind::AssignmentExpression(assignment) => {
            text(source, assignment.left.span()) == receiver_text
        }
        AstKind::UpdateExpression(update) => text(source, update.argument.span()) == receiver_text,
        _ => false,
    }
}

/// The findings on the accesses of a branch by another key than the checked one.
fn branch_findings(
    source: &Source,
    rule: &'static str,
    branch: NodeId,
    guard: &MethodCall,
    findings: &mut Vec<Finding>,
) {
    let nodes = source.semantic.nodes();
    let receiver = guard.object;
    let subtree: Vec<&AstNode> = std::iter::once(nodes.get_node(branch))
        .chain(descendants(source, branch))
        .filter(|node| !is_out_of_reach(source, node.id(), branch, receiver))
        .collect();
    if subtree
        .iter()
        .any(|node| writes_receiver(source, node, receiver))
    {
        return;
    }
    let Some(&[checked]) = guard.arguments().as_deref() else {
        return;
    };
    for node in subtree {
        let AstKind::CallExpression(call) = node.kind() else {
            continue;
        };
        let Some(access) = method_call(call)
            .filter(|method| matches!(method.name, "get" | "set") && method.is_plain())
        else {
            continue;
        };
        let Some(key) = access
            .arguments()
            .and_then(|arguments| arguments.first().copied())
        else {
            continue;
        };
        if !same_reference_binding(source, receiver, access.object)
            || !different_keys(source, checked, key)
        {
            continue;
        }
        let finding = source.finding(rule, key.span().start, MESSAGE.to_owned());
        let reported = findings.iter().any(|each| {
            each.rule == rule && each.line == finding.line && each.column == finding.column
        });
        if !reported {
            findings.push(finding);
        }
    }
}

impl Rule for NoMismatchedMapKey {
    fn name(&self) -> &'static str {
        "unicorn/no-mismatched-map-key"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let (guard, branches): (Option<MethodCall>, Vec<NodeId>) = match node.kind() {
            AstKind::LogicalExpression(logical) => {
                (guard_call(&logical.left), vec![logical.right.node_id()])
            }
            AstKind::ConditionalExpression(conditional) => (
                guard_call(guard_test(&conditional.test)),
                vec![
                    conditional.consequent.node_id(),
                    conditional.alternate.node_id(),
                ],
            ),
            AstKind::WhileStatement(statement) => (
                guard_call(guard_test(&statement.test)),
                single_statement(&statement.body)
                    .map(Statement::node_id)
                    .into_iter()
                    .collect(),
            ),
            AstKind::IfStatement(statement) => {
                let alternate = statement
                    .alternate
                    .as_ref()
                    .filter(|alternate| !matches!(alternate, Statement::IfStatement(_)))
                    .and_then(single_statement);
                (
                    guard_call(guard_test(&statement.test)),
                    single_statement(&statement.consequent)
                        .into_iter()
                        .chain(alternate)
                        .map(Statement::node_id)
                        .collect(),
                )
            }
            _ => return,
        };
        let Some(guard) = guard else {
            return;
        };
        if matches!(kind_of(source, guard.object), Some(kind) if kind != Kind::Map) {
            return;
        }
        for branch in branches {
            branch_findings(source, self.name(), branch, &guard, findings);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoMismatchedMapKey;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoMismatchedMapKey, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_mismatched_keys() {
        let message = ["Do not check a Map key before accessing a different key."];

        assert_eq!(
            check("const value = map.has(key) ? map.get(anotherKey) : fallback;"),
            message
        );
        assert_eq!(
            check("if (!map.has(key)) {\n\tmap.set(anotherKey, value);\n}"),
            message
        );
        assert_eq!(check("const v = map.has('a') && map.get('b');"), message);
        assert_eq!(check("while (map.has(a)) map.get(b);"), message);
        assert_eq!(
            check("if (ready && map.has(a)) { use(map.get(b)); }"),
            message
        );
        assert_eq!(check("if (map.has(a)) {} else map.set(b, 1);"), message);
        assert_eq!(
            check("const v = this.cache.has(a.id) ? this.cache.get(a.key) : 0;"),
            message
        );
        // The finding sits on the key: `const value = map.has(key) ? map.get(` is 37 characters.
        let findings = lint(
            &NoMismatchedMapKey,
            "playground/a.ts",
            "const value = map.has(key) ? map.get(anotherKey) : fallback;",
        );
        assert_eq!(findings[0].column, 38);
    }

    #[test]
    fn accepts_the_same_key_and_what_the_guard_does_not_reach() {
        for code in [
            "const value = map.has(key) ? map.get(key) : fallback;",
            "if (!map.has(key)) {\n\tmap.set(key, value);\n}",
            "const v = map.has(1) && map.get(1.0);",
            "const v = map.has(key) ? other.get(anotherKey) : 0;",
            "const v = map.has(f()) ? map.get(g) : 0;",
            "const v = map.has(key) ? map.get(make()) : 0;",
            "const v = map.has(key) ? map.get(`${a}`) : 0;",
            "if (map.has(a)) { use(); map.get(b); }",
            "if (map.has(a) || ready) { map.get(b); }",
            "if (map.has(a)) { list.forEach(() => map.get(b)); }",
            "if (map.has(a)) { if (map.has(b)) { map.get(b); } }",
            "if (map.has(a)) map = other, map.get(b);",
            "do map.get(b); while (map.has(a));",
            "const v = map.get(b) && map.has(a);",
            "const v = map?.has(a) ? map.get(b) : 0;",
            "const set = new Set(); const v = set.has(a) ? set.get(b) : 0;",
            "const cache = new WeakMap(); const v = cache.has(a) ? cache.get(b) : 0;",
            "function f(a: Map<string, number>, other: Map<string, number>) { return a.has(k) ? other.get(j) : 0; }",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
