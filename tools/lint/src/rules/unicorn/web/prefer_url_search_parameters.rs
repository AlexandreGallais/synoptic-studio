//! `unicorn/prefer-url-search-parameters`: `query.split('&').map(part => part.split('='))`,
//! alone or given to `Object.fromEntries()`, `new Map()` or `new URLSearchParams()`, parses a
//! query string by hand; `URLSearchParams` decodes it (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/prefer-url-search-parameters.md`). Reported only: `ESLint` only suggests.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Argument, ArrowFunctionBody, BindingPattern, CallExpression, Expression, FormalParameters,
    FunctionBody, Statement, TSLiteral, TSType,
};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::dom::nodes::static_string;
use crate::rules::unicorn::syntax::{Value, method_call, static_value, symbol_of, unwrap};
use crate::source::Source;

/// The rule.
pub struct PreferUrlSearchParameters;

/// A plain call of a static method by name, with a number of arguments in a range, none spread.
fn plain_call<'b, 'a>(
    expression: &'b Expression<'a>,
    name: &str,
    low: usize,
    high: usize,
) -> Option<&'b CallExpression<'a>> {
    let Expression::CallExpression(call) = unwrap(expression) else {
        return None;
    };
    let method = method_call(call)?;
    (method.is_plain()
        && method.name == name
        && (low..=high).contains(&call.arguments.len())
        && !call.arguments.iter().any(Argument::is_spread))
    .then_some(call)
}

/// The receiver of a call of a static method.
fn receiver<'b, 'a>(call: &'b CallExpression<'a>) -> Option<&'b Expression<'a>> {
    method_call(call).map(|method| method.object)
}

/// Whether a written type is surely no string.
fn is_non_string_type(written: &TSType) -> bool {
    match written {
        TSType::TSLiteralType(literal) => !matches!(
            literal.literal,
            TSLiteral::StringLiteral(_) | TSLiteral::TemplateLiteral(_)
        ),
        TSType::TSBigIntKeyword(_)
        | TSType::TSBooleanKeyword(_)
        | TSType::TSNeverKeyword(_)
        | TSType::TSNumberKeyword(_)
        | TSType::TSSymbolKeyword(_)
        | TSType::TSVoidKeyword(_)
        | TSType::TSArrayType(_)
        | TSType::TSTupleType(_)
        | TSType::TSTypeLiteral(_)
        | TSType::TSFunctionType(_)
        | TSType::TSConstructorType(_) => true,
        _ => false,
    }
}

/// Whether a receiver is surely no string: a literal of another kind, a value that is not one,
/// or a name annotated with another type.
fn is_non_string(source: &Source, expression: &Expression) -> bool {
    let expression = unwrap(expression);
    if matches!(
        expression,
        Expression::ArrayExpression(_)
            | Expression::ObjectExpression(_)
            | Expression::ArrowFunctionExpression(_)
            | Expression::FunctionExpression(_)
            | Expression::ClassExpression(_)
    ) {
        return true;
    }
    if static_value(source, expression).is_some_and(|value| !matches!(value, Value::Text(_))) {
        return true;
    }
    let Expression::Identifier(name) = expression else {
        return false;
    };
    let Some(symbol) = symbol_of(source, name) else {
        return false;
    };
    let nodes = source.semantic.nodes();
    let annotation = match nodes.kind(source.semantic.scoping().symbol_declaration(symbol)) {
        AstKind::VariableDeclarator(declarator) => declarator.type_annotation.as_ref(),
        AstKind::FormalParameter(parameter) => parameter.type_annotation.as_ref(),
        _ => None,
    };
    annotation.is_some_and(|annotation| is_non_string_type(&annotation.type_annotation))
}

/// The expression a callback of one plain parameter returns, with the parameter's name.
fn returned<'b, 'a>(callback: &'b Expression<'a>) -> Option<(&'b Expression<'a>, &'a str)> {
    let (parameters, body): (&FormalParameters, Option<&FunctionBody>) = match callback {
        Expression::ArrowFunctionExpression(arrow) if !arrow.r#async => match &arrow.body {
            ArrowFunctionBody::FunctionBody(body) => (&arrow.params, Some(body)),
            _ => (&arrow.params, None),
        },
        Expression::FunctionExpression(function) if !function.r#async && !function.generator => {
            (&function.params, Some(function.body.as_ref()?))
        }
        _ => return None,
    };
    let ([only], None) = (parameters.items.as_slice(), &parameters.rest) else {
        return None;
    };
    let BindingPattern::BindingIdentifier(name) = &only.pattern else {
        return None;
    };
    if only.initializer.is_some() {
        return None;
    }
    let returned = match (body, callback) {
        (None, Expression::ArrowFunctionExpression(arrow)) => arrow.body.as_expression()?,
        (Some(body), _) => match body.statements.as_slice() {
            [Statement::ReturnStatement(statement)] => statement.argument.as_ref()?,
            _ => return None,
        },
        _ => return None,
    };
    Some((returned, name.name.as_str()))
}

