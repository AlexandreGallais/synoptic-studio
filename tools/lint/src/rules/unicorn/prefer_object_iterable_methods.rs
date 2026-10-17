//! `unicorn/prefer-object-iterable-methods`: the `Object` method that gives what the code reads —
//! `Object.values()` or `Object.entries()` rather than `Object.keys()` read back by key,
//! `Object.keys()` or `Object.values()` rather than `Object.entries()` half used — in `for…of`
//! loops and callbacks (`eslint-plugin-unicorn` 78.0.0). Autofixed, as in `ESLint`.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    BindingPattern, CallExpression, Expression, ForStatementLeft, FormalParameters,
    VariableDeclarationKind,
};
use oxc_semantic::{AstNode, NodeId, ScopeId, SymbolId};
use oxc_span::{GetSpan, Span};

use super::syntax::{
    Edit, MethodCall, available_name, combined_fix, descendants, has_comment, is_function_or_class,
    method_call, method_named, path_up, same_reference_binding, text, unwrap,
};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferObjectIterableMethods;

/// What a rewrite reads and where.
struct Target<'b, 'a> {
    /// The `Object.keys(x)` or `Object.entries(x)` call.
    call: MethodCall<'b, 'a>,
    /// The binding pattern of the key, the value or the entry.
    binding: &'b BindingPattern<'a>,
    /// The span the binding is written in, its annotation included.
    binding_span: Span,
    /// Whether that binding is an arrow's only parameter, written without parentheses.
    bare_arrow_parameter: bool,
    /// The node the loop's body or the callback's body is.
    body: NodeId,
    /// The scope the binding is declared in.
    scope: ScopeId,
    /// Whether a fix keeps the code's meaning: no early exit that would change.
    can_fix: bool,
}

/// A plain `Object.<method>(x)` call with one argument.
fn object_call<'b, 'a>(expression: &'b Expression<'a>, method: &str) -> Option<MethodCall<'b, 'a>> {
    method_named(expression, &[method]).filter(|call| {
        call.is_plain()
            && call.call.arguments.len() == 1
            && call.call.arguments[0].as_expression().is_some()
            && matches!(call.object, Expression::Identifier(object) if object.name == "Object")
    })
}

/// The nodes of a body, those inside functions and classes left out.
fn body_nodes<'s, 'a>(source: &'s Source<'_, 'a>, body: NodeId) -> Vec<&'s AstNode<'a>> {
    std::iter::once(source.semantic.nodes().get_node(body))
        .chain(descendants(source, body))
        .filter(|node| {
            !path_up(source, node.id(), body)
                .skip(1)
                .any(|step| is_function_or_class(&step.kind()))
        })
        .collect()
}

/// The references of a binding inside a body, `None` when one writes it.
fn references_in(source: &Source, symbol: SymbolId, body: Span) -> Option<Vec<Span>> {
    let scoping = source.semantic.scoping();
    let nodes = source.semantic.nodes();
    let mut found = Vec::new();
    for reference in scoping.get_resolved_reference_ids(symbol) {
        let reference = scoping.get_reference(*reference);
        let at = nodes.kind(reference.node_id()).span();
        if at.start >= body.start && at.end <= body.end {
            if reference.is_write() {
                return None;
            }
            found.push(at);
        }
    }
    Some(found)
}

/// Whether a node of the body writes the object: its name reassigned, a member of it assigned.
fn writes_object(source: &Source, nodes: &[&AstNode], object: &Expression) -> bool {
    nodes.iter().any(|node| match node.kind() {
        AstKind::IdentifierReference(identifier) => {
            identifier.reference_id.get().is_some_and(|reference| {
                source.semantic.scoping().get_reference(reference).is_write()
            }) && matches!(unwrap(object), Expression::Identifier(name) if name.name == identifier.name)
        }
        AstKind::AssignmentExpression(assignment) => match &assignment.left {
            oxc_ast::ast::AssignmentTarget::StaticMemberExpression(member) => {
                same_reference_binding(source, &member.object, object)
            }
            oxc_ast::ast::AssignmentTarget::ComputedMemberExpression(member) => {
                same_reference_binding(source, &member.object, object)
            }
            _ => false,
        },
        AstKind::UpdateExpression(update) => match &update.argument {
            oxc_ast::ast::SimpleAssignmentTarget::StaticMemberExpression(member) => {
                same_reference_binding(source, &member.object, object)
            }
            oxc_ast::ast::SimpleAssignmentTarget::ComputedMemberExpression(member) => {
                same_reference_binding(source, &member.object, object)
            }
            _ => false,
        },
        _ => false,
    })
}

