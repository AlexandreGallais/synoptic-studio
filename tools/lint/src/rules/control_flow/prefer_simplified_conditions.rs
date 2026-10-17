//! `unicorn/prefer-simplified-conditions`: De Morgan's laws applied to a negated `&&` or `||`
//! when that leaves fewer negations, a common leading term factored out of `(a && b) || (a && c)`,
//! and an absorbed condition dropped from `a || (a && b)` (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/prefer-simplified-conditions.md`, `negatedConditions: 'simplify'`). Factoring and
//! absorption hold only for operands that can be repeated or dropped safely, in a chain read for
//! its truth or made of booleans. Autofixed unless a comment is inside.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    BinaryOperator, Expression, LogicalExpression, LogicalOperator, UnaryExpression, UnaryOperator,
    VariableDeclarationKind,
};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::expressions::{
    has_comment_in, is_boolean, is_boolean_context, is_control_flow_test, is_global,
    is_parenthesized, negated_text, parent, same_reference, symbol, text, unwrapped,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferSimplifiedConditions;

/// The rule's message.
const MESSAGE: &str = "Prefer a simplified condition.";

/// The global methods answering a boolean that can be called twice without harm.
const REPEATABLE_CALLS: [(&str, &str); 7] = [
    ("Array", "isArray"),
    ("ArrayBuffer", "isView"),
    ("Error", "isError"),
    ("Number", "isFinite"),
    ("Number", "isInteger"),
    ("Number", "isNaN"),
    ("Number", "isSafeInteger"),
];

/// Whether an expression is `!` of something.
fn negation<'s, 'a>(expression: &'s Expression<'a>) -> Option<&'s UnaryExpression<'a>> {
    match unwrapped(expression) {
        Expression::UnaryExpression(unary) if unary.operator == UnaryOperator::LogicalNot => {
            Some(unary)
        }
        _ => None,
    }
}

/// The opposite of an equality operator.
fn negated_equality(operator: BinaryOperator) -> Option<BinaryOperator> {
    match operator {
        BinaryOperator::StrictEquality => Some(BinaryOperator::StrictInequality),
        BinaryOperator::StrictInequality => Some(BinaryOperator::StrictEquality),
        BinaryOperator::Equality => Some(BinaryOperator::Inequality),
        BinaryOperator::Inequality => Some(BinaryOperator::Equality),
        _ => None,
    }
}

/// The other logical operator.
fn other(operator: LogicalOperator) -> Option<LogicalOperator> {
    match operator {
        LogicalOperator::And => Some(LogicalOperator::Or),
        LogicalOperator::Or => Some(LogicalOperator::And),
        LogicalOperator::Coalesce => None,
    }
}

/// The negations an expression holds: `!`, `!==` and `!=`.
fn weight(expression: &Expression) -> usize {
    match unwrapped(expression) {
        Expression::UnaryExpression(unary) if unary.operator == UnaryOperator::LogicalNot => {
            1 + weight(&unary.argument)
        }
        Expression::BinaryExpression(binary) => {
            let own = usize::from(matches!(
                binary.operator,
                BinaryOperator::StrictInequality | BinaryOperator::Inequality
            ));
            own + weight(&binary.left) + weight(&binary.right)
        }
        Expression::LogicalExpression(logical) => weight(&logical.left) + weight(&logical.right),
        _ => 0,
    }
}

/// The negations an operand holds once negated.
fn negated_weight(source: &Source, expression: &Expression, truthiness: bool) -> usize {
    let expression = unwrapped(expression);
    if let Some(unary) = negation(expression) {
        return if truthiness || is_boolean(source, &unary.argument) {
            weight(&unary.argument)
        } else {
            1 + weight(expression)
        };
    }
    if let Expression::BinaryExpression(binary) = expression
        && negated_equality(binary.operator).is_some()
    {
        let own = usize::from(matches!(
            binary.operator,
            BinaryOperator::StrictEquality | BinaryOperator::Equality
        ));
        return own + weight(&binary.left) + weight(&binary.right);
    }
    1 + weight(expression)
}

/// The text of a node, with its parentheses when written.
fn written(source: &Source, expression: &Expression) -> String {
    let inner = text(source, expression.span());
    if is_parenthesized(source, expression.span()) {
        format!("({inner})")
    } else {
        inner.to_owned()
    }
}

/// The text of an operand negated.
fn negated_operand(source: &Source, expression: &Expression, truthiness: bool) -> String {
    if let Some(unary) = negation(expression) {
        return if truthiness || is_boolean(source, &unary.argument) {
            written(source, &unary.argument)
        } else {
            format!("!{}", written(source, expression))
        };
    }
    if let Expression::BinaryExpression(binary) = expression
        && let Some(opposite) = negated_equality(binary.operator)
    {
        let between = text(
            source,
            oxc_span::Span::new(binary.left.span().end, binary.right.span().start),
        );
        let replaced = between.replacen(binary.operator.as_str(), opposite.as_str(), 1);
        return format!(
            "{}{replaced}{}",
            text(source, binary.left.span()),
            text(source, binary.right.span())
        );
    }
    negated_text(source, expression)
}

/// The replacement of a node by a logical expression of an operator, parenthesized as its place
/// needs.
fn placed(
    source: &Source,
    node: &AstNode,
    span: oxc_span::Span,
    operator: LogicalOperator,
    replacement: String,
) -> String {
    if is_parenthesized(source, span) {
        return replacement;
    }
    let wraps = match parent(source, node.id()) {
        AstKind::LogicalExpression(outer) => {
            outer.operator == LogicalOperator::Coalesce
                || (outer.operator == LogicalOperator::And && operator == LogicalOperator::Or)
        }
        AstKind::TSAsExpression(_)
        | AstKind::TSSatisfiesExpression(_)
        | AstKind::TSNonNullExpression(_)
        | AstKind::TSTypeAssertion(_)
        | AstKind::UnaryExpression(_)
        | AstKind::AwaitExpression(_)
        | AstKind::BinaryExpression(_) => true,
        _ => false,
    };
    if wraps {
        format!("({replacement})")
    } else {
        replacement
    }
}

/// Whether an operand can be written twice: a name, a literal, `this`, their negations and
/// strict comparisons, or a global boolean method of them.
fn is_repeatable(source: &Source, expression: &Expression) -> bool {
    let expression = unwrapped(expression);
    let simple = |each: &Expression| is_simple_repeatable(each);
    simple(expression)
        || matches!(expression, Expression::CallExpression(call)
            if !call.optional
                && call.arguments.len() == 1
                && call.arguments[0].as_expression().is_some_and(simple)
                && matches!(&call.callee, Expression::StaticMemberExpression(member)
                    if !member.optional
                        && matches!(&member.object, Expression::Identifier(object)
                            if is_global(source, object)
                                && REPEATABLE_CALLS.contains(&(object.name.as_str(), member.property.name.as_str())))))
}

/// Whether an operand is a name, a literal, `this`, or their negations and strict comparisons.
fn is_simple_repeatable(expression: &Expression) -> bool {
    match unwrapped(expression) {
        Expression::Identifier(_) | Expression::ThisExpression(_) => true,
        each if each.is_literal() => true,
        Expression::UnaryExpression(unary) => {
            unary.operator == UnaryOperator::LogicalNot && is_simple_repeatable(&unary.argument)
        }
        Expression::BinaryExpression(binary) => {
            matches!(
                binary.operator,
                BinaryOperator::StrictEquality | BinaryOperator::StrictInequality
            ) && is_simple_repeatable(&binary.left)
                && is_simple_repeatable(&binary.right)
        }
        _ => false,
    }
}

/// Whether two conditions are the same.
fn same_condition(source: &Source, left: &Expression, right: &Expression) -> bool {
    same_reference(left, right)
        || (is_repeatable(source, left)
            && is_repeatable(source, right)
            && text(source, left.span()) == text(source, right.span()))
}

/// Whether dropping an operand skips nothing: a literal, a parameter, a function's name, a
/// `var`, or a negation of one.
fn is_droppable(source: &Source, expression: &Expression) -> bool {
    match unwrapped(expression) {
        each if each.is_literal() => true,
        Expression::UnaryExpression(unary) if unary.operator == UnaryOperator::LogicalNot => is_droppable(source, &unary.argument),
        Expression::Identifier(name) => symbol(source, name).is_some_and(|symbol| {
            let nodes = source.semantic.nodes();
            let declaration = source.semantic.scoping().symbol_declaration(symbol);
            match nodes.kind(declaration) {
                AstKind::FormalParameter(_) | AstKind::Function(_) => true,
                AstKind::VariableDeclarator(_) => matches!(nodes.parent_kind(declaration),
                    AstKind::VariableDeclaration(each) if each.kind == VariableDeclarationKind::Var),
                _ => false,
            }
        }),
        _ => false,
    }
}

impl PreferSimplifiedConditions {
    /// The finding on a negated `&&` or `||`.
    fn de_morgan(
        &self,
        source: &Source,
        node: &AstNode,
        unary: &UnaryExpression,
    ) -> Option<Finding> {
        let Expression::LogicalExpression(inner) = &unary.argument else {
            return None;
        };
        let operator = other(inner.operator)?;
        if matches!(parent(source, node.id()), AstKind::UnaryExpression(up) if up.operator == UnaryOperator::LogicalNot)
        {
            return None;
        }
        let truthiness = is_control_flow_test(source, node.id());
        let before = 1 + weight(&unary.argument);
        let after = negated_weight(source, &inner.left, truthiness)
            + negated_weight(source, &inner.right, truthiness);
        if after >= before {
            return None;
        }
        if has_comment_in(source, unary.span) {
            return Some(source.finding(self.name(), unary.span.start, MESSAGE.to_owned()));
        }
        let replacement = format!(
            "{} {} {}",
            negated_operand(source, &inner.left, truthiness),
            operator.as_str(),
            negated_operand(source, &inner.right, truthiness)
        );
        let fix = Fix {
            start: unary.span.start,
            end: unary.span.end,
            text: placed(source, node, unary.span, operator, replacement),
        };
        Some(source.fixed(self.name(), unary.span.start, MESSAGE.to_owned(), fix))
    }

    /// The condition left when one absorbs the other, `a || (a && b)`.
    fn absorbed<'s, 'a>(
        source: &Source,
        node: &AstNode,
        logical: &'s LogicalExpression<'a>,
    ) -> Option<&'s Expression<'a>> {
        let inner_operator = other(logical.operator)?;
        let droppable = |kept: &Expression, removed: &Expression| {
            is_droppable(source, removed)
                && (is_boolean_context(source, node.id())
                    || (is_boolean(source, kept) && is_boolean(source, removed)))
        };
        let replacement = |each: &Expression| {
            matches!(
                unwrapped(each),
                Expression::Identifier(_) | Expression::ThisExpression(_)
            ) || unwrapped(each).is_literal()
        };
        if let Expression::LogicalExpression(right) = &logical.right
            && right.operator == inner_operator
            && replacement(&logical.left)
            && (same_condition(source, &logical.left, &right.left)
                || (same_condition(source, &logical.left, &right.right)
                    && droppable(&logical.left, &right.left)))
        {
            return Some(&logical.left);
        }
        if let Expression::LogicalExpression(left) = &logical.left
            && left.operator == inner_operator
            && replacement(&logical.right)
            && ((same_condition(source, &left.left, &logical.right)
                && droppable(&logical.right, &left.right))
                || (same_condition(source, &left.right, &logical.right)
                    && droppable(&logical.right, &left.left)))
        {
            return Some(&logical.right);
        }
        None
    }

    /// The text of `(a && b) || (a && c)` factored, `a && (b || c)`.
    fn factored(source: &Source, node: &AstNode, logical: &LogicalExpression) -> Option<String> {
        let inner_operator = other(logical.operator)?;
        let (Expression::LogicalExpression(left), Expression::LogicalExpression(right)) =
            (&logical.left, &logical.right)
        else {
            return None;
        };
        if left.operator != inner_operator
            || right.operator != inner_operator
            || !same_condition(source, &left.left, &right.left)
            || same_condition(source, &left.right, &right.right)
        {
            return None;
        }
        let operands = [&left.left, &left.right, &right.right];
        let optional = operands
            .iter()
            .any(|each| text(source, each.span()).contains("?."));
        let can = !optional
            && operands.iter().all(|each| is_repeatable(source, each))
            && (is_boolean_context(source, node.id())
                || operands.iter().all(|each| is_boolean(source, each)));
        can.then(|| {
            format!(
                "{} {} ({} {} {})",
                written(source, &left.left),
                inner_operator.as_str(),
                written(source, &left.right),
                logical.operator.as_str(),
                written(source, &right.right)
            )
        })
    }
}

impl Rule for PreferSimplifiedConditions {
    fn name(&self) -> &'static str {
        "unicorn/prefer-simplified-conditions"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        match node.kind() {
            AstKind::UnaryExpression(unary) if unary.operator == UnaryOperator::LogicalNot => {
                let inside_negated =
                    matches!(parent(source, node.id()), AstKind::LogicalExpression(_))
                        && matches!(parent(source, source.semantic.nodes().parent_id(node.id())),
                        AstKind::UnaryExpression(up) if up.operator == UnaryOperator::LogicalNot);
                if !inside_negated {
                    findings.extend(self.de_morgan(source, node, unary));
                }
            }
            AstKind::LogicalExpression(logical) => {
                if matches!(parent(source, node.id()), AstKind::UnaryExpression(up) if up.operator == UnaryOperator::LogicalNot)
                {
                    return;
                }
                let replacement = if let Some(kept) = Self::absorbed(source, node, logical) {
                    written(source, kept)
                } else if let Some(text) = Self::factored(source, node, logical) {
                    text
                } else {
                    return;
                };
                let at = logical.span.start;
                if has_comment_in(source, logical.span) {
                    findings.push(source.finding(self.name(), at, MESSAGE.to_owned()));
                    return;
                }
                let operator = match (&logical.left, Self::absorbed(source, node, logical)) {
                    (Expression::LogicalExpression(left), None) => left.operator,
                    _ => logical.operator,
                };
                let fix = Fix {
                    start: logical.span.start,
                    end: logical.span.end,
                    text: placed(source, node, logical.span, operator, replacement),
                };
                findings.push(source.fixed(self.name(), at, MESSAGE.to_owned(), fix));
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferSimplifiedConditions;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferSimplifiedConditions, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn simplifies_negations_factors_and_absorptions() {
        // The documentation's five cases.
        assert_eq!(
            check("if (!(key === 'y' && !isEditable(target))) {}"),
            (
                vec![(1, 5)],
                "if (key !== 'y' || isEditable(target)) {}".to_owned()
            )
        );
        assert_eq!(check("if (!(!a && !b)) {}").1, "if (a || b) {}");
        assert_eq!(
            check("if ((a && b) || (a && c)) {}").1,
            "if (a && (b || c)) {}"
        );
        assert_eq!(
            check("if ((c || a) && (c || b)) {}").1,
            "if (c || (a && b)) {}"
        );
        assert_eq!(
            check("function f(a, b) { if (a || (a && b)) {} }").1,
            "function f(a, b) { if (a) {} }"
        );
        // A factored chain inside `&&` keeps its parentheses.
        assert_eq!(
            check("if (x && ((a || b) && (a || c))) {}").1,
            "if (x && (a || (b && c))) {}"
        );
    }

    #[test]
    fn leaves_what_would_not_be_simpler_or_safe() {
        for code in [
            "const value = (a && b) || (a && c);",
            "if (!(min <= value && value <= max)) {}",
            "if ((object.property && a) || a) {}",
            "if (!(a && b)) {}",
            "if (!!(a && b)) {}",
            "const result = !(!a && !b);",
            "if (!(a ?? b)) {}",
            "if ((f() && b) || (f() && c)) {}",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
        let commented = "if (!(!a /* x */ && !b)) {}";
        assert_eq!(check(commented), (vec![(1, 5)], commented.to_owned()));
    }
}
