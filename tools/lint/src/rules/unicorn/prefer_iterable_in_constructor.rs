//! `unicorn/prefer-iterable-in-constructor`: no empty `Set`, `WeakSet`, `Map`, `WeakMap` or
//! `URLSearchParams` filled by the `for…of` loop that follows, nor
//! `new URLSearchParams(Object.entries(record))`: the constructor takes the iterable or the
//! record itself (`eslint-plugin-unicorn` 78.0.0). Autofixed, as in `ESLint`.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    ArrayExpressionElement, BindingPattern, Expression, ForStatementLeft, ObjectPropertyKind,
    Statement, VariableDeclaration, VariableDeclarationKind,
};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use super::syntax::{
    has_comment, is_referenced_in, method_named, next_statement, parenthesized_text,
    single_statement, statement_removal, text,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferIterableInConstructor;

/// The argument of a plain `Object.entries(x)`.
fn object_entries_argument<'b, 'a>(expression: &'b Expression<'a>) -> Option<&'b Expression<'a>> {
    let call = method_named(expression, &["entries"]).filter(|method| {
        method.is_plain()
            && matches!(method.object, Expression::Identifier(object) if object.name == "Object")
    })?;
    match call.arguments().as_deref() {
        Some(&[argument]) => Some(argument),
        _ => None,
    }
}

/// Whether a source cannot be handed over as it is: an object with a computed key, a value no
/// record for `URLSearchParams`, an array of non-pairs for a map.
fn is_unsafe_source(constructor: &str, source_node: &Expression) -> bool {
    let computed = matches!(source_node, Expression::ObjectExpression(object)
        if object.properties.iter().any(|property| matches!(property, ObjectPropertyKind::ObjectProperty(each) if each.computed)));
    let not_record = matches!(
        source_node,
        Expression::ArrayExpression(_)
            | Expression::StringLiteral(_)
            | Expression::NumericLiteral(_)
            | Expression::BooleanLiteral(_)
            | Expression::NullLiteral(_)
            | Expression::BigIntLiteral(_)
            | Expression::RegExpLiteral(_)
            | Expression::UnaryExpression(_)
            | Expression::TemplateLiteral(_)
            | Expression::NewExpression(_)
    ) || matches!(source_node, Expression::Identifier(identifier)
        if matches!(identifier.name.as_str(), "Infinity" | "NaN" | "undefined"));
    let non_pairs = matches!(source_node, Expression::ArrayExpression(array)
        if array.elements.iter().any(|element| !matches!(element, ArrayExpressionElement::ArrayExpression(_))));
    computed
        || (constructor == "URLSearchParams" && not_record)
        || (matches!(constructor, "Map" | "WeakMap") && non_pairs)
}

/// The finding on `new URLSearchParams(Object.entries(record))`.
fn entries_finding(source: &Source, rule: &'static str, node: &AstNode) -> Option<Finding> {
    let AstKind::NewExpression(new) = node.kind() else {
        return None;
    };
    let named =
        matches!(&new.callee, Expression::Identifier(callee) if callee.name == "URLSearchParams");
    let [argument] = new.arguments.as_slice() else {
        return None;
    };
    let entries = argument.as_expression().filter(|_| named)?;
    let record = object_entries_argument(entries)?;
    if is_unsafe_source("URLSearchParams", record) {
        return None;
    }
    let Expression::CallExpression(call) = entries else {
        return None;
    };
    let message = format!(
        "Pass `{}` directly to `new URLSearchParams()`.",
        text(source, record.span())
    );
    if has_comment(source, call.span) {
        return Some(source.finding(rule, call.callee.span().start, message));
    }
    Some(source.fixed(
        rule,
        call.callee.span().start,
        message,
        Fix {
            start: call.span.start,
            end: call.span.end,
            text: parenthesized_text(source, record),
        },
    ))
}

/// The names of a `[a, b]` pattern of two identifiers.
fn pair<'a>(pattern: &'a BindingPattern) -> Option<(&'a str, &'a str)> {
    let BindingPattern::ArrayPattern(array) = pattern else {
        return None;
    };
    match array.elements.as_slice() {
        [
            Some(BindingPattern::BindingIdentifier(first)),
            Some(BindingPattern::BindingIdentifier(second)),
        ] if array.rest.is_none() => Some((first.name.as_str(), second.name.as_str())),
        _ => None,
    }
}

