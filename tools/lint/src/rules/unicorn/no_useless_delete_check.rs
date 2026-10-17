//! `unicorn/no-useless-delete-check`: no `in` check before `delete`ing the same key of a known
//! object, nor `has()` before `delete()` of a local `const` collection, a deletion being safe
//! without (`eslint-plugin-unicorn` 78.0.0). The collection's check is removed by an autofix,
//! as in `ESLint`, unless a comment sits in the `if`; the object's is a suggestion there:
//! reported only.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    BinaryOperator, CallExpression, Expression, IfStatement, Statement, TSType, UnaryOperator,
};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::known_values::{KeyName, expression_key_name};
use super::syntax::{
    Binding, binding_of, const_init, has_comment, is_global, method_call, same_reference,
    symbol_of, text, unwrap,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoUselessDeleteCheck;

/// What the rule reports.
const MESSAGE: &str = "The existence check before deletion is unnecessary.";

/// Whether an expression may run code when read: a call, an assignment, a member read.
fn has_side_effect(expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::Identifier(_)
        | Expression::ThisExpression(_)
        | Expression::StringLiteral(_)
        | Expression::NumericLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_)
        | Expression::BigIntLiteral(_)
        | Expression::RegExpLiteral(_)
        | Expression::FunctionExpression(_)
        | Expression::ArrowFunctionExpression(_) => false,
        Expression::TemplateLiteral(template) => template.expressions.iter().any(has_side_effect),
        Expression::UnaryExpression(unary) => {
            unary.operator == UnaryOperator::Delete || has_side_effect(&unary.argument)
        }
        Expression::BinaryExpression(binary) => {
            has_side_effect(&binary.left) || has_side_effect(&binary.right)
        }
        Expression::LogicalExpression(logical) => {
            has_side_effect(&logical.left) || has_side_effect(&logical.right)
        }
        Expression::ConditionalExpression(conditional) => {
            has_side_effect(&conditional.test)
                || has_side_effect(&conditional.consequent)
                || has_side_effect(&conditional.alternate)
        }
        _ => true,
    }
}

/// Whether an annotation names an object: `object`, a function or a constructor type, or a
/// union of them.
fn is_object_type(annotation: &TSType) -> bool {
    match annotation {
        TSType::TSObjectKeyword(_) | TSType::TSFunctionType(_) | TSType::TSConstructorType(_) => {
            true
        }
        TSType::TSUnionType(union) => union.types.iter().all(is_object_type),
        _ => false,
    }
}

/// Whether a name refers to a function or a class the file declares.
fn is_function_or_class(source: &Source, expression: &Expression) -> bool {
    let Expression::Identifier(identifier) = unwrap(expression) else {
        return false;
    };
    symbol_of(source, identifier).is_some_and(|symbol| {
        matches!(
            source
                .semantic
                .nodes()
                .kind(source.semantic.scoping().symbol_declaration(symbol)),
            AstKind::Function(_) | AstKind::Class(_)
        )
    })
}

/// Whether an expression is written as an object: a literal of one, or a function or a class.
fn is_object_syntax(expression: &Expression) -> bool {
    matches!(
        unwrap(expression),
        Expression::ObjectExpression(_)
            | Expression::ArrayExpression(_)
            | Expression::ArrowFunctionExpression(_)
            | Expression::FunctionExpression(_)
            | Expression::ClassExpression(_)
    )
}

/// Whether a value is known to be an object: written so, bound by a `const` to one, or annotated
/// with an object type (`isKnownObject`).
fn is_known_object(source: &Source, expression: &Expression) -> bool {
    if is_object_syntax(expression) {
        return true;
    }
    if let Some(init) = const_init(source, unwrap(expression)) {
        return is_object_syntax(init);
    }
    if is_function_or_class(source, expression) {
        return false;
    }
    let annotation = match binding_of(source, unwrap(expression)) {
        Some(Binding::Variable(declarator, _)) => declarator.type_annotation.as_ref(),
        Some(Binding::Parameter(parameter)) => parameter.type_annotation.as_ref(),
        None => None,
    };
    annotation.is_some_and(|annotation| is_object_type(&annotation.type_annotation))
}

