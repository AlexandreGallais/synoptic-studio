//! `unicorn/prefer-set-methods`: the `Set` methods for set operations — `union()` rather than
//! spreading Sets into a new one, `intersection()` and `difference()` rather than filtering by
//! `has()`, `isSubsetOf()` and `isDisjointFrom()` rather than `every()`, `some()` or an empty
//! result's size (`eslint-plugin-unicorn` 78.0.0). Autofixed where `ESLint` fixes; the filters
//! only reported, their fix being a suggestion there.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    ArrayExpression, ArrayExpressionElement, BinaryExpression, BinaryOperator, BindingPattern,
    Expression, UnaryOperator,
};
use oxc_semantic::AstNode;
use oxc_span::Span;

use super::syntax::{
    Kind, MethodCall, has_comment, has_side_effect, is_global, kind_of, method_call, method_named,
    object_text, outer_parent, parenthesized_text, unwrap,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferSetMethods;

/// The message on a union by spreading.
const UNION: &str = "Use `Set#union()` instead of spreading Sets into a new Set.";

/// Whether an expression is a known `Set`.
fn is_set(source: &Source, expression: &Expression) -> bool {
    kind_of(source, expression) == Some(Kind::Set)
}

/// Whether reading an expression may run code, getters included.
fn may_run_code(expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::StaticMemberExpression(_)
        | Expression::ComputedMemberExpression(_)
        | Expression::PrivateFieldExpression(_) => true,
        other => has_side_effect(other),
    }
}

/// The Sets spread into an array literal, when it holds nothing else and each is safe to read
/// again.
fn union_operands<'b, 'a>(
    source: &Source,
    array: &'b ArrayExpression<'a>,
) -> Option<Vec<&'b Expression<'a>>> {
    let operands: Option<Vec<&Expression>> = array
        .elements
        .iter()
        .map(|element| match element {
            ArrayExpressionElement::SpreadElement(spread) => {
                let argument = &spread.argument;
                let empty_set = matches!(argument, Expression::NewExpression(new)
                    if new.arguments.is_empty() && is_global(source, &new.callee, "Set"));
                (is_set(source, argument) && (empty_set || !may_run_code(argument)))
                    .then_some(argument)
            }
            _ => None,
        })
        .collect();
    operands.filter(|each| each.len() >= 2)
}

/// The one Set spread into an array literal of one element.
fn single_spread_set<'b, 'a>(
    source: &Source,
    expression: &'b Expression<'a>,
) -> Option<&'b Expression<'a>> {
    let Expression::ArrayExpression(array) = expression else {
        return None;
    };
    match array.elements.as_slice() {
        [ArrayExpressionElement::SpreadElement(spread)] if is_set(source, &spread.argument) => {
            Some(&spread.argument)
        }
        _ => None,
    }
}

/// A set operation a callback's body makes: `other.has(value)` or `!other.has(value)`, with the
/// other Set and whether it is negated.
fn set_operation<'b, 'a>(
    source: &Source,
    callback: &'b Expression<'a>,
) -> Option<(&'b Expression<'a>, bool)> {
    let Expression::ArrowFunctionExpression(arrow) = callback else {
        return None;
    };
    let [parameter] = arrow.params.items.as_slice() else {
        return None;
    };
    let BindingPattern::BindingIdentifier(binding) = &parameter.pattern else {
        return None;
    };
    let symbol = binding.symbol_id.get()?;
    let once = source
        .semantic
        .scoping()
        .get_resolved_reference_ids(symbol)
        .len()
        == 1;
    if arrow.r#async || arrow.params.rest.is_some() || !once {
        return None;
    }
    let body = arrow.get_expression()?;
    let (call, negated) = match body {
        Expression::UnaryExpression(unary) if unary.operator == UnaryOperator::LogicalNot => {
            (&unary.argument, true)
        }
        other => (other, false),
    };
    let has = method_named(call, &["has"]).filter(MethodCall::is_plain)?;
    let Some(&[argument]) = has.arguments().as_deref() else {
        return None;
    };
    let reads_parameter =
        matches!(argument, Expression::Identifier(each) if each.name == binding.name);
    let other_is_parameter =
        matches!(has.object, Expression::Identifier(each) if each.name == binding.name);
    (reads_parameter
        && !other_is_parameter
        && is_set(source, has.object)
        && !may_run_code(has.object))
    .then_some((has.object, negated))
}

/// The text before a replacement that starts where a keyword ends: a space.
fn keyword_space(source: &Source, start: u32) -> &'static str {
    let last = source
        .text
        .get(..start as usize)
        .and_then(|before| before.chars().last());
    if last.is_some_and(|character| {
        character.is_alphanumeric() || character == '_' || character == '$'
    }) {
        " "
    } else {
        ""
    }
}

