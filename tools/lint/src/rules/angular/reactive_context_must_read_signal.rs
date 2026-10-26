//! `@angular-eslint/reactive-context-must-read-signal`: the function `computed()`,
//! `linkedSignal()`, `effect()` or `afterRenderEffect()` tracks reads at least one signal, or the
//! context never runs again (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests, its
//! `checkResources` option left off). It reports only when nothing in the function might read a
//! signal; where the plugin asks TypeScript whether a call reaches the standard library, the tree
//! shows it here: a call of a global of the standard library or a member of a literal's value
//! (`'x'.toUpperCase()`, an array property's `filter`), its arguments none that could be a
//! function. Any other call, member or destructuring the tree cannot tell keeps it silent.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Argument, ArrayExpressionElement, CallExpression, ClassElement, Expression, ObjectPropertyKind,
};
use oxc_semantic::{AstNode, NodeId};
use oxc_span::GetSpan;

use super::signals::{class_members, is_getter, is_signal, key_is};
use crate::expressions::{const_value, is_global, symbol};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{descendants, unwrap};
use crate::source::Source;

/// The rule.
pub struct ReactiveContextMustReadSignal;

/// The globals of the standard library whose calls and members read no signal.
const STANDARD_GLOBALS: [&str; 22] = [
    "Array",
    "BigInt",
    "Boolean",
    "Date",
    "Error",
    "Intl",
    "JSON",
    "Map",
    "Math",
    "Number",
    "Object",
    "RegExp",
    "Set",
    "String",
    "Symbol",
    "URL",
    "console",
    "isFinite",
    "isNaN",
    "parseFloat",
    "parseInt",
    "structuredClone",
];

/// The options of each reactive context whose function it tracks, beside its first argument
/// (`DEFAULT_PRIMITIVES`).
fn tracked_options(primitive: &str) -> Option<&'static [&'static str]> {
    match primitive {
        "afterRenderEffect" => Some(&["earlyRead", "write", "mixedReadWrite", "read"]),
        "computed" | "effect" => Some(&[]),
        "linkedSignal" => Some(&["source", "computation"]),
        _ => None,
    }
}

/// What a reactive context's tracked functions show.
#[derive(Default)]
struct Analysis {
    /// It has a function to read.
    tracked: bool,
    /// A signal is read in one.
    read: bool,
    /// Something the tree cannot tell might read one.
    unknown: bool,
}

/// Whether an expression is a literal or an array of them, whose members are the standard
/// library's.
fn is_literal_value(expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::StringLiteral(_)
        | Expression::NumericLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_)
        | Expression::BigIntLiteral(_) => true,
        Expression::TemplateLiteral(template) => template.expressions.is_empty(),
        Expression::ArrayExpression(array) => array.elements.iter().all(|element| {
            element.as_expression().is_some_and(is_literal_value)
                && !matches!(element, ArrayExpressionElement::SpreadElement(_))
        }),
        _ => false,
    }
}

/// Whether the tree shows a value to be of the standard library: a literal, a constant or a
/// property of `this` set to one, a global of the standard library, or one of its classes made.
fn is_standard_value(source: &Source, id: NodeId, expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::Identifier(name) => match symbol(source, name) {
            Some(symbol) => const_value(source, symbol).is_some_and(is_literal_value),
            None => STANDARD_GLOBALS.contains(&name.name.as_str()),
        },
        Expression::StaticMemberExpression(member)
            if matches!(member.object, Expression::ThisExpression(_)) =>
        {
            class_members(source, id).iter().any(|element| {
                matches!(element, ClassElement::PropertyDefinition(property)
                    if property.key.static_name().is_some_and(|key| key == member.property.name)
                        && property.value.as_ref().is_some_and(is_literal_value))
            })
        }
        Expression::NewExpression(new) => is_standard_callee(source, &new.callee),
        other => is_literal_value(other),
    }
}

/// Whether a callee is a global of the standard library: `Math.floor`, `parseInt`, `Date`.
fn is_standard_callee(source: &Source, callee: &Expression) -> bool {
    matches!(unwrap(callee), Expression::Identifier(name)
        if is_global(source, name) && STANDARD_GLOBALS.contains(&name.name.as_str()))
}

