//! `unicorn/no-loop-iterable-mutation`: no `Array`, `Set` nor `Map` mutation of the iterable a
//! `for…of` loop is reading, which skips, repeats or adds items; updating or deleting the
//! current entry of a `Set` or a `Map` is allowed, deleting then re-inserting it is not
//! (`eslint-plugin-unicorn` 78.0.0). Whether a branch always leaves the loop is read from its
//! last statement.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    BindingPattern, Expression, ForOfStatement, ForStatementLeft, Statement,
    VariableDeclarationKind,
};
use oxc_semantic::{AstNode, NodeId, SymbolId};
use oxc_span::GetSpan;

use super::syntax::{
    Kind, call_of, descendants, has_optional_chain, is_function_or_class, is_reference, kind_of,
    method_call, path_up, same_reference_binding, static_member, symbol_of, text, unwrap,
};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoLoopIterableMutation;

/// The methods that change an array.
const ARRAY_MUTATIONS: [&str; 9] = [
    "copyWithin",
    "fill",
    "pop",
    "push",
    "reverse",
    "shift",
    "sort",
    "splice",
    "unshift",
];
/// The methods that change a set.
const SET_MUTATIONS: [&str; 3] = ["add", "clear", "delete"];
/// The methods that change a map.
const MAP_MUTATIONS: [&str; 3] = ["clear", "delete", "set"];

/// How a loop reads its iterable: directly, or by `entries()`, `keys()` or `values()`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Read {
    /// `for (… of iterable)`.
    Direct,
    /// `iterable.entries()`.
    Entries,
    /// `iterable.keys()`.
    Keys,
    /// `iterable.values()`.
    Values,
}

/// The iterable a loop reads live, its kind, and how.
struct Live<'b, 'a> {
    /// The reference read.
    reference: &'b Expression<'a>,
    /// What it is known to be.
    kind: Option<Kind>,
    /// How the loop reads it.
    read: Read,
}

/// What a loop allows itself: the bindings that name the current entry.
#[derive(Default, Clone, Copy)]
struct Current {
    /// What `add()` may add back: the current value.
    add: Option<SymbolId>,
    /// What `set()` may set: the current key.
    set: Option<SymbolId>,
    /// What `delete()` may delete: the current entry.
    delete: Option<SymbolId>,
}

/// The iterable a loop's right side reads live.
fn live_iterable<'b, 'a>(source: &Source, right: &'b Expression<'a>) -> Option<Live<'b, 'a>> {
    let right = unwrap(right);
    if has_optional_chain(right) {
        return None;
    }
    if is_reference(right) {
        return Some(Live {
            reference: right,
            kind: kind_of(source, right),
            read: Read::Direct,
        });
    }
    let Expression::CallExpression(call) = right else {
        return None;
    };
    let (object, name) = static_member(&call.callee)?;
    let read = match name.as_str() {
        "entries" => Read::Entries,
        "keys" => Read::Keys,
        "values" => Read::Values,
        _ => return None,
    };
    (call.arguments.is_empty() && is_reference(object)).then(|| Live {
        reference: object,
        kind: kind_of(source, object),
        read,
    })
}

/// The symbol a binding pattern names directly.
fn identifier(pattern: &BindingPattern) -> Option<SymbolId> {
    match pattern {
        BindingPattern::BindingIdentifier(identifier) => identifier.symbol_id.get(),
        _ => None,
    }
}

/// The symbol of the first element of an array pattern.
fn first_element(pattern: &BindingPattern) -> Option<SymbolId> {
    match pattern {
        BindingPattern::ArrayPattern(array) => {
            array.elements.first()?.as_ref().and_then(identifier)
        }
        _ => None,
    }
}

/// The bindings that name the current entry of a `const` loop.
fn current(statement: &ForOfStatement, live: &Live) -> Current {
    let ForStatementLeft::VariableDeclaration(declaration) = &statement.left else {
        return Current::default();
    };
    let Some(binding) = declaration.declarations.first().map(|each| &each.id) else {
        return Current::default();
    };
    if declaration.kind != VariableDeclarationKind::Const {
        return Current::default();
    }
    let add = match (live.read, live.kind) {
        (Read::Entries, _) => first_element(binding),
        (Read::Direct, Some(Kind::Map)) => None,
        _ => identifier(binding),
    };
    let set = match (live.read, live.kind) {
        (Read::Keys, _) => identifier(binding),
        (Read::Entries, _) => first_element(binding),
        (Read::Direct, kind) if kind != Some(Kind::Set) => first_element(binding),
        _ => None,
    };
    let delete = match (live.read, live.kind) {
        (Read::Direct | Read::Values, Some(Kind::Set)) => identifier(binding),
        (Read::Direct | Read::Entries, _) => identifier(binding).or_else(|| first_element(binding)),
        _ => set,
    };
    Current { add, set, delete }
}

