//! `unicorn/no-unnecessary-default-arguments`: trailing arguments of a call to a local function
//! that write the very literal its parameters default to, and properties of an object argument
//! that write the literal its destructuring defaults to (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/no-unnecessary-default-arguments.md`). The function is declared once, by a
//! declaration or a `const`, never reassigned, generic, nor reading `arguments`; an imported one
//! needs types and is left. In TypeScript the plugin only suggests the removal, so no fix.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Argument, BindingPattern, Expression, FormalParameters, ObjectExpression, ObjectPropertyKind,
    PropertyKind, VariableDeclarationKind,
};
use oxc_semantic::SymbolId;
use oxc_span::GetSpan;

use super::calls::Callable;
use super::primitives::{Primitive, primitive};
use crate::expressions::{symbol, unwrapped};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoUnnecessaryDefaultArguments;

/// The names `Object.prototype` holds, which a destructuring default cannot be told from.
const PROTOTYPE_NAMES: [&str; 12] = [
    "constructor",
    "hasOwnProperty",
    "isPrototypeOf",
    "propertyIsEnumerable",
    "toLocaleString",
    "toString",
    "valueOf",
    "__defineGetter__",
    "__defineSetter__",
    "__lookupGetter__",
    "__lookupSetter__",
    "__proto__",
];

/// What a parameter defaults to: its own literal, or its destructured properties' literals.
enum Default {
    Value(Option<Primitive>),
    Properties(Vec<(String, Option<Primitive>)>),
}

/// The static names of an object's plain properties, or `None` when one is not plain or repeats.
fn property_names<'s, 'a>(
    object: &'s ObjectExpression<'a>,
) -> Option<Vec<(String, &'s Expression<'a>, oxc_span::Span)>> {
    let mut found: Vec<(String, &Expression, oxc_span::Span)> = Vec::new();
    for each in &object.properties {
        let ObjectPropertyKind::ObjectProperty(property) = each else {
            return None;
        };
        let name = property.key.static_name()?.to_string();
        if property.computed
            || property.method
            || property.kind != PropertyKind::Init
            || name == "__proto__"
            || found.iter().any(|known| known.0 == name)
        {
            return None;
        }
        found.push((name, &property.value, property.span));
    }
    Some(found)
}

/// The defaults of a function's parameters.
fn defaults(parameters: &FormalParameters) -> Vec<Default> {
    parameters
        .items
        .iter()
        .map(|parameter| match &parameter.pattern {
            BindingPattern::BindingIdentifier(_) => {
                Default::Value(parameter.initializer.as_deref().and_then(primitive))
            }
            BindingPattern::ObjectPattern(pattern) => {
                let mut found: Vec<(String, Option<Primitive>)> = Vec::new();
                for property in &pattern.properties {
                    let Some(name) = property.key.static_name().map(|name| name.to_string()) else {
                        return Default::Properties(Vec::new());
                    };
                    if property.computed
                        || name == "__proto__"
                        || found.iter().any(|known| known.0 == name)
                    {
                        return Default::Properties(Vec::new());
                    }
                    let value = match &property.value {
                        BindingPattern::AssignmentPattern(assignment)
                            if matches!(assignment.left, BindingPattern::BindingIdentifier(_))
                                && !PROTOTYPE_NAMES.contains(&name.as_str()) =>
                        {
                            primitive(&assignment.right)
                        }
                        _ => None,
                    };
                    found.push((name, value));
                }
                Default::Properties(found)
            }
            _ => Default::Value(None),
        })
        .collect()
}

/// The function a callee names: declared once, never written, not generic nor reading
/// `arguments`.
fn callee_function<'a>(source: &Source<'_, 'a>, symbol: SymbolId) -> Option<Callable<'a>> {
    let scoping = source.semantic.scoping();
    let nodes = source.semantic.nodes();
    if scoping.symbol_redeclarations(symbol).len() > 1
        || scoping
            .get_resolved_references(symbol)
            .any(oxc_semantic::Reference::is_write)
    {
        return None;
    }
    let declaration = scoping.symbol_declaration(symbol);
    let callable = match nodes.kind(declaration) {
        AstKind::Function(function) if function.body.is_some() => Callable::Function(function),
        AstKind::VariableDeclarator(declarator) => {
            let is_const = matches!(nodes.parent_kind(declaration),
                AstKind::VariableDeclaration(each) if each.kind == VariableDeclarationKind::Const);
            match declarator.init.as_ref().map(unwrapped) {
                Some(Expression::FunctionExpression(function))
                    if is_const && function.body.is_some() =>
                {
                    Callable::Function(function)
                }
                Some(Expression::ArrowFunctionExpression(arrow)) if is_const => {
                    Callable::Arrow(arrow)
                }
                _ => return None,
            }
        }
        _ => return None,
    };
    let generic = match callable {
        Callable::Function(function) => function
            .type_parameters
            .as_ref()
            .is_some_and(|each| !each.params.is_empty()),
        Callable::Arrow(arrow) => arrow
            .type_parameters
            .as_ref()
            .is_some_and(|each| !each.params.is_empty()),
    };
    (!generic && !callable.reads_arguments()).then_some(callable)
}

