//! `unicorn/prefer-simple-sort-comparator`: no chain of comparisons returning `1`, `-1` and `0`
//! to sort by one key, where a subtraction of the two sides says it (`eslint-plugin-unicorn`
//! 78.0.0). Its fix is a suggestion in `ESLint`: none here.

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryExpression, BinaryOperator, Expression, Statement, UnaryOperator};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::syntax::{callback, method_call, same_reference, skips_non_array_receiver};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferSimpleSortComparator;

/// A comparator's body reduced to tests and returned signs.
enum Tree<'b, 'a> {
    /// A returned number, by its sign.
    Leaf(f64),
    /// A test, the tree when it holds and the tree otherwise.
    Branch(&'b Expression<'a>, Box<Tree<'b, 'a>>, Box<Tree<'b, 'a>>),
}

/// The sign of a numeric literal, signed or not.
fn literal_sign(expression: &Expression) -> Option<f64> {
    match expression {
        Expression::NumericLiteral(literal) => Some(literal.value),
        Expression::UnaryExpression(unary)
            if matches!(
                unary.operator,
                UnaryOperator::UnaryNegation | UnaryOperator::UnaryPlus
            ) =>
        {
            let Expression::NumericLiteral(literal) = &unary.argument else {
                return None;
            };
            Some(if unary.operator == UnaryOperator::UnaryNegation {
                -literal.value
            } else {
                literal.value
            })
        }
        _ => None,
    }
    .map(|value| if value == 0.0 { 0.0 } else { value.signum() })
}

/// The tree of an expression: literals and conditionals of literals.
fn expression_tree<'b, 'a>(expression: &'b Expression<'a>) -> Option<Tree<'b, 'a>> {
    if let Some(sign) = literal_sign(expression) {
        return Some(Tree::Leaf(sign));
    }
    let Expression::ConditionalExpression(conditional) = expression else {
        return None;
    };
    Some(Tree::Branch(
        &conditional.test,
        Box::new(expression_tree(&conditional.consequent)?),
        Box::new(expression_tree(&conditional.alternate)?),
    ))
}

/// The statements a branch of an `if` holds.
fn branch_statements<'b, 'a>(statement: &'b Statement<'a>) -> &'b [Statement<'a>] {
    match statement {
        Statement::BlockStatement(block) => &block.body,
        other => std::slice::from_ref(other),
    }
}

/// The tree of statements: `return`s and `if`s down to returned literals.
fn statements_tree<'b, 'a>(statements: &'b [Statement<'a>]) -> Option<Tree<'b, 'a>> {
    match statements.split_first()? {
        (Statement::ReturnStatement(statement), _) => expression_tree(statement.argument.as_ref()?),
        (Statement::IfStatement(statement), rest) => {
            let consequent = statements_tree(branch_statements(&statement.consequent))?;
            let alternate = match &statement.alternate {
                Some(alternate) => statements_tree(branch_statements(alternate))?,
                None => statements_tree(rest)?,
            };
            Some(Tree::Branch(
                &statement.test,
                Box::new(consequent),
                Box::new(alternate),
            ))
        }
        _ => None,
    }
}

impl<'b, 'a> Tree<'b, 'a> {
    /// Every test, outermost first.
    fn tests(&self, tests: &mut Vec<&'b Expression<'a>>) {
        if let Tree::Branch(test, consequent, alternate) = self {
            tests.push(test);
            consequent.tests(tests);
            alternate.tests(tests);
        }
    }

    /// Every returned sign.
    fn signs(&self, signs: &mut Vec<f64>) {
        match self {
            Tree::Leaf(sign) => signs.push(*sign),
            Tree::Branch(_, consequent, alternate) => {
                consequent.signs(signs);
                alternate.signs(signs);
            }
        }
    }

    /// The sign returned when every test holds.
    fn true_sign(&self) -> f64 {
        match self {
            Tree::Leaf(sign) => *sign,
            Tree::Branch(_, consequent, _) => consequent.true_sign(),
        }
    }

    /// Whether each branch returns the sign of `minuend - subtrahend` when its test holds.
    fn matches_subtraction(&self, minuend: &Expression, subtrahend: &Expression) -> bool {
        let Tree::Branch(test, consequent, alternate) = self else {
            return true;
        };
        let Expression::BinaryExpression(test) = test else {
            return false;
        };
        let greater = is_greater(test);
        let left_is_minuend =
            same_reference(&test.left, minuend) && same_reference(&test.right, subtrahend);
        let expected = if left_is_minuend == greater {
            1.0
        } else {
            -1.0
        };
        (self.true_sign() - expected).abs() < f64::EPSILON
            && consequent.matches_subtraction(minuend, subtrahend)
            && alternate.matches_subtraction(minuend, subtrahend)
    }
}

/// Whether a comparison reads `>` or `>=`.
fn is_greater(test: &BinaryExpression) -> bool {
    matches!(
        test.operator,
        BinaryOperator::GreaterThan | BinaryOperator::GreaterEqualThan
    )
}

/// Whether two operands mirror each other when the parameters swap: `a` and `b`, `a.x` and `b.x`.
fn is_swap(left: &Expression, right: &Expression, first: &str, second: &str) -> bool {
    match (left, right) {
        (Expression::Identifier(left), Expression::Identifier(right)) => {
            (left.name == first && right.name == second)
                || (left.name == second && right.name == first)
        }
        (Expression::StaticMemberExpression(left), Expression::StaticMemberExpression(right)) => {
            left.property.name == right.property.name
                && is_swap(&left.object, &right.object, first, second)
        }
        (
            Expression::ComputedMemberExpression(left),
            Expression::ComputedMemberExpression(right),
        ) => {
            same_reference(&left.expression, &right.expression)
                && is_swap(&left.object, &right.object, first, second)
        }
        _ => false,
    }
}

