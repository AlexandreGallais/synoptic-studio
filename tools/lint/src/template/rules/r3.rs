//! The template as Angular's compiler hands it to the plugin's rules (`render3/r3_ast.ts`): the
//! nodes a list of siblings becomes, comments left out and an `@if`, `@for` or `@defer` joined
//! to its connected blocks, the whitespace-only texts between them and after them dropped
//! (`findConnectedBlocks` of `render3/r3_template_transform.ts` of `@angular/compiler` 22.2.3,
//! REF-ANGULAR-R3-TRANSFORM);
//! and an expression as Angular's expression parser builds it (`expression_parser/ast.ts`), so
//! that two expressions compare as `areEquivalentASTs` of `@angular-eslint/eslint-plugin-template`
//! 22.6.0 compares them (`src/utils/are-equivalent-asts.ts`).

use oxc_ast::ast::{
    Argument, ArrayExpressionElement, CallExpression, ChainElement, Expression, LogicalOperator,
    ObjectPropertyKind, UnaryOperator,
};
use oxc_span::GetSpan;

use super::nested_at;
use crate::finding::Fix;
use crate::template::expressions::Bound;
use crate::template::lexer::Span;
use crate::template::tree::{Block, Element, Node, Text};

/// A node of a list of siblings, as Angular's tree holds it.
#[derive(Clone, Debug)]
pub enum R3<'t> {
    /// An element, a `<ng-template>` and an element with a structural directive among them.
    Element(&'t Element),
    /// A text.
    Text(&'t Text),
    /// An `@if`, its `@else if` and `@else` branches after it.
    If(Vec<&'t Block>),
    /// A `@for` and its `@empty`.
    For(&'t Block, Option<&'t Block>),
    /// Any other node: another block, a `@let`, an expansion.
    Other(&'t Node),
}

/// Whether a node is a text of whitespace only.
fn is_blank(node: &Node) -> bool {
    matches!(node, Node::Text(text) if text.value.trim().is_empty())
}

/// The blocks connected to the block at an index: those the predicate accepts, comments and
/// blank texts between skipped; with the index of the first sibling past them.
fn connected<'t>(
    nodes: &'t [Node],
    index: usize,
    accepts: &dyn Fn(&str) -> bool,
) -> (Vec<&'t Block>, usize) {
    let mut found = Vec::new();
    let mut next = index + 1;
    while let Some(node) = nodes.get(next) {
        match node {
            Node::Comment(_) => {}
            _ if is_blank(node) => {}
            Node::Block(block) if accepts(&block.name) => found.push(block),
            _ => break,
        }
        next += 1;
    }
    (found, next)
}

/// The nodes a list of siblings becomes in Angular's tree.
#[must_use]
pub fn siblings(nodes: &[Node]) -> Vec<R3<'_>> {
    let mut found = Vec::new();
    let mut index = 0;
    while let Some(node) = nodes.get(index) {
        index += 1;
        match node {
            Node::Comment(_) => {}
            Node::Element(element) => found.push(R3::Element(element)),
            Node::Text(text) => found.push(R3::Text(text)),
            Node::Block(block) => {
                let accepts: &dyn Fn(&str) -> bool = match block.name.as_str() {
                    "if" => &|name| name == "else" || name == "else if",
                    "for" => &|name| name == "empty",
                    "defer" => &|name| matches!(name, "placeholder" | "loading" | "error"),
                    "boundary" => &|name| name == "error",
                    _ => {
                        found.push(R3::Other(node));
                        continue;
                    }
                };
                let first = index;
                let (blocks, next) = connected(nodes, index - 1, accepts);
                index = next;
                match block.name.as_str() {
                    "if" => {
                        let mut branches = vec![block];
                        branches.extend(blocks);
                        found.push(R3::If(branches));
                    }
                    "for" => found.push(R3::For(block, blocks.first().copied())),
                    // A `@defer`'s `@placeholder`, `@loading` and `@error`, a `@boundary`'s
                    // `@error`: their content is walked as the block's.
                    _ => {
                        found.push(R3::Other(node));
                        let joined = nodes[first..next]
                            .iter()
                            .filter(|each| matches!(each, Node::Block(_)));
                        found.extend(joined.map(R3::Other));
                    }
                }
            }
            Node::LetDeclaration(_) | Node::Expansion(_) => found.push(R3::Other(node)),
        }
    }
    found
}

/// A literal as Angular reads it.
#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    /// A string.
    Text(String),
    /// A number.
    Number(f64),
    /// A boolean.
    Bool(bool),
    /// `null`.
    Null,
    /// `undefined`.
    Undefined,
}