/// Whether an expression is `query.split('&').map(part => part.split('='))`.
fn is_pipeline(source: &Source, expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::CallExpression(call) => is_pipeline_call(source, call),
        _ => false,
    }
}

/// Whether a call is `query.split('&').map(part => part.split('='))`.
fn is_pipeline_call(source: &Source, map: &CallExpression) -> bool {
    let is_map = method_call(map).is_some_and(|method| {
        method.is_plain()
            && method.name == "map"
            && map.arguments.len() == 1
            && !map.arguments[0].is_spread()
    });
    if !is_map || map.type_arguments.is_some() {
        return false;
    }
    let Some(split) = receiver(map).and_then(|object| plain_call(object, "split", 1, 1)) else {
        return false;
    };
    let ampersand = split.arguments[0]
        .as_expression()
        .is_some_and(|separator| static_string(unwrap(separator)).as_deref() == Some("&"));
    if !ampersand || receiver(split).is_none_or(|query| is_non_string(source, query)) {
        return false;
    }
    let Some((value, parameter)) = map.arguments[0].as_expression().and_then(returned) else {
        return false;
    };
    let Some(pair) = plain_call(value, "split", 1, 2) else {
        return false;
    };
    let same = matches!(receiver(pair).map(unwrap), Some(Expression::Identifier(name)) if name.name == parameter);
    let equals = pair.arguments[0]
        .as_expression()
        .is_some_and(|separator| static_string(unwrap(separator)).as_deref() == Some("="));
    let limit = pair.arguments.get(1).is_none_or(|limit| {
        limit
            .as_expression()
            .and_then(|limit| static_value(source, limit))
            .is_some_and(|value| value == Value::Number(2.0))
    });
    same && equals && limit
}

/// Whether a name is free of a value binding of the file at a node.
fn is_free(source: &Source, node: &AstNode, name: &str) -> bool {
    let scoping = source.semantic.scoping();
    scoping
        .find_binding(node.scope_id(), name.into())
        .is_none_or(|symbol| {
            matches!(
                source
                    .semantic
                    .nodes()
                    .kind(scoping.symbol_declaration(symbol)),
                AstKind::TSTypeAliasDeclaration(_) | AstKind::TSInterfaceDeclaration(_)
            )
        })
}

/// Whether `URLSearchParams` is the global, or a value imported from `url`.
fn has_search_params(source: &Source, node: &AstNode) -> bool {
    let scoping = source.semantic.scoping();
    let Some(symbol) = scoping.find_binding(node.scope_id(), "URLSearchParams".into()) else {
        return true;
    };
    let nodes = source.semantic.nodes();
    let declaration = scoping.symbol_declaration(symbol);
    match nodes.kind(declaration) {
        AstKind::TSTypeAliasDeclaration(_) | AstKind::TSInterfaceDeclaration(_) => true,
        AstKind::ImportSpecifier(specifier) => {
            matches!(nodes.parent_kind(declaration), AstKind::ImportDeclaration(import)
                if matches!(import.source.value.as_str(), Some("url" | "node:url")) && !import.import_kind.is_type())
                && !specifier.import_kind.is_type()
                && specifier.imported.name() == "URLSearchParams"
        }
        _ => false,
    }
}

/// Whether a call is `Object.fromEntries(x)` by its shape alone.
fn looks_like_from_entries(call: &CallExpression) -> bool {
    call.arguments.len() == 1
        && match &call.callee {
            Expression::StaticMemberExpression(member) => {
                member.property.name == "fromEntries"
                    && matches!(&member.object, Expression::Identifier(object) if object.name == "Object")
            }
            Expression::ComputedMemberExpression(member) => {
                static_string(&member.expression).as_deref() == Some("fromEntries")
                    && matches!(&member.object, Expression::Identifier(object) if object.name == "Object")
            }
            _ => false,
        }
}

