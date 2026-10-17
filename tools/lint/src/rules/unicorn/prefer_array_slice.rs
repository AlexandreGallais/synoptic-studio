//! `unicorn/prefer-array-slice`: no `Array#splice()` whose result is only read, by an index or
//! `.at()`, where `slice()` reads without changing the array (`eslint-plugin-unicorn` 78.0.0).
//! Its fix is a suggestion in `ESLint`: none here.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, UnaryOperator};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::syntax::{Kind, kind_of, method_call};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferArraySlice;

/// Whether the node a member is, the object of which is the call, is written or deleted rather
/// than read.
fn is_written(source: &Source, member: &AstNode) -> bool {
    match source.semantic.nodes().parent_kind(member.id()) {
        AstKind::AssignmentExpression(assignment) => assignment.left.span() == member.span(),
        AstKind::UpdateExpression(_)
        | AstKind::ArrayAssignmentTarget(_)
        | AstKind::AssignmentTargetWithDefault(_)
        | AstKind::AssignmentTargetPropertyProperty(_)
        | AstKind::AssignmentTargetRest(_)
        | AstKind::ForOfStatement(_)
        | AstKind::ForInStatement(_) => true,
        AstKind::UnaryExpression(unary) => unary.operator == UnaryOperator::Delete,
        _ => false,
    }
}

/// Whether the call's result is read by an index or by `.at(…)` with one argument.
fn is_read(source: &Source, node: &AstNode) -> bool {
    let nodes = source.semantic.nodes();
    let parent = nodes.parent_node(node.id());
    match parent.kind() {
        AstKind::ComputedMemberExpression(member) => {
            !member.optional && member.object.node_id() == node.id() && !is_written(source, parent)
        }
        AstKind::StaticMemberExpression(member) => {
            let at_call = matches!(
                nodes.parent_kind(parent.id()),
                AstKind::CallExpression(call)
                    if call.callee.node_id() == parent.id()
                        && !call.optional
                        && call.arguments.len() == 1
                        && call.arguments[0].as_expression().is_some()
            );
            !member.optional
                && member.property.name == "at"
                && member.object.node_id() == node.id()
                && at_call
        }
        _ => false,
    }
}

impl Rule for PreferArraySlice {
    fn name(&self) -> &'static str {
        "unicorn/prefer-array-slice"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(splice) = method_call(call).filter(|method| {
            method.name == "splice"
                && method.is_plain()
                && matches!(method.arguments().as_deref(), Some([_]))
        }) else {
            return;
        };
        let non_array = !matches!(kind_of(source, splice.object), None | Some(Kind::Array));
        if is_read(source, node) && !non_array && !matches!(splice.object, Expression::Super(_)) {
            findings.push(source.finding(
                self.name(),
                splice.property.start,
                "Prefer `Array#slice()` over `Array#splice()` when reading from the returned array."
                    .to_owned(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferArraySlice;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferArraySlice, "src/a.ts", code))
    }

    #[test]
    fn reports_the_documented_reads_of_a_splice() {
        let message =
            ["Prefer `Array#slice()` over `Array#splice()` when reading from the returned array."];

        assert_eq!(check("const foo = process.argv.splice(2)[0];"), message);
        assert_eq!(check("const foo = array.splice(index).at(0);"), message);
        assert_eq!(check("const foo = array.splice(index)[i + 1];"), message);
        assert_eq!(check("let x; x = array.splice(index)[0];"), message);
    }

    #[test]
    fn accepts_mutations_and_other_reads() {
        for code in [
            "array.splice(index);",
            "array.splice(index, deleteCount)[0];",
            "const foo = array.splice(index).length;",
            "const foo = array.splice(index)?.[0];",
            "const foo = array.splice(index)?.at(0);",
            "const foo = array.splice(index).at(0, 1);",
            "const foo = array.splice(index).at;",
            "array.splice(index)[0] = 1;",
            "array.splice(index)[0]++;",
            "delete array.splice(index)[0];",
            "[array.splice(index)[0]] = [1];",
            "for (array.splice(index)[0] of items) {}",
            "const foo = array?.splice(index)[0];",
            "const foo = x[array.splice(index)];",
            "const set = new Set(); const foo = set.splice(1)[0];",
            "const foo = new Uint8Array(4).splice(1)[0];",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
