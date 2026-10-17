//! `unicorn/prefer-group-by`: `Object.groupBy()` or `Map.groupBy()` rather than grouping by hand
//! in a `reduce()` or in the `for…of` loop right after an empty object or map
//! (`eslint-plugin-unicorn` 78.0.0). Autofixed, as in `ESLint`.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    ArrayExpressionElement, AssignmentOperator, AssignmentTarget, BindingPattern, Expression,
    ForStatementLeft, FormalParameters, LogicalOperator, Statement, VariableDeclaration,
    VariableDeclarationKind,
};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use super::syntax::{
    MethodCall, concise_body_text, descendants, has_comment, has_optional_chain, has_side_effect,
    is_known_non_indexed, method_call, method_named, next_statement, parenthesized_text,
    same_reference, statement_removal, text, unwrap,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferGroupBy;

/// The names a grouping reads: the accumulator, the element, and the reducer's index and array.
struct Parts<'a> {
    /// The accumulator.
    accumulator: &'a str,
    /// The element.
    element: &'a str,
    /// The index, for a reducer.
    index: Option<&'a str>,
    /// The array, for a reducer.
    array: Option<&'a str>,
}

/// Whether an expression is the identifier of a name.
fn is_name(expression: &Expression, name: &str) -> bool {
    matches!(unwrap(expression), Expression::Identifier(identifier) if identifier.name == name)
}

/// Whether an expression is `[]`.
fn is_empty_array(expression: &Expression) -> bool {
    matches!(expression, Expression::ArrayExpression(array) if array.elements.is_empty())
}

/// Whether an expression, or what it holds, reads a name.
fn references(source: &Source, expression: &Expression, name: Option<&str>) -> bool {
    let Some(name) = name else {
        return false;
    };
    let id = expression.node_id();
    std::iter::once(source.semantic.nodes().get_node(id))
        .chain(descendants(source, id))
        .any(|node| matches!(node.kind(), AstKind::IdentifierReference(identifier) if identifier.name == name))
}

/// The method an initial value groups into: `{}` or `Object.create(null)` for `Object.groupBy`,
/// `new Map()` for `Map.groupBy`.
fn group_method(expression: &Expression) -> Option<&'static str> {
    match expression {
        Expression::ObjectExpression(object) if object.properties.is_empty() => {
            Some("Object.groupBy")
        }
        Expression::NewExpression(new)
            if new.arguments.is_empty()
                && matches!(&new.callee, Expression::Identifier(callee) if callee.name == "Map") =>
        {
            Some("Map.groupBy")
        }
        _ => {
            let create = method_named(expression, &["create"])?;
            let null = matches!(
                create.arguments().as_deref(),
                Some([Expression::NullLiteral(_)])
            );
            (is_name(create.object, "Object") && create.is_plain() && null)
                .then_some("Object.groupBy")
        }
    }
}

/// The `const name = value;` a statement is.
fn single_const<'b, 'a>(statement: &'b Statement<'a>) -> Option<(&'a str, &'b Expression<'a>)> {
    let Statement::VariableDeclaration(declaration) = statement else {
        return None;
    };
    let [declarator] = declaration.declarations.as_slice() else {
        return None;
    };
    let BindingPattern::BindingIdentifier(identifier) = &declarator.id else {
        return None;
    };
    (declaration.kind == VariableDeclarationKind::Const)
        .then_some(())
        .and(declarator.init.as_ref())
        .map(|init| (identifier.name.as_str(), init))
}

/// The key an expression must read: the bound name, or the same reference as the key.
fn is_expected(key: &Expression, expected: &Expression, bound: Option<&str>) -> bool {
    match bound {
        Some(name) => is_name(key, name),
        None => {
            !has_optional_chain(key)
                && !has_optional_chain(expected)
                && same_reference(key, expected)
        }
    }
}

/// The key of `accumulator[key]`.
fn group_member<'b, 'a>(
    expression: &'b Expression<'a>,
    parts: &Parts,
) -> Option<&'b Expression<'a>> {
    match expression {
        Expression::ComputedMemberExpression(member)
            if !member.optional && is_name(&member.object, parts.accumulator) =>
        {
            Some(&member.expression)
        }
        _ => None,
    }
}