/// An expression as Angular's parser builds it, the parts the plugin's rules compare.
#[derive(Clone, Debug, PartialEq)]
pub enum Ng {
    /// The component, read implicitly (`name`).
    Implicit,
    /// `this`.
    This,
    /// `receiver.name`, `receiver?.name`.
    Read(Box<Self>, String, bool),
    /// `receiver[key]`, `receiver?.[key]`.
    Keyed(Box<Self>, Box<Self>, bool),
    /// `receiver(args)`, `receiver?.(args)`.
    Call(Box<Self>, Vec<Self>, bool),
    /// `expression!`.
    NonNull(Box<Self>),
    /// `!expression`.
    Not(Box<Self>),
    /// `-expression`, `+expression`.
    Unary(String, Box<Self>),
    /// `typeof expression`.
    Typeof(Box<Self>),
    /// `left operation right`, the logical operators among them.
    Binary(String, Box<Self>, Box<Self>),
    /// `condition ? yes : no`.
    Conditional(Box<Self>, Box<Self>, Box<Self>),
    /// A literal.
    Literal(Literal),
    /// `[elements]`.
    Array(Vec<Self>),
    /// `{key: value}`: each key, `None` for a spread, and the values.
    Map(Vec<Option<String>>, Vec<Self>),
    /// `expression | name: args`.
    Pipe(String, Box<Self>, Vec<Self>),
    /// `(expression)`, never equivalent to another.
    Parenthesized(Box<Self>),
    /// A template literal's expressions, never equivalent to another.
    Template(Vec<Self>),
    /// `void expression`, never equivalent to another.
    Void(Box<Self>),
    /// Anything the plugin never compares equal: a template literal, an arrow, a void.
    Other,
}

/// A bound read as Angular's tree: its expression, each pipe wrapping the one before.
#[must_use]
pub fn of_bound(bound: &Bound) -> Ng {
    let mut built = bound
        .expression
        .as_ref()
        .map_or(Ng::Other, |expression| of_expression(expression, bound));
    for pipe in &bound.pipes {
        built = Ng::Pipe(
            pipe.name.clone(),
            Box::new(built),
            pipe.arguments.iter().map(of_bound).collect(),
        );
    }
    built
}

/// The arguments of a call.
fn arguments(call: &CallExpression, bound: &Bound) -> Vec<Ng> {
    call.arguments
        .iter()
        .map(|argument| match argument {
            Argument::SpreadElement(_) => Ng::Other,
            other => other
                .as_expression()
                .map_or(Ng::Other, |each| of_expression(each, bound)),
        })
        .collect()
}

/// A call as Angular reads it.
fn of_call(call: &CallExpression, bound: &Bound) -> Ng {
    Ng::Call(
        Box::new(of_expression(&call.callee, bound)),
        arguments(call, bound),
        call.optional,
    )
}

/// An element of an optional chain as Angular reads it.
fn of_chain(element: &ChainElement, bound: &Bound) -> Ng {
    match element {
        ChainElement::CallExpression(call) => of_call(call, bound),
        ChainElement::TSNonNullExpression(assertion) => {
            Ng::NonNull(Box::new(of_expression(&assertion.expression, bound)))
        }
        ChainElement::StaticMemberExpression(member) => Ng::Read(
            Box::new(of_expression(&member.object, bound)),
            member.property.name.to_string(),
            member.optional,
        ),
        ChainElement::ComputedMemberExpression(member) => Ng::Keyed(
            Box::new(of_expression(&member.object, bound)),
            Box::new(of_expression(&member.expression, bound)),
            member.optional,
        ),
        ChainElement::PrivateFieldExpression(_) => Ng::Other,
    }
}