/// Whether an argument could be a function the tree cannot look into (`isOpaqueFunctionValue`):
/// not an inline function, a value of the standard library, nor an operator's result.
fn is_opaque(source: &Source, id: NodeId, argument: &Argument) -> bool {
    let Some(expression) = argument.as_expression() else {
        return true;
    };
    !matches!(
        unwrap(expression),
        Expression::ArrowFunctionExpression(_)
            | Expression::FunctionExpression(_)
            | Expression::BinaryExpression(_)
            | Expression::UnaryExpression(_)
            | Expression::UpdateExpression(_)
    ) && !is_standard_value(source, id, expression)
}

/// Whether a call reaches only the standard library (`analyseCallLike`): its callee a global of
/// it or a member of one of its values, no argument opaque.
fn is_standard_call(source: &Source, id: NodeId, call: &CallExpression) -> bool {
    let callee = match unwrap(&call.callee) {
        Expression::StaticMemberExpression(member) => is_standard_value(source, id, &member.object),
        other => is_standard_callee(source, other),
    };
    callee
        && !call
            .arguments
            .iter()
            .any(|argument| is_opaque(source, id, argument))
}

/// What one node of a tracked function tells.
fn read_node(source: &Source, node: &AstNode, analysis: &mut Analysis) {
    let id = node.id();
    match node.kind() {
        AstKind::CallExpression(call) => {
            if is_signal(source, id, &call.callee) {
                analysis.read = true;
            } else if !is_standard_call(source, id, call) {
                analysis.unknown = true;
            }
        }
        AstKind::NewExpression(new) => {
            let known = is_standard_callee(source, &new.callee)
                && !new
                    .arguments
                    .iter()
                    .any(|argument| is_opaque(source, id, argument));
            analysis.unknown |= !known;
        }
        AstKind::TaggedTemplateExpression(_) => analysis.unknown = true,
        AstKind::StaticMemberExpression(member) => {
            // A getter runs code: a member of `this` is known when the class declares no
            // getter of its name; any other member only on a value of the standard library.
            let known = match &member.object {
                Expression::ThisExpression(_) => !is_getter(source, id, &member.property.name),
                object => is_standard_value(source, id, object),
            };
            analysis.unknown |= !known;
        }
        AstKind::ObjectPattern(pattern) => {
            // Destructuring `this` reads its members as a member expression does.
            let from_this = matches!(source.semantic.nodes().parent_kind(id),
                AstKind::VariableDeclarator(declarator)
                    if declarator.init.as_ref().is_some_and(|init| matches!(init, Expression::ThisExpression(_))));
            let known = from_this
                && pattern.rest.is_none()
                && pattern.properties.iter().all(|property| {
                    !property.computed
                        && property.key.static_name().is_some_and(|name| {
                            name.as_str()
                                .is_some_and(|name| !is_getter(source, id, name))
                        })
                });
            analysis.unknown |= !known;
        }
        _ => {}
    }
}

/// Registers a tracked function, or marks what cannot be read (`registerTrackedFunction`).
fn track<'e, 'a>(
    expression: &'e Expression<'a>,
    tracked: &mut Vec<&'e Expression<'a>>,
    analysis: &mut Analysis,
) {
    match unwrap(expression) {
        Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_) => {
            analysis.tracked = true;
            tracked.push(expression);
        }
        _ => analysis.unknown = true,
    }
}