/// Whether a key may need converting each time it is used: an object, a function, a class, or a
/// `const` built by `new` or an opaque call.
fn may_need_coercion(source: &Source, expression: &Expression) -> bool {
    if is_known_object(source, expression) || is_function_or_class(source, expression) {
        return true;
    }
    const_init(source, unwrap(expression)).is_some_and(|init| match unwrap(init) {
        Expression::NewExpression(_) => true,
        Expression::CallExpression(_) => expression_key_name(source, init).is_none(),
        _ => false,
    })
}

/// The key a constant gives, written as `String()` writes it; no symbol.
fn static_key(source: &Source, expression: &Expression) -> Option<String> {
    match expression_key_name(source, expression)? {
        KeyName::Text(text) => Some(text),
        KeyName::Symbol(_) => None,
    }
}

/// The only expression a branch holds, alone or the only statement of its block.
fn only_expression<'b, 'a>(
    statement: &'b Statement<'a>,
) -> Option<(&'b Expression<'a>, &'b Statement<'a>)> {
    let inner = match statement {
        Statement::BlockStatement(block) => match block.body.as_slice() {
            [only] => only,
            _ => return None,
        },
        other => other,
    };
    match inner {
        Statement::ExpressionStatement(expression) => Some((&expression.expression, inner)),
        _ => None,
    }
}

/// The plain `name(argument)` call of an expression.
fn one_argument_call<'b, 'a>(
    expression: &'b Expression<'a>,
    name: &str,
) -> Option<&'b CallExpression<'a>> {
    let Expression::CallExpression(call) = unwrap(expression) else {
        return None;
    };
    let method = method_call(call)?;
    (method.name == name
        && method.is_plain()
        && call.arguments.len() == 1
        && call.arguments[0].as_expression().is_some())
    .then_some(call)
}

/// Whether a receiver is a local `const` built by `new Map()`, `Set`, `WeakMap` or `WeakSet`.
fn is_known_collection(source: &Source, expression: &Expression) -> bool {
    const_init(source, unwrap(expression)).is_some_and(|init| {
        matches!(unwrap(init), Expression::NewExpression(new)
            if ["Map", "Set", "WeakMap", "WeakSet"].iter().any(|name| is_global(source, &new.callee, name)))
    })
}

impl NoUselessDeleteCheck {
    /// Reports a check before a deletion, and its fix when the collection's.
    fn check_if(
        &self,
        source: &Source,
        node: &AstNode,
        statement: &IfStatement,
        findings: &mut Vec<Finding>,
    ) {
        let is_else_if = matches!(source.semantic.nodes().parent_kind(node.id()),
            AstKind::IfStatement(parent) if parent.alternate.as_ref().is_some_and(|alternate| alternate.span() == statement.span));
        if statement.alternate.is_some() || is_else_if {
            return;
        }
        let Some((expression, inner)) = only_expression(&statement.consequent) else {
            return;
        };
        if let Expression::UnaryExpression(unary) = expression
            && unary.operator == UnaryOperator::Delete
        {
            if Self::is_object_check(source, &statement.test, &unary.argument) {
                findings.push(source.finding(
                    self.name(),
                    statement.test.span().start,
                    MESSAGE.to_owned(),
                ));
            }
            return;
        }
        let (Some(has), Some(delete)) = (
            one_argument_call(&statement.test, "has"),
            one_argument_call(expression, "delete"),
        ) else {
            return;
        };
        let (Some(has_method), Some(delete_method)) = (method_call(has), method_call(delete))
        else {
            return;
        };
        let (Some(key), Some(deleted)) = (
            has.arguments[0].as_expression(),
            delete.arguments[0].as_expression(),
        ) else {
            return;
        };
        if !is_known_collection(source, has_method.object)
            || !same_reference(has_method.object, delete_method.object)
            || !same_reference(key, deleted)
            || has_side_effect(key)
        {
            return;
        }
        if has_comment(source, statement.span) {
            findings.push(source.finding(
                self.name(),
                statement.test.span().start,
                MESSAGE.to_owned(),
            ));
            return;
        }
        let mut replacement = text(source, inner.span()).to_owned();
        if !replacement.trim_end().ends_with(';') {
            replacement.push(';');
        }
        findings.push(source.fixed(
            self.name(),
            statement.test.span().start,
            MESSAGE.to_owned(),
            Fix {
                start: statement.span.start,
                end: statement.span.end,
                text: replacement,
            },
        ));
    }