/// The key of an assignment target `accumulator[key]`.
fn target_member<'b, 'a>(
    target: &'b AssignmentTarget<'a>,
    parts: &Parts,
) -> Option<&'b Expression<'a>> {
    match target {
        AssignmentTarget::ComputedMemberExpression(member)
            if !member.optional && is_name(&member.object, parts.accumulator) =>
        {
            Some(&member.expression)
        }
        _ => None,
    }
}

/// The key of `accumulator[key] ??= []`, `||= []`, or `accumulator[key] = accumulator[key] || []`.
fn initializer_key<'b, 'a>(
    expression: &'b Expression<'a>,
    parts: &Parts,
) -> Option<&'b Expression<'a>> {
    let Expression::AssignmentExpression(assignment) = expression else {
        return None;
    };
    match assignment.operator {
        AssignmentOperator::LogicalNullish | AssignmentOperator::LogicalOr
            if is_empty_array(&assignment.right) =>
        {
            target_member(&assignment.left, parts)
        }
        AssignmentOperator::Assign => {
            let Expression::LogicalExpression(fallback) = &assignment.right else {
                return None;
            };
            let key = target_member(&assignment.left, parts)?;
            let fallback_ok = matches!(
                fallback.operator,
                LogicalOperator::Or | LogicalOperator::Coalesce
            ) && is_empty_array(&fallback.right);
            let same =
                group_member(&fallback.left, parts).is_some_and(|other| same_reference(other, key));
            (fallback_ok && same).then_some(key)
        }
        _ => None,
    }
}

/// The key of `(…).push(element)`: through an initializer, or `accumulator[key]` when allowed.
fn push_key<'b, 'a>(
    expression: &'b Expression<'a>,
    parts: &Parts,
    requires_initializer: bool,
) -> Option<&'b Expression<'a>> {
    let push = method_named(expression, &["push"]).filter(MethodCall::is_plain)?;
    let Some(&[pushed]) = push.arguments().as_deref() else {
        return None;
    };
    if !is_name(pushed, parts.element) {
        return None;
    }
    if matches!(push.object, Expression::AssignmentExpression(_)) {
        return initializer_key(push.object, parts);
    }
    if requires_initializer {
        return None;
    }
    group_member(push.object, parts)
}

/// The expression of an expression statement.
fn expression_of<'b, 'a>(statement: &'b Statement<'a>) -> Option<&'b Expression<'a>> {
    match statement {
        Statement::ExpressionStatement(only) => Some(&only.expression),
        _ => None,
    }
}

/// A key bound to a name by `const name = key;`.
type BoundKey<'b, 'a> = Option<(&'a str, &'b Expression<'a>)>;

/// A key bound by a first `const key = …;` that reads neither the accumulator, the index nor
/// the array; `None` when it reads one, the statements unchanged when there is no such line.
fn key_binding<'b, 'a>(
    source: &Source,
    statements: &'b [Statement<'a>],
    parts: &Parts,
) -> Option<(&'b [Statement<'a>], BoundKey<'b, 'a>)> {
    let Some(first) = statements.first().and_then(single_const) else {
        return Some((statements, None));
    };
    let reads = references(source, first.1, Some(parts.accumulator))
        || references(source, first.1, parts.index)
        || references(source, first.1, parts.array);
    (!reads).then(|| (&statements[1..], Some(first)))
}

/// Whether a key reads nothing it may not: no `?.`, no accumulator, index nor array.
fn is_valid_key(source: &Source, key: &Expression, parts: &Parts) -> bool {
    !has_optional_chain(key)
        && !references(source, key, Some(parts.accumulator))
        && !references(source, key, parts.index)
        && !references(source, key, parts.array)
}

/// The key of an object grouping's statements.
fn object_key<'b, 'a>(
    source: &Source,
    statements: &'b [Statement<'a>],
    parts: &Parts,
) -> Option<&'b Expression<'a>> {
    let (statements, binding) = key_binding(source, statements, parts)?;
    let (bound, bound_key) = (binding.map(|each| each.0), binding.map(|each| each.1));
    let key = match statements {
        [only] => {
            let pushed = push_key(expression_of(only)?, parts, true)?;
            let key = bound_key.unwrap_or(pushed);
            is_expected(pushed, key, bound).then_some(key)?
        }
        [first, second] => {
            let initialized = initializer_key(expression_of(first)?, parts)?;
            let pushed = push_key(expression_of(second)?, parts, false)?;
            let key = bound_key.unwrap_or(initialized);
            (is_expected(initialized, key, bound) && is_expected(pushed, key, bound))
                .then_some(key)?
        }
        _ => return None,
    };
    is_valid_key(source, key, parts).then_some(key)
}