/// Whether a pipeline is the first argument of a call or `new` that wraps it.
fn is_wrapped(source: &Source, node: &AstNode) -> bool {
    let nodes = source.semantic.nodes();
    let mut child = node.id();
    let mut parent = nodes.parent_node(child);
    while matches!(
        parent.kind(),
        AstKind::TSAsExpression(_)
            | AstKind::TSSatisfiesExpression(_)
            | AstKind::TSNonNullExpression(_)
            | AstKind::TSTypeAssertion(_)
            | AstKind::TSInstantiationExpression(_)
    ) {
        child = parent.id();
        parent = nodes.parent_node(child);
    }
    let span = nodes.kind(child).span();
    match parent.kind() {
        AstKind::CallExpression(call) => {
            call.arguments
                .first()
                .is_some_and(|first| first.span() == span)
                && looks_like_from_entries(call)
        }
        AstKind::NewExpression(new) => {
            new.arguments.len() == 1
                && new.arguments[0].span() == span
                && matches!(&new.callee, Expression::Identifier(name)
                    if matches!(name.name.as_str(), "Map" | "URLSearchParams"))
        }
        _ => false,
    }
}

impl Rule for PreferUrlSearchParameters {
    fn name(&self) -> &'static str {
        "unicorn/prefer-url-search-parameters"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let found = match node.kind() {
            AstKind::CallExpression(call) => {
                let from_entries = method_call(call).is_some_and(|method| {
                    method.is_plain()
                        && method.name == "fromEntries"
                        && matches!(&call.callee, Expression::StaticMemberExpression(_))
                        && matches!(method.object, Expression::Identifier(object) if object.name == "Object")
                        && call.arguments.len() == 1
                        && !call.arguments[0].is_spread()
                        && is_free(source, node, "Object")
                });
                let argument = call.arguments.first().and_then(Argument::as_expression);
                let wraps =
                    from_entries && argument.is_some_and(|argument| is_pipeline(source, argument));
                (wraps || (!is_wrapped(source, node) && is_pipeline_call(source, call)))
                    .then_some(call.span.start)
            }
            AstKind::NewExpression(new) => {
                let constructor = match &new.callee {
                    Expression::Identifier(name) if name.name == "Map" => {
                        is_free(source, node, "Map")
                    }
                    Expression::Identifier(name) => name.name == "URLSearchParams",
                    _ => false,
                };
                let argument = (new.arguments.len() == 1)
                    .then(|| new.arguments[0].as_expression())
                    .flatten();
                (constructor && argument.is_some_and(|argument| is_pipeline(source, argument)))
                    .then_some(new.span.start)
            }
            _ => None,
        };
        if let Some(offset) = found
            && has_search_params(source, node)
        {
            findings.push(source.finding(
                self.name(),
                offset,
                "Prefer `URLSearchParams` over manually splitting query strings.".to_owned(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferUrlSearchParameters;
    use crate::testing::lint;

    /// The places of the findings.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&PreferUrlSearchParameters, "scripts/a.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_query_strings_split_by_hand() {
        // The documentation's three cases, then the pipeline alone, a `function` and a limit.
        assert_eq!(
            check(
                "const parameters = Object.fromEntries(query.split('&').map(part => part.split('=')));\nconst p2 = new Map(query.split('&').map(part => part.split('=')));\nconst p3 = new URLSearchParams(query.split('&').map(part => part.split('=')));\nconst p4 = query.split('&').map(function (part) { return part.split('=', 2); });\nconst p5 = (s.split(`&`).map(p => (p as string).split(\"=\")) as string[][]);"
            ),
            [(1, 20), (2, 12), (3, 12), (4, 12), (5, 13)]
        );
    }

    #[test]
    fn leaves_other_splits() {
        for code in [
            "const parameters = Object.fromEntries(new URLSearchParams(query));",
            "const p = s.split('&').map(p => q.split('='));",
            "const p = s.split(',').map(p => p.split('='));",
            "const p = s.split('&').map(p => p.split('=', 3));",
            "const p = s.split('&').map((p, i) => p.split('='));",
            "const p = [1].split('&').map(p => p.split('='));",
            "const p = s.split('&').map(async p => p.split('='));",
            "function f(n: number) { return n.split('&').map(p => p.split('=')); }",
            "const p = s?.split('&').map(p => p.split('='));",
            "const URLSearchParams = 1; const p = s.split('&').map(p => p.split('='));",
        ] {
            assert_eq!(check(code), Vec::new(), "{code}");
        }
    }
}
