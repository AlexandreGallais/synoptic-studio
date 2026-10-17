//! `unicorn/prefer-minimal-ternary`: a conditional expression whose branches share their shape
//! and differ in one part only — one argument of a call or a `new`, one side of a binary
//! operator, the computed key of a member read, one value of an object or an array — moves into
//! that part (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-minimal-ternary.md`, default
//! options). Shared parts before the varying one must be names or literals. Autofixed unless a
//! comment is inside, an object's other values are not literals (TypeScript without types), or
//! moving a shared part before the test could change what runs first.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Argument, ArrayExpressionElement, BinaryOperator, ConditionalExpression, Expression,
    ObjectExpression, ObjectPropertyKind, PropertyKind, UnaryOperator,
};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::expressions::{edited, has_comment_in, has_side_effect, parent, same_tokens, text};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferMinimalTernary;

/// The rule's message.
const MESSAGE: &str = "Move the ternary into the varying part of the expression.";

/// Whether two expressions are the same: the same text, or the same tokens with no function or
/// class inside.
fn same(source: &Source, left: &Expression, right: &Expression) -> bool {
    let (a, b) = (text(source, left.span()), text(source, right.span()));
    a == b
        || (same_tokens(source, left.span(), right.span())
            && !a.contains("=>")
            && !a.contains("function")
            && !a.contains("class"))
}

/// Whether a shared part is a name, a literal, `this` or `super`.
fn is_safe_shared(expression: &Expression) -> bool {
    matches!(
        expression,
        Expression::Identifier(_)
            | Expression::ThisExpression(_)
            | Expression::Super(_)
            | Expression::StringLiteral(_)
            | Expression::NumericLiteral(_)
            | Expression::BooleanLiteral(_)
            | Expression::NullLiteral(_)
            | Expression::BigIntLiteral(_)
            | Expression::RegExpLiteral(_)
    )
}

/// Whether a callee shared by both branches is safe: a safe part, or a member of one by a static
/// or literal key.
fn is_safe_callee(callee: &Expression) -> bool {
    is_safe_shared(callee)
        || match callee {
            Expression::StaticMemberExpression(member) => is_safe_shared(&member.object),
            Expression::ComputedMemberExpression(member) => {
                is_safe_shared(&member.object) && member.expression.is_literal()
            }
            _ => false,
        }
}

/// Whether an expression can be read before the test without changing anything.
fn is_reorderable(expression: &Expression) -> bool {
    let reorderable = match expression.get_inner_expression() {
        each if is_safe_shared(each) => true,
        Expression::StaticMemberExpression(member) => is_reorderable(&member.object),
        Expression::ComputedMemberExpression(member) => {
            is_reorderable(&member.object) && is_reorderable(&member.expression)
        }
        Expression::UnaryExpression(unary) => {
            matches!(
                unary.operator,
                UnaryOperator::LogicalNot | UnaryOperator::Typeof | UnaryOperator::Void
            ) && is_reorderable(&unary.argument)
        }
        Expression::BinaryExpression(binary) => {
            matches!(
                binary.operator,
                BinaryOperator::StrictEquality | BinaryOperator::StrictInequality
            ) && is_reorderable(&binary.left)
                && is_reorderable(&binary.right)
        }
        Expression::LogicalExpression(logical) => {
            is_reorderable(&logical.left) && is_reorderable(&logical.right)
        }
        Expression::ConditionalExpression(conditional) => {
            is_reorderable(&conditional.test)
                && is_reorderable(&conditional.consequent)
                && is_reorderable(&conditional.alternate)
        }
        _ => false,
    };
    reorderable && !has_side_effect(expression)
}

/// The varying item of two lists, when exactly one differs and is no conditional expression.
fn varying_index(source: &Source, left: &[&Expression], right: &[&Expression]) -> Option<usize> {
    if left.len() != right.len() {
        return None;
    }
    let mut found = None;
    for (index, (a, b)) in left.iter().zip(right).enumerate() {
        if same(source, a, b) {
            continue;
        }
        if found.is_some()
            || matches!(a, Expression::ConditionalExpression(_))
            || matches!(b, Expression::ConditionalExpression(_))
        {
            return None;
        }
        found = Some(index);
    }
    found
}

/// The varying item of two lists whose shared items before it are safe.
fn minimal_item(source: &Source, left: &[&Expression], right: &[&Expression]) -> Option<usize> {
    let index = varying_index(source, left, right)?;
    left[..index]
        .iter()
        .all(|each| is_safe_shared(each))
        .then_some(index)
}

