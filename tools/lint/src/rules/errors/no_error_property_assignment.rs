//! `unicorn/no-error-property-assignment`: `name`, `stack` and `cause` — and `errors` of an
//! `AggregateError` — written on a value known to be a built-in error, by an assignment, an
//! update or `Object.assign` (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/no-error-property-assignment.md`). A value is known when it is a built-in error's
//! construction, or a `const` holding one, declared directly in a block: the plugin follows `let`
//! reassignments too, which cannot occur here (`functional/no-let`). No fix.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Argument, AssignmentTarget, BindingPattern, Expression, ObjectPropertyKind,
    SimpleAssignmentTarget, VariableDeclarationKind,
};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::expressions::{is_global, symbol, unwrapped};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoErrorPropertyAssignment;

/// The language's error constructors.
pub const BUILTIN_ERRORS: [&str; 9] = [
    "Error",
    "EvalError",
    "RangeError",
    "ReferenceError",
    "SyntaxError",
    "TypeError",
    "URIError",
    "AggregateError",
    "SuppressedError",
];

/// The built-in error a construction makes, `new TypeError(…)` or `TypeError(…)`.
fn constructed(source: &Source, expression: &Expression) -> Option<&'static str> {
    let callee = match unwrapped(expression) {
        Expression::NewExpression(call) => &call.callee,
        Expression::CallExpression(call) if !call.optional => &call.callee,
        _ => return None,
    };
    match callee {
        Expression::Identifier(name) if is_global(source, name) => BUILTIN_ERRORS
            .iter()
            .copied()
            .find(|each| *each == name.name.as_str()),
        _ => None,
    }
}

/// The built-in error a value is known to be: a construction, or a `const` declared directly in
/// a block holding one, before the value is read.
fn known(source: &Source, expression: &Expression) -> Option<&'static str> {
    if let Some(found) = constructed(source, expression) {
        return Some(found);
    }
    let Expression::Identifier(name) = unwrapped(expression) else {
        return None;
    };
    let found = symbol(source, name)?;
    let nodes = source.semantic.nodes();
    let declaration = source.semantic.scoping().symbol_declaration(found);
    let AstKind::VariableDeclarator(declarator) = nodes.kind(declaration) else {
        return None;
    };
    if declarator.span.end > name.span.start {
        return None;
    }
    let statement = nodes.parent_id(declaration);
    let direct = matches!(nodes.kind(statement), AstKind::VariableDeclaration(each) if each.kind == VariableDeclarationKind::Const)
        && matches!(
            nodes.parent_kind(statement),
            AstKind::Program(_)
                | AstKind::BlockStatement(_)
                | AstKind::FunctionBody(_)
                | AstKind::StaticBlock(_)
                | AstKind::SwitchCase(_)
                | AstKind::ForStatement(_)
        );
    if !direct || !matches!(declarator.id, BindingPattern::BindingIdentifier(_)) {
        return None;
    }
    constructed(source, declarator.init.as_ref()?)
}

/// Whether a property may not be written on a built-in error.
fn forbidden(constructor: &str, property: &str) -> bool {
    matches!(property, "name" | "stack" | "cause")
        || (constructor == "AggregateError" && property == "errors")
}

/// The message on a property.
fn message(property: &str) -> String {
    format!("Do not assign to `{property}` on a built-in error.")
}

impl NoErrorPropertyAssignment {
    /// The finding on a written member, `error.stack = …`.
    fn written(&self, source: &Source, target: &SimpleAssignmentTarget) -> Option<Finding> {
        let member = target.as_member_expression()?;
        if member.optional() {
            return None;
        }
        let constructor = known(source, member.object())?;
        let (property, at) = match member {
            oxc_ast::ast::MemberExpression::StaticMemberExpression(each) => {
                (each.property.name.as_str(), each.property.span.start)
            }
            oxc_ast::ast::MemberExpression::ComputedMemberExpression(each) => {
                match &each.expression {
                    Expression::StringLiteral(key) => (key.value.as_str(), key.span.start),
                    _ => return None,
                }
            }
            oxc_ast::ast::MemberExpression::PrivateFieldExpression(_) => return None,
        };
        forbidden(constructor, property).then(|| source.finding(self.name(), at, message(property)))
    }
}

impl Rule for NoErrorPropertyAssignment {
    fn name(&self) -> &'static str {
        "unicorn/no-error-property-assignment"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        match node.kind() {
            AstKind::AssignmentExpression(assignment) => {
                if let AssignmentTarget::StaticMemberExpression(_)
                | AssignmentTarget::ComputedMemberExpression(_) = &assignment.left
                    && let Some(target) = assignment.left.as_simple_assignment_target()
                {
                    findings.extend(self.written(source, target));
                }
            }
            AstKind::UpdateExpression(update) => {
                findings.extend(self.written(source, &update.argument));
            }
            AstKind::CallExpression(call) => {
                let assign = matches!(&call.callee, Expression::StaticMemberExpression(member)
                    if !member.optional && member.property.name == "assign"
                        && matches!(&member.object, Expression::Identifier(object) if object.name == "Object" && is_global(source, object)));
                if !assign || call.optional || call.arguments.len() < 2 {
                    return;
                }
                let Some(constructor) = call.arguments[0]
                    .as_expression()
                    .and_then(|target| known(source, target))
                else {
                    return;
                };
                for argument in &call.arguments[1..] {
                    let Argument::ObjectExpression(object) = argument else {
                        continue;
                    };
                    for property in &object.properties {
                        let ObjectPropertyKind::ObjectProperty(property) = property else {
                            continue;
                        };
                        if let Some(name) = property.key.static_name()
                            && forbidden(constructor, &name)
                        {
                            findings.push(source.finding(
                                self.name(),
                                property.key.span().start,
                                message(&name),
                            ));
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoErrorPropertyAssignment;
    use crate::testing::{lint, messages};

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&NoErrorPropertyAssignment, "scripts/a.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_built_in_properties_written_on_known_errors() {
        // The documentation's cases.
        assert_eq!(
            places("Object.assign(new Error('message'), {name});"),
            [(1, 38)]
        );
        assert_eq!(
            places("const error = new Error('message');\nerror.stack = stack;"),
            [(2, 7)]
        );
        assert_eq!(
            places("Object.assign(new AggregateError(errors, 'message'), {errors});"),
            [(1, 55)]
        );
        assert_eq!(
            messages(&lint(
                &NoErrorPropertyAssignment,
                "scripts/a.ts",
                "const e = TypeError('x');\ne['cause'] = c;\ne.name += 'x';"
            )),
            [
                "Do not assign to `cause` on a built-in error.",
                "Do not assign to `name` on a built-in error."
            ]
        );
    }

    #[test]
    fn leaves_unknown_values_and_other_properties() {
        for code in [
            "Object.assign(error, {name});",
            "Object.assign(later, {name});\nconst later = new Error('m');",
            "Object.assign(new Error('message'), {code});",
            "const error = new Error('message');\nerror.message = message;",
            "Object.assign(new Error('m'), {errors});",
            "const e = new MyError('m');\ne.stack = s;",
            "const Error = f;\nconst e = new Error('m');\ne.stack = s;",
            "const e = cond && new Error('m');\ne.stack = s;",
            "function f(e = new Error()) { e.stack = s; }",
            "const e = new Error('m');\ne?.stack;",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