/// Whether a node may change something when it runs.
fn has_effect(node: &AstNode) -> bool {
    match node.kind() {
        AstKind::CallExpression(_)
        | AstKind::NewExpression(_)
        | AstKind::AssignmentExpression(_)
        | AstKind::UpdateExpression(_)
        | AstKind::AwaitExpression(_)
        | AstKind::YieldExpression(_)
        | AstKind::TaggedTemplateExpression(_)
        | AstKind::ImportExpression(_) => true,
        AstKind::UnaryExpression(unary) => unary.operator == oxc_ast::ast::UnaryOperator::Delete,
        _ => false,
    }
}

/// The `object[key]` reads of a body, `None` when one of them is written or called.
fn value_members<'a>(
    source: &Source<'_, 'a>,
    all: &[&AstNode<'a>],
    symbol: SymbolId,
    object: &Expression,
) -> Option<Vec<&'a oxc_ast::ast::ComputedMemberExpression<'a>>> {
    let nodes = source.semantic.nodes();
    let mut members = Vec::new();
    for node in all {
        let AstKind::ComputedMemberExpression(member) = node.kind() else {
            continue;
        };
        let reads_key = matches!(&member.expression, Expression::Identifier(each)
            if super::syntax::symbol_of(source, each) == Some(symbol));
        if member.optional
            || !reads_key
            || !same_reference_binding(source, unwrap(&member.object), unwrap(object))
        {
            continue;
        }
        let parent = nodes.parent_kind(node.id());
        let mutated = match parent {
            AstKind::AssignmentExpression(assignment) => assignment.left.span() == member.span,
            AstKind::CallExpression(call) => call.callee.span() == member.span,
            AstKind::UpdateExpression(_)
            | AstKind::ArrayAssignmentTarget(_)
            | AstKind::AssignmentTargetWithDefault(_)
            | AstKind::ForOfStatement(_)
            | AstKind::ForInStatement(_) => true,
            AstKind::UnaryExpression(unary) => {
                unary.operator == oxc_ast::ast::UnaryOperator::Delete
            }
            _ => false,
        };
        if mutated {
            return None;
        }
        members.push(member);
    }
    Some(members)
}

/// The finding on `Object.keys()` read back by key.
fn keys_finding(source: &Source, rule: &'static str, target: &Target) -> Option<Finding> {
    let BindingPattern::BindingIdentifier(key) = target.binding else {
        return None;
    };
    let symbol = key.symbol_id.get()?;
    let nodes = source.semantic.nodes();
    let body_span = nodes.kind(target.body).span();
    let references = references_in(source, symbol, body_span)?;
    let object = target.call.arguments()?[0];
    let all = body_nodes(source, target.body);
    if writes_object(source, &all, object) {
        return None;
    }
    let members = value_members(source, &all, symbol, object)?;
    if members.is_empty() {
        return None;
    }
    let wrapped: Vec<&str> = members
        .iter()
        .filter(|member| {
            !matches!(member.object, Expression::Identifier(_))
                && unwrap(&member.object).span() != member.object.span()
        })
        .map(|member| text(source, member.object.span()))
        .collect();
    let mut casts = wrapped.clone();
    casts.sort_unstable();
    casts.dedup();
    if casts.len() > 1 {
        return None;
    }
    let key_reads = references
        .iter()
        .filter(|at| {
            !members
                .iter()
                .any(|member| member.expression.span() == **at)
        })
        .count();
    let preferred = if key_reads == 0 { "values" } else { "entries" };
    let message = format!("Prefer `Object.{preferred}()` over `Object.keys()`.");
    let value = available_name(source, target.scope, "value");
    let unsafe_effects = members.iter().any(|member| {
        all.iter().any(|node| {
            let at = node.kind().span();
            (at.end <= member.span.start || at.start >= member.span.end) && has_effect(node)
        })
    });
    let comments = has_comment(source, target.binding_span)
        || members
            .iter()
            .any(|member| has_comment(source, member.span))
        || (!casts.is_empty() && has_comment(source, object.span()));
    if !target.can_fix || unsafe_effects || comments {
        return Some(source.finding(rule, target.call.property.start, message));
    }
    let binding = if preferred == "values" {
        value.clone()
    } else if target.bare_arrow_parameter {
        format!("([{}, {value}])", key.name)
    } else {
        format!("[{}, {value}]", key.name)
    };
    let mut edits = vec![
        Edit {
            span: target.call.property,
            text: preferred.to_owned(),
        },
        Edit {
            span: target.binding_span,
            text: binding,
        },
    ];
    edits.extend(members.iter().map(|member| Edit {
        span: member.span,
        text: value.clone(),
    }));
    if let Some(cast) = casts.first() {
        edits.push(Edit {
            span: object.span(),
            text: (*cast).to_owned(),
        });
    }
    let fix = combined_fix(source, edits)?;
    Some(source.fixed(rule, target.call.property.start, message, fix))
}