/// The expressions of arguments, when none is spread.
fn arguments<'s, 'a>(list: &'s [Argument<'a>]) -> Option<Vec<&'s Expression<'a>>> {
    list.iter().map(Argument::as_expression).collect()
}

/// The expressions of array elements, when none is spread or a hole.
fn elements<'s, 'a>(list: &'s [ArrayExpressionElement<'a>]) -> Option<Vec<&'s Expression<'a>>> {
    list.iter()
        .map(ArrayExpressionElement::as_expression)
        .collect()
}

/// The keys and values of an object's plain properties, when all are plain.
fn properties<'s, 'a>(
    list: &'s [ObjectPropertyKind<'a>],
) -> Option<Vec<(Option<String>, &'s Expression<'a>, bool)>> {
    list.iter()
        .map(|each| match each {
            ObjectPropertyKind::ObjectProperty(property)
                if property.kind == PropertyKind::Init
                    && !property.computed
                    && !property.method =>
            {
                let name = property.key.static_name().map(|name| name.to_string());
                (name.as_deref() != Some("__proto__") || property.shorthand).then_some((
                    name,
                    &property.value,
                    property.shorthand,
                ))
            }
            _ => None,
        })
        .collect()
}

/// A minimal shape found: the parts that vary, the expressions moved before the test, and
/// whether TypeScript without types keeps the fix from being safe.
struct Shape<'s, 'a> {
    /// The varying part of each branch.
    parts: [&'s Expression<'a>; 2],
    /// The text replacing the whole expression, from the varying part's minimal text.
    build: Build,
    /// The shared parts that will be read before the test.
    moved: Vec<&'s Expression<'a>>,
    /// Whether the fix is withheld.
    unsafe_fix: bool,
}

/// How the replacement is written.
enum Build {
    /// The consequent's text with the varying part replaced by the conditional.
    Replace,
    /// The object read with the key `[test ? a : b]`.
    Key,
    /// A shorthand property expanded, `key: test ? a : b`.
    Shorthand(String),
}

/// The minimal shape of two branches.
fn shape<'s, 'a>(
    source: &Source,
    left: &'s Expression<'a>,
    right: &'s Expression<'a>,
) -> Option<Shape<'s, 'a>> {
    let plain = |parts, moved| {
        Some(Shape {
            parts,
            build: Build::Replace,
            moved,
            unsafe_fix: false,
        })
    };
    match (left, right) {
        (Expression::CallExpression(a), Expression::CallExpression(b)) => {
            if a.optional || b.optional || matches!(a.callee, Expression::ChainExpression(_)) {
                return None;
            }
            let (x, y) = (arguments(&a.arguments)?, arguments(&b.arguments)?);
            if !same(source, &a.callee, &b.callee) || !is_safe_callee(&a.callee) {
                return None;
            }
            let index = varying_index(source, &x, &y)?;
            let mut moved = vec![&a.callee];
            moved.extend(&x[..index]);
            plain([x[index], y[index]], moved)
        }
        (Expression::NewExpression(a), Expression::NewExpression(b)) => {
            let same_callee = matches!((&a.callee, &b.callee), (Expression::Identifier(c), Expression::Identifier(d)) if c.name == d.name);
            let type_arguments = |each: &oxc_ast::ast::NewExpression| {
                each.type_arguments
                    .as_ref()
                    .map(|written| text(source, written.span))
            };
            if !same_callee || type_arguments(a) != type_arguments(b) {
                return None;
            }
            let (x, y) = (arguments(&a.arguments)?, arguments(&b.arguments)?);
            let index = minimal_item(source, &x, &y)?;
            plain([x[index], y[index]], x[..index].to_vec())
        }
        (Expression::BinaryExpression(a), Expression::BinaryExpression(b)) => {
            if a.operator != b.operator || matches!(a.left, Expression::PrivateInExpression(_)) {
                return None;
            }
            let (left_same, right_same) = (
                same(source, &a.left, &b.left),
                same(source, &a.right, &b.right),
            );
            if left_same == right_same || (!right_same && !is_safe_shared(&a.left)) {
                return None;
            }
            if right_same {
                plain([&a.left, &b.left], Vec::new())
            } else {
                plain([&a.right, &b.right], vec![&a.left])
            }
        }
        (Expression::ComputedMemberExpression(a), Expression::ComputedMemberExpression(b)) => {
            let dynamic = |key: &Expression| {
                !key.is_literal()
                    && !matches!(key, Expression::TemplateLiteral(t) if t.expressions.is_empty())
            };
            if a.optional
                || b.optional
                || !is_safe_shared(&a.object)
                || !same(source, &a.object, &b.object)
                || !dynamic(&a.expression)
                || !dynamic(&b.expression)
                || same(source, &a.expression, &b.expression)
            {
                return None;
            }
            Some(Shape {
                parts: [&a.expression, &b.expression],
                build: Build::Key,
                moved: vec![&a.object],
                unsafe_fix: false,
            })
        }
        (Expression::ArrayExpression(a), Expression::ArrayExpression(b)) => {
            let (x, y) = (elements(&a.elements)?, elements(&b.elements)?);
            let index = minimal_item(source, &x, &y)?;
            plain([x[index], y[index]], x[..index].to_vec())
        }
        (Expression::ObjectExpression(a), Expression::ObjectExpression(b)) => {
            object_shape(source, a, b)
        }
        _ => None,
    }
}