/// Whether a call's first argument is a binding.
fn first_argument_is(
    source: &Source,
    call: &oxc_ast::ast::CallExpression,
    symbol: Option<SymbolId>,
) -> bool {
    let Some(symbol) = symbol else {
        return false;
    };
    call.arguments
        .first()
        .and_then(|argument| argument.as_expression())
        .is_some_and(|argument| match unwrap(argument) {
            Expression::Identifier(identifier) => symbol_of(source, identifier) == Some(symbol),
            _ => false,
        })
}

/// Whether a statement deletes the current entry before what follows it runs.
fn deletes_current(
    source: &Source,
    statement: &Statement,
    iterable: &Expression,
    allowed: Current,
) -> bool {
    match statement {
        Statement::ExpressionStatement(only) => call_of(&only.expression).is_some_and(|call| {
            method_call(call).is_some_and(|method| {
                method.name == "delete"
                    && same_reference_binding(source, method.object, iterable)
                    && first_argument_is(source, call, allowed.delete)
            })
        }),
        Statement::BlockStatement(block) => block
            .body
            .iter()
            .any(|each| deletes_current(source, each, iterable, allowed)),
        Statement::IfStatement(branch) => {
            (!always_exits(&branch.consequent) && deletes_current(source, &branch.consequent, iterable, allowed))
                || branch.alternate.as_ref().is_some_and(|alternate| {
                    !always_exits(alternate) && deletes_current(source, alternate, iterable, allowed)
                })
        }
        Statement::SwitchStatement(switch) => switch.cases.iter().enumerate().any(|(index, case)| {
            let leaves = switch.cases[index..]
                .iter()
                .find_map(|each| {
                    if each.consequent.iter().any(|statement| {
                        matches!(statement, Statement::ContinueStatement(_) | Statement::ReturnStatement(_) | Statement::ThrowStatement(_))
                    }) {
                        Some(true)
                    } else if each.consequent.iter().any(|statement| {
                        matches!(statement, Statement::BreakStatement(each) if each.label.is_none())
                    }) {
                        Some(false)
                    } else {
                        None
                    }
                })
                .unwrap_or(false);
            !leaves && case.consequent.iter().any(|each| deletes_current(source, each, iterable, allowed))
        }),
        _ => false,
    }
}

/// Whether a branch always leaves: it ends with `return`, `throw`, `break` or `continue`.
fn always_exits(statement: &Statement) -> bool {
    match statement {
        Statement::ReturnStatement(_)
        | Statement::ThrowStatement(_)
        | Statement::BreakStatement(_)
        | Statement::ContinueStatement(_) => true,
        Statement::BlockStatement(block) => block.body.last().is_some_and(always_exits),
        _ => false,
    }
}

/// Whether a statement before the call, in its list or an enclosing one up to the loop's body,
/// deletes the current entry.
fn deleted_before(
    source: &Source,
    call: NodeId,
    body: NodeId,
    iterable: &Expression,
    allowed: Current,
) -> bool {
    let nodes = source.semantic.nodes();
    let mut child = call;
    for ancestor in nodes.ancestors(call) {
        let list: Option<&[Statement]> = match ancestor.kind() {
            AstKind::BlockStatement(block) => Some(&block.body),
            AstKind::SwitchCase(case) => Some(&case.consequent),
            _ => None,
        };
        if let Some(list) = list {
            let before = list.iter().take_while(|each| each.node_id() != child);
            if before
                .into_iter()
                .any(|each| deletes_current(source, each, iterable, allowed))
            {
                return true;
            }
            if ancestor.id() == body {
                return false;
            }
        }
        if is_function_or_class(&ancestor.kind()) {
            return false;
        }
        child = ancestor.id();
    }
    false
}

/// Whether a method changes a collection of a kind.
fn mutates(method: &str, kind: Option<Kind>) -> bool {
    match kind {
        Some(Kind::Array) => ARRAY_MUTATIONS.contains(&method),
        Some(Kind::Set) => SET_MUTATIONS.contains(&method),
        Some(Kind::Map) => MAP_MUTATIONS.contains(&method),
        _ => {
            ARRAY_MUTATIONS.contains(&method)
                || SET_MUTATIONS.contains(&method)
                || MAP_MUTATIONS.contains(&method)
        }
    }
}

