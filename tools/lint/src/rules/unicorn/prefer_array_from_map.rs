//! `unicorn/prefer-array-from-map`: no `Array.from(iterable).map(…)` nor an empty array filled by
//! the next `for…of` loop with one mapped value each turn, where `Array.from(iterable, mapper)`
//! maps directly (`eslint-plugin-unicorn` 78.0.0). The loop is autofixed, as in `ESLint`; the
//! `.map()` call only reported, its fix being a suggestion there.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    AssignmentOperator, AssignmentTarget, BindingPattern, Expression, ForOfStatement,
    ForStatementLeft, Statement, VariableDeclaration, VariableDeclarationKind,
};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use super::syntax::{
    Kind, concise_body_text, has_comment, has_suspension, is_global, is_global_name_free,
    is_referenced_in, kind_of, method_call, method_named, next_statement, parenthesized_text,
    single_statement, text,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferArrayFromMap;

/// Its message.
const MESSAGE: &str = "Pass the mapping function to `Array.from()` directly.";

/// The bindings of a `for…of` loop: its element, and its index for `[index, element]`.
struct LoopBinding<'a> {
    /// The element's name and range.
    element: (&'a str, Span),
    /// The index's name and range, for a pair.
    index: Option<(&'a str, Span)>,
}

/// The bindings a `for…of` declares with `const` or `let`: one name, or a pair of names.
fn loop_binding<'a>(statement: &'a ForOfStatement<'a>) -> Option<LoopBinding<'a>> {
    let ForStatementLeft::VariableDeclaration(declaration) = &statement.left else {
        return None;
    };
    let [declarator] = declaration.declarations.as_slice() else {
        return None;
    };
    match &declarator.id {
        BindingPattern::BindingIdentifier(identifier) => Some(LoopBinding {
            element: (identifier.name.as_str(), identifier.span),
            index: None,
        }),
        BindingPattern::ArrayPattern(pattern) if pattern.rest.is_none() => {
            match pattern.elements.as_slice() {
                [
                    Some(BindingPattern::BindingIdentifier(index)),
                    Some(BindingPattern::BindingIdentifier(element)),
                ] => Some(LoopBinding {
                    element: (element.name.as_str(), element.span),
                    index: Some((index.name.as_str(), index.span)),
                }),
                _ => None,
            }
        }
        _ => None,
    }
}

/// `Array.from(iterable, parameters => body)`.
fn array_from(
    source: &Source,
    iterable: &Expression,
    parameters: &str,
    body: &Expression,
) -> String {
    format!(
        "Array.from({}, {parameters} => {})",
        parenthesized_text(source, iterable),
        concise_body_text(source, body)
    )
}

/// The replacement of a loop that fills an empty array, when it maps one value each turn.
fn loop_replacement(
    source: &Source,
    statement: &ForOfStatement,
    name: &str,
    symbol: oxc_semantic::SymbolId,
) -> Option<String> {
    let binding = loop_binding(statement)?;
    if binding.element.0 == name || binding.index.is_some_and(|index| index.0 == name) {
        return None;
    }
    if is_referenced_in(source, symbol, statement.right.span()) {
        return None;
    }
    let Statement::ExpressionStatement(only) = single_statement(&statement.body)? else {
        return None;
    };
    let entries = method_named(&statement.right, &["entries"])
        .filter(|method| method.is_plain() && method.call.arguments.is_empty());
    let element_text = text(source, binding.element.1);
    let clean = |value: &Expression| {
        !is_referenced_in(source, symbol, value.span()) && !has_suspension(source, value.node_id())
    };
    if let Some(push) = method_named(&only.expression, &["push"]).filter(|method| {
        method.is_plain()
            && matches!(method.object, Expression::Identifier(object) if object.name == name)
    }) {
        let Some(&[pushed]) = push.arguments().as_deref() else {
            return None;
        };
        if !clean(pushed) {
            return None;
        }
        let Some(index) = binding.index else {
            let copy = matches!(pushed, Expression::Identifier(identifier) if identifier.name == binding.element.0);
            return (!copy).then(|| array_from(source, &statement.right, element_text, pushed));
        };
        let entries = entries?;
        let index_text = text(source, index.1);
        return Some(if kind_of(source, entries.object) == Some(Kind::Array) {
            array_from(
                source,
                entries.object,
                &format!("({element_text}, {index_text})"),
                pushed,
            )
        } else {
            array_from(
                source,
                &statement.right,
                &format!("([{index_text}, {element_text}])"),
                pushed,
            )
        });
    }
    let (index, entries) = (binding.index?, entries?);
    let Expression::AssignmentExpression(assignment) = &only.expression else {
        return None;
    };
    let AssignmentTarget::ComputedMemberExpression(target) = &assignment.left else {
        return None;
    };
    let writes_by_index = assignment.operator == AssignmentOperator::Assign
        && kind_of(source, entries.object) == Some(Kind::Array)
        && matches!(&target.object, Expression::Identifier(object) if object.name == name)
        && matches!(&target.expression, Expression::Identifier(key) if key.name == index.0)
        && clean(&assignment.right);
    writes_by_index.then(|| {
        array_from(
            source,
            entries.object,
            &format!("({element_text}, {})", text(source, index.1)),
            &assignment.right,
        )
    })
}

/// The finding on `const array = [];` followed by a loop that fills it.
fn loop_finding(
    source: &Source,
    rule: &'static str,
    node: &AstNode,
    declaration: &VariableDeclaration,
) -> Option<Finding> {
    if !matches!(
        declaration.kind,
        VariableDeclarationKind::Const | VariableDeclarationKind::Let
    ) {
        return None;
    }
    let [declarator] = declaration.declarations.as_slice() else {
        return None;
    };
    let BindingPattern::BindingIdentifier(identifier) = &declarator.id else {
        return None;
    };
    let empty = matches!(&declarator.init, Some(Expression::ArrayExpression(array)) if array.elements.is_empty());
    if !empty || !is_global_name_free(source, node, "Array") {
        return None;
    }
    let Statement::ForOfStatement(statement) = next_statement(source, node.id())? else {
        return None;
    };
    if statement.r#await {
        return None;
    }
    let symbol = identifier.symbol_id.get()?;
    let replacement = loop_replacement(source, statement, identifier.name.as_str(), symbol)?;
    let range = Span::new(declaration.span.start, statement.span.end);
    if has_comment(source, range) {
        return Some(source.finding(rule, statement.span.start, MESSAGE.to_owned()));
    }
    let init = declarator.init.as_ref()?;
    let target = text(source, Span::new(identifier.span.start, init.span().start))
        .trim_end()
        .trim_end_matches('=')
        .trim_end();
    Some(source.fixed(
        rule,
        statement.span.start,
        MESSAGE.to_owned(),
        Fix {
            start: range.start,
            end: range.end,
            text: format!("{} {target} = {replacement};", declaration.kind.as_str()),
        },
    ))
}

/// Whether a call's result is flattened by `.flat()` or `.flat(1)` next.
fn is_flattened(source: &Source, node: &AstNode) -> bool {
    let nodes = source.semantic.nodes();
    let parent = nodes.parent_node(node.id());
    let AstKind::StaticMemberExpression(member) = parent.kind() else {
        return false;
    };
    let AstKind::CallExpression(call) = nodes.parent_kind(parent.id()) else {
        return false;
    };
    let default_depth = match call.arguments.as_slice() {
        [] => true,
        [argument] => {
            matches!(argument.as_expression(), Some(Expression::NumericLiteral(literal)) if literal.raw.is_some_and(|raw| raw == "1"))
        }
        _ => false,
    };
    member.property.name == "flat" && !member.optional && !call.optional && default_depth
}

/// The finding on `Array.from(iterable).map(arrow)`.
fn map_finding(source: &Source, rule: &'static str, node: &AstNode) -> Option<Finding> {
    let AstKind::CallExpression(call) = node.kind() else {
        return None;
    };
    let map = method_call(call).filter(|method| {
        method.name == "map"
            && method.is_plain()
            && (1..=2).contains(&method.call.arguments.len())
            && method.call.type_arguments.is_none()
    })?;
    method_named(map.object, &["from"]).filter(|method| {
        method.is_plain()
            && method.call.arguments.len() == 1
            && method.call.type_arguments.is_none()
            && is_global(source, method.object, "Array")
    })?;
    let Some(Expression::ArrowFunctionExpression(arrow)) = call.arguments[0].as_expression() else {
        return None;
    };
    let supported = arrow.params.items.len() <= 2 && arrow.params.rest.is_none();
    (supported && !is_flattened(source, node))
        .then(|| source.finding(rule, map.property.start, MESSAGE.to_owned()))
}

impl Rule for PreferArrayFromMap {
    fn name(&self) -> &'static str {
        "unicorn/prefer-array-from-map"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        match node.kind() {
            AstKind::VariableDeclaration(declaration) => {
                findings.extend(loop_finding(source, self.name(), node, declaration));
            }
            AstKind::CallExpression(_) => findings.extend(map_finding(source, self.name(), node)),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferArrayFromMap;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferArrayFromMap, "playground/a.ts", code))
    }

    /// The code after the fixes.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&PreferArrayFromMap, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_maps_after_array_from() {
        let message = ["Pass the mapping function to `Array.from()` directly."];

        assert_eq!(
            check("const foo = Array.from(bar).map(element => element.id);"),
            message
        );
        assert_eq!(
            check("const foo = Array.from(bar).map((element, index) => `${index}:${element}`);"),
            message
        );
        // Reported, not fixed: the fix is a suggestion.
        assert_eq!(
            fix("const foo = Array.from(bar).map(element => element.id);"),
            "const foo = Array.from(bar).map(element => element.id);"
        );
    }

    #[test]
    fn reports_and_fixes_the_documented_loops() {
        assert_eq!(
            check("const foo = [];\nfor (const element of bar) {\n\tfoo.push(element.id);\n}"),
            ["Pass the mapping function to `Array.from()` directly."]
        );
        assert_eq!(
            fix("const foo = [];\nfor (const element of bar) {\n\tfoo.push(element.id);\n}"),
            "const foo = Array.from(bar, element => element.id);"
        );
        assert_eq!(
            fix(
                "const foo = [];\nfor (const [index, element] of bar.entries()) {\n\tfoo.push(`${index}:${element}`);\n}"
            ),
            "const foo = Array.from(bar.entries(), ([index, element]) => `${index}:${element}`);"
        );
        assert_eq!(
            fix(
                "const bar = [1];\nconst foo: string[] = [];\nfor (const [i, e] of bar.entries()) foo.push(String(e + i));"
            ),
            "const bar = [1];\nconst foo: string[] = Array.from(bar, (e, i) => String(e + i));"
        );
        assert_eq!(
            fix(
                "const bar = [1];\nconst foo = [];\nfor (const [i, e] of bar.entries()) { foo[i] = { e }; }"
            ),
            "const bar = [1];\nconst foo = Array.from(bar, (e, i) => ({ e }));"
        );
        assert_eq!(
            fix("export const foo = [];\nfor (const e of bar) foo.push(e * 2);"),
            "export const foo = Array.from(bar, e => e * 2);"
        );
    }

    #[test]
    fn reports_without_a_fix_around_a_comment() {
        let code = "const foo = [];\n// fill\nfor (const e of bar) foo.push(e * 2);";
        let findings = lint(&PreferArrayFromMap, "playground/a.ts", code);

        assert_eq!(findings.len(), 1);
        assert_eq!(fixed(code, &findings), code);
    }

    #[test]
    fn accepts_copies_and_loops_that_do_more() {
        for code in [
            "const foo = Array.from(bar, element => element.id);",
            "const foo = Array.from(bar).map((element, index, array) => array.includes(element));",
            "const foo = Array.from(bar).map(f);",
            "const foo = Array.from(bar).map((...all) => all);",
            "const foo = Array.from(bar).map(x => [x]).flat();",
            "const foo = Array.from(bar, f).map(x => x);",
            "const foo = Array.from<number>(bar).map(x => x);",
            "const foo = [];\nfor (const element of bar) {\n\tfoo.push(element);\n}",
            "const foo = [1];\nfor (const e of bar) foo.push(e * 2);",
            "const foo = [];\nfor (const e of bar) { foo.push(e * 2); log(e); }",
            "const foo = [];\nfor (const e of bar) foo.push(foo.length);",
            "const foo = [];\nfor (const e of foo) foo.push(e);",
            "const foo = [];\nfor (const e of bar) other.push(e * 2);",
            "const foo = [];\nfor (const e of bar) foo.push(e, 1);",
            "async function f() { const foo = []; for (const e of bar) foo.push(await e); }",
            "async function f() { const foo = []; for await (const e of bar) foo.push(e * 2); }",
            "const foo = [];\nfor (const [i, e] of bar.entries()) { foo[i] = e; }",
            "const foo = [];\nfor (const { e } of bar) foo.push(e * 2);",
            "const foo = [];\nlog(foo);\nfor (const e of bar) foo.push(e * 2);",
            "const Array = 1; const foo = [];\nfor (const e of bar) foo.push(e * 2);",
            "const foo = [], baz = [];\nfor (const e of bar) foo.push(e * 2);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