/// An Oxc expression of a bound as Angular reads it; a masked pipe expression its nested bound.
#[must_use]
pub fn of_expression(expression: &Expression, bound: &Bound) -> Ng {
    let boxed = |each: &Expression| Box::new(of_expression(each, bound));
    match expression {
        Expression::Identifier(name) => {
            if let Some(nested) = nested_at(bound, name.span) {
                return of_bound(nested);
            }
            if name.name == "undefined" {
                return Ng::Literal(Literal::Undefined);
            }
            Ng::Read(Box::new(Ng::Implicit), name.name.to_string(), false)
        }
        Expression::ThisExpression(_) => Ng::This,
        Expression::StaticMemberExpression(member) => Ng::Read(
            boxed(&member.object),
            member.property.name.to_string(),
            member.optional,
        ),
        Expression::ComputedMemberExpression(member) => Ng::Keyed(
            boxed(&member.object),
            boxed(&member.expression),
            member.optional,
        ),
        Expression::CallExpression(call) => of_call(call, bound),
        Expression::ChainExpression(chain) => of_chain(&chain.expression, bound),
        Expression::TSNonNullExpression(assertion) => Ng::NonNull(boxed(&assertion.expression)),
        Expression::UnaryExpression(unary) => match unary.operator {
            UnaryOperator::LogicalNot => Ng::Not(boxed(&unary.argument)),
            UnaryOperator::Typeof => Ng::Typeof(boxed(&unary.argument)),
            UnaryOperator::Void => Ng::Void(boxed(&unary.argument)),
            UnaryOperator::UnaryNegation | UnaryOperator::UnaryPlus => {
                Ng::Unary(unary.operator.as_str().to_owned(), boxed(&unary.argument))
            }
            _ => Ng::Other,
        },
        Expression::BinaryExpression(binary) => Ng::Binary(
            binary.operator.as_str().to_owned(),
            boxed(&binary.left),
            boxed(&binary.right),
        ),
        Expression::LogicalExpression(logical) => Ng::Binary(
            match logical.operator {
                LogicalOperator::And => "&&",
                LogicalOperator::Or => "||",
                LogicalOperator::Coalesce => "??",
            }
            .to_owned(),
            boxed(&logical.left),
            boxed(&logical.right),
        ),
        Expression::ConditionalExpression(conditional) => Ng::Conditional(
            boxed(&conditional.test),
            boxed(&conditional.consequent),
            boxed(&conditional.alternate),
        ),
        Expression::StringLiteral(string) => Ng::Literal(Literal::Text(string.value.to_string())),
        Expression::NumericLiteral(number) => Ng::Literal(Literal::Number(number.value)),
        Expression::BooleanLiteral(boolean) => Ng::Literal(Literal::Bool(boolean.value)),
        Expression::NullLiteral(_) => Ng::Literal(Literal::Null),
        Expression::ArrayExpression(array) => Ng::Array(
            array
                .elements
                .iter()
                .map(|element| match element {
                    ArrayExpressionElement::SpreadElement(_)
                    | ArrayExpressionElement::Elision(_) => Ng::Other,
                    other => other
                        .as_expression()
                        .map_or(Ng::Other, |each| of_expression(each, bound)),
                })
                .collect(),
        ),
        Expression::ObjectExpression(object) => {
            let mut keys = Vec::new();
            let mut values = Vec::new();
            for property in &object.properties {
                match property {
                    ObjectPropertyKind::ObjectProperty(property) => {
                        keys.push(property.key.static_name().map(|name| name.to_string()));
                        values.push(of_expression(&property.value, bound));
                    }
                    ObjectPropertyKind::SpreadProperty(spread) => {
                        keys.push(None);
                        values.push(of_expression(&spread.argument, bound));
                    }
                }
            }
            Ng::Map(keys, values)
        }
        Expression::ParenthesizedExpression(inner) => Ng::Parenthesized(boxed(&inner.expression)),
        Expression::TemplateLiteral(literal) => Ng::Template(
            literal
                .expressions
                .iter()
                .map(|each| of_expression(each, bound))
                .collect(),
        ),
        _ => Ng::Other,
    }
}