impl Rule for NoLoopIterableMutation {
    fn name(&self) -> &'static str {
        "unicorn/no-loop-iterable-mutation"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::ForOfStatement(statement) = node.kind() else {
            return;
        };
        if statement.r#await {
            return;
        }
        let Some(live) = live_iterable(source, &statement.right) else {
            return;
        };
        let allowed = current(statement, &live);
        let body = statement.body.node_id();
        let nodes = source.semantic.nodes();
        let calls = std::iter::once(nodes.get_node(body)).chain(descendants(source, body));
        for each in calls {
            let AstKind::CallExpression(call) = each.kind() else {
                continue;
            };
            if path_up(source, each.id(), body).any(|step| is_function_or_class(&step.kind())) {
                continue;
            }
            let Some((object, method)) = static_member(&call.callee) else {
                continue;
            };
            if !mutates(&method, live.kind)
                || !same_reference_binding(source, object, live.reference)
            {
                continue;
            }
            if method == "delete" && first_argument_is(source, call, allowed.delete) {
                continue;
            }
            let keeps_current = |allowed: Current| {
                (method == "add" && first_argument_is(source, call, allowed.add))
                    || (method == "set" && first_argument_is(source, call, allowed.set))
            };
            if keeps_current(allowed)
                && !deleted_before(source, each.id(), body, live.reference, allowed)
            {
                continue;
            }
            let nested = nodes
                .ancestors(each.id())
                .take_while(|step| step.id() != node.id())
                .find_map(|step| {
                    let AstKind::ForOfStatement(inner) = step.kind() else {
                        return None;
                    };
                    let inner_live =
                        live_iterable(source, &inner.right).filter(|_| !inner.r#await)?;
                    same_reference_binding(source, inner_live.reference, live.reference)
                        .then(|| current(inner, &inner_live))
                });
            if nested.is_some_and(keeps_current) {
                continue;
            }
            let property = match &call.callee {
                Expression::ComputedMemberExpression(member) => member.expression.span().start,
                Expression::StaticMemberExpression(member) => member.property.span.start,
                other => other.span().start,
            };
            let finding = source.finding(
                self.name(),
                property,
                format!(
                    "Do not mutate `{}` while iterating over it.",
                    text(source, live.reference.span())
                ),
            );
            let reported = findings.iter().any(|known| {
                known.rule == finding.rule
                    && known.line == finding.line
                    && known.column == finding.column
            });
            if !reported {
                findings.push(finding);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoLoopIterableMutation;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoLoopIterableMutation, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_mutations() {
        assert_eq!(
            check("for (const item of items) {\n\titems.push(item.clone());\n}"),
            ["Do not mutate `items` while iterating over it."]
        );
        assert_eq!(
            check("for (const value of values) {\n\tvalues.shift();\n}"),
            ["Do not mutate `values` while iterating over it."]
        );
        assert_eq!(
            check(
                "const set = new Set();\nfor (const value of set) {\n\tset.delete(value);\n\tset.add(value);\n}"
            ),
            ["Do not mutate `set` while iterating over it."]
        );
        assert_eq!(
            check("for (const item of this.items) { this.items.splice(0, 1); }").len(),
            1
        );
        assert_eq!(
            check("for (const key of map.keys()) { map.set(other, 1); }").len(),
            1
        );
        assert_eq!(
            check("for (const item of items) { if (a) { items.pop(); } }").len(),
            1
        );
        assert_eq!(
            check("for (const item of items) { items['push'](1); }").len(),
            1
        );
        assert_eq!(
            check("const m = new Map();\nfor (const [k] of m) { m.delete(k); if (x) { m.set(k, 1); } }").len(),
            1
        );
    }

    #[test]
    fn accepts_the_documented_safe_updates() {
        for code in [
            "for (const item of [...items]) {\n\titems.push(item.clone());\n}",
            "for (const key of Object.keys(object)) {\n\tdelete object[key];\n}",
            "const set = new Set();\nfor (const value of set) {\n\tset.add(value);\n}",
            "const map = new Map();\nfor (const key of map.keys()) {\n\tmap.set(key, newValue);\n}",
            "const map = new Map();\nfor (const [key] of map) {\n\tmap.delete(key);\n}",
            "for (const item of items) { other.push(item); }",
            "for (const item of items) { items.map(f); }",
            "for (const item of items) { list.forEach(() => items.push(1)); }",
            "for (const item of items?.list) { items.list.push(1); }",
            "async function f() { for await (const item of items) { items.push(1); } }",
            "const set = new Set();\nfor (const v of set) { if (x) { set.delete(v); continue; } set.add(v); }",
            "const set = new Set();\nfor (const v of set) { for (const w of set) { set.add(w); } }",
            "const array = [1];\nfor (const v of array) { array.add(v); }",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
