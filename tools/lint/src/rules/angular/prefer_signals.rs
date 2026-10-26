//! `@angular-eslint/prefer-signals`: signal inputs and queries rather than `@Input()` and the
//! query decorators, and every signal property `readonly` (`@angular-eslint/eslint-plugin`
//! 22.6.0, the cases of its tests), with its default options: the type checker is not asked, a
//! property is a signal by its written type or the factory that sets it. The added `readonly` is
//! not carried.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    CallExpression, Expression, PropertyDefinition, PropertyDefinitionType, TSType, TSTypeName,
};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::classes::called_name;
use super::signals::SIGNAL_TYPES;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferSignals;

/// The functions that make a signal (`KNOWN_SIGNAL_CREATION_FUNCTIONS`).
const CREATION_FUNCTIONS: [&str; 10] = [
    "computed",
    "contentChild",
    "contentChildren",
    "input",
    "linkedSignal",
    "model",
    "signal",
    "toSignal",
    "viewChild",
    "viewChildren",
];

/// The query decorators and the function each gives way to.
const QUERIES: [(&str, &str); 4] = [
    ("ContentChild", "contentChild"),
    ("ContentChildren", "contentChildren"),
    ("ViewChild", "viewChild"),
    ("ViewChildren", "viewChildren"),
];

/// Whether a property is a signal by its written type, a signal type with its arguments, or
/// without one by its value, the call of a signal factory, `.required` or `.asReadonly()` of one.
fn is_signal_property(property: &PropertyDefinition) -> bool {
    if let Some(annotation) = &property.type_annotation {
        return matches!(&annotation.type_annotation, TSType::TSTypeReference(reference)
            if reference.type_arguments.is_some()
                && matches!(&reference.type_name, TSTypeName::IdentifierReference(name)
                    if SIGNAL_TYPES.contains(&name.name.as_str())));
    }
    let Some(Expression::CallExpression(first)) = &property.value else {
        return false;
    };
    let mut call: &CallExpression = first;
    if let Expression::StaticMemberExpression(member) = &call.callee
        && member.property.name == "asReadonly"
    {
        let Expression::CallExpression(inner) = &member.object else {
            return false;
        };
        call = inner;
    }
    let callee = match &call.callee {
        Expression::StaticMemberExpression(member) if member.property.name == "required" => {
            &member.object
        }
        Expression::StaticMemberExpression(_) => return false,
        other => other,
    };
    matches!(callee, Expression::Identifier(name) if CREATION_FUNCTIONS.contains(&name.name.as_str()))
}

impl Rule for PreferSignals {
    fn name(&self) -> &'static str {
        "@angular-eslint/prefer-signals"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        match node.kind() {
            AstKind::PropertyDefinition(property)
                if !property.readonly
                    && property.r#type == PropertyDefinitionType::PropertyDefinition
                    && is_signal_property(property) =>
            {
                findings.push(source.finding(
                    self.name(),
                    property.key.span().start,
                    "Properties declared using signals should be marked as `readonly` since they should not be reassigned".into(),
                ));
            }
            AstKind::Decorator(decorator) => {
                let Some(name) = called_name(decorator) else {
                    return;
                };
                let message = if name == "Input" {
                    "Use `InputSignal`s (e.g. via `input()`) for Component input properties rather than the legacy `@Input()` decorator".to_owned()
                } else if let Some((_, function)) = QUERIES.iter().find(|(query, _)| *query == name)
                {
                    format!("Use the `{function}` function instead of the `{name}` decorator")
                } else {
                    return;
                };
                findings.push(source.finding(self.name(), decorator.span.start, message));
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferSignals;
    use crate::testing::lint;

    /// The place and message of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize, String)> {
        lint(&PreferSignals, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column, each.message))
            .collect()
    }

    /// A class of one member.
    fn class(member: &str) -> String {
        format!("class Test {{\n  {member}\n}}\n")
    }

    #[test]
    fn f51_ac2_accepts_readonly_signals_and_other_properties() {
        // The plugin's valid cases with its default options.
        for member in [
            "testSubject = new Subject();",
            "testSubject = new ReplaySubject(1);",
            "testValue = test();",
            "testValue: number;",
            "testValue: Widget<number>;",
            "readonly testSignal: Signal<number>;",
            "readonly testSignal: InputSignal<number>;",
            "readonly testSignal: ModelSignal<number>;",
            "readonly testSignal: WritableSignal<number>;",
            "readonly testSignal = computed(() => 0);",
            "readonly testSignal = linkedSignal(() => source);",
            "readonly testSignal = contentChild('test');",
            "readonly testSignal = contentChild.required('test');",
            "readonly testSignal = contentChildren('test');",
            "readonly testSignal = input('');",
            "readonly testSignal = input.required();",
            "readonly testSignal = model();",
            "readonly testSignal = signal(true);",
            "readonly testSignal = signal(true).asReadonly();",
            "readonly testSignal = toSignal(source);",
            "readonly testSignal = viewChild.required('test');",
            "readonly testSignal = viewChildren('test');",
            "testSignal = createSignal('test');",
            "readonly value = input();",
            // A signal type without its argument, and a member other than `required`.
            "testSignal: Signal;",
            "testSignal = signal.other();",
        ] {
            assert_eq!(check(&class(member)), [], "{member}");
        }
    }

    #[test]
    fn f51_ac2_reports_a_signal_property_not_readonly_at_its_name() {
        // The plugin's invalid cases: the name at column 3 of line 2.
        for member in [
            "testSignal: Signal<number>;",
            "testSignal: InputSignal<number>;",
            "testSignal: ModelSignal<number>;",
            "testSignal: WritableSignal<number>;",
            "testSignal = computed(() => 0);",
            "testSignal = linkedSignal(() => source);",
            "testSignal = contentChild('test');",
            "testSignal = contentChild.required('test');",
            "testSignal = contentChildren('test');",
            "testSignal = input('');",
            "testSignal = input.required('');",
            "testSignal = model(42);",
            "testSignal = model.required();",
            "testSignal = signal(42);",
            "testSignal = signal(42).asReadonly();",
            "testSignal = toSignal(source);",
            "testSignal = viewChild('test');",
            "testSignal = viewChild.required('test');",
            "testSignal = viewChildren('test');",
        ] {
            assert_eq!(
                check(&class(member)),
                [(
                    2,
                    3,
                    "Properties declared using signals should be marked as `readonly` since they should not be reassigned".to_owned()
                )],
                "{member}"
            );
        }
    }

    #[test]
    fn f51_ac2_reports_input_and_query_decorators_at_the_decorator() {
        assert_eq!(
            check("class Test {\n  @Input()\n  value = 1;\n}\n"),
            [(
                2,
                3,
                "Use `InputSignal`s (e.g. via `input()`) for Component input properties rather than the legacy `@Input()` decorator".to_owned()
            )]
        );
        for (decorator, function) in [
            ("ViewChild", "viewChild"),
            ("ViewChildren", "viewChildren"),
            ("ContentChild", "contentChild"),
            ("ContentChildren", "contentChildren"),
        ] {
            let code = format!("class Test {{\n  @{decorator}('test')\n  value: Widget;\n}}\n");

            assert_eq!(
                check(&code),
                [(
                    2,
                    3,
                    format!("Use the `{function}` function instead of the `{decorator}` decorator")
                )],
                "{code}"
            );
        }
    }
}
