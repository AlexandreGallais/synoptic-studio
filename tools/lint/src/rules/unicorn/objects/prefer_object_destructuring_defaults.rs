//! `unicorn/prefer-object-destructuring-defaults`: a declaration destructuring an object
//! literal of defaults ended by a spread takes the defaults in its pattern,
//! `const {foo = false} = {...options}` (`eslint-plugin-unicorn` 78.0.0). Its rewrite is a
//! suggestion in `ESLint`, the two forms differing on an `undefined` value: none here.

use std::collections::HashMap;

use oxc_ast::AstKind;
use oxc_ast::ast::{
    BinaryOperator, BindingPattern, Expression, ObjectPropertyKind, PropertyKey, PropertyKind,
    UnaryOperator,
};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferObjectDestructuringDefaults;

/// The properties of `Object.prototype`, which a destructuring reads through any object.
const PROTOTYPE_PROPERTIES: [&str; 12] = [
    "__defineGetter__",
    "__defineSetter__",
    "__lookupGetter__",
    "__lookupSetter__",
    "__proto__",
    "constructor",
    "hasOwnProperty",
    "isPrototypeOf",
    "propertyIsEnumerable",
    "toLocaleString",
    "toString",
    "valueOf",
];

/// The name a key written statically gives.
fn key_name(key: &PropertyKey) -> Option<String> {
    match key {
        PropertyKey::StaticIdentifier(identifier) => Some(identifier.name.to_string()),
        PropertyKey::StringLiteral(literal) => Some(literal.value.to_string()),
        PropertyKey::NumericLiteral(literal) => Some(
            crate::rules::unicorn::known_values::number_text(literal.value),
        ),
        _ => None,
    }
}

/// Whether an expression is a literal of a primitive.
fn is_primitive_literal(expression: &Expression) -> bool {
    matches!(
        expression,
        Expression::StringLiteral(_)
            | Expression::NumericLiteral(_)
            | Expression::BooleanLiteral(_)
            | Expression::NullLiteral(_)
            | Expression::BigIntLiteral(_)
    )
}

/// Whether evaluating an expression may run code: a call, an assignment, a member read (a
/// getter), or a conversion of a value that is not a literal (`hasSideEffect` with
/// `considerGetters` and `considerImplicitTypeConversion`).
fn has_side_effect(expression: &Expression) -> bool {
    match expression {
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
        Expression::UnaryExpression(unary) => match unary.operator {
            UnaryOperator::Delete => true,
            UnaryOperator::UnaryNegation | UnaryOperator::UnaryPlus | UnaryOperator::BitwiseNot => {
                !is_primitive_literal(&unary.argument) || has_side_effect(&unary.argument)
            }
            _ => has_side_effect(&unary.argument),
        },
        Expression::BinaryExpression(binary) => {
            let converts = !matches!(
                binary.operator,
                BinaryOperator::StrictEquality
                    | BinaryOperator::StrictInequality
                    | BinaryOperator::Instanceof
                    | BinaryOperator::Exponential
            ) && (!is_primitive_literal(&binary.left)
                || !is_primitive_literal(&binary.right));
            converts || has_side_effect(&binary.left) || has_side_effect(&binary.right)
        }
        Expression::LogicalExpression(logical) => {
            has_side_effect(&logical.left) || has_side_effect(&logical.right)
        }
        Expression::ConditionalExpression(conditional) => {
            has_side_effect(&conditional.test)
                || has_side_effect(&conditional.consequent)
                || has_side_effect(&conditional.alternate)
        }
        Expression::ArrayExpression(array) => array.elements.iter().any(|element| {
            element.as_expression().is_some_and(has_side_effect)
                || matches!(element, oxc_ast::ast::ArrayExpressionElement::SpreadElement(spread)
                    if has_side_effect(&spread.argument))
        }),
        Expression::ObjectExpression(object) => object.properties.iter().any(|each| match each {
            ObjectPropertyKind::ObjectProperty(property) => {
                (property.computed
                    && property
                        .key
                        .as_expression()
                        .is_some_and(|key| !is_primitive_literal(key) || has_side_effect(key)))
                    || has_side_effect(&property.value)
            }
            ObjectPropertyKind::SpreadProperty(spread) => has_side_effect(&spread.argument),
        }),
        Expression::TSAsExpression(wrapper) => has_side_effect(&wrapper.expression),
        Expression::TSSatisfiesExpression(wrapper) => has_side_effect(&wrapper.expression),
        Expression::TSNonNullExpression(wrapper) => has_side_effect(&wrapper.expression),
        Expression::TSTypeAssertion(wrapper) => has_side_effect(&wrapper.expression),
        _ => true,
    }
}