/// Whether two expressions are equivalent (`areEquivalentASTs`): `this.x` is `x`, a literal is
/// its value, a parenthesized expression, a template literal or an arrow never equivalent.
#[must_use]
pub fn equivalent(one: &Ng, other: &Ng) -> bool {
    let all = |ones: &[Ng], others: &[Ng]| {
        ones.len() == others.len() && ones.iter().zip(others).all(|(x, y)| equivalent(x, y))
    };
    match (one, other) {
        (Ng::Implicit | Ng::This, Ng::Implicit | Ng::This) => true,
        (Ng::Read(receiver, name, safe), Ng::Read(receiver_too, name_too, safe_too)) => {
            name == name_too && safe == safe_too && equivalent(receiver, receiver_too)
        }
        (Ng::Keyed(receiver, key, safe), Ng::Keyed(receiver_too, key_too, safe_too)) => {
            safe == safe_too && equivalent(key, key_too) && equivalent(receiver, receiver_too)
        }
        (Ng::Call(receiver, arguments, safe), Ng::Call(receiver_too, arguments_too, safe_too)) => {
            safe == safe_too && all(arguments, arguments_too) && equivalent(receiver, receiver_too)
        }
        (Ng::NonNull(inner), Ng::NonNull(inner_too))
        | (Ng::Not(inner), Ng::Not(inner_too))
        | (Ng::Typeof(inner), Ng::Typeof(inner_too)) => equivalent(inner, inner_too),
        (Ng::Unary(operator, inner), Ng::Unary(operator_too, inner_too)) => {
            operator == operator_too && equivalent(inner, inner_too)
        }
        (Ng::Binary(operator, left, right), Ng::Binary(operator_too, left_too, right_too)) => {
            operator == operator_too && equivalent(left, left_too) && equivalent(right, right_too)
        }
        (Ng::Conditional(test, yes, no), Ng::Conditional(test_too, yes_too, no_too)) => {
            equivalent(test, test_too) && equivalent(yes, yes_too) && equivalent(no, no_too)
        }
        (Ng::Literal(value), Ng::Literal(value_too)) => value == value_too,
        (Ng::Array(elements), Ng::Array(elements_too)) => all(elements, elements_too),
        (Ng::Map(keys, values), Ng::Map(keys_too, values_too)) => {
            keys == keys_too && all(values, values_too)
        }
        (Ng::Pipe(name, inner, arguments), Ng::Pipe(name_too, inner_too, arguments_too)) => {
            name == name_too && equivalent(inner, inner_too) && all(arguments, arguments_too)
        }
        _ => false,
    }
}

/// Whether an expression reads a `length` (`isLengthRead`), and what from.
#[must_use]
pub fn length_receiver(node: &Ng) -> Option<&Ng> {
    match node {
        Ng::Read(receiver, name, false) if name == "length" => Some(receiver),
        _ => None,
    }
}

/// Whether an expression is the literal `0` (`isZero`).
#[must_use]
pub fn is_zero(node: &Ng) -> bool {
    matches!(node, Ng::Literal(Literal::Number(number)) if *number == 0.0)
}

/// The fixes of one report merged into one, as `ESLint` merges them: sorted, the text between
/// kept, from the first range's start to the last range's end; `None` when two overlap.
#[must_use]
pub fn merged(text: &str, mut fixes: Vec<Fix>) -> Option<Fix> {
    fixes.sort_by_key(|fix| (fix.start, fix.end));
    let start = fixes.first()?.start;
    let mut end = start;
    let mut written = String::new();
    for fix in &fixes {
        if fix.start < end {
            return None;
        }
        written.push_str(text.get(end as usize..fix.start as usize)?);
        written.push_str(&fix.text);
        end = fix.end;
    }
    Some(Fix {
        start,
        end,
        text: written,
    })
}

/// A replacement of a span.
#[must_use]
pub fn replace(start: u32, end: u32, text: &str) -> Fix {
    Fix {
        start,
        end,
        text: text.to_owned(),
    }
}

/// The text of a range of the template.
#[must_use]
pub fn slice(text: &str, start: u32, end: u32) -> &str {
    text.get(start as usize..end as usize).unwrap_or_default()
}

/// The span of an Oxc node of a bound, placed in the template.
#[must_use]
pub fn span_of(bound: &Bound, node: &impl GetSpan) -> Span {
    let span = node.span();
    Span::new(
        (bound.start + span.start) as usize,
        (bound.start + span.end) as usize,
    )
}

