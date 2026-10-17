//! `unicorn/prefer-map-from-entries`: `new Map(…)` rather than `Object.fromEntries(…)` for a
//! `const` only read as a map — by a static key, `Object.hasOwn()`, `Object.keys()`, `values()`
//! or `entries()`, set or deleted as a statement (`eslint-plugin-unicorn` 78.0.0). Autofixed, as
//! in `ESLint`: the call and every use rewritten together.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    ArrayExpressionElement, AssignmentOperator, Expression, UnaryOperator, VariableDeclarator,
};
use oxc_semantic::{AstNode, NodeId};
use oxc_span::{GetSpan, Span};

use super::syntax::{
    Value, has_comment, is_global_name_free, method_call, method_named, static_value, text,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferMapFromEntries;

/// The names every object inherits, which a map would not hold.
const INHERITED: [&str; 12] = [
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

/// Whether a key names an array index, whose order an object and a map keep differently.
fn is_array_index(key: &str) -> bool {
    key.parse::<u64>()
        .is_ok_and(|number| number <= (1 << 32) - 2 && number.to_string() == key)
}

/// A key as a single-quoted string literal.
fn quoted(key: &str) -> String {
    let mut out = String::from("'");
    for character in key.chars() {
        match character {
            '\\' => out.push_str("\\\\"),
            '\'' => out.push_str("\\'"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            other => out.push(other),
        }
    }
    out.push('\'');
    out
}

/// The string a key is written as.
fn static_string(source: &Source, expression: &Expression) -> Option<String> {
    match static_value(source, expression)? {
        Value::Text(text) => Some(text),
        _ => None,
    }
}

/// Whether the entries are known to have string keys that are no array index: `Object.entries(x)`
/// or a literal array of literal pairs.
fn has_string_keys(source: &Source, node: &AstNode, entries: &Expression) -> bool {
    if let Some(call) = method_named(entries, &["entries"]) {
        return call.is_plain()
            && call.call.arguments.len() == 1
            && matches!(call.object, Expression::Identifier(object) if object.name == "Object")
            && is_global_name_free(source, node, "Object");
    }
    let Expression::ArrayExpression(array) = entries else {
        return false;
    };
    !array.elements.is_empty()
        && array.elements.iter().all(|element| match element {
            ArrayExpressionElement::ArrayExpression(pair) => {
                let key = pair
                    .elements
                    .first()
                    .and_then(ArrayExpressionElement::as_expression);
                pair.elements.len() >= 2
                    && key
                        .and_then(|key| match key {
                            Expression::StringLiteral(_) | Expression::TemplateLiteral(_) => {
                                static_string(source, key)
                            }
                            _ => None,
                        })
                        .is_some_and(|key| !is_array_index(&key))
            }
            _ => false,
        })
}

/// One edit of the fix.
struct Edit {
    /// The range replaced.
    span: Span,
    /// The new text.
    text: String,
}

/// Whether a node sits in a destructuring pattern, a `new` callee or an optional chain.
fn is_in_awkward_place(source: &Source, member: NodeId) -> bool {
    let nodes = source.semantic.nodes();
    nodes.ancestors(member).any(|each| {
        matches!(
            each.kind(),
            AstKind::ArrayAssignmentTarget(_)
                | AstKind::ObjectAssignmentTarget(_)
                | AstKind::ArrayPattern(_)
                | AstKind::ObjectPattern(_)
                | AstKind::ChainExpression(_)
        )
    }) || {
        let mut current = member;
        loop {
            let parent = nodes.parent_node(current);
            match parent.kind() {
                AstKind::StaticMemberExpression(_)
                | AstKind::ComputedMemberExpression(_)
                | AstKind::TSAsExpression(_)
                | AstKind::TSSatisfiesExpression(_)
                | AstKind::TSNonNullExpression(_)
                | AstKind::TSTypeAssertion(_)
                | AstKind::TSInstantiationExpression(_) => current = parent.id(),
                AstKind::NewExpression(new) => break new.callee.node_id() == current,
                _ => break false,
            }
        }
    }
}

/// The edit of `Object.hasOwn(name, key)`, `Object.keys(name)`, `values()` or `entries()`.
fn object_method_edit(
    source: &Source,
    parent: &AstNode,
    call: &oxc_ast::ast::CallExpression,
    reference: NodeId,
    name: &str,
) -> Option<Edit> {
    let method = method_call(call).filter(|method| {
        method.is_plain()
            && matches!(method.object, Expression::Identifier(object) if object.name == "Object")
            && matches!(method.name, "hasOwn" | "entries" | "keys" | "values")
    })?;
    if !is_global_name_free(source, parent, "Object") {
        return None;
    }
    let arguments = method.arguments()?;
    if arguments
        .first()
        .is_none_or(|first| first.node_id() != reference)
    {
        return None;
    }
    let text = if method.name == "hasOwn" {
        let [_, key] = arguments.as_slice() else {
            return None;
        };
        format!("{name}.has({})", quoted(&static_string(source, key)?))
    } else if arguments.len() == 1 {
        format!("[...{name}.{}()]", method.name)
    } else {
        return None;
    };
    Some(Edit {
        span: call.span,
        text,
    })
}

/// The edit one read of the variable needs, when it reads it as a map.
fn edit_for(source: &Source, reference: NodeId, name: &str) -> Option<Edit> {
    let nodes = source.semantic.nodes();
    let parent = nodes.parent_node(reference);
    if let AstKind::CallExpression(call) = parent.kind() {
        return object_method_edit(source, parent, call, reference, name);
    }
    let (object, key) = match parent.kind() {
        AstKind::StaticMemberExpression(member) if !member.optional => {
            (&member.object, member.property.name.to_string())
        }
        AstKind::ComputedMemberExpression(member) if !member.optional => (
            &member.object,
            static_string(source, &member.expression).or_else(|| {
                match static_value(source, &member.expression)? {
                    Value::Number(number) => Some(number.to_string()),
                    _ => None,
                }
            })?,
        ),
        _ => return None,
    };
    if object.node_id() != reference
        || is_array_index(&key)
        || INHERITED.contains(&key.as_str())
        || is_in_awkward_place(source, parent.id())
    {
        return None;
    }
    let member_span = parent.kind().span();
    let wrapped = matches!(
        nodes.parent_kind(parent.id()),
        AstKind::TSAsExpression(_)
            | AstKind::TSSatisfiesExpression(_)
            | AstKind::TSNonNullExpression(_)
            | AstKind::TSTypeAssertion(_)
            | AstKind::TSInstantiationExpression(_)
    );
    if wrapped {
        return None;
    }
    let statement = |id: NodeId| matches!(nodes.parent_kind(id), AstKind::ExpressionStatement(_));
    let quoted_key = quoted(&key);
    match nodes.parent_kind(parent.id()) {
        AstKind::AssignmentExpression(assignment) if assignment.left.span() == member_span => {
            let id = nodes.parent_id(parent.id());
            (assignment.operator == AssignmentOperator::Assign && statement(id)).then(|| {
                let value = if matches!(assignment.right, Expression::SequenceExpression(_)) {
                    format!("({})", text(source, assignment.right.span()))
                } else {
                    text(source, assignment.right.span()).to_owned()
                };
                Edit {
                    span: assignment.span,
                    text: format!("{name}.set({quoted_key}, {value})"),
                }
            })
        }
        AstKind::UnaryExpression(unary) if unary.operator == UnaryOperator::Delete => {
            let id = nodes.parent_id(parent.id());
            statement(id).then(|| Edit {
                span: unary.span,
                text: format!("{name}.delete({quoted_key})"),
            })
        }
        AstKind::UpdateExpression(_)
        | AstKind::ForInStatement(_)
        | AstKind::ForOfStatement(_)
        | AstKind::AssignmentExpression(_)
        | AstKind::TaggedTemplateExpression(_) => None,
        AstKind::CallExpression(call) if call.callee.span() == member_span => None,
        AstKind::NewExpression(new) if new.callee.span() == member_span => None,
        _ => Some(Edit {
            span: member_span,
            text: format!("{name}.get({quoted_key})"),
        }),
    }
}

/// The finding on a declarator, with its fix when every use reads it as a map.
fn declarator_finding(
    source: &Source,
    rule: &'static str,
    node: &AstNode,
    declarator: &VariableDeclarator,
) -> Option<Finding> {
    let oxc_ast::ast::BindingPattern::BindingIdentifier(identifier) = &declarator.id else {
        return None;
    };
    let nodes = source.semantic.nodes();
    let AstKind::VariableDeclaration(declaration) = nodes.parent_kind(node.id()) else {
        return None;
    };
    let exported = matches!(
        nodes.parent_kind(nodes.parent_id(node.id())),
        AstKind::ExportDeclaration(_)
    );
    if declarator.type_annotation.is_some()
        || declaration.kind != oxc_ast::ast::VariableDeclarationKind::Const
        || exported
    {
        return None;
    }
    let init = declarator.init.as_ref()?;
    let from = method_named(init, &["fromEntries"]).filter(|method| {
        method.is_plain()
            && method.call.type_arguments.is_none()
            && matches!(method.object, Expression::Identifier(object) if object.name == "Object")
    })?;
    let Some(&[entries]) = from.arguments().as_deref() else {
        return None;
    };
    if !is_global_name_free(source, node, "Object")
        || !is_global_name_free(source, node, "Map")
        || !has_string_keys(source, node, entries)
    {
        return None;
    }
    let symbol = identifier.symbol_id.get()?;
    let scoping = source.semantic.scoping();
    let name = identifier.name.as_str();
    let mut edits = Vec::new();
    for reference in scoping.get_resolved_reference_ids(symbol) {
        let reference_node = scoping.get_reference(*reference).node_id();
        edits.push(edit_for(source, reference_node, name)?);
    }
    if edits.is_empty() {
        return None;
    }
    edits.sort_by_key(|edit| edit.span.start);
    if edits
        .windows(2)
        .any(|pair| pair[0].span.end > pair[1].span.start)
    {
        return None;
    }
    let message = "Prefer `new Map()` over `Object.fromEntries()` when using the result as a map.";
    let callee = from.call.callee.span();
    let lossy =
        has_comment(source, callee) || edits.iter().any(|edit| has_comment(source, edit.span));
    if lossy {
        return Some(source.finding(rule, init.span().start, message.to_owned()));
    }
    edits.insert(
        0,
        Edit {
            span: callee,
            text: "new Map".to_owned(),
        },
    );
    let start = callee.start;
    let end = edits
        .iter()
        .map(|edit| edit.span.end)
        .max()
        .unwrap_or(callee.end);
    let mut replacement = String::new();
    let mut at = start;
    for edit in &edits {
        replacement.push_str(text(source, Span::new(at, edit.span.start)));
        replacement.push_str(&edit.text);
        at = edit.span.end;
    }
    Some(source.fixed(
        rule,
        init.span().start,
        message.to_owned(),
        Fix {
            start,
            end,
            text: replacement,
        },
    ))
}

impl Rule for PreferMapFromEntries {
    fn name(&self) -> &'static str {
        "unicorn/prefer-map-from-entries"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        if let AstKind::VariableDeclarator(declarator) = node.kind() {
            findings.extend(declarator_finding(source, self.name(), node, declarator));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferMapFromEntries;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferMapFromEntries, "playground/a.ts", code))
    }

    /// The code after the fixes.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&PreferMapFromEntries, "playground/a.ts", code))
    }

    #[test]
    fn reports_and_fixes_the_documented_map_use() {
        let code = "const object = Object.fromEntries(Object.entries(source));\nif (Object.hasOwn(object, 'foo')) {\n\tconsole.log(object.foo);\n}";
        assert_eq!(
            check(code),
            ["Prefer `new Map()` over `Object.fromEntries()` when using the result as a map."]
        );
        assert_eq!(
            fix(code),
            "const object = new Map(Object.entries(source));\nif (object.has('foo')) {\n\tconsole.log(object.get('foo'));\n}"
        );
    }

    #[test]
    fn rewrites_writes_deletes_and_iterations() {
        assert_eq!(
            fix(
                "const o = Object.fromEntries([['a', 1], ['b', 2]]);\no.c = 3;\ndelete o['a'];\nconst k = Object.keys(o);"
            ),
            "const o = new Map([['a', 1], ['b', 2]]);\no.set('c', 3);\no.delete('a');\nconst k = [...o.keys()];"
        );
    }

    #[test]
    fn reports_without_a_fix_around_a_comment() {
        let code = "const o = Object.fromEntries(Object.entries(s));\nuse(o./* a */a);";
        let findings = lint(&PreferMapFromEntries, "playground/a.ts", code);
        assert_eq!(findings.len(), 1);
        assert_eq!(fixed(code, &findings), code);
    }

    #[test]
    fn accepts_objects_used_as_objects() {
        for code in [
            "const object = Object.fromEntries(entries);\nfoo(object);",
            "const object = Object.fromEntries(Object.entries(source));\nfoo(object);",
            "const object = Object.fromEntries(Object.entries(source));",
            "const o = Object.fromEntries(Object.entries(s));\nuse(o.toString);",
            "const o = Object.fromEntries(Object.entries(s));\nuse(o[0]);",
            "const o = Object.fromEntries(Object.entries(s));\nuse(o[key]);",
            "const o = Object.fromEntries(Object.entries(s));\no.a();",
            "const o = Object.fromEntries(Object.entries(s));\no.a++;",
            "const o = Object.fromEntries(Object.entries(s));\nconst x = o.a = 1;",
            "const o = Object.fromEntries(Object.entries(s));\no.a += 1;",
            "const o = Object.fromEntries(Object.entries(s));\nuse(o?.a);",
            "const o = Object.fromEntries(Object.entries(s));\nuse(o.a as number);",
            "const o = Object.fromEntries(Object.entries(s));\nconst { a } = o;",
            "const o = Object.fromEntries(Object.entries(s));\nnew o.A();",
            "const o = Object.fromEntries([['0', 1]]);\nuse(o.a);",
            "const o = Object.fromEntries([[key, 1]]);\nuse(o.a);",
            "const o: Record<string, number> = Object.fromEntries(Object.entries(s));\nuse(o.a);",
            "let o = Object.fromEntries(Object.entries(s));\nuse(o.a);",
            "export const o = Object.fromEntries(Object.entries(s));\nuse(o.a);",
            "const Map = 1; const o = Object.fromEntries(Object.entries(s));\nuse(o.a);",
            "const o = Object.fromEntries<number>(Object.entries(s));\nuse(o.a);",
            "const o = Object.fromEntries(Object.entries(s));\nuse(Object.hasOwn(o, key));",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