impl NoUnnecessaryDefaultArguments {
    /// The findings on a call's arguments.
    fn check(
        &self,
        source: &Source,
        arguments: &[Argument],
        parameters: &[Default],
        findings: &mut Vec<Finding>,
    ) {
        let spread = arguments.iter().position(Argument::is_spread);
        if spread.is_none() {
            let mut first = arguments.len();
            while first > 0 {
                let matches = match (
                    parameters.get(first - 1),
                    arguments[first - 1].as_expression(),
                ) {
                    (Some(Default::Value(Some(value))), Some(argument)) => {
                        primitive(argument).as_ref() == Some(value)
                    }
                    _ => false,
                };
                if !matches {
                    break;
                }
                first -= 1;
            }
            if first < arguments.len() {
                findings.push(source.finding(
                    self.name(),
                    arguments[first].span().start,
                    "Omit trailing arguments that match their parameter defaults.".to_owned(),
                ));
            }
        }
        for (index, argument) in arguments[..spread.unwrap_or(arguments.len())]
            .iter()
            .enumerate()
        {
            let (Some(Default::Properties(defaults)), Some(Expression::ObjectExpression(object))) = (
                parameters.get(index),
                argument.as_expression().map(unwrapped),
            ) else {
                continue;
            };
            for (name, value, span) in property_names(object).unwrap_or_default() {
                let default = defaults
                    .iter()
                    .find(|each| each.0 == name)
                    .and_then(|each| each.1.as_ref());
                if default.is_some() && primitive(value).as_ref() == default {
                    findings.push(source.finding(
                        self.name(),
                        span.start,
                        format!("Property `{name}` matches its destructuring default."),
                    ));
                }
            }
        }
    }
}

impl Rule for NoUnnecessaryDefaultArguments {
    fn name(&self) -> &'static str {
        "unicorn/no-unnecessary-default-arguments"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let nodes = source.semantic.nodes();
        let eval = nodes.iter().any(|each| {
            matches!(each.kind(), AstKind::CallExpression(other) if other.callee.is_specific_id("eval"))
        });
        if eval {
            return;
        }
        for node in nodes.iter() {
            if let AstKind::CallExpression(call) = node.kind() {
                self.check_call(source, call, findings);
            }
        }
    }
}

impl NoUnnecessaryDefaultArguments {
    /// The findings on one call.
    fn check_call(
        &self,
        source: &Source,
        call: &oxc_ast::ast::CallExpression,
        findings: &mut Vec<Finding>,
    ) {
        let Expression::Identifier(callee) = unwrapped(&call.callee) else {
            return;
        };
        let last_literal = call
            .arguments
            .last()
            .and_then(Argument::as_expression)
            .and_then(primitive)
            .is_some();
        let object = call.arguments.iter().any(|each| {
            matches!(
                each.as_expression().map(unwrapped),
                Some(Expression::ObjectExpression(_))
            )
        });
        if !last_literal && !object {
            return;
        }
        let Some(function) =
            symbol(source, callee).and_then(|symbol| callee_function(source, symbol))
        else {
            return;
        };
        self.check(
            source,
            &call.arguments,
            &defaults(function.parameters()),
            findings,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::NoUnnecessaryDefaultArguments;
    use crate::testing::{lint, messages};

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&NoUnnecessaryDefaultArguments, "scripts/a.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_arguments_and_properties_matching_their_defaults() {
        // The documentation's three cases.
        assert_eq!(
            places("function run(always = true) {}\nrun(true);"),
            [(2, 5)]
        );
        assert_eq!(
            places(
                "const format = (value, {unit = 'px', digits = 0} = {}) => {};\nformat(width, {unit: 'px', digits: 2});"
            ),
            [(2, 16)]
        );
        assert_eq!(
            places("function run({enabled = true} = {enabled: false}) {}\nrun({enabled: true});"),
            [(2, 6)]
        );
        assert_eq!(
            messages(&lint(
                &NoUnnecessaryDefaultArguments,
                "scripts/a.ts",
                "function run(always = true) {}\nrun(true);\nconst f = ({ a = 1 }) => a;\nf({ a: 1 });"
            )),
            [
                "Omit trailing arguments that match their parameter defaults.",
                "Property `a` matches its destructuring default."
            ]
        );
        // Several trailing ones, signed numbers and templates.
        assert_eq!(
            places("function f(a = 1, b = -1, c = `x`) {}\nf(2, -1, 'x');\nf(1, -1, `x`);"),
            [(2, 6), (3, 3)]
        );
    }

    #[test]
    fn leaves_middle_values_other_values_and_unknown_functions() {
        for code in [
            "function run(always = true) {}\nrun();\nrun(false);",
            "function f(a = 1, b = 2) {}\nf(1, 3);",
            "function f(a = 0) {}\nf(-0);",
            "function f(a = 1) {}\nf(...b, 1);",
            "function f(a = 1) { return arguments; }\nf(1);",
            "function f<T>(a = 1) {}\nf(1);",
            "import { g } from './g';\ng(1);",
            "const h = (a = 1) => a;\nobject.h(1);",
            "let k = (a = 1) => a;\nk(1);",
            "function m(a = 1) {}\nm(1);\nm = n;",
            "function p(a = undefined) {}\np(undefined);",
            "const q = ({ toString = 1 }) => 1;\nq({ toString: 1 });",
            "function r(a = 1) {}\nr(1);\neval('');",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