/// Whether an argument is the identifier of a name.
fn is_name(expression: Option<&&Expression>, name: &str) -> bool {
    matches!(expression, Some(Expression::Identifier(identifier)) if identifier.name == name)
}

/// Whether the loop's only call fills the collection with the current item: `add(item)` for a
/// set, `set(key, value)` for a map, `append` or `set` for `URLSearchParams`.
fn fills(
    constructor: &str,
    method: &str,
    arguments: &[&Expression],
    left: &BindingPattern,
) -> bool {
    match constructor {
        "Set" | "WeakSet" => {
            method == "add"
                && arguments.len() == 1
                && matches!(left, BindingPattern::BindingIdentifier(each) if is_name(arguments.first(), &each.name))
        }
        _ => {
            let methods_ok = if constructor == "URLSearchParams" {
                matches!(method, "append" | "set")
            } else {
                method == "set"
            };
            methods_ok
                && arguments.len() == 2
                && pair(left).is_some_and(|(key, value)| {
                    is_name(arguments.first(), key) && is_name(arguments.get(1), value)
                })
        }
    }
}

/// The finding on an empty collection filled by the next loop.
fn loop_finding(
    source: &Source,
    rule: &'static str,
    node: &AstNode,
    declaration: &VariableDeclaration,
) -> Option<Finding> {
    let [declarator] = declaration.declarations.as_slice() else {
        return None;
    };
    let BindingPattern::BindingIdentifier(collection) = &declarator.id else {
        return None;
    };
    let Some(Expression::NewExpression(new)) = &declarator.init else {
        return None;
    };
    let Expression::Identifier(callee) = &new.callee else {
        return None;
    };
    let constructor = callee.name.as_str();
    let supported = matches!(
        constructor,
        "Set" | "WeakSet" | "Map" | "WeakMap" | "URLSearchParams"
    );
    if !supported || !new.arguments.is_empty() {
        return None;
    }
    let Statement::ForOfStatement(statement) = next_statement(source, node.id())? else {
        return None;
    };
    let Statement::ExpressionStatement(only) = single_statement(&statement.body)? else {
        return None;
    };
    let Expression::CallExpression(_) = &only.expression else {
        return None;
    };
    let source_node = if constructor == "URLSearchParams" {
        object_entries_argument(&statement.right)?
    } else {
        &statement.right
    };
    let ForStatementLeft::VariableDeclaration(left) = &statement.left else {
        return None;
    };
    let lexical = matches!(
        left.kind,
        VariableDeclarationKind::Const | VariableDeclarationKind::Let
    );
    let [left_declarator] = left.declarations.as_slice() else {
        return None;
    };
    let name = collection.name.as_str();
    let redeclared = left_declarator
        .id
        .get_binding_identifiers()
        .iter()
        .any(|identifier| identifier.name == name);
    let read_in_source = collection
        .symbol_id
        .get()
        .is_some_and(|symbol| is_referenced_in(source, symbol, statement.right.span()));
    if statement.r#await
        || !lexical
        || redeclared
        || read_in_source
        || is_unsafe_source(constructor, source_node)
    {
        return None;
    }
    let call = method_named(&only.expression, &["add", "set", "append"]).filter(|method| {
        method.is_plain()
            && matches!(method.object, Expression::Identifier(object) if object.name == name)
    })?;
    let arguments = call.arguments()?;
    let matches_loop = fills(constructor, call.name, &arguments, &left_declarator.id);
    if !matches_loop {
        return None;
    }
    let message = format!(
        "Pass `{}` directly to `new {constructor}()`.",
        text(source, source_node.span())
    );
    if has_comment(
        source,
        Span::new(declaration.span.start, statement.span.end),
    ) {
        return Some(source.finding(rule, statement.span.start, message));
    }
    Some(source.fixed(
        rule,
        statement.span.start,
        message,
        constructor_fix(source, new, source_node, statement.span),
    ))
}