/// Whether a range reads a name among some.
fn reads_any(source: &Source, span: Span, names: &[&str]) -> bool {
    source.semantic.nodes().iter().any(|node| {
        matches!(node.kind(), AstKind::IdentifierReference(identifier)
            if names.contains(&identifier.name.as_str())
                && identifier.span.start >= span.start
                && identifier.span.end <= span.end)
    })
}

impl Rule for PreferObjectDestructuringDefaults {
    fn name(&self) -> &'static str {
        "unicorn/prefer-object-destructuring-defaults"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::VariableDeclarator(declarator) = node.kind() else {
            return;
        };
        let BindingPattern::ObjectPattern(pattern) = &declarator.id else {
            return;
        };
        let Some(Expression::ObjectExpression(object)) = &declarator.init else {
            return;
        };
        if declarator.type_annotation.is_some() || pattern.rest.is_some() {
            return;
        }
        let mut bindings: Vec<(String, &str)> = Vec::new();
        for property in &pattern.properties {
            let BindingPattern::BindingIdentifier(binding) = &property.value else {
                return;
            };
            if property.computed {
                return;
            }
            let Some(name) = key_name(&property.key) else {
                return;
            };
            if PROTOTYPE_PROPERTIES.contains(&name.as_str()) {
                return;
            }
            bindings.push((name, binding.name.as_str()));
        }
        let Some((ObjectPropertyKind::SpreadProperty(spread), defaults)) =
            object.properties.split_last()
        else {
            return;
        };
        if bindings.is_empty() || has_side_effect(&spread.argument) {
            return;
        }
        let mut values: HashMap<String, &Expression> = HashMap::new();
        for each in defaults {
            let ObjectPropertyKind::ObjectProperty(property) = each else {
                return;
            };
            if property.kind != PropertyKind::Init
                || property.computed
                || property.method
                || has_side_effect(&property.value)
            {
                return;
            }
            let Some(name) = key_name(&property.key) else {
                return;
            };
            if values.insert(name, &property.value).is_some() {
                return;
            }
        }
        if bindings.len() != values.len() {
            return;
        }
        let names: Vec<&str> = bindings.iter().map(|(_, binding)| *binding).collect();
        let fits = bindings.iter().all(|(name, _)| {
            values
                .get(name)
                .is_some_and(|value| !reads_any(source, value.span(), &names))
        });
        if fits {
            findings.push(source.finding(
                self.name(),
                declarator.span.start,
                "Prefer object destructuring defaults over default object literals with spread."
                    .to_owned(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferObjectDestructuringDefaults;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(
            &PreferObjectDestructuringDefaults,
            "playground/a.ts",
            code,
        ))
    }

    const MESSAGE: &str =
        "Prefer object destructuring defaults over default object literals with spread.";

    #[test]
    fn reports_the_documented_defaults_with_spread() {
        assert_eq!(
            check("const {foo, bar} = {\n\tfoo: false,\n\tbar: 1,\n\t...options,\n};"),
            [MESSAGE]
        );
        assert_eq!(
            check("const {foo: localFoo} = {\n\tfoo: false,\n\t...options,\n};"),
            [MESSAGE]
        );
        assert_eq!(check("const {a} = {a: `x`, ...o};"), [MESSAGE]);
        assert_eq!(check("const {a} = {a: -1, ...o};"), [MESSAGE]);
        assert_eq!(check("const {a} = {a: `${x}`, ...o};"), [MESSAGE]);
    }

    #[test]
    fn accepts_the_documented_and_unsafe_forms() {
        for code in [
            "const {foo, bar} = options;",
            "const {foo = false, bar = 1} = {...options};",
            "const {foo} = {foo: false, bar: 1, ...options};",
            "const {foo, ...rest} = {foo: false, ...options};",
            "const {foo} = {...options, foo: false};",
            "const {foo} = {foo: f(), ...options};",
            "const {foo} = {foo: a.b, ...options};",
            "const {foo} = {foo: false, ...g()};",
            "const {foo, bar} = {foo: 1, bar: foo, ...options};",
            "const {toString} = {toString: 1, ...options};",
            "const {[k]: foo} = {foo: 1, ...options};",
            "const {foo}: T = {foo: 1, ...options};",
            "const {foo} = {foo: x + 1, ...options};",
            "const {foo} = {foo: 1, foo: 2, ...options};",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
