//! `unicorn/no-unnecessary-parameters`: a parameter of a local function that receives the same
//! value at every call from outside it — two calls at least — or always its default, or always
//! `undefined` (`eslint-plugin-unicorn` 78.0.0, `docs/rules/no-unnecessary-parameters.md`,
//! `minimumCallCount: 2`). A value is a primitive literal, `undefined`, `void 0`, or a binding
//! never written and reachable from outside the function, a global of the language included; a
//! recursive call forwarding the parameter unchanged is read through. Shallow destructured
//! properties are checked one by one. A function reading `arguments`, and a parameter a type
//! predicate names, are left. In TypeScript the plugin only suggests the change, so no fix.

use super::calls::{Call, Callable, Target, inside, targets};
use super::primitives::{Primitive, primitive};
use crate::expressions::{symbol, unwrapped};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;
use oxc_ast::AstKind;
use oxc_ast::ast::{
    Argument, BindingPattern, Expression, FormalParameter, ObjectExpression, ObjectPropertyKind,
    PropertyKind, TSTypePredicateName, UnaryOperator, VariableDeclarationKind,
};
use oxc_semantic::{ScopeId, SymbolId};

/// The rule.
pub struct NoUnnecessaryParameters;

/// The fewest calls from outside a function that tell a parameter is unnecessary.
const MINIMUM_CALLS: usize = 2;

/// The language's own globals, which nothing can write: a name the file does not declare and
/// that is one of these is a stable value.
const LANGUAGE_GLOBALS: [&str; 40] = [
    "AggregateError",
    "Array",
    "ArrayBuffer",
    "Atomics",
    "BigInt",
    "BigInt64Array",
    "BigUint64Array",
    "Boolean",
    "DataView",
    "Date",
    "Error",
    "EvalError",
    "Float32Array",
    "Float64Array",
    "Function",
    "Infinity",
    "Int8Array",
    "Int16Array",
    "Int32Array",
    "Intl",
    "JSON",
    "Map",
    "Math",
    "NaN",
    "Number",
    "Object",
    "Promise",
    "Proxy",
    "RangeError",
    "ReferenceError",
    "Reflect",
    "RegExp",
    "Set",
    "String",
    "Symbol",
    "SyntaxError",
    "TypeError",
    "URIError",
    "WeakMap",
    "WeakSet",
];

/// A value an argument gives.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Value {
    /// `undefined`, written or left out.
    Undefined,
    /// A primitive literal.
    Primitive(Primitive),
    /// A variable of the file, never written.
    Binding(SymbolId),
    /// A global of the language.
    Global(String),
}

/// The value an argument gives, when known.
fn value(source: &Source, argument: Option<&Expression>) -> Option<Value> {
    let Some(argument) = argument.map(unwrapped) else {
        return Some(Value::Undefined);
    };
    if let Some(found) = primitive(argument) {
        return Some(Value::Primitive(found));
    }
    match argument {
        Expression::UnaryExpression(unary)
            if unary.operator == UnaryOperator::Void
                && matches!(unwrapped(&unary.argument), Expression::NumericLiteral(zero) if zero.value == 0.0) =>
        {
            Some(Value::Undefined)
        }
        Expression::Identifier(name) => match symbol(source, name) {
            None if name.name == "undefined" => Some(Value::Undefined),
            None => LANGUAGE_GLOBALS
                .contains(&name.name.as_str())
                .then(|| Value::Global(name.name.to_string())),
            Some(found) => stable(source, found).then_some(Value::Binding(found)),
        },
        _ => None,
    }
}

/// Whether a variable is never written and declared once, not by `var` nor `declare`.
fn stable(source: &Source, found: SymbolId) -> bool {
    let scoping = source.semantic.scoping();
    let nodes = source.semantic.nodes();
    if scoping
        .get_resolved_references(found)
        .any(oxc_semantic::Reference::is_write)
        || scoping.symbol_redeclarations(found).len() > 1
    {
        return false;
    }
    let declaration = scoping.symbol_declaration(found);
    match nodes.kind(declaration) {
        AstKind::VariableDeclarator(_) => matches!(nodes.parent_kind(declaration),
            AstKind::VariableDeclaration(each) if each.kind != VariableDeclarationKind::Var && !each.declare),
        AstKind::Function(function) => !function.declare && function.body.is_some(),
        AstKind::FormalParameter(_) | AstKind::CatchParameter(_) | AstKind::Class(_) => true,
        _ => false,
    }
}