/// The minimal shape of two objects: the same keys in order, one value varying.
fn object_shape<'s, 'a>(
    source: &Source,
    a: &'s ObjectExpression<'a>,
    b: &'s ObjectExpression<'a>,
) -> Option<Shape<'s, 'a>> {
    let (x, y) = (properties(&a.properties)?, properties(&b.properties)?);
    if x.len() != y.len() || x.iter().zip(&y).any(|(c, d)| c.0.is_none() || c.0 != d.0) {
        return None;
    }
    let (xs, ys): (Vec<&Expression>, Vec<&Expression>) = (
        x.iter().map(|each| each.1).collect(),
        y.iter().map(|each| each.1).collect(),
    );
    let index = minimal_item(source, &xs, &ys)?;
    let others_literal = xs
        .iter()
        .enumerate()
        .all(|(at, each)| at == index || each.is_literal());
    let typed_or_function = [xs[index], ys[index]].iter().any(|each| {
        matches!(
            each,
            Expression::TSAsExpression(_)
                | Expression::TSSatisfiesExpression(_)
                | Expression::TSNonNullExpression(_)
                | Expression::TSTypeAssertion(_)
                | Expression::ArrowFunctionExpression(_)
                | Expression::FunctionExpression(_)
                | Expression::ClassExpression(_)
        )
    });
    let build = if x[index].2 {
        Build::Shorthand(x[index].0.clone().unwrap_or_default())
    } else {
        Build::Replace
    };
    Some(Shape {
        parts: [xs[index], ys[index]],
        build,
        moved: xs[..index].to_vec(),
        unsafe_fix: !others_literal || typed_or_function,
    })
}

/// The text of a conditional's child, parenthesized when it binds looser than `? :`.
fn child_text(source: &Source, expression: &Expression) -> String {
    let inner = text(source, expression.span());
    if matches!(
        expression,
        Expression::SequenceExpression(_)
            | Expression::AssignmentExpression(_)
            | Expression::ArrowFunctionExpression(_)
            | Expression::YieldExpression(_)
            | Expression::ConditionalExpression(_)
    ) {
        format!("({inner})")
    } else {
        inner.to_owned()
    }
}

impl PreferMinimalTernary {
    /// The fix of a conditional expression of a shape, when safe.
    fn fix(
        source: &Source,
        node: &AstNode,
        conditional: &ConditionalExpression,
        shape: &Shape,
    ) -> Option<Fix> {
        let all_literal = shape.moved.iter().all(|each| each.is_literal());
        let reorder = all_literal
            || (is_reorderable(&conditional.test)
                && shape.moved.iter().all(|each| is_reorderable(each)));
        let member_up = matches!(
            conditional.consequent,
            Expression::ComputedMemberExpression(_)
        ) && matches!(
            parent(source, node.id()),
            AstKind::CallExpression(_)
                | AstKind::TaggedTemplateExpression(_)
                | AstKind::UnaryExpression(_)
        );
        if shape.unsafe_fix || !reorder || member_up || has_comment_in(source, conditional.span) {
            return None;
        }
        let test = child_text(source, &conditional.test);
        let [a, b] = shape.parts;
        let choice = format!(
            "{test} ? {} : {}",
            child_text(source, a),
            child_text(source, b)
        );
        let consequent = &conditional.consequent;
        let mut replacement = match &shape.build {
            Build::Key => {
                let Expression::ComputedMemberExpression(member) = consequent else {
                    return None;
                };
                format!("{}[{choice}]", text(source, member.object.span()))
            }
            Build::Shorthand(key) => {
                let mut edits = vec![(a.span().start, a.span().end, format!("{key}: {choice}"))];
                edited(source, consequent.span(), &mut edits)
            }
            Build::Replace => {
                let wrapped = if matches!(consequent, Expression::BinaryExpression(_)) {
                    format!("({choice})")
                } else {
                    choice
                };
                let mut edits = vec![(a.span().start, a.span().end, wrapped)];
                edited(source, consequent.span(), &mut edits)
            }
        };
        let in_operator = matches!(consequent, Expression::BinaryExpression(binary) if binary.operator == BinaryOperator::In);
        if matches!(consequent, Expression::ObjectExpression(_)) || in_operator {
            replacement = format!("({replacement})");
        }
        Some(Fix {
            start: conditional.span.start,
            end: conditional.span.end,
            text: replacement,
        })
    }
}