/// The finding on a predicate replaced by `isSubsetOf()` or `isDisjointFrom()`, fixed unless a
/// comment would be lost.
fn predicate_finding(
    source: &Source,
    rule: &'static str,
    node: &AstNode,
    span: Span,
    (set, other, method, negated): (&Expression, &Expression, &str, bool),
) -> Finding {
    let message = if method == "isSubsetOf" {
        "Use `Set#isSubsetOf()` to check whether a Set is a subset of another Set."
    } else {
        "Use `Set#isDisjointFrom()` to check whether Sets have any elements in common."
    };
    if has_comment(source, span) {
        return source.finding(rule, span.start, message.to_owned());
    }
    let call = format!(
        "{}.{method}({})",
        object_text(source, set),
        parenthesized_text(source, other)
    );
    let replacement = if negated {
        let tight_parent = match source.semantic.nodes().parent_kind(node.id()) {
            AstKind::StaticMemberExpression(member) => member.object.node_id() == node.id(),
            AstKind::ComputedMemberExpression(member) => member.object.node_id() == node.id(),
            AstKind::CallExpression(parent) => parent.callee.node_id() == node.id(),
            AstKind::NewExpression(parent) => parent.callee.node_id() == node.id(),
            AstKind::TaggedTemplateExpression(parent) => parent.tag.node_id() == node.id(),
            AstKind::BinaryExpression(parent) => {
                parent.operator == BinaryOperator::Exponential && parent.left.node_id() == node.id()
            }
            AstKind::TSNonNullExpression(_) => true,
            _ => false,
        };
        if tight_parent {
            format!("(!{call})")
        } else {
            format!("!{call}")
        }
    } else {
        call
    };
    source.fixed(
        rule,
        span.start,
        message.to_owned(),
        Fix {
            start: span.start,
            end: span.end,
            text: format!("{}{replacement}", keyword_space(source, span.start)),
        },
    )
}

/// The finding on `[...set].every(…)` or `[...set].some(…)` by membership in another Set.
fn array_predicate_finding(
    source: &Source,
    rule: &'static str,
    node: &AstNode,
    method: &MethodCall,
) -> Option<Finding> {
    if !matches!(method.name, "every" | "some") || !method.is_plain() {
        return None;
    }
    let Some(&[callback]) = method.arguments().as_deref() else {
        return None;
    };
    let set = single_spread_set(source, method.object)?;
    let (other, difference) = set_operation(source, callback)?;
    let every = method.name == "every";
    let name = match (every, difference) {
        (true, false) | (false, true) => "isSubsetOf",
        (true, true) | (false, false) => "isDisjointFrom",
    };
    Some(predicate_finding(
        source,
        rule,
        node,
        method.call.span,
        (set, other, name, !every),
    ))
}

/// The finding on `[...set].filter(…)` by membership in another Set, at the reported node.
fn filter_finding(
    source: &Source,
    rule: &'static str,
    method: &MethodCall,
    at: Span,
) -> Option<Finding> {
    if method.name != "filter" || !method.is_plain() {
        return None;
    }
    let Some(&[callback]) = method.arguments().as_deref() else {
        return None;
    };
    single_spread_set(source, method.object)?;
    let (_, difference) = set_operation(source, callback)?;
    let message = if difference {
        "Use `Set#difference()` instead of filtering by `Set#has()`."
    } else {
        "Use `Set#intersection()` instead of filtering by `Set#has()`."
    };
    Some(source.finding(rule, at.start, message.to_owned()))
}

/// The finding on `set.intersection(other).size === 0` and its kin.
fn size_finding(
    source: &Source,
    rule: &'static str,
    node: &AstNode,
    binary: &BinaryExpression,
) -> Option<Finding> {
    let is_zero = |expression: &Expression| matches!(expression, Expression::NumericLiteral(literal) if literal.value == 0.0);
    let strict = matches!(
        binary.operator,
        BinaryOperator::StrictEquality | BinaryOperator::StrictInequality
    );
    let positive = (binary.operator == BinaryOperator::GreaterThan && is_zero(&binary.right))
        || (binary.operator == BinaryOperator::LessThan && is_zero(&binary.left));
    if !strict && !positive {
        return None;
    }
    let (size, zero) = if is_zero(&binary.right) {
        (&binary.left, &binary.right)
    } else {
        (&binary.right, &binary.left)
    };
    let Expression::StaticMemberExpression(member) = size else {
        return None;
    };
    if !is_zero(zero) || member.optional || member.property.name != "size" {
        return None;
    }
    let operation = method_named(&member.object, &["intersection", "difference"])
        .filter(MethodCall::is_plain)?;
    let Some(&[other]) = operation.arguments().as_deref() else {
        return None;
    };
    if !is_set(source, operation.object) || !is_set(source, other) {
        return None;
    }
    let name = if operation.name == "intersection" {
        "isDisjointFrom"
    } else {
        "isSubsetOf"
    };
    let negated = binary.operator != BinaryOperator::StrictEquality;
    Some(predicate_finding(
        source,
        rule,
        node,
        binary.span,
        (operation.object, other, name, negated),
    ))
}

