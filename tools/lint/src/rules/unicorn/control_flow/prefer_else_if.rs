//! `unicorn/prefer-else-if`: two adjacent `if`s — or `if`/`else if` chains without a last
//! `else` — comparing the same name or static member with `===` to values that cannot both hold,
//! or testing the same boolean for opposite values, are joined with `else` (`eslint-plugin-unicorn`
//! 78.0.0, `docs/rules/prefer-else-if.md`). A first chain with a branch that always leaves, or that
//! changes the compared value, is left. Autofixed when the compared value is a name, the first
//! chain's branches run nothing and the second chain's tests read nothing that can run code.

use std::collections::BTreeSet;

use oxc_ast::ast::{
    AssignmentTarget, BinaryOperator, Expression, IfStatement, SimpleAssignmentTarget, Statement,
    UnaryOperator,
};
use oxc_ast_visit::{Visit, walk};
use oxc_semantic::AstNode;

use crate::exits::always_exits;
use crate::expressions::{
    is_boolean, is_global, may_run_code, same_reference, statements, unwrapped,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferElseIf;

/// The rule's message.
const MESSAGE: &str = "Prefer `else if` over an adjacent `if` with a related condition.";

/// What a chain compares: the value, and the keys of the values it is compared to.
struct Comparison<'s, 'a> {
    discriminant: &'s Expression<'a>,
    values: BTreeSet<String>,
}

/// Whether an expression is a static reference: a name, `this`, `super`, or a member read
/// through one by a static key.
fn is_static_reference(expression: &Expression) -> bool {
    match unwrapped(expression) {
        Expression::Identifier(_) | Expression::ThisExpression(_) | Expression::Super(_) => true,
        Expression::StaticMemberExpression(member) => {
            !member.optional && is_static_reference(&member.object)
        }
        Expression::PrivateFieldExpression(member) => {
            !member.optional && is_static_reference(&member.object)
        }
        Expression::ComputedMemberExpression(member) => {
            !member.optional
                && member.expression.is_literal()
                && !matches!(member.expression, Expression::RegExpLiteral(_))
                && is_static_reference(&member.object)
        }
        _ => false,
    }
}

/// The key of a static value compared with `===`: a literal but a regular expression, or
/// `undefined`.
fn value_key(expression: &Expression) -> Option<String> {
    match expression {
        Expression::StringLiteral(each) => Some(format!("string:{}", each.value.as_str()?)),
        Expression::NumericLiteral(each) => Some(format!("number:{}", each.value)),
        Expression::BooleanLiteral(each) => Some(format!("boolean:{}", each.value)),
        Expression::NullLiteral(_) => Some("object:null".to_owned()),
        Expression::BigIntLiteral(each) => Some(format!(
            "bigint:{}",
            each.raw.as_deref().unwrap_or_default()
        )),
        Expression::Identifier(each) if each.name == "undefined" => {
            Some("undefined:undefined".to_owned())
        }
        _ => None,
    }
}

/// What a boolean test compares: `flag`, `!flag` or `Boolean(flag)` of a static boolean.
fn boolean_comparison<'s, 'a>(
    source: &Source,
    test: &'s Expression<'a>,
) -> Option<Comparison<'s, 'a>> {
    let mut value = true;
    let mut current = unwrapped(test);
    while let Expression::UnaryExpression(unary) = current {
        if unary.operator != UnaryOperator::LogicalNot {
            break;
        }
        value = !value;
        current = unwrapped(&unary.argument);
    }
    if let Expression::CallExpression(call) = current
        && let Expression::Identifier(callee) = &call.callee
        && callee.name == "Boolean"
        && call.arguments.len() == 1
        && is_global(source, callee)
        && let Some(argument) = call.arguments[0].as_expression()
    {
        current = unwrapped(argument);
    }
    if matches!(current, Expression::ChainExpression(_))
        || !is_static_reference(current)
        || !is_boolean(source, current)
    {
        return None;
    }
    Some(Comparison {
        discriminant: current,
        values: BTreeSet::from([format!("boolean:{value}")]),
    })
}

/// The `===` comparisons of a test joined by `||`, or none when another operand stands there.
fn equalities<'s, 'a>(
    test: &'s Expression<'a>,
    into: &mut Vec<&'s oxc_ast::ast::BinaryExpression<'a>>,
) -> bool {
    match test {
        Expression::LogicalExpression(logical)
            if logical.operator == oxc_ast::ast::LogicalOperator::Or =>
        {
            equalities(&logical.left, into) && equalities(&logical.right, into)
        }
        Expression::BinaryExpression(binary)
            if binary.operator == BinaryOperator::StrictEquality =>
        {
            into.push(binary);
            true
        }
        _ => false,
    }
}

/// What a test compares.
fn comparison<'s, 'a>(source: &Source, test: &'s Expression<'a>) -> Option<Comparison<'s, 'a>> {
    if let Some(found) = boolean_comparison(source, test) {
        return Some(found);
    }
    let mut found = Vec::new();
    if !equalities(test, &mut found) {
        return None;
    }
    let mut discriminant: Option<&Expression> = None;
    let mut values = BTreeSet::new();
    for each in found {
        let (left, right) = (value_key(&each.left), value_key(&each.right));
        let (candidate, value) = match (left, right) {
            (Some(value), None) => (&each.right, value),
            (None, Some(value)) => (&each.left, value),
            _ => return None,
        };
        if matches!(candidate, Expression::ChainExpression(_)) || !is_static_reference(candidate) {
            return None;
        }
        if discriminant.is_some_and(|known| !same_reference(known, candidate)) {
            return None;
        }
        discriminant = Some(candidate);
        values.insert(value);
    }
    Some(Comparison {
        discriminant: discriminant?,
        values,
    })
}

/// The `if`s of a chain, or `None` when it ends with an `else`.
fn chain<'s, 'a>(first: &'s IfStatement<'a>) -> Option<Vec<&'s IfStatement<'a>>> {
    let mut found = vec![first];
    let mut current = first;
    loop {
        match &current.alternate {
            Some(Statement::IfStatement(next)) => {
                found.push(next);
                current = next;
            }
            Some(_) => return None,
            None => return Some(found),
        }
    }
}

/// What a chain compares, the same value through all its tests.
fn chain_comparison<'s, 'a>(
    source: &Source,
    links: &[&'s IfStatement<'a>],
) -> Option<Comparison<'s, 'a>> {
    let mut merged: Option<Comparison> = None;
    for link in links {
        let found = comparison(source, &link.test)?;
        match &mut merged {
            Some(known) => {
                if !same_reference(known.discriminant, found.discriminant) {
                    return None;
                }
                known.values.extend(found.values);
            }
            None => merged = Some(found),
        }
    }
    merged
}

/// The search of a branch for a change of the compared value or of an object holding it.
struct Mutation<'d, 'a> {
    prefixes: Vec<&'d Expression<'a>>,
    found: bool,
}

impl<'a> Mutation<'_, 'a> {
    /// Notes a written target that is the compared value or one of its objects.
    fn target(&mut self, written: &Expression<'a>) {
        if self
            .prefixes
            .iter()
            .any(|prefix| same_reference(prefix, written))
        {
            self.found = true;
        }
    }
}