/// A parameter checked: its name, variable, place, and for a destructured property its key.
struct Parameter<'s, 'a> {
    name: &'a str,
    at: u32,
    symbol: SymbolId,
    index: usize,
    property: Option<String>,
    default: Option<&'s Expression<'a>>,
    whole: &'s FormalParameter<'a>,
}

/// The parameters a function checks, each destructured property apart.
fn parameters<'s, 'a>(callable: Callable<'a>, predicates: &[&str]) -> Vec<Parameter<'s, 'a>>
where
    'a: 's,
{
    let mut found = Vec::new();
    for (index, whole) in callable.parameters().items.iter().enumerate() {
        match &whole.pattern {
            BindingPattern::BindingIdentifier(name) => {
                if predicates.contains(&name.name.as_str()) {
                    continue;
                }
                if let Some(symbol) = name.symbol_id.get() {
                    found.push(Parameter {
                        name: name.name.as_str(),
                        at: name.span.start,
                        symbol,
                        index,
                        property: None,
                        default: whole.initializer.as_deref(),
                        whole,
                    });
                }
            }
            BindingPattern::ObjectPattern(pattern) => {
                let empty_default = whole.initializer.as_deref().is_none_or(|default| {
                    matches!(default, Expression::ObjectExpression(object) if object.properties.is_empty())
                });
                if !empty_default {
                    continue;
                }
                let names: Vec<Option<String>> = pattern
                    .properties
                    .iter()
                    .map(|property| {
                        (!property.computed)
                            .then(|| property.key.static_name().map(|name| name.to_string()))
                            .flatten()
                    })
                    .collect();
                if names.iter().any(Option::is_none) {
                    continue;
                }
                for (property, key) in pattern.properties.iter().zip(names.into_iter().flatten()) {
                    let (binding, default) = match &property.value {
                        BindingPattern::BindingIdentifier(binding) => (binding, None),
                        BindingPattern::AssignmentPattern(assignment) => match &assignment.left {
                            BindingPattern::BindingIdentifier(binding) => {
                                (binding, Some(&assignment.right))
                            }
                            _ => continue,
                        },
                        _ => continue,
                    };
                    if let Some(symbol) = binding.symbol_id.get() {
                        found.push(Parameter {
                            name: binding.name.as_str(),
                            at: binding.span.start,
                            symbol,
                            index,
                            property: Some(key),
                            default,
                            whole,
                        });
                    }
                }
            }
            _ => {}
        }
    }
    found
}

/// The names a function's written type predicate holds, `value is T` or `asserts value`.
fn predicate_names(callable: Callable<'_>) -> Vec<&str> {
    let returns = match callable {
        Callable::Function(function) => function.return_type.as_deref(),
        Callable::Arrow(arrow) => arrow.return_type.as_deref(),
    };
    match returns.map(|written| &written.type_annotation) {
        Some(oxc_ast::ast::TSType::TSTypePredicate(predicate)) => match &predicate.parameter_name {
            TSTypePredicateName::Identifier(name) => vec![name.name.as_str()],
            TSTypePredicateName::This(_) => Vec::new(),
        },
        _ => Vec::new(),
    }
}

/// The plain properties of an object literal, by name, or `None` when one is not plain.
fn object_properties<'s, 'a>(
    object: &'s ObjectExpression<'a>,
) -> Option<Vec<(String, &'s Expression<'a>)>> {
    object
        .properties
        .iter()
        .map(|each| match each {
            ObjectPropertyKind::ObjectProperty(property)
                if !property.method
                    && property.kind == PropertyKind::Init
                    && !property.computed =>
            {
                property
                    .key
                    .static_name()
                    .map(|name| (name.to_string(), &property.value))
            }
            _ => None,
        })
        .collect()
}

/// What one call gives a parameter: a value, an unknown one, or a forwarding of itself.
enum Given {
    Value(Value),
    Unknown,
    Forwarding,
}

/// What every call gives a parameter, or `None` when a call spreads before it.
fn given(source: &Source, target: &Target, parameter: &Parameter) -> Option<Vec<Given>> {
    let mut found = Vec::new();
    for call in &target.calls {
        if call.arguments[..call.arguments.len().min(parameter.index + 1)]
            .iter()
            .any(Argument::is_spread)
        {
            return None;
        }
        let argument = call
            .arguments
            .get(parameter.index)
            .and_then(Argument::as_expression);
        found.push(match &parameter.property {
            Some(key) => property_given(source, parameter, argument, key)?,
            None => plain_given(source, target, call, parameter, argument)?,
        });
    }
    Some(found)
}

/// What a call gives a destructured property: the object literal's property, or the default
/// object when the argument is missing.
fn property_given(
    source: &Source,
    parameter: &Parameter,
    argument: Option<&Expression>,
    key: &str,
) -> Option<Given> {
    let missing = argument.is_none() || value(source, argument) == Some(Value::Undefined);
    let object = if missing {
        parameter.whole.initializer.as_deref()
    } else {
        argument.map(unwrapped)
    };
    let Some(Expression::ObjectExpression(object)) = object else {
        return None;
    };
    let properties = object_properties(object)?;
    let property = properties.iter().find(|each| each.0 == key);
    Some(match property {
        Some((_, written)) => value(source, Some(written)).map_or(Given::Unknown, Given::Value),
        None => Given::Value(Value::Undefined),
    })
}

/// What a call gives a plain parameter.
fn plain_given(
    source: &Source,
    target: &Target,
    call: &Call,
    parameter: &Parameter,
    argument: Option<&Expression>,
) -> Option<Given> {
    if let Some(Expression::Identifier(name)) = argument.map(unwrapped)
        && inside(call.span, target.callable.span())
        && symbol(source, name) == Some(parameter.symbol)
    {
        let written = source
            .semantic
            .scoping()
            .get_resolved_references(parameter.symbol)
            .any(oxc_semantic::Reference::is_write);
        return (!written).then_some(Given::Forwarding);
    }
    Some(value(source, argument).map_or(Given::Unknown, Given::Value))
}

/// Whether a binding is reachable by its name from outside a function.
fn reachable_outside(source: &Source, found: &Value, outside: Option<ScopeId>) -> bool {
    let Value::Binding(binding) = found else {
        return true;
    };
    let scoping = source.semantic.scoping();
    outside.is_some_and(|scope| {
        scoping.find_binding(scope, scoping.symbol_name(*binding).into()) == Some(*binding)
    })
}

impl NoUnnecessaryParameters {
    /// The finding on a parameter, when every call gives it the same value.
    fn check(&self, source: &Source, target: &Target, parameter: &Parameter) -> Option<Finding> {
        let given = given(source, target, parameter)?;
        let incoming: Vec<&Value> = given
            .iter()
            .filter_map(|each| match each {
                Given::Value(found) => Some(Some(found)),
                Given::Unknown => Some(None),
                Given::Forwarding => None,
            })
            .collect::<Option<Vec<&Value>>>()?;
        let forwards = given.iter().any(|each| matches!(each, Given::Forwarding));
        let default = match parameter.default {
            Some(written) => value(source, Some(written)),
            None => Some(Value::Undefined),
        };
        let outside = target
            .callable
            .scope()
            .and_then(|scope| source.semantic.scoping().scope_parent_id(scope));
        let name = parameter.name;
        if incoming.iter().all(|each| **each == Value::Undefined) {
            let usable = default
                .as_ref()
                .is_some_and(|found| reachable_outside(source, found, outside));
            if forwards && !usable {
                return None;
            }
            let message = if parameter.default.is_some() {
                format!("Parameter `{name}` always receives its default value.")
            } else {
                format!("Parameter `{name}` is always `undefined`.")
            };
            return Some(source.finding(self.name(), parameter.at, message));
        }
        let values: Vec<Option<&Value>> = incoming
            .iter()
            .map(|each| {
                if **each == Value::Undefined {
                    default.as_ref()
                } else {
                    Some(*each)
                }
            })
            .collect();
        let first = values.first().copied().flatten()?;
        if values.iter().any(|each| *each != Some(first))
            || !reachable_outside(source, first, outside)
        {
            return None;
        }
        let message = if parameter.default.is_some() && default.as_ref() == Some(first) {
            format!("Parameter `{name}` always receives its default value.")
        } else {
            format!("Parameter `{name}` receives the same value at every call.")
        };
        Some(source.finding(self.name(), parameter.at, message))
    }
}

impl Rule for NoUnnecessaryParameters {
    fn name(&self) -> &'static str {
        "unicorn/no-unnecessary-parameters"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for target in targets(source) {
            let outside_calls = target
                .calls
                .iter()
                .filter(|call| !inside(call.span, target.callable.span()))
                .count();
            if outside_calls < MINIMUM_CALLS || target.callable.reads_arguments() {
                continue;
            }
            let predicates = predicate_names(target.callable);
            for parameter in parameters(target.callable, &predicates) {
                findings.extend(self.check(source, &target, &parameter));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoUnnecessaryParameters;
    use crate::testing::{lint, messages};

    /// The places and messages of the findings.
    fn found(code: &str) -> Vec<(usize, usize, String)> {
        lint(&NoUnnecessaryParameters, "scripts/a.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column, each.message))
            .collect()
    }

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        found(code)
            .into_iter()
            .map(|(line, column, _)| (line, column))
            .collect()
    }

    #[test]
    fn reports_parameters_given_the_same_value() {
        // The documentation's cases, its global binding written as a language global.
        assert_eq!(
            places(
                "function log(message, level) {\n\tconsole.log(level, message);\n}\nlog('Starting', 'info');\nlog('Finished', 'info');"
            ),
            [(1, 23)]
        );
        assert_eq!(
            places(
                "function format(value, unit, digits) {\n\treturn `${value.toFixed(digits)}${unit}`;\n}\nformat(1, 'px', 0);\nformat(2, 'px', 1);"
            ),
            [(1, 24)]
        );
        assert_eq!(
            places(
                "const limit = 10;\nfunction clamp(value, maximum) {\n\treturn Math.min(value, maximum);\n}\nclamp(1, limit);\nclamp(2, limit);"
            ),
            [(2, 23)]
        );
        assert_eq!(
            found(
                "function print(message, extra = false) {\n\tconsole.log(message, extra);\n}\nprint('Starting');\nprint('Finished', false);\nprint('Done', undefined);"
            ),
            [(
                1,
                25,
                "Parameter `extra` always receives its default value.".to_owned()
            )]
        );
        assert_eq!(
            places(
                "function pair(first, second = first) {\n\treturn [first, second];\n}\npair(1);\npair(2);"
            ),
            [(1, 22)]
        );
        assert_eq!(
            places(
                "function run({path, verbose, dryRun = false}) {\n\tconsole.log(path, verbose, dryRun);\n}\nrun({path: 'first', verbose: true});\nrun({path: 'second', verbose: true, dryRun: false});"
            ),
            [(1, 21), (1, 30)]
        );
        assert_eq!(
            places(
                "function find(selector, root) { return root.querySelector(selector); }\nfind('.first', Math);\nfind('.second', Math);"
            ),
            [(1, 25)]
        );
        assert_eq!(
            messages(&lint(
                &NoUnnecessaryParameters,
                "scripts/a.ts",
                "const f = (a, b) => a;\nf(1);\nf(2, void 0);"
            )),
            ["Parameter `b` is always `undefined`."]
        );
    }

    #[test]
    fn leaves_varying_unknown_forwarded_or_shielded_parameters() {
        for code in [
            "function walk(node, seen = new Set()) {\n\tif (seen.has(node)) {\n\t\treturn seen;\n\t}\n\tseen.add(node);\n\treturn node.next ? walk(node.next, seen) : seen;\n}\nwalk({next: {}});\nwalk({});",
            "function f(a) {}\nf(1);",
            "function f(a) {}\nf(1);\nf(2);",
            "function f(a) {}\nf(g());\nf(g());",
            "function f(a) {}\nf(document);\nf(document);",
            "export function f(a) {}\nf(1);\nf(1);",
            "function f(a) { return arguments; }\nf(1);\nf(1);",
            "function f(a): a is string { return true; }\nf(1);\nf(1);",
            "function f(a) {}\nf(...b);\nf(1);",
            "function g(a) {}\n{ const x = 1; g(x); g(x); }",
            "function f({ a }) {}\nf(b);\nf(b);",
            "function f(a = 1) { return a ? f(a - 1) : 0; }\nf(2);\nf(3);",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