/// The finding on `Object.entries()` that reads only the keys or only the values.
fn entries_finding(source: &Source, rule: &'static str, target: &Target) -> Option<Finding> {
    let BindingPattern::ArrayPattern(pattern) = target.binding else {
        return None;
    };
    if pattern.rest.is_some() || pattern.elements.len() > 2 {
        return None;
    }
    let slot = |index: usize| -> Result<Option<(&str, SymbolId)>, ()> {
        match pattern.elements.get(index) {
            None | Some(None) => Ok(None),
            Some(Some(BindingPattern::BindingIdentifier(identifier))) => identifier
                .symbol_id
                .get()
                .map(|symbol| Some((identifier.name.as_str(), symbol)))
                .ok_or(()),
            Some(Some(_)) => Err(()),
        }
    };
    let (Ok(key), Ok(value)) = (slot(0), slot(1)) else {
        return None;
    };
    if key.is_none() && value.is_none() {
        return None;
    }
    let body_span = source.semantic.nodes().kind(target.body).span();
    let count = |binding: Option<(&str, SymbolId)>| match binding {
        Some((_, symbol)) => references_in(source, symbol, body_span).map(|found| found.len()),
        None => Some(0),
    };
    let (key_reads, value_reads) = (count(key)?, count(value)?);
    if key_reads == value_reads || (key_reads > 0 && value_reads > 0) {
        return None;
    }
    let (preferred, kept) = if key_reads == 0 {
        ("values", value?)
    } else {
        ("keys", key?)
    };
    let message = format!("Prefer `Object.{preferred}()` over `Object.entries()`.");
    if !target.can_fix || has_comment(source, target.binding_span) {
        return Some(source.finding(rule, target.call.property.start, message));
    }
    let fix = combined_fix(
        source,
        vec![
            Edit {
                span: target.call.property,
                text: preferred.to_owned(),
            },
            Edit {
                span: target.binding_span,
                text: kept.0.to_owned(),
            },
        ],
    )?;
    Some(source.fixed(rule, target.call.property.start, message, fix))
}

/// The finding on a target, by its method.
fn finding(source: &Source, rule: &'static str, target: &Target) -> Option<Finding> {
    match target.call.name {
        "keys" => keys_finding(source, rule, target),
        _ => entries_finding(source, rule, target),
    }
}

/// Whether a body exits early: `break`, `continue`, `return` or `throw` (or only `throw` for a
/// callback), outside the functions it holds.
fn exits_early(source: &Source, body: NodeId, loop_body: bool) -> bool {
    body_nodes(source, body)
        .iter()
        .any(|node| match node.kind() {
            AstKind::ThrowStatement(_) => true,
            AstKind::BreakStatement(_)
            | AstKind::ContinueStatement(_)
            | AstKind::ReturnStatement(_) => loop_body,
            _ => false,
        })
}