impl Rule for PreferMinimalTernary {
    fn name(&self) -> &'static str {
        "unicorn/prefer-minimal-ternary"
    }

    fn needs_tokens(&self) -> bool {
        true
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::ConditionalExpression(conditional) = node.kind() else {
            return;
        };
        let Some(shape) = shape(source, &conditional.consequent, &conditional.alternate) else {
            return;
        };
        let at = conditional.span.start;
        findings.push(match Self::fix(source, node, conditional, &shape) {
            Some(fix) => source.fixed(self.name(), at, MESSAGE.to_owned(), fix),
            None => source.finding(self.name(), at, MESSAGE.to_owned()),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::PreferMinimalTernary;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferMinimalTernary, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn moves_the_ternary_into_the_varying_part() {
        // The documentation's cases with the default options.
        assert_eq!(
            check("const foo = test ? call(a) : call(b);"),
            (vec![(1, 13)], "const foo = call(test ? a : b);".to_owned())
        );
        assert_eq!(
            check("const foo = test ? object.method(a) : object.method(b);").1,
            "const foo = object.method(test ? a : b);"
        );
        assert_eq!(
            check("const foo = test ? a + 1 : b + 1;").1,
            "const foo = (test ? a : b) + 1;"
        );
        assert_eq!(
            check("const value = test ? cache[a] : cache[b];").1,
            "const value = cache[test ? a : b];"
        );
        assert_eq!(
            check("const array = test ? [1, 2] : [1, 3];").1,
            "const array = [1, test ? 2 : 3];"
        );
        assert_eq!(
            check("const date = test ? new Date(a) : new Date(b);").1,
            "const date = new Date(test ? a : b);"
        );
        assert_eq!(
            check("const object = test ? {a: 1} : {a: 2};").1,
            "const object = ({a: test ? 1 : 2});"
        );
        // A shorthand property expanded; another value not a literal leaves the fix out.
        assert_eq!(
            check("const o = t ? { a } : { a: 2 };").1,
            "const o = ({ a: t ? a : 2 });"
        );
        let typed = "const o = t ? { a: x, b: 1 } : { a: x, b: 2 };";
        assert_eq!(check(typed), (vec![(1, 11)], typed.to_owned()));
    }

    #[test]
    fn leaves_varying_bases_names_and_other_shapes() {
        for code in [
            "a ? (b ? c : d) : e;",
            "const foo = test ? a() : b();",
            "const foo = test ? a.method(value) : b.method(value);",
            "const foo = test ? a.value : b.value;",
            "const value = test ? object.a : object.b;",
            "const v = t ? f(a, b) : f(c, d);",
            "const v = t ? f(...a) : f(...b);",
            "const v = t ? [1, 2] : [1];",
            "const v = t ? { a: 1 } : { b: 1 };",
            "const v = t ? 'cat' : 'car';",
            "const v = t ? a! : b!;",
            "const v = t ? x.y(a) : x.y(b) as T;",
            "const v = t ? g().h(a) : g().h(b);",
            "const v = t ? f?.(a) : f?.(b);",
            "const v = t ? c[0] : c[1];",
            "const v = t ? a + 1 : b + 2;",
            "const v = t ? [f(), 1] : [f(), 2];",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
    }

    #[test]
    fn withholds_the_fix_when_order_or_comments_matter() {
        for code in [
            "const v = f() ? g(x, a) : g(x, b);",
            "const v = t ? /* a */ g(a) : g(b);",
            "const v = a > b ? x[a] : x[b];",
        ] {
            let (places, result) = check(code);
            assert_eq!(places.len(), 1, "{code}");
            assert_eq!(result, code);
        }
    }
}