/// The key of `accumulator.<method>(key)`.
fn map_call_key<'b, 'a>(
    expression: &'b Expression<'a>,
    parts: &Parts,
    method: &str,
) -> Option<&'b Expression<'a>> {
    let call = method_named(expression, &[method])
        .filter(|each| each.is_plain() && is_name(each.object, parts.accumulator))?;
    match call.arguments().as_deref() {
        Some(&[key]) => Some(key),
        _ => None,
    }
}

/// Whether `accumulator.set(key, value)` sets the key to a value the check accepts.
fn is_map_set(
    expression: &Expression,
    parts: &Parts,
    key: &Expression,
    bound: Option<&str>,
    value: impl Fn(&Expression) -> bool,
) -> bool {
    method_named(expression, &["set"])
        .filter(|each| each.is_plain() && is_name(each.object, parts.accumulator))
        .and_then(|each| each.arguments())
        .is_some_and(|arguments| match arguments.as_slice() {
            [set_key, set_value] => is_expected(set_key, key, bound) && value(set_value),
            _ => false,
        })
}

/// The only expression of a block of one expression statement.
fn block_expression<'b, 'a>(statement: &'b Statement<'a>) -> Option<&'b Expression<'a>> {
    match statement {
        Statement::BlockStatement(block) => match block.body.as_slice() {
            [only] => expression_of(only),
            _ => None,
        },
        _ => None,
    }
}

/// The key of a map grouping's statements: an `if (has) get().push() else set([element])`, or
/// `const group = get() ?? []; group.push(element); set(key, group);`.
fn map_key<'b, 'a>(
    source: &Source,
    statements: &'b [Statement<'a>],
    parts: &Parts,
) -> Option<&'b Expression<'a>> {
    let (statements, binding) =
        key_binding(source, statements, parts).unwrap_or((statements, None));
    let (bound, bound_key) = (binding.map(|each| each.0), binding.map(|each| each.1));
    let key = match statements {
        [Statement::IfStatement(statement)] => {
            let tested = map_call_key(&statement.test, parts, "has")?;
            let key = bound_key.unwrap_or(tested);
            let consequent = block_expression(&statement.consequent)?;
            let alternate = block_expression(statement.alternate.as_ref()?)?;
            let pushes = method_named(consequent, &["push"])
                .filter(MethodCall::is_plain)
                .is_some_and(|push| {
                    matches!(push.arguments().as_deref(), Some(&[pushed]) if is_name(pushed, parts.element))
                        && map_call_key(push.object, parts, "get").is_some_and(|got| is_expected(got, key, bound))
                });
            let sets = is_map_set(alternate, parts, key, bound, |value| {
                matches!(value, Expression::ArrayExpression(array) if matches!(
                    array.elements.as_slice(),
                    [ArrayExpressionElement::Identifier(only)] if only.name == parts.element
                ))
            });
            (is_expected(tested, key, bound) && pushes && sets).then_some(key)?
        }
        [first, second, third] => {
            let (group, init) = single_const(first)?;
            let Expression::LogicalExpression(fallback) = init else {
                return None;
            };
            if !matches!(
                fallback.operator,
                LogicalOperator::Or | LogicalOperator::Coalesce
            ) || !is_empty_array(&fallback.right)
            {
                return None;
            }
            let got = map_call_key(&fallback.left, parts, "get")?;
            let key = bound_key.unwrap_or(got);
            let pushes = method_named(expression_of(second)?, &["push"])
                .filter(|each| each.is_plain() && is_name(each.object, group))
                .is_some_and(|push| matches!(push.arguments().as_deref(), Some(&[pushed]) if is_name(pushed, parts.element)));
            let sets = is_map_set(expression_of(third)?, parts, key, bound, |value| {
                is_name(value, group)
            });
            (is_expected(got, key, bound) && pushes && sets).then_some(key)?
        }
        _ => return None,
    };
    is_valid_key(source, key, parts).then_some(key)
}

/// The names the statements declare.
fn local_names<'a>(statements: &[Statement<'a>]) -> Vec<&'a str> {
    statements
        .iter()
        .filter_map(|statement| match statement {
            Statement::VariableDeclaration(declaration) => Some(declaration),
            _ => None,
        })
        .flat_map(|declaration| declaration.declarations.iter())
        .flat_map(|declarator| declarator.id.get_binding_identifiers())
        .map(|identifier| identifier.name.as_str())
        .collect()
}