/// The target of a callback method on `Object.keys(x)` or `Object.entries(x)`.
fn callback_target<'b, 'a>(
    source: &Source,
    node: &AstNode,
    call: &'b CallExpression<'a>,
) -> Option<Target<'b, 'a>> {
    let method = method_call(call).filter(super::syntax::MethodCall::is_plain)?;
    let (binding_index, minimum) = match method.name {
        "every" | "findIndex" | "findLastIndex" | "flatMap" | "forEach" | "map" | "some" => (0, 1),
        "reduce" | "reduceRight" => (1, 2),
        _ => return None,
    };
    let keys_map = method.name == "map" && object_call(method.object, "keys").is_some();
    let entries = object_call(method.object, "entries");
    let object_method = if keys_map {
        object_call(method.object, "keys")
    } else {
        entries
    };
    let object_method = object_method?;
    if !(minimum..=2).contains(&call.arguments.len()) {
        return None;
    }
    let callback = call.arguments[0].as_expression()?;
    let (params, body, is_arrow, uses_arguments): (&FormalParameters, NodeId, bool, bool) =
        match callback {
            Expression::ArrowFunctionExpression(arrow) if !arrow.r#async => {
                let body_id = arrow.get_expression().map_or_else(
                    || {
                        arrow
                            .get_function_body()
                            .map_or(node.id(), oxc_ast::ast::FunctionBody::node_id)
                    },
                    Expression::node_id,
                );
                (&arrow.params, body_id, true, false)
            }
            Expression::FunctionExpression(function)
                if !function.r#async && !function.generator =>
            {
                let body = function.body.as_ref()?;
                let arguments = body_nodes(source, body.node_id())
                .iter()
                .any(|each| matches!(each.kind(), AstKind::IdentifierReference(identifier) if identifier.name == "arguments"));
                (&function.params, body.node_id(), false, arguments)
            }
            _ => return None,
        };
    let count = params.items.len() + usize::from(params.rest.is_some());
    let allowed = binding_index + if entries.is_some() { 2 } else { 1 };
    let last_plain = count == binding_index + 1
        || (params.rest.is_none()
            && matches!(
                params.items.last().map(|each| &each.pattern),
                Some(BindingPattern::BindingIdentifier(_))
            ));
    let returns_function = matches!(
        source.semantic.nodes().kind(body),
        AstKind::ArrowFunctionExpression(_) | AstKind::Function(_) | AstKind::Class(_)
    );
    if count <= binding_index
        || count > allowed
        || !last_plain
        || uses_arguments
        || returns_function
    {
        return None;
    }
    let parameter = params.items.get(binding_index)?;
    let annotated = parameter.type_annotation.is_some();
    if object_method.name == "keys" && annotated {
        return None;
    }
    let bare = is_arrow && !source.text[params.span.start as usize..].starts_with('(');
    Some(Target {
        call: object_method,
        binding: &parameter.pattern,
        binding_span: parameter.span,
        bare_arrow_parameter: bare,
        body,
        scope: source.semantic.nodes().get_node(body).scope_id(),
        can_fix: !exits_early(source, body, false),
    })
}