impl<'a> Visit<'a> for Mutation<'_, 'a> {
    fn visit_simple_assignment_target(&mut self, target: &SimpleAssignmentTarget<'a>) {
        match target {
            SimpleAssignmentTarget::AssignmentTargetIdentifier(name) => {
                if self.prefixes.iter().any(|prefix| matches!(unwrapped(prefix), Expression::Identifier(each) if each.name == name.name)) {
                    self.found = true;
                }
            }
            other => {
                if let Some(member) = other.as_member_expression() {
                    for prefix in self.prefixes.clone() {
                        if let Some(prefix_member) = unwrapped(prefix).as_member_expression()
                            && prefix_member.static_property_name() == member.static_property_name()
                            && same_reference(prefix_member.object(), member.object())
                        {
                            self.found = true;
                        }
                    }
                }
            }
        }
        walk::walk_simple_assignment_target(self, target);
    }

    fn visit_expression(&mut self, expression: &Expression<'a>) {
        if let Expression::UnaryExpression(unary) = expression
            && unary.operator == UnaryOperator::Delete
        {
            self.target(&unary.argument);
        }
        walk::walk_expression(self, expression);
    }

    fn visit_assignment_target(&mut self, target: &AssignmentTarget<'a>) {
        walk::walk_assignment_target(self, target);
    }

    fn visit_function(
        &mut self,
        _function: &oxc_ast::ast::Function<'a>,
        _flags: oxc_semantic::ScopeFlags,
    ) {
    }

    fn visit_arrow_function_expression(
        &mut self,
        _arrow: &oxc_ast::ast::ArrowFunctionExpression<'a>,
    ) {
    }
}

/// The compared value and the objects it is read through, `a.b.c`, `a.b`, `a`.
fn prefixes<'s, 'a>(discriminant: &'s Expression<'a>) -> Vec<&'s Expression<'a>> {
    let mut found = vec![unwrapped(discriminant)];
    let mut current = unwrapped(discriminant);
    while let Some(member) = current.as_member_expression() {
        current = unwrapped(member.object());
        found.push(current);
    }
    found
}