/// The key of a grouping, reading no name the statements declare.
fn grouping_key<'b, 'a>(
    source: &Source,
    method: &str,
    statements: &'b [Statement<'a>],
    parts: &Parts,
) -> Option<&'b Expression<'a>> {
    let key = if method == "Object.groupBy" {
        object_key(source, statements, parts)?
    } else {
        map_key(source, statements, parts)?
    };
    let locals = local_names(statements);
    (!locals
        .iter()
        .any(|name| references(source, key, Some(name))))
    .then_some(key)
}

/// Whether a receiver is an array with holes, which `reduce()` skips and `groupBy()` does not.
fn is_sparse(expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::ArrayExpression(array) => array
            .elements
            .iter()
            .any(|element| matches!(element, ArrayExpressionElement::Elision(_))),
        Expression::CallExpression(call)
            if is_name(&call.callee, "Array") && call.arguments.len() == 1 =>
        {
            true
        }
        Expression::NewExpression(new)
            if is_name(&new.callee, "Array") && new.arguments.len() == 1 =>
        {
            true
        }
        Expression::StaticMemberExpression(member) => is_sparse(&member.object),
        Expression::ComputedMemberExpression(member) => is_sparse(&member.object),
        Expression::CallExpression(call) => match &call.callee {
            Expression::StaticMemberExpression(member) => is_sparse(&member.object),
            Expression::ComputedMemberExpression(member) => is_sparse(&member.object),
            _ => false,
        },
        _ => false,
    }
}

/// The plain identifier names of a reducer's parameters: accumulator, element, then the optional
/// index and array, all distinct.
fn reducer_names<'a>(params: &'a FormalParameters<'a>) -> Option<Vec<&'a str>> {
    if params.rest.is_some() || !(2..=4).contains(&params.items.len()) {
        return None;
    }
    let names: Option<Vec<&str>> = params
        .items
        .iter()
        .map(|parameter| match &parameter.pattern {
            BindingPattern::BindingIdentifier(identifier) if parameter.initializer.is_none() => {
                Some(identifier.name.as_str())
            }
            _ => None,
        })
        .collect();
    let names = names?;
    let mut unique = names.clone();
    unique.sort_unstable();
    unique.dedup();
    (unique.len() == names.len()).then_some(names)
}

/// Whether a binding is written anywhere but where it is declared.
fn is_written(source: &Source, symbol: Option<oxc_semantic::SymbolId>) -> bool {
    let Some(symbol) = symbol else {
        return true;
    };
    let scoping = source.semantic.scoping();
    scoping
        .get_resolved_reference_ids(symbol)
        .iter()
        .any(|reference| scoping.get_reference(*reference).is_write())
}

/// The parameter's text as an arrow's, parenthesized when annotated or optional.
fn parameter_text(source: &Source, span: Span, annotated: bool) -> String {
    let written = text(source, span);
    if annotated {
        format!("({written})")
    } else {
        written.to_owned()
    }
}

/// Whether a key reads what belongs to its function: `this`, `arguments` or `new.target`.
fn reads_its_function(source: &Source, key: &Expression) -> bool {
    text(source, key.span()).contains("new.target")
        || std::iter::once(source.semantic.nodes().get_node(key.node_id()))
            .chain(descendants(source, key.node_id()))
            .any(|each| match each.kind() {
                AstKind::ThisExpression(_) => true,
                AstKind::IdentifierReference(identifier) => identifier.name == "arguments",
                _ => false,
            })
}

/// A reducer's parameters and block, whether it is typed by a return type or type parameters,
/// and whether it is a `function`: an arrow or a function with a block, neither async nor a
/// generator.
fn reducer<'b, 'a>(
    callback: &'b Expression<'a>,
) -> Option<(&'b FormalParameters<'a>, &'b [Statement<'a>], bool, bool)> {
    match callback {
        Expression::ArrowFunctionExpression(arrow) if !arrow.r#async => Some((
            &arrow.params,
            arrow.get_function_body()?.statements.as_slice(),
            arrow.return_type.is_some() || arrow.type_parameters.is_some(),
            false,
        )),
        Expression::FunctionExpression(function) if !function.r#async && !function.generator => {
            Some((
                &function.params,
                function.body.as_ref()?.statements.as_slice(),
                function.return_type.is_some() || function.type_parameters.is_some(),
                true,
            ))
        }
        _ => None,
    }
}

