//! `@angular-eslint/prefer-signal-model`: `model()` rather than an `input()` and an `output()` of
//! the same name and `Change`, a two-way binding (`@angular-eslint/eslint-plugin` 22.6.0, the
//! cases of its tests), with its default options: the written type arguments are compared as
//! text, a side without one matching any. An input with a `transform` is left, `model()` having
//! none. The rewrite into `model()` is not carried.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Argument, CallExpression, ClassElement, Expression, ObjectPropertyKind, PropertyDefinition,
    PropertyKey,
};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use crate::expressions::text;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferSignalModel;

/// An `input()` or an `output()` property: its name, where it starts and its type argument.
struct Declared<'a> {
    /// Its name, a string key with its quotes (`getPropertyDefinitionName`).
    name: &'a str,
    /// Where the property starts.
    start: u32,
    /// The span of its first type argument.
    type_argument: Option<Span>,
}

/// The kind of signal a property's value makes, and the place of its options among the call's
/// arguments: `input(initial, options)`, `input.required(options)`, `output(options)`.
fn signal_call<'b, 'a>(value: &'b Expression<'a>) -> Option<(bool, usize, &'b CallExpression<'a>)> {
    let Expression::CallExpression(call) = value else {
        return None;
    };
    match &call.callee {
        Expression::Identifier(callee) if callee.name == "input" => Some((true, 1, call)),
        Expression::Identifier(callee) if callee.name == "output" => Some((false, 0, call)),
        Expression::StaticMemberExpression(member)
            if member.property.name == "required"
                && matches!(&member.object, Expression::Identifier(object) if object.name == "input") =>
        {
            Some((true, 0, call))
        }
        _ => None,
    }
}

/// Whether options hold a `transform` (`hasTransformOption`).
fn has_transform(options: Option<&Argument>) -> bool {
    let Some(Argument::ObjectExpression(object)) = options else {
        return false;
    };
    object.properties.iter().any(|property| match property {
        ObjectPropertyKind::ObjectProperty(property) => match &property.key {
            PropertyKey::StaticIdentifier(key) => key.name == "transform",
            PropertyKey::Identifier(key) => key.name == "transform",
            PropertyKey::StringLiteral(key) => key.value == "transform",
            _ => false,
        },
        ObjectPropertyKind::SpreadProperty(_) => false,
    })
}

/// The name of a property: an identifier's not computed, a string's text with its quotes.
fn property_name<'a>(
    source: &Source<'_, 'a>,
    property: &PropertyDefinition<'a>,
) -> Option<&'a str> {
    match &property.key {
        PropertyKey::StaticIdentifier(key) if !property.computed => Some(key.name.as_str()),
        PropertyKey::StringLiteral(key) => Some(text(source, key.span)),
        _ => None,
    }
}

impl Rule for PreferSignalModel {
    fn name(&self) -> &'static str {
        "@angular-eslint/prefer-signal-model"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        let mut inputs: Vec<Declared> = Vec::new();
        let mut outputs: Vec<Declared> = Vec::new();
        for element in &class.body.body {
            let ClassElement::PropertyDefinition(property) = element else {
                continue;
            };
            let Some((is_input, options, call)) = property.value.as_ref().and_then(signal_call)
            else {
                continue;
            };
            let Some(name) = property_name(source, property) else {
                continue;
            };
            if has_transform(call.arguments.get(options)) {
                continue;
            }
            let declared = Declared {
                name,
                start: property.span.start,
                type_argument: call
                    .type_arguments
                    .as_ref()
                    .and_then(|arguments| arguments.params.first())
                    .map(GetSpan::span),
            };
            let list = if is_input { &mut inputs } else { &mut outputs };
            match list.iter_mut().find(|each| each.name == name) {
                Some(earlier) => *earlier = declared,
                None => list.push(declared),
            }
        }
        for input in &inputs {
            let change = format!("{}Change", input.name);
            let Some(output) = outputs.iter().find(|output| output.name == change) else {
                continue;
            };
            let same_types = match (input.type_argument, output.type_argument) {
                (Some(left), Some(right)) => text(source, left) == text(source, right),
                _ => true,
            };
            if same_types {
                findings.push(source.finding(
                    self.name(),
                    input.start,
                    "Use `model` for two-way bindings instead of `input()` and `output()`".into(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferSignalModel;
    use crate::testing::lint;

    /// The lines of the findings on a class body, each at column 3.
    fn check(body: &str) -> Vec<usize> {
        let code = format!("class Test {{\n{body}}}\n");
        lint(&PreferSignalModel, "playground/src/app/a.ts", &code)
            .into_iter()
            .map(|each| {
                assert_eq!(
                    each.message,
                    "Use `model` for two-way bindings instead of `input()` and `output()`"
                );
                assert_eq!(each.column, 3);
                each.line
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_models_lone_signals_transforms_and_other_types() {
        // The plugin's valid cases without type checking.
        for body in [
            "  readonly enabled = model();\n",
            "  readonly enabled = model<boolean>();\n",
            "  readonly enabled = input();\n",
            "  readonly enabledChange = output();\n",
            "  readonly enabled = input();\n  readonly onChange = output();\n",
            "  readonly enabled = input(false, { transform: booleanAttribute });\n  readonly enabledChange = output<boolean>();\n",
            "  readonly count = input(0, { transform: numberAttribute });\n  readonly countChange = output<number>();\n",
            "  readonly enabled = input(false, { ...base, 'transform': booleanAttribute });\n  readonly enabledChange = output<boolean>();\n",
            "  readonly value = input<string>();\n  readonly valueChange = output<number>();\n",
            "  readonly value = input<string | null>();\n  readonly valueChange = output<string>();\n",
            "  readonly value = input.required<string>();\n  readonly valueChange = output<number>();\n",
        ] {
            assert_eq!(check(body), Vec::<usize>::new(), "{body}");
        }
    }

    #[test]
    fn f51_ac2_reports_each_input_paired_with_its_change_output_at_the_input() {
        // The plugin's invalid cases without type checking; the class body starts on line 2.
        for (body, lines) in [
            (
                "  readonly enabled = input();\n  readonly enabledChange = output();\n",
                vec![2],
            ),
            (
                "  readonly enabled = input();\n  readonly enabledChange = output();\n  readonly test = input();\n  readonly testChange = output();\n",
                vec![2, 4],
            ),
            (
                "  readonly value = input();\n  readonly otherProp = input();\n  readonly valueChange = output();\n",
                vec![2],
            ),
            (
                "  readonly count = input<number>();\n  readonly countChange = output<number>();\n",
                vec![2],
            ),
            (
                "  readonly enabledChange = output();\n  readonly enabled = input();\n",
                vec![3],
            ),
            (
                "  readonly enabled = input(false, { transform: booleanAttribute });\n  readonly enabledChange = output<boolean>();\n  readonly count = input(0);\n  readonly countChange = output<number>();\n",
                vec![4],
            ),
            (
                "  readonly style = input({ transform: 'scale(2)' });\n  readonly styleChange = output<object>();\n",
                vec![2],
            ),
            (
                "  readonly value = input.required<string>();\n  readonly valueChange = output<string>();\n",
                vec![2],
            ),
            (
                "  readonly input = input<string>();\n  readonly inputChange = output<string>();\n",
                vec![2],
            ),
            (
                "  readonly enabled = input<boolean>();\n  readonly enabledChange = output<boolean>();\n  forward(): unknown {\n    return this.enabledChange;\n  }\n",
                vec![2],
            ),
        ] {
            assert_eq!(check(body), lines, "{body}");
        }
    }
}