impl Rule for ReactiveContextMustReadSignal {
    fn name(&self) -> &'static str {
        "@angular-eslint/reactive-context-must-read-signal"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Expression::Identifier(callee) = &call.callee else {
            return;
        };
        let Some(options) = tracked_options(&callee.name) else {
            return;
        };
        let mut analysis = Analysis::default();
        let mut tracked: Vec<&Expression> = Vec::new();
        match call.arguments.first() {
            None => {}
            Some(Argument::ObjectExpression(object)) if !options.is_empty() => {
                for property in &object.properties {
                    match property {
                        ObjectPropertyKind::SpreadProperty(_) => analysis.unknown = true,
                        ObjectPropertyKind::ObjectProperty(property) => {
                            if !property.computed
                                && options.iter().any(|option| key_is(&property.key, option))
                            {
                                track(&property.value, &mut tracked, &mut analysis);
                            }
                        }
                    }
                }
            }
            Some(first) => match first.as_expression() {
                Some(expression) => track(expression, &mut tracked, &mut analysis),
                None => analysis.unknown = true,
            },
        }
        let starts: Vec<u32> = tracked.iter().map(|each| each.span().start).collect();
        for function in descendants(source, node.id()).filter(|each| {
            matches!(
                each.kind(),
                AstKind::ArrowFunctionExpression(_) | AstKind::Function(_)
            ) && starts.contains(&each.kind().span().start)
        }) {
            for inner in descendants(source, function.id()) {
                read_node(source, inner, &mut analysis);
            }
        }
        if analysis.tracked && !analysis.read && !analysis.unknown {
            findings.push(source.finding(
                self.name(),
                call.span.start,
                format!(
                    "{}() must read at least one signal, otherwise it will never re-run when state changes.",
                    callee.name
                ),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ReactiveContextMustReadSignal;
    use crate::testing::lint;

    /// The plugin's first line, on which each case's code goes on: what it names, imported.
    const IMPORT: &str = "import { afterRenderEffect, computed, effect, InputSignal, linkedSignal, ModelSignal, resource, signal, Signal, untracked } from \"@angular/core\";";

    /// The place and message of each finding on a piece of code written after the import.
    fn check(code: &str) -> Vec<(usize, usize, String)> {
        lint(
            &ReactiveContextMustReadSignal,
            "playground/src/app/a.ts",
            &format!("{IMPORT}\n{code}"),
        )
        .into_iter()
        .map(|each| (each.line - 1, each.column, each.message))
        .collect()
    }

    /// The message on a context of a name.
    fn message(primitive: &str) -> String {
        format!(
            "{primitive}() must read at least one signal, otherwise it will never re-run when state changes."
        )
    }

    #[test]
    fn f51_ac1_accepts_a_context_that_reads_a_signal_or_might() {
        // The plugin's valid cases.
        for code in [
            "const a = signal(0);\nconst c = computed(() => a() * 2);",
            "let a: Signal<number>;\nconst c = computed(() => a() + 1);",
            "interface WritableSignal<T> {\n  (): T;\n  set(value: T): void;\n}\ndeclare const a: WritableSignal<number>;\nconst c = computed(() => a() + 1);",
            "let a: InputSignal<number>;\nconst c = computed(() => a());",
            "let a: ModelSignal<number>;\nconst c = computed(() => a() + 1);",
            "let a: Signal<number[]>;\nconst c = computed(() => a().map((x) => x * 2));",
            "class Test {\n  count = signal(0);\n  c = computed(() => this.count() + 1);\n}",
            "function helper(): number {\n  return 1;\n}\nconst c = computed(() => helper());",
            "class Service {\n  getValue(): number {\n    return 1;\n  }\n}\nclass Test {\n  service = new Service();\n  c = computed(() => this.service.getValue());\n}",
            "const c = computed(() => untracked(() => 1));",
            "class Test {\n  items = [1, 2, 3];\n  transform(value: number): number {\n    return value;\n  }\n  c = computed(() => this.items.map(this.transform));\n}",
            "declare const transform: any;\nclass Test {\n  items = [1, 2, 3];\n  c = computed(() => this.items.map(transform));\n}",
            "class Test {\n  items = [1, 2, 3];\n  transform: ((value: number) => number) | ((value: number) => string) =\n    (value) => value;\n  c = computed(() => this.items.map(this.transform));\n}",
            "class Test {\n  values = [1, 2, 3];\n  c = computed(() => Math.max(...this.values));\n}",
            "class Box {\n  value = 1;\n}\nconst c = computed(() => new Box().value);",
            "declare function html(\n  strings: TemplateStringsArray,\n  ...values: unknown[]\n): string;\nconst outside = html`ignored`;\nconst c = computed(() => html`static`);",
            "class Test {\n  count = signal(0);\n  get double(): number {\n    return this.count() * 2;\n  }\n  c = computed(() => this.double);\n}",
            "class Test {\n  count = signal(0);\n  get double(): number {\n    return this.count() * 2;\n  }\n  c = computed(() => {\n    const { double } = this;\n    return double;\n  });\n}",
            "class Test {\n  count = signal(0);\n  get double(): number {\n    return this.count() * 2;\n  }\n  c = computed(() => {\n    const { count, ...rest } = this;\n    return rest;\n  });\n}",
            "let a: Signal<number>;\nconst c = computed(() => {\n  const inner = computed(() => a() * 2);\n  return inner() + 1;\n});",
            "function compute(): number {\n  return 1;\n}\nconst c = computed(compute);",
            "let a: Signal<number>;\nconst c = linkedSignal(() => a() + 1);",
            "let a: Signal<number>;\nconst c = linkedSignal({ source: () => a(), computation: (s) => s * 2 });",
            "let a: Signal<number>;\nconst c = linkedSignal({ source: () => 1, computation: () => a() });",
            "let a: Signal<string[]>;\nconst c = linkedSignal({ source: a, computation: (options) => options[0] });",
            "declare const options: { source: () => number };\nconst c = linkedSignal({ ...options, computation: (s) => s + 1 });",
            "let a: Signal<number>;\neffect(() => {\n  console.log(a());\n});",
            "let a: Signal<number>;\nafterRenderEffect(() => a() + 1);",
            "let a: Signal<number>;\nafterRenderEffect({\n  earlyRead: () => 1,\n  write: () => a(),\n});",
            "const r = resource({ params: () => 1, loader: () => null });",
            "declare const Test: { computed: (fn: () => unknown) => unknown };\nTest.computed(() => 1);",
            "const c = computed();",
            "const x = computed;",
            "const callback = ({\n  valueOf,\n  toString,\n  constructor,\n  hasOwnProperty,\n}: {\n  valueOf: () => unknown;\n  toString: () => unknown;\n  constructor: () => unknown;\n  hasOwnProperty: () => unknown;\n}) => {\n  valueOf();\n  toString();\n  constructor();\n  hasOwnProperty();\n};",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac1_reports_a_context_that_reads_no_signal_at_its_call() {
        // The plugin's invalid cases but `resource()`, whose option is left off: `const c = `
        // puts the call at column 11, `  c = ` in a class at 7.
        for (code, line, column, primitive) in [
            (
                "const value = 42;\nconst c = computed(() => value * 2);",
                2,
                11,
                "computed",
            ),
            ("const c = computed(() => 42);", 1, 11, "computed"),
            (
                "class Test {\n  name = 'x';\n  c = computed(() => this.name);\n}",
                3,
                7,
                "computed",
            ),
            (
                "class Test {\n  name = 'x';\n  c = computed(() => {\n    const { name } = this;\n    return name;\n  });\n}",
                3,
                7,
                "computed",
            ),
            (
                "class Test {\n  name = 'x';\n  c = computed(() => this.name.toUpperCase());\n}",
                3,
                7,
                "computed",
            ),
            (
                "class Test {\n  items = [1, 2, 3];\n  c = computed(() => this.items.filter((item) => item > 1));\n}",
                3,
                7,
                "computed",
            ),
            (
                "const x = 1.5;\nconst c = computed(() => Math.floor(x));",
                2,
                11,
                "computed",
            ),
            (
                "const c = computed(() => new Date().getFullYear());",
                1,
                11,
                "computed",
            ),
            (
                "const c = computed(function () {\n  return 42;\n});",
                1,
                11,
                "computed",
            ),
            (
                "const c = computed(() => 1, { equal: (a, b) => a === b });",
                1,
                11,
                "computed",
            ),
            ("const c = linkedSignal(() => 42);", 1, 11, "linkedSignal"),
            (
                "const c = linkedSignal({ source: () => 1, computation: (s) => s + 1 });",
                1,
                11,
                "linkedSignal",
            ),
            ("const value = 1;\neffect(() => value + 1);", 2, 1, "effect"),
            (
                "effect(() => {\n  console.log('side effect');\n});",
                1,
                1,
                "effect",
            ),
            ("afterRenderEffect(() => 42);", 1, 1, "afterRenderEffect"),
            (
                "afterRenderEffect({\n  earlyRead: () => 1,\n  write: (value) => value + 1,\n});",
                1,
                1,
                "afterRenderEffect",
            ),
        ] {
            assert_eq!(check(code), [(line, column, message(primitive))], "{code}");
        }
    }
}