/// The finding on a grouping `reduce()`.
fn reduce_finding(source: &Source, rule: &'static str, node: &AstNode) -> Option<Finding> {
    let AstKind::CallExpression(call) = node.kind() else {
        return None;
    };
    let reduce = method_call(call).filter(|method| method.name == "reduce" && method.is_plain())?;
    let Some(&[callback, initial]) = reduce.arguments().as_deref() else {
        return None;
    };
    if has_optional_chain(reduce.object)
        || is_sparse(reduce.object)
        || is_known_non_indexed(source, reduce.object)
    {
        return None;
    }
    let (params, statements, special, function_like) = reducer(callback)?;
    let names = reducer_names(params)?;
    let parts = Parts {
        accumulator: names[0],
        element: names[1],
        index: names.get(2).copied(),
        array: names.get(3).copied(),
    };
    let element_symbol = match &params.items[1].pattern {
        BindingPattern::BindingIdentifier(identifier) => identifier.symbol_id.get(),
        _ => None,
    };
    if is_written(source, element_symbol) {
        return None;
    }
    let nodes = source.semantic.nodes();
    let declarator = nodes
        .ancestors(node.id())
        .find_map(|each| match each.kind() {
            AstKind::VariableDeclarator(declarator) => Some((each.id(), declarator)),
            _ => None,
        });
    let method = group_method(initial)?;
    let conflict = declarator.is_some_and(|(id, _)| match nodes.parent_kind(id) {
        AstKind::VariableDeclaration(declaration) => declaration.declarations.iter().any(|each| {
            each.id
                .get_binding_identifiers()
                .iter()
                .any(|identifier| method.starts_with(&format!("{}.", identifier.name)))
        }),
        _ => false,
    });
    let (last, body) = statements.split_last()?;
    let returns_accumulator = matches!(last, Statement::ReturnStatement(statement)
        if statement.argument.as_ref().is_some_and(|argument| is_name(argument, parts.accumulator)));
    if conflict || !returns_accumulator {
        return None;
    }
    let key = grouping_key(source, method, body, &parts)?;
    let message = format!("Prefer `{method}()` over `Array#reduce()`.");
    let accumulator_annotated = params.items[0].type_annotation.is_some();
    let element = &params.items[1];
    let keyword_element = function_like && matches!(parts.element, "await" | "yield");
    let function_specific = function_like && reads_its_function(source, key);
    let declared_annotated =
        declarator.is_some_and(|(_, declarator)| declarator.type_annotation.is_some());
    let unfixable = call.type_arguments.is_some()
        || matches!(initial, Expression::NewExpression(new) if new.type_arguments.is_some())
        || accumulator_annotated
        || special
        || declared_annotated
        || keyword_element
        || function_specific
        || has_side_effect(key)
        || has_comment(source, call.span);
    if unfixable {
        return Some(source.finding(rule, reduce.property.start, message));
    }
    let annotated = element.type_annotation.is_some() || element.optional;
    let replacement = format!(
        "{method}({}, {} => {})",
        parenthesized_text(source, reduce.object),
        parameter_text(source, element.span, annotated),
        concise_body_text(source, key)
    );
    Some(source.fixed(
        rule,
        reduce.property.start,
        message,
        Fix {
            start: call.span.start,
            end: call.span.end,
            text: replacement,
        },
    ))
}