/// Whether a comparator reduces to a subtraction of its parameters' sides.
fn is_simple(tree: &Tree, first: &str, second: &str) -> bool {
    let mut tests = Vec::new();
    tree.tests(&mut tests);
    let comparisons: Option<Vec<&BinaryExpression>> = tests
        .iter()
        .map(|test| match test {
            Expression::BinaryExpression(binary)
                if binary.operator.is_compare()
                    && !binary.operator.is_equality()
                    && is_swap(&binary.left, &binary.right, first, second) =>
            {
                Some(&**binary)
            }
            _ => None,
        })
        .collect();
    let Some(comparisons) = comparisons else {
        return false;
    };
    let Some(outer) = comparisons.first() else {
        return false;
    };
    let same_pair = comparisons.iter().all(|test| {
        (same_reference(&test.left, &outer.left) && same_reference(&test.right, &outer.right))
            || (same_reference(&test.left, &outer.right)
                && same_reference(&test.right, &outer.left))
    });
    let mut signs = Vec::new();
    tree.signs(&mut signs);
    let sign = tree.true_sign();
    let two_ways = signs.iter().any(|each| *each > 0.0) && signs.iter().any(|each| *each < 0.0);
    if !same_pair || !two_ways || sign == 0.0 {
        return false;
    }
    let left_is_minuend = if is_greater(outer) {
        sign > 0.0
    } else {
        sign < 0.0
    };
    let (minuend, subtrahend) = if left_is_minuend {
        (&outer.left, &outer.right)
    } else {
        (&outer.right, &outer.left)
    };
    tree.matches_subtraction(minuend, subtrahend)
}

impl Rule for PreferSimpleSortComparator {
    fn name(&self) -> &'static str {
        "unicorn/prefer-simple-sort-comparator"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(sort) =
            method_call(call).filter(|method| matches!(method.name, "sort" | "toSorted"))
        else {
            return;
        };
        let Some(&[comparator]) = sort.arguments().as_deref() else {
            return;
        };
        let Some(function) = callback(comparator) else {
            return;
        };
        let generator =
            matches!(comparator, Expression::FunctionExpression(function) if function.generator);
        let is_async = match comparator {
            Expression::ArrowFunctionExpression(arrow) => arrow.r#async,
            Expression::FunctionExpression(function) => function.r#async,
            _ => false,
        };
        let Some(&[first, second]) = function.parameter_names().as_deref() else {
            return;
        };
        if generator || is_async {
            return;
        }
        let tree = match function.expression {
            Some(expression) => expression_tree(expression),
            None => statements_tree(function.statements),
        };
        let Some(tree @ Tree::Branch(..)) = tree else {
            return;
        };
        if is_simple(&tree, first, second) && !skips_non_array_receiver(source, sort.object) {
            findings.push(source.finding(
                self.name(),
                comparator.span().start,
                "Prefer a simple comparison function for `Array#sort()`.".to_owned(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferSimpleSortComparator;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferSimpleSortComparator, "src/a.ts", code))
    }

    #[test]
    fn reports_the_documented_verbose_comparators() {
        let message = ["Prefer a simple comparison function for `Array#sort()`."];

        assert_eq!(
            check(
                "array.sort((a, b) => { if (a > b) { return 1; } if (a < b) { return -1; } return 0; });"
            ),
            message
        );
        assert_eq!(check("array.sort((a, b) => a > b ? 1 : -1);"), message);
        assert_eq!(
            check("array.sort((a, b) => a.foo > b.foo ? -1 : 1);"),
            message
        );
        assert_eq!(
            check("const s = array.toSorted((a: T, b: T) => a[0] < b[0] ? -1 : +1);"),
            message
        );
        assert_eq!(
            check("array.sort(function (a, b) { if (b.x >= a.x) return -2; else return 3; });"),
            message
        );
        assert_eq!(
            check("array.sort((a, b) => a > b ? 1 : b > a ? -1 : 0);"),
            message
        );
        // On the comparator: `array.sort(` is 11 characters.
        let findings = lint(
            &PreferSimpleSortComparator,
            "src/a.ts",
            "array.sort((a, b) => a > b ? 1 : -1);",
        );
        assert_eq!(findings[0].column, 12);
    }

    #[test]
    fn accepts_subtractions_and_other_comparators() {
        for code in [
            "array.sort((a, b) => a - b);",
            "array.sort((a, b) => b.foo - a.foo);",
            "array.sort((a, b) => a.localeCompare(b));",
            "array.sort((a, b) => a.foo - b.foo || a.bar - b.bar);",
            "array.sort((a, b) => a.foo > b.foo ? 1 : a.bar > b.bar ? 1 : -1);",
            "array.sort(() => Math.random() > 0.5 ? 1 : -1);",
            "array.sort((a, b) => a?.x > b?.x ? 1 : -1);",
            "array.sort((a, b) => a > b ? 1 : 0);",
            "array.sort((a, b) => a > b ? 0 : 1);",
            "array.sort((a, b) => a === b ? 0 : -1);",
            "array.sort((a, b) => a > c ? 1 : -1);",
            "array.sort((a, b) => a > b ? 1 : a < b ? 1 : -1);",
            "array.sort(async (a, b) => a > b ? 1 : -1);",
            "array.sort(([a], b) => a > b ? 1 : -1);",
            "array.sort((a, b, c) => a > b ? 1 : -1);",
            "array.sort((a, b) => { log(a); return a > b ? 1 : -1; });",
            "array.sort(compare);",
            "array.sort((a, b) => a > b ? 1 : -1, x);",
            "new Set().sort((a, b) => a > b ? 1 : -1);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