/// Whether a branch changes the compared value.
fn changes(branch: &Statement, discriminant: &Expression) -> bool {
    let mut search = Mutation {
        prefixes: prefixes(discriminant),
        found: false,
    };
    search.visit_statement(branch);
    search.found
}

/// Whether a branch may run code: a call, an assignment, a member read.
fn branch_runs_code(branch: &Statement) -> bool {
    /// The search.
    #[derive(Default)]
    struct Search {
        found: bool,
    }
    impl<'a> Visit<'a> for Search {
        fn visit_expression(&mut self, expression: &Expression<'a>) {
            self.found |= may_run_code(expression);
        }
    }
    let mut search = Search::default();
    search.visit_statement(branch);
    search.found
}

impl PreferElseIf {
    /// The finding on an `if` that follows another.
    fn check(
        &self,
        source: &Source,
        previous: &IfStatement,
        current: &IfStatement,
    ) -> Option<Finding> {
        let previous_chain = chain(previous)?;
        let current_chain = chain(current)?;
        if previous_chain
            .iter()
            .any(|link| always_exits(&link.consequent))
        {
            return None;
        }
        let before = chain_comparison(source, &previous_chain)?;
        let after = chain_comparison(source, &current_chain)?;
        if !same_reference(before.discriminant, after.discriminant)
            || !before.values.is_disjoint(&after.values)
            || previous_chain
                .iter()
                .any(|link| changes(&link.consequent, before.discriminant))
        {
            return None;
        }
        let fixable = matches!(unwrapped(before.discriminant), Expression::Identifier(_))
            && !previous_chain
                .iter()
                .any(|link| branch_runs_code(&link.consequent))
            && !current_chain.iter().any(|link| may_run_code(&link.test));
        if !fixable {
            return Some(source.finding(self.name(), current.span.start, MESSAGE.to_owned()));
        }
        let fix = Fix {
            start: current.span.start,
            end: current.span.start,
            text: "else ".to_owned(),
        };
        Some(source.fixed(self.name(), current.span.start, MESSAGE.to_owned(), fix))
    }
}

impl Rule for PreferElseIf {
    fn name(&self) -> &'static str {
        "unicorn/prefer-else-if"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let Some(list) = statements(node.kind()) else {
            return;
        };
        for pair in list.windows(2) {
            if let [
                Statement::IfStatement(previous),
                Statement::IfStatement(current),
            ] = pair
            {
                findings.extend(self.check(source, previous, current));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferElseIf;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferElseIf, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn joins_exclusive_comparisons_with_else() {
        // The documentation's case.
        assert_eq!(
            check("if (foo === 1) {\n\tone();\n}\nif (foo === 2) {\n\ttwo();\n}"),
            (
                vec![(4, 1)],
                "if (foo === 1) {\n\tone();\n}\nif (foo === 2) {\n\ttwo();\n}".to_owned()
            )
        );
        assert_eq!(
            check("if (foo === 1) {}\nif (foo === 2) {}").1,
            "if (foo === 1) {}\nelse if (foo === 2) {}"
        );
        // A chain on one side, `||` of values, a boolean, a member only reported.
        assert_eq!(
            check("if (a === 1) {} else if (a === 2) {}\nif (a === 3 || 4 === a) {}").0,
            [(2, 1)]
        );
        assert_eq!(
            check("function f(flag: boolean) { if (flag) { g(); } if (!flag) { h(); } }").0,
            [(1, 48)]
        );
        let member = "if (a.b === 'x') { g(); }\nif (a.b === 'y') { h(); }";
        assert_eq!(check(member), (vec![(2, 1)], member.to_owned()));
    }

    #[test]
    fn leaves_overlapping_exiting_changing_or_unrelated_ifs() {
        for code in [
            "if (foo === 1) { one(); }\nif (foo === 1) { two(); }",
            "function f() { if (foo === 1) { return; }\nif (foo === 2) { two(); } }",
            "if (foo === 1) { foo = 2; }\nif (foo === 2) { two(); }",
            "if (a.b === 1) { a.b = 2; }\nif (a.b === 2) { two(); }",
            "if (foo === 1) { one(); }\nif (bar === 2) { two(); }",
            "if (foo === 1) { one(); } else { other(); }\nif (foo === 2) { two(); }",
            "if (foo > 1) { one(); }\nif (foo === 2) { two(); }",
            "if (flag) { one(); }\nif (!flag) { two(); }",
            "if (f() === 1) { one(); }\nif (f() === 2) { two(); }",
            "if (a?.b === 1) { one(); }\nif (a?.b === 2) { two(); }",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