/// The finding on an empty object or map filled by the `for…of` loop that follows.
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
    let BindingPattern::BindingIdentifier(accumulator) = &declarator.id else {
        return None;
    };
    let init = declarator.init.as_ref()?;
    let method = group_method(init)?;
    if method.starts_with(&format!("{}.", accumulator.name)) {
        return None;
    }
    let Statement::ForOfStatement(statement) = next_statement(source, node.id())? else {
        return None;
    };
    let ForStatementLeft::VariableDeclaration(left) = &statement.left else {
        return None;
    };
    let Some(BindingPattern::BindingIdentifier(element)) =
        left.declarations.first().map(|each| &each.id)
    else {
        return None;
    };
    let left_kind_ok = matches!(
        left.kind,
        VariableDeclarationKind::Const | VariableDeclarationKind::Let
    );
    let element_symbol = element.symbol_id.get();
    let in_iterable = element_symbol.is_some_and(|symbol| {
        super::syntax::is_referenced_in(source, symbol, statement.right.span())
    });
    let Statement::BlockStatement(block) = &statement.body else {
        return None;
    };
    if statement.r#await || !left_kind_ok || is_written(source, element_symbol) || in_iterable {
        return None;
    }
    let statements = block.body.as_slice();
    let locals = local_names(statements);
    let accumulator_name = accumulator.name.as_str();
    let element_name = element.name.as_str();
    if accumulator_name == element_name
        || references(source, &statement.right, Some(accumulator_name))
        || locals
            .iter()
            .any(|name| *name == accumulator_name || *name == element_name)
    {
        return None;
    }
    let parts = Parts {
        accumulator: accumulator_name,
        element: element_name,
        index: None,
        array: None,
    };
    let key = grouping_key(source, method, statements, &parts)?;
    let message = format!("Prefer `{method}()` over a `for-of` loop.");
    let removal = statement_removal(source, statement.span);
    let waits = std::iter::once(source.semantic.nodes().get_node(key.node_id()))
        .chain(descendants(source, key.node_id()))
        .any(|each| {
            matches!(
                each.kind(),
                AstKind::AwaitExpression(_) | AstKind::YieldExpression(_)
            )
        });
    let unfixable = has_comment(source, Span::new(init.span().start, removal.end))
        || declarator.type_annotation.is_some()
        || matches!(init, Expression::NewExpression(new) if new.type_arguments.is_some())
        || waits;
    if unfixable {
        return Some(source.finding(rule, statement.span.start, message));
    }
    let left_text = text(source, element.span);
    let replacement = format!(
        "{method}({}, {left_text} => {}){}",
        parenthesized_text(source, &statement.right),
        concise_body_text(source, key),
        text(source, Span::new(init.span().end, removal.start)),
    );
    Some(source.fixed(
        rule,
        statement.span.start,
        message,
        Fix {
            start: init.span().start,
            end: removal.end,
            text: replacement,
        },
    ))
}