    /// Whether `key in object` guards `delete object[key]` or `delete object.key`.
    fn is_object_check(source: &Source, test: &Expression, argument: &Expression) -> bool {
        let Expression::BinaryExpression(binary) = test else {
            return false;
        };
        if binary.operator != BinaryOperator::In {
            return false;
        }
        let (object, property, computed) = match argument {
            Expression::StaticMemberExpression(member) if !member.optional => {
                (&member.object, None, Some(member.property.name.to_string()))
            }
            Expression::ComputedMemberExpression(member) if !member.optional => {
                (&member.object, Some(&member.expression), None)
            }
            _ => return false,
        };
        if !same_reference(&binary.right, object)
            || !is_known_object(source, &binary.right)
            || has_side_effect(&binary.left)
            || has_side_effect(&binary.right)
            || property.is_some_and(has_side_effect)
        {
            return false;
        }
        match (property, computed) {
            (Some(property), _) => {
                if same_reference(&binary.left, property) {
                    !may_need_coercion(source, &binary.left)
                } else {
                    let left = static_key(source, &binary.left);
                    left.is_some() && left == static_key(source, property)
                }
            }
            (None, Some(name)) => static_key(source, &binary.left) == Some(name),
            (None, None) => false,
        }
    }
}

impl Rule for NoUselessDeleteCheck {
    fn name(&self) -> &'static str {
        "unicorn/no-useless-delete-check"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        if let AstKind::IfStatement(statement) = node.kind() {
            self.check_if(source, node, statement, findings);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MESSAGE, NoUselessDeleteCheck};
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoUselessDeleteCheck, "playground/a.ts", code))
    }

    /// The code once fixed.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&NoUselessDeleteCheck, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_checks() {
        let object = "const object = {};\n\nif (key in object) {\n\tdelete object[key];\n}";
        assert_eq!(check(object), [MESSAGE]);
        assert_eq!(fix(object), object);
        let map = "const map = new Map();\n\nif (map.has(key)) {\n\tmap.delete(key);\n}";
        assert_eq!(check(map), [MESSAGE]);
        assert_eq!(fix(map), "const map = new Map();\n\nmap.delete(key);");
        assert_eq!(
            fix("const set = new Set();\n\nif (set.has(value)) {\n\tset.delete(value);\n}"),
            "const set = new Set();\n\nset.delete(value);"
        );
        assert_eq!(
            fix("const s = new WeakSet(); if (s.has(v)) s.delete(v)"),
            "const s = new WeakSet(); s.delete(v);"
        );
        assert_eq!(
            check("const o = {a: 1}; if ('a' in o) delete o.a;"),
            [MESSAGE]
        );
    }

    #[test]
    fn accepts_the_documented_and_unknown_cases() {
        for code in [
            "const map = new Map();\n\nif (map.has(key)) {\n\tmap.delete(key);\n} else {\n\thandleMissingKey();\n}",
            "if (map.has(key)) { map.delete(key); }",
            "let map = new Map(); if (map.has(key)) { map.delete(key); }",
            "const map = new Map(); if (map.has(key) && ok) { map.delete(key); }",
            "const map = new Map(); if (map.has(a)) { map.delete(b); }",
            "const map = new Map(); if (map.has(f())) { map.delete(f()); }",
            "if (key in object) { delete object[key]; }",
            "const o = {}; if (Object.hasOwn(o, k)) { delete o[k]; }",
            "const o = {}; if (k in o) { delete o[k]; f(); }",
            "const o = {}; if (a) {} else if (k in o) { delete o[k]; }",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
