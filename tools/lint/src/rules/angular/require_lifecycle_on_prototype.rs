//! `@angular-eslint/require-lifecycle-on-prototype`: a lifecycle method is defined on the class's
//! prototype, never as a property nor by an assignment to an instance, where Angular does not look
//! for it (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests).

use oxc_ast::AstKind;
use oxc_ast::ast::{
    AssignmentOperator, AssignmentTarget, Expression, PropertyDefinitionType, PropertyKey,
};
use oxc_semantic::AstNode;

use super::classes::is_lifecycle_method;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct RequireLifecycleOnPrototype;

/// The name of a property key as the plugin's selector reads it: an identifier, computed or not,
/// or a string.
fn key_name<'a>(key: &PropertyKey<'a>) -> Option<(&'a str, u32)> {
    match key {
        PropertyKey::StaticIdentifier(key) => Some((key.name.as_str(), key.span.start)),
        PropertyKey::Identifier(key) => Some((key.name.as_str(), key.span.start)),
        PropertyKey::StringLiteral(key) => Some((key.value.as_str()?, key.span.start)),
        _ => None,
    }
}

/// The name of a member's property and where it starts: `b` in `a.b`, `a[b]` and `a['b']`.
fn member_name<'a>(property: &'a Expression<'a>) -> Option<(&'a str, u32)> {
    match property {
        Expression::Identifier(name) => Some((name.name.as_str(), name.span.start)),
        Expression::StringLiteral(name) => Some((name.value.as_str()?, name.span.start)),
        _ => None,
    }
}

/// Whether an object, through its `as` casts, is a `prototype` (`isPrototype`).
fn is_prototype(object: &Expression) -> bool {
    match object {
        Expression::TSAsExpression(cast) => is_prototype(&cast.expression),
        Expression::StaticMemberExpression(member) => member.property.name == "prototype",
        Expression::ComputedMemberExpression(member) => {
            member_name(&member.expression).is_some_and(|(name, _)| name == "prototype")
        }
        _ => false,
    }
}

impl RequireLifecycleOnPrototype {
    /// The finding on a lifecycle method's name.
    fn report(&self, source: &Source, name: &str, at: u32, findings: &mut Vec<Finding>) {
        findings.push(source.finding(
            self.name(),
            at,
            format!(
                "The {name} lifecycle method should be defined on the object's prototype. See more at https://github.com/angular/angular/issues/38241"
            ),
        ));
    }
}

impl Rule for RequireLifecycleOnPrototype {
    fn name(&self) -> &'static str {
        "@angular-eslint/require-lifecycle-on-prototype"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        match node.kind() {
            AstKind::PropertyDefinition(property)
                if property.r#type == PropertyDefinitionType::PropertyDefinition =>
            {
                if let Some((name, at)) = key_name(&property.key)
                    && is_lifecycle_method(name)
                {
                    self.report(source, name, at, findings);
                }
            }
            AstKind::AssignmentExpression(assignment)
                if assignment.operator == AssignmentOperator::Assign =>
            {
                let (object, found) = match &assignment.left {
                    AssignmentTarget::StaticMemberExpression(member) => (
                        &member.object,
                        Some((member.property.name.as_str(), member.property.span.start)),
                    ),
                    AssignmentTarget::ComputedMemberExpression(member) => {
                        (&member.object, member_name(&member.expression))
                    }
                    _ => return,
                };
                if let Some((name, at)) = found
                    && is_lifecycle_method(name)
                    && !is_prototype(object)
                {
                    self.report(source, name, at, findings);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RequireLifecycleOnPrototype;
    use crate::testing::lint;

    /// The place and method of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize, String)> {
        lint(&RequireLifecycleOnPrototype, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| {
                let method = each
                    .message
                    .strip_prefix("The ")
                    .and_then(|rest| rest.split_once(' '))
                    .map(|(method, _)| method.to_owned())
                    .unwrap_or_default();
                assert!(each.message.ends_with(
                    "lifecycle method should be defined on the object's prototype. See more at https://github.com/angular/angular/issues/38241"
                ));
                (each.line, each.column, method)
            })
            .collect()
    }

    #[test]
    fn f51_ac1_accepts_methods_and_assignments_to_the_prototype() {
        // The plugin's valid cases.
        for code in [
            "@Component({})\nclass Test {\n  ngOnInit() {}\n}\n",
            "@Injectable({})\nclass Test {\n  ngOnDestroy() {}\n}\n",
            "function hook(type) {\n  type.prototype.ngOnDestroy = () => {};\n}\n",
            "function hook(type) {\n  (type.prototype as any).ngOnDestroy = () => {};\n}\n",
            "function hook(type) {\n  type['prototype'].ngOnDestroy = () => {};\n}\n",
            "function hook(type) {\n  (type.prototype as unknown as { ngOnDestroy: () => void }).ngOnDestroy = () => {};\n}\n",
            "@Component({})\nclass Test {\n  onDestroy = () => {}\n}\n",
            "class Test {\n  constructor() {\n    this.onDestroy = () => {}\n  }\n}\n",
            "class Test {\n  constructor() {\n    this['onDestroy'] = () => {}\n  }\n}\n",
            "class Test {\n  constructor() {\n    let ngOnDestroy;\n    ngOnDestroy = () => {};\n  }\n}\n",
            // A compound assignment, as the plugin's `[operator="="]`.
            "function hook(c) {\n  c.ngOnInit ??= func;\n}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac1_reports_each_hook_set_on_an_instance() {
        // Each hook as a property of a component: its name at line 3, column 3.
        for method in super::super::classes::LIFECYCLE_METHODS {
            let code = format!("@Component({{}})\nclass Test {{\n  {method} = func;\n}}\n");

            assert_eq!(check(&code), [(3, 3, method.to_owned())], "{code}");
        }
        let on_destroy = |line, column| vec![(line, column, "ngOnDestroy".to_owned())];
        // In a constructor or a method, `    this.` (9 columns) then the name; `this['` puts the
        // string's quote at column 10.
        assert_eq!(
            check("class Test {\n  constructor() {\n    this.ngOnDestroy = func;\n  }\n}\n"),
            on_destroy(3, 10)
        );
        assert_eq!(
            check("class Test {\n  constructor() {\n    this['ngOnDestroy'] = func;\n  }\n}\n"),
            on_destroy(3, 10)
        );
        assert_eq!(
            check("class Test {\n  run() {\n    this.ngOnDestroy = func;\n  }\n}\n"),
            on_destroy(3, 10)
        );
        // Outside a class, `  component.` is 12 columns; through a cast, `  (component as any).`
        // is 21.
        assert_eq!(
            check("function hook(component) {\n  component.ngOnDestroy = func;\n}\n"),
            on_destroy(2, 13)
        );
        assert_eq!(
            check("function hook(component) {\n  (component as any).ngOnDestroy = func;\n}\n"),
            on_destroy(2, 22)
        );
        // A string key in brackets: its quote at column 4.
        assert_eq!(
            check("class Test {\n  ['ngOnDestroy'] = func;\n}\n"),
            on_destroy(2, 4)
        );
    }
}