/// The finding on `new Set([...set, ...other])`, fixed unless a comment would be lost.
fn union_finding(
    source: &Source,
    rule: &'static str,
    new_span: Span,
    operands: &[&Expression],
) -> Finding {
    if has_comment(source, new_span) {
        return source.finding(rule, new_span.start, UNION.to_owned());
    }
    let (first, rest) = operands
        .split_first()
        .map_or((None, &[][..]), |(a, b)| (Some(a), b));
    let replacement = rest.iter().fold(
        first.map_or_else(String::new, |first| object_text(source, first)),
        |text, operand| text + ".union(" + &parenthesized_text(source, operand) + ")",
    );
    source.fixed(
        rule,
        new_span.start,
        UNION.to_owned(),
        Fix {
            start: new_span.start,
            end: new_span.end,
            text: replacement,
        },
    )
}

impl Rule for PreferSetMethods {
    fn name(&self) -> &'static str {
        "unicorn/prefer-set-methods"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        match node.kind() {
            AstKind::NewExpression(new) => {
                let [argument] = new.arguments.as_slice() else {
                    return;
                };
                let Some(argument) = argument.as_expression() else {
                    return;
                };
                if !is_global(source, &new.callee, "Set") {
                    return;
                }
                if let Expression::ArrayExpression(array) = argument
                    && let Some(operands) = union_operands(source, array)
                {
                    findings.push(union_finding(source, self.name(), new.span, &operands));
                    return;
                }
                let filter = unwrap(argument);
                let member_object = matches!(
                    outer_parent(source, filter.node_id()).0,
                    AstKind::StaticMemberExpression(_) | AstKind::ComputedMemberExpression(_)
                );
                let Expression::CallExpression(call) = filter else {
                    return;
                };
                if let Some(method) = method_call(call).filter(|_| !member_object) {
                    findings.extend(filter_finding(source, self.name(), &method, new.span));
                }
            }
            AstKind::CallExpression(call) => {
                let Some(method) = method_call(call) else {
                    return;
                };
                if let Some(finding) = array_predicate_finding(source, self.name(), node, &method) {
                    findings.push(finding);
                    return;
                }
                let (parent, child) = outer_parent(source, node.id());
                let part_of_call = match parent {
                    AstKind::CallExpression(outer) => {
                        outer.callee.node_id() == child
                            || outer.arguments.iter().any(|each| {
                                each.as_expression()
                                    .is_some_and(|each| each.node_id() == child)
                            })
                    }
                    AstKind::NewExpression(outer) => {
                        outer.callee.node_id() == child
                            || outer.arguments.iter().any(|each| {
                                each.as_expression()
                                    .is_some_and(|each| each.node_id() == child)
                            })
                    }
                    _ => false,
                };
                let member_object = matches!(
                    parent,
                    AstKind::StaticMemberExpression(_) | AstKind::ComputedMemberExpression(_)
                );
                let wrapped = matches!(
                    source.semantic.nodes().parent_kind(node.id()),
                    AstKind::TSAsExpression(_)
                        | AstKind::TSSatisfiesExpression(_)
                        | AstKind::TSNonNullExpression(_)
                        | AstKind::TSTypeAssertion(_)
                        | AstKind::TSInstantiationExpression(_)
                );
                if !part_of_call && !member_object && !wrapped {
                    findings.extend(filter_finding(source, self.name(), &method, call.span));
                }
            }
            AstKind::BinaryExpression(binary) => {
                findings.extend(size_finding(source, self.name(), node, binary));
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferSetMethods;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The declarations the cases read.
    const SETUP: &str = "const set = new Set([1]);\nconst otherSet = new Set([2]);\n";

    /// The messages on a piece of code after the declarations.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(
            &PreferSetMethods,
            "src/a.ts",
            &format!("{SETUP}{code}"),
        ))
    }

    /// The code after the fixes, the declarations left out.
    fn fix(code: &str) -> String {
        let all = format!("{SETUP}{code}");
        fixed(&all, &lint(&PreferSetMethods, "src/a.ts", &all))[SETUP.len()..].to_owned()
    }

    #[test]
    fn reports_and_fixes_the_documented_unions() {
        // [F19.AC11] A kept Unicorn rule, on the cases of its plugin's documentation.
        assert_eq!(
            check("const union = new Set([...set, ...otherSet]);"),
            ["Use `Set#union()` instead of spreading Sets into a new Set."]
        );
        assert_eq!(
            fix("const union = new Set([...set, ...otherSet]);"),
            "const union = set.union(otherSet);"
        );
        assert_eq!(
            fix("const union = new Set([...set, ...otherSet, ...new Set()]);"),
            "const union = set.union(otherSet).union(new Set());"
        );
    }

    #[test]
    fn reports_the_documented_filters() {
        assert_eq!(
            check("const intersection = [...set].filter(value => otherSet.has(value));"),
            ["Use `Set#intersection()` instead of filtering by `Set#has()`."]
        );
        assert_eq!(
            check("const difference = [...set].filter(value => !otherSet.has(value));"),
            ["Use `Set#difference()` instead of filtering by `Set#has()`."]
        );
        assert_eq!(
            check("const difference = new Set([...set].filter(value => !otherSet.has(value)));"),
            ["Use `Set#difference()` instead of filtering by `Set#has()`."]
        );
        // Reported, not fixed: the fix is a suggestion.
        assert_eq!(
            fix("const i = [...set].filter(value => otherSet.has(value));"),
            "const i = [...set].filter(value => otherSet.has(value));"
        );
    }

    #[test]
    fn reports_and_fixes_the_documented_predicates() {
        assert_eq!(
            check("const subset = [...set].every(value => otherSet.has(value));"),
            ["Use `Set#isSubsetOf()` to check whether a Set is a subset of another Set."]
        );
        let cases = [
            (
                "const subset = [...set].every(value => otherSet.has(value));",
                "const subset = set.isSubsetOf(otherSet);",
            ),
            (
                "const overlaps = [...set].some(value => otherSet.has(value));",
                "const overlaps = !set.isDisjointFrom(otherSet);",
            ),
            (
                "const disjoint = set.intersection(otherSet).size === 0;",
                "const disjoint = set.isDisjointFrom(otherSet);",
            ),
            (
                "const contained = set.difference(otherSet).size === 0;",
                "const contained = set.isSubsetOf(otherSet);",
            ),
            (
                "const a = 0 === set.intersection(otherSet).size;",
                "const a = set.isDisjointFrom(otherSet);",
            ),
            (
                "const a = set.intersection(otherSet).size > 0;",
                "const a = !set.isDisjointFrom(otherSet);",
            ),
            (
                "const a = 0 < set.difference(otherSet).size;",
                "const a = !set.isSubsetOf(otherSet);",
            ),
            (
                "const a = set.difference(otherSet).size !== 0;",
                "const a = !set.isSubsetOf(otherSet);",
            ),
            (
                "const a = [...set].every(value => !otherSet.has(value));",
                "const a = set.isDisjointFrom(otherSet);",
            ),
            (
                "const a = [...set].some(value => !otherSet.has(value));",
                "const a = !set.isSubsetOf(otherSet);",
            ),
            (
                "const a = [...set].some(value => otherSet.has(value)).valueOf();",
                "const a = (!set.isDisjointFrom(otherSet)).valueOf();",
            ),
            (
                "function f() { return[...set].every(value => otherSet.has(value)); }",
                "function f() { return set.isSubsetOf(otherSet); }",
            ),
        ];
        for (code, after) in cases {
            assert_eq!(fix(code), after, "{code}");
        }
    }

    #[test]
    fn accepts_iterables_and_other_callbacks() {
        for code in [
            "new Set([...iterable, ...otherIterable]);",
            "new Set([...set]);",
            "new Set([...set, 1]);",
            "new Set([...set, ...a.b]);",
            "const u = set.union(otherSet);",
            "const i = [...set].filter(value => other.has(value));",
            "const i = [...set].filter(value => otherSet.has(value) && value > 1);",
            "const i = [...set].filter((value) => { return otherSet.has(value); });",
            "const i = [...set].filter(async value => otherSet.has(value));",
            "const i = [...set].filter(value => otherSet.has(value + value));",
            "const i = [...set].filter(value => otherSet.has(other));",
            "const i = [...set].filter(value => otherSet?.has(value));",
            "const i = [...set, 1].filter(value => otherSet.has(value));",
            "const n = [...set].filter(value => otherSet.has(value)).length;",
            "use([...set].filter(value => otherSet.has(value)));",
            "const i = [...set].filter(value => otherSet.has(value)) as number[];",
            "const d = set.intersection(otherSet).size === 1;",
            "const d = set.intersection(other).size === 0;",
            "const d = set.union(otherSet).size === 0;",
            "const d = set.intersection(otherSet).size >= 0;",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