impl Rule for PreferObjectIterableMethods {
    fn name(&self) -> &'static str {
        "unicorn/prefer-object-iterable-methods"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        match node.kind() {
            AstKind::ForOfStatement(statement) if !statement.r#await => {
                let ForStatementLeft::VariableDeclaration(declaration) = &statement.left else {
                    return;
                };
                let [declarator] = declaration.declarations.as_slice() else {
                    return;
                };
                let lexical = matches!(
                    declaration.kind,
                    VariableDeclarationKind::Const | VariableDeclarationKind::Let
                );
                let call = object_call(&statement.right, "keys")
                    .or_else(|| object_call(&statement.right, "entries"));
                let (Some(call), true) = (call, lexical) else {
                    return;
                };
                let body = statement.body.node_id();
                let target = Target {
                    call,
                    binding: &declarator.id,
                    binding_span: declarator.id.span(),
                    bare_arrow_parameter: false,
                    body,
                    scope: source.semantic.nodes().get_node(body).scope_id(),
                    can_fix: !exits_early(source, body, true),
                };
                if matches!(declarator.id, BindingPattern::BindingIdentifier(_))
                    && declarator.type_annotation.is_some()
                {
                    return;
                }
                findings.extend(finding(source, self.name(), &target));
            }
            AstKind::CallExpression(call) => {
                if let Some(target) = callback_target(source, node, call) {
                    findings.extend(finding(source, self.name(), &target));
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferObjectIterableMethods;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferObjectIterableMethods, "playground/a.ts", code))
    }

    /// The code after the fixes.
    fn fix(code: &str) -> String {
        fixed(
            code,
            &lint(&PreferObjectIterableMethods, "playground/a.ts", code),
        )
    }

    #[test]
    fn reports_and_fixes_the_documented_cases() {
        assert_eq!(
            check("for (const key of Object.keys(object)) {\n\tfoo(object[key]);\n}"),
            ["Prefer `Object.values()` over `Object.keys()`."]
        );
        let cases = [
            (
                "for (const key of Object.keys(object)) {\n\tfoo(object[key]);\n}",
                "for (const value of Object.values(object)) {\n\tfoo(value);\n}",
            ),
            (
                "Object.keys(object).map(key => foo(object[key], key));",
                "Object.entries(object).map(([key, value]) => foo(value, key));",
            ),
            (
                "Object.entries(object).reduce((result, [key, value]) => result + value, 0);",
                "Object.values(object).reduce((result, value) => result + value, 0);",
            ),
            (
                "for (const [key] of Object.entries(object)) {\n\tfoo(key);\n}",
                "for (const key of Object.keys(object)) {\n\tfoo(key);\n}",
            ),
            (
                "for (const key of Object.keys(object)) {\n\tfoo((object as Record<string, unknown>)[key]);\n}",
                "for (const value of Object.values(object as Record<string, unknown>)) {\n\tfoo(value);\n}",
            ),
            (
                "const value = 1;\nfor (const key of Object.keys(o)) { use(o[key], value); }",
                "const value = 1;\nfor (const value_ of Object.values(o)) { use(value_, value); }",
            ),
        ];
        for (code, after) in cases {
            assert_eq!(fix(code), after, "{code}");
        }
    }

    #[test]
    fn reports_without_a_fix_where_it_would_change_the_code() {
        for code in [
            "for (const key of Object.keys(o)) { if (o[key]) break; }",
            "for (const key of Object.keys(o)) { log(); use(o[key]); }",
            "Object.keys(o).map(key => { if (!o[key]) throw new Error(); return o[key]; });",
        ] {
            let findings = lint(&PreferObjectIterableMethods, "playground/a.ts", code);
            assert_eq!(findings.len(), 1, "{code}");
            assert_eq!(fixed(code, &findings), code, "{code}");
        }
    }

    #[test]
    fn accepts_the_most_specific_methods() {
        for code in [
            "for (const value of Object.values(object)) {\n\tfoo(value);\n}",
            "Object.entries(object).map(([key, value]) => foo(value, key));",
            "for (const key of Object.keys(object)) { foo(key); }",
            "for (const key of Object.keys(object)) { object[key] = 1; }",
            "for (const key of Object.keys(object)) { object[key](); }",
            "for (const key of Object.keys(object)) { use(other[key]); }",
            "for (const key of Object.keys(object)) { object.x = 1; use(object[key]); }",
            "for (const [k, v] of Object.entries(object)) { use(k, v); }",
            "for (const [k, v] of Object.entries(object)) {}",
            "for (const [k, ...rest] of Object.entries(object)) { use(k); }",
            "Object.keys(object).filter(key => use(object[key]));",
            "Object.entries(object).reduce(([k, v]) => v);",
            "Object.keys(object).map((key: string) => object[key]);",
            "Object.keys(object).map(function (key) { return arguments.length + object[key]; });",
            "Object.keys(object).map(async key => object[key]);",
            "Object.keys(object, x).map(key => object[key]);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