/// The fix that hands the source to the constructor and removes the loop.
fn constructor_fix(
    source: &Source,
    new: &oxc_ast::ast::NewExpression,
    source_node: &Expression,
    loop_span: Span,
) -> Fix {
    let head_end = new
        .type_arguments
        .as_ref()
        .map_or(new.callee.span().end, |arguments| arguments.span.end);
    let removal = statement_removal(source, loop_span);
    Fix {
        start: new.span.start,
        end: removal.end,
        text: format!(
            "{}({}){}",
            text(source, Span::new(new.span.start, head_end)).trim_end(),
            parenthesized_text(source, source_node),
            text(source, Span::new(new.span.end, removal.start)),
        ),
    }
}

impl Rule for PreferIterableInConstructor {
    fn name(&self) -> &'static str {
        "unicorn/prefer-iterable-in-constructor"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        match node.kind() {
            AstKind::VariableDeclaration(declaration) => {
                findings.extend(loop_finding(source, self.name(), node, declaration));
            }
            AstKind::NewExpression(_) => {
                findings.extend(entries_finding(source, self.name(), node));
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferIterableInConstructor;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferIterableInConstructor, "playground/a.ts", code))
    }

    /// The code after the fixes.
    fn fix(code: &str) -> String {
        fixed(
            code,
            &lint(&PreferIterableInConstructor, "playground/a.ts", code),
        )
    }

    #[test]
    fn reports_and_fixes_the_documented_loops() {
        assert_eq!(
            check("const set = new Set();\nfor (const item of items) {\n\tset.add(item);\n}"),
            ["Pass `items` directly to `new Set()`."]
        );
        let cases = [
            (
                "const set = new Set();\nfor (const item of items) {\n\tset.add(item);\n}\n",
                "const set = new Set(items);\n",
            ),
            (
                "const map = new Map();\nfor (const [key, value] of entries) {\n\tmap.set(key, value);\n}\n",
                "const map = new Map(entries);\n",
            ),
            (
                "const searchParameters = new URLSearchParams();\nfor (const [key, value] of Object.entries(record)) {\n\tsearchParameters.set(key, value);\n}\n",
                "const searchParameters = new URLSearchParams(record);\n",
            ),
            (
                "const searchParameters = new URLSearchParams(Object.entries(record));",
                "const searchParameters = new URLSearchParams(record);",
            ),
            (
                "const m = new Map<string, number>();\nfor (const [k, v] of e) m.set(k, v);\nuse(m);\n",
                "const m = new Map<string, number>(e);\nuse(m);\n",
            ),
        ];
        for (code, after) in cases {
            assert_eq!(fix(code), after, "{code}");
        }
    }

    #[test]
    fn accepts_generic_pairs_and_loops_that_do_more() {
        for code in [
            "const set = new Set(items);",
            "const searchParameters = new URLSearchParams();\nfor (const [key, value] of entries) {\n\tsearchParameters.set(key, value);\n}",
            "const set = new Set();\nfor (const item of items) { set.add(item); log(item); }",
            "const set = new Set();\nfor (const item of items) { set.add(item.id); }",
            "const set = new Set([1]);\nfor (const item of items) { set.add(item); }",
            "const set = new Set();\nfor (const item of set) { set.add(item); }",
            "const map = new Map();\nfor (const [k, v] of [1, 2]) { map.set(k, v); }",
            "const map = new Map();\nfor (const [k, v] of entries) { map.set(v, k); }",
            "const map = new Map();\nfor (const [k, v] of entries) { other.set(k, v); }",
            "const p = new URLSearchParams(Object.entries([1]));",
            "const p = new URLSearchParams(Object.entries({ [k]: 1 }));",
            "const form = new FormData();\nfor (const [k, v] of entries) { form.append(k, v); }",
            "const set = new Set();\nfor (var item of items) { set.add(item); }",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