/// Whether an expression is a constant (`isConstantExpression`): built from literals with
/// operators, conditionals, parentheses, `typeof`, `void`, `!` and `!` assertions only.
#[must_use]
pub fn is_constant(node: &Ng) -> bool {
    match node {
        Ng::Literal(_) => true,
        Ng::Map(_, values) | Ng::Array(values) | Ng::Template(values) => {
            values.iter().all(is_constant)
        }
        Ng::Parenthesized(inner)
        | Ng::Unary(_, inner)
        | Ng::Not(inner)
        | Ng::Typeof(inner)
        | Ng::Void(inner)
        | Ng::NonNull(inner) => is_constant(inner),
        Ng::Binary(_, left, right) => is_constant(left) && is_constant(right),
        Ng::Conditional(condition, yes, no) => {
            is_constant(condition) && is_constant(yes) && is_constant(no)
        }
        _ => false,
    }
}

/// The expression inside parentheses (`unwrapParenthesizedExpression`).
#[must_use]
pub fn unwrapped(node: &Ng) -> &Ng {
    match node {
        Ng::Parenthesized(inner) => unwrapped(inner),
        other => other,
    }
}

#[cfg(test)]
mod tests {
    //! This module's own cases: the plugin tests these helpers through its rules only.

    use oxc_allocator::Allocator;

    use super::{R3, equivalent, is_constant, merged, of_bound, replace, siblings};
    use crate::template::expressions::read;
    use crate::template::tree::parse;

    /// Each sibling's kind, an `@if` with its number of branches.
    fn kinds(source: &str) -> Vec<String> {
        siblings(&parse(source).nodes)
            .iter()
            .map(|node| match node {
                R3::Element(element) => format!("<{}>", element.name),
                R3::Text(text) => format!("text {:?}", text.value),
                R3::If(branches) => format!("@if {}", branches.len()),
                R3::For(_, empty) => format!("@for {}", empty.is_some()),
                R3::Other(crate::template::tree::Node::Block(block)) => format!("@{}", block.name),
                R3::Other(_) => "other".to_owned(),
            })
            .collect()
    }

    #[test]
    fn joins_connected_blocks_and_drops_the_blank_texts_after_them() {
        // The comment and the line ends between `@if` and `@else` go, as do those after; the
        // text after an element stays.
        assert_eq!(
            kinds("@if (a) {} <!-- c -->\n@else {}\n@if (b) {}\n<i></i> t"),
            ["@if 2", "@if 1", "<i>", "text \" t\""]
        );
        assert_eq!(kinds("@for (x of y; track x) {} @empty {}"), ["@for true"]);
        // A `@defer`'s connected blocks follow it, to be walked.
        assert_eq!(
            kinds("@defer {} @placeholder {x} @loading {y}"),
            ["@defer", "@placeholder", "@loading"]
        );
    }

    #[test]
    fn compares_expressions_as_angular_s_trees() {
        let allocator = Allocator::default();
        let ng = |text: &str| read(&allocator, text, 0).map(|bound| of_bound(&bound));
        let same = |one: &str, other: &str| match (ng(one), ng(other)) {
            (Ok(one), Ok(other)) => equivalent(&one, &other),
            _ => false,
        };

        assert!(same("this.a.b", "a.b"));
        assert!(same("a?.[0] | p: 1", "a?.[0] | p: 1"));
        assert!(!same("a?.b", "a.b"));
        assert!(!same("(a)", "(a)"));
        assert!(!same("`a`", "`a`"));
        assert!(same("{x: 1, ...y}", "{x: 1, ...y}"));
        assert!(!same("1", "'1'"));
    }

    #[test]
    fn tells_a_constant_expression() {
        let allocator = Allocator::default();
        let constant = |text: &str| {
            read(&allocator, text, 0).is_ok_and(|bound| is_constant(&of_bound(&bound)))
        };

        assert!(constant("{color: 'red', width: 1 + 2}"));
        assert!(constant("!true ? `a${1}` : [null, undefined]"));
        assert!(!constant("{color: c}"));
        assert!(!constant("'a' | p"));
    }

    #[test]
    fn merges_the_fixes_of_a_report_or_refuses_overlapping_ones() {
        let text = "abcdef";

        // `b` replaced and `x` inserted before `e`, the text between kept.
        assert_eq!(
            merged(text, vec![replace(4, 4, "x"), replace(1, 2, "B")])
                .map(|fix| (fix.start, fix.end, fix.text)),
            Some((1, 4, "Bcdx".to_owned()))
        );
        assert_eq!(
            merged(text, vec![replace(1, 3, ""), replace(2, 4, "")]),
            None
        );
        assert_eq!(merged(text, Vec::new()), None);
    }
}
