//! What the Angular rules read of a class, after `@angular-eslint/utils` 22.6.0's `ast-utils.ts`
//! and `selectors.ts`: the class decorators Angular knows, the lifecycle methods and those each
//! decorator's class has Angular call, and a member's name.

use oxc_ast::AstKind;
use oxc_ast::ast::{Class, Decorator, Expression, MethodDefinition, PropertyKey};
use oxc_semantic::NodeId;

use crate::source::Source;

/// The lifecycle methods (`AngularLifecycleMethods`).
pub const LIFECYCLE_METHODS: [&str; 9] = [
    "ngAfterContentChecked",
    "ngAfterContentInit",
    "ngAfterViewChecked",
    "ngAfterViewInit",
    "ngDoBootstrap",
    "ngDoCheck",
    "ngOnChanges",
    "ngOnDestroy",
    "ngOnInit",
];

/// The class decorators (`AngularClassDecorators`).
pub const CLASS_DECORATORS: [&str; 6] = [
    "Component",
    "Directive",
    "Injectable",
    "NgModule",
    "Pipe",
    "Service",
];

/// The lifecycle methods Angular calls on a class of a decorator
/// (`ANGULAR_CLASS_DECORATOR_LIFECYCLE_METHOD_MAPPER`): every one but `ngDoBootstrap` on a
/// component or a directive, `ngDoBootstrap` alone on a module, `ngOnDestroy` alone on the others.
#[must_use]
pub fn called_lifecycle_methods(decorator: &str) -> &'static [&'static str] {
    match decorator {
        "Component" | "Directive" => &[
            "ngAfterContentChecked",
            "ngAfterContentInit",
            "ngAfterViewChecked",
            "ngAfterViewInit",
            "ngDoCheck",
            "ngOnChanges",
            "ngOnDestroy",
            "ngOnInit",
        ],
        "NgModule" => &["ngDoBootstrap"],
        _ => &["ngOnDestroy"],
    }
}

/// Whether a name is a lifecycle method's.
#[must_use]
pub fn is_lifecycle_method(name: &str) -> bool {
    LIFECYCLE_METHODS.contains(&name)
}

/// The name a decorator calls, `Component` in `@Component({…})` (the selectors'
/// `Decorator[expression.callee.name=…]`); none for a decorator written without a call.
#[must_use]
pub fn called_name<'a>(decorator: &Decorator<'a>) -> Option<&'a str> {
    match &decorator.expression {
        Expression::CallExpression(call) => match &call.callee {
            Expression::Identifier(callee) => Some(callee.name.as_str()),
            _ => None,
        },
        _ => None,
    }
}

/// The name of a decorator, called or not (`getDecoratorName`).
#[must_use]
pub fn decorator_name<'a>(decorator: &Decorator<'a>) -> Option<&'a str> {
    match &decorator.expression {
        Expression::Identifier(identifier) => Some(identifier.name.as_str()),
        _ => called_name(decorator),
    }
}

/// The first Angular class decorator of a class (`getAngularClassDecorator`).
#[must_use]
pub fn angular_decorator<'a>(class: &Class<'a>) -> Option<&'a str> {
    class
        .decorators
        .iter()
        .filter_map(decorator_name)
        .find(|name| CLASS_DECORATORS.contains(name))
}

/// The name of a method (`getMethodName`): its string key, or its identifier when not computed.
#[must_use]
pub fn method_name<'a>(method: &MethodDefinition<'a>) -> Option<&'a str> {
    match &method.key {
        PropertyKey::StringLiteral(key) => key.value.as_str(),
        PropertyKey::StaticIdentifier(key) => Some(key.name.as_str()),
        _ => None,
    }
}

/// The name a key has as the selectors' `[key.name=…]` read it: an identifier's, computed or not.
#[must_use]
pub fn key_identifier<'a>(key: &PropertyKey<'a>) -> Option<&'a str> {
    match key {
        PropertyKey::StaticIdentifier(key) => Some(key.name.as_str()),
        PropertyKey::Identifier(key) => Some(key.name.as_str()),
        _ => None,
    }
}

/// The names the class decorators around a node call, from the nearest out: those of each
/// decorator of a class declaration the node sits in (the selectors'
/// `ClassDeclaration > Decorator[expression.callee.name=…] …`).
#[must_use]
pub fn enclosing_decorators<'a>(source: &Source<'_, 'a>, id: NodeId) -> Vec<&'a str> {
    let nodes = source.semantic.nodes();
    nodes
        .ancestors(id)
        .filter_map(|ancestor| match ancestor.kind() {
            AstKind::Decorator(decorator) => match nodes.parent_kind(ancestor.id()) {
                AstKind::Class(class) if class.is_declaration() => called_name(decorator),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

/// The nearest class declaration around a node (`getNearestNodeFrom(…, isClassDeclaration)`).
#[must_use]
pub fn enclosing_class<'s, 'a>(source: &'s Source<'_, 'a>, id: NodeId) -> Option<&'s Class<'a>> {
    source
        .semantic
        .nodes()
        .ancestors(id)
        .find_map(|ancestor| match ancestor.kind() {
            AstKind::Class(class) if class.is_declaration() => Some(class),
            _ => None,
        })
}