impl Rule for PreferGroupBy {
    fn name(&self) -> &'static str {
        "unicorn/prefer-group-by"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        match node.kind() {
            AstKind::CallExpression(_) => {
                findings.extend(reduce_finding(source, self.name(), node));
            }
            AstKind::VariableDeclaration(declaration) => {
                findings.extend(loop_finding(source, self.name(), node, declaration));
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferGroupBy;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferGroupBy, "playground/a.ts", code))
    }

    /// The code after the fixes.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&PreferGroupBy, "playground/a.ts", code))
    }

    #[test]
    fn reports_and_fixes_the_documented_reducers() {
        let object = "const grouped = items.reduce((groups, item) => {\n\tgroups[item.type] ??= [];\n\tgroups[item.type].push(item);\n\treturn groups;\n}, {});";
        assert_eq!(
            check(object),
            ["Prefer `Object.groupBy()` over `Array#reduce()`."]
        );
        assert_eq!(
            fix(object),
            "const grouped = Object.groupBy(items, item => item.type);"
        );
        let map = "const grouped = items.reduce((groups, item) => {\n\tif (groups.has(item.category)) {\n\t\tgroups.get(item.category).push(item);\n\t} else {\n\t\tgroups.set(item.category, [item]);\n\t}\n\treturn groups;\n}, new Map());";
        assert_eq!(
            check(map),
            ["Prefer `Map.groupBy()` over `Array#reduce()`."]
        );
        assert_eq!(
            fix(map),
            "const grouped = Map.groupBy(items, item => item.category);"
        );
    }

    #[test]
    fn reports_and_fixes_the_documented_loops() {
        let object = "const groups = {};\nfor (const item of items) {\n\tgroups[item.type] ??= [];\n\tgroups[item.type].push(item);\n}\nuse(groups);\n";
        assert_eq!(
            check(object),
            ["Prefer `Object.groupBy()` over a `for-of` loop."]
        );
        assert_eq!(
            fix(object),
            "const groups = Object.groupBy(items, item => item.type);\nuse(groups);\n"
        );
        let map = "const groups = new Map();\nfor (const item of items) {\n\tconst group = groups.get(item.category) ?? [];\n\tgroup.push(item);\n\tgroups.set(item.category, group);\n}\n";
        assert_eq!(check(map), ["Prefer `Map.groupBy()` over a `for-of` loop."]);
        assert_eq!(
            fix(map),
            "const groups = Map.groupBy(items, item => item.category);\n"
        );
    }

    #[test]
    fn reads_the_other_documented_shapes() {
        let cases = [
            (
                "const g = items.reduce((acc, x) => { (acc[x.k] ??= []).push(x); return acc; }, Object.create(null));",
                "const g = Object.groupBy(items, x => x.k);",
            ),
            (
                "const g = items.reduce((acc, x) => { const key = x.k; acc[key] ||= []; acc[key].push(x); return acc; }, {});",
                "const g = Object.groupBy(items, x => x.k);",
            ),
            (
                "const g = items.reduce(function (acc, x) { acc[x.k] = acc[x.k] || []; acc[x.k].push(x); return acc; }, {});",
                "const g = Object.groupBy(items, x => x.k);",
            ),
            (
                "const g = items.reduce((acc, x: Item) => { (acc[x.k] ??= []).push(x); return acc; }, {});",
                "const g = Object.groupBy(items, (x: Item) => x.k);",
            ),
        ];
        for (code, after) in cases {
            assert_eq!(fix(code), after, "{code}");
        }
        // Reported without a fix: a typed accumulator, a key with a side effect.
        for code in [
            "const g = items.reduce((acc: Record<string, Item[]>, x) => { (acc[x.k] ??= []).push(x); return acc; }, {});",
            "const g = items.reduce((acc, x) => { const key = f(x); (acc[key] ??= []).push(x); return acc; }, {});",
            "const g: Record<string, Item[]> = items.reduce((acc, x) => { (acc[x.k] ??= []).push(x); return acc; }, {});",
        ] {
            let findings = lint(&PreferGroupBy, "playground/a.ts", code);
            assert_eq!(findings.len(), 1, "{code}");
            assert_eq!(fixed(code, &findings), code, "{code}");
        }
    }

    #[test]
    fn accepts_groupings_that_do_more_or_differ() {
        for code in [
            "const grouped = Object.groupBy(items, item => item.type);",
            "const g = items.reduce((acc, x) => { acc[x.k] ??= []; acc[x.k].push(x); log(x); return acc; }, {});",
            "const g = items.reduce((acc, x) => { acc[x.k] ??= []; acc[x.j].push(x); return acc; }, {});",
            "const g = items.reduce((acc, x) => { acc[x.k] ??= []; acc[x.k].push(other); return acc; }, {});",
            "const g = items.reduce((acc, x) => { acc[x.k] ??= []; acc[x.k].push(x); return other; }, {});",
            "const g = items.reduce((acc, x) => { acc[x.k] ??= []; acc[x.k].push(x); return acc; }, []);",
            "const g = items.reduce((acc, x) => { acc[x.k] ??= []; acc[x.k].push(x); return acc; });",
            "const g = items.reduce((acc, x, i) => { acc[i] ??= []; acc[i].push(x); return acc; }, {});",
            "const g = items.reduce((acc, x) => { acc[x?.k] ??= []; acc[x?.k].push(x); return acc; }, {});",
            "const g = items?.reduce((acc, x) => { acc[x.k] ??= []; acc[x.k].push(x); return acc; }, {});",
            "const g = [1, , 2].reduce((acc, x) => { acc[x] ??= []; acc[x].push(x); return acc; }, {});",
            "const g = items.reduce(async (acc, x) => { acc[x.k] ??= []; acc[x.k].push(x); return acc; }, {});",
            "const g = items.reduce((acc, acc2) => { return acc; }, {});",
            "const g = items.reduce((acc, x) => { const k = acc.size; acc[k] ??= []; acc[k].push(x); return acc; }, {});",
            "const groups = {};\nfor (const item of items) {\n\tgroups[item.type] ??= [];\n\tgroups[item.type].push(item);\n\tcount += 1;\n}",
            "const groups = {};\nfor (const item of groups.list) {\n\tgroups[item.type] ??= [];\n\tgroups[item.type].push(item);\n}",
            "const groups = { a: [] };\nfor (const item of items) {\n\tgroups[item.type] ??= [];\n\tgroups[item.type].push(item);\n}",
            "const groups = {};\nfor (const item of items) groups[item.type] = [item];",
            "const groups = {};\nfor (const [k, v] of items) {\n\tgroups[k] ??= [];\n\tgroups[k].push(v);\n}",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
