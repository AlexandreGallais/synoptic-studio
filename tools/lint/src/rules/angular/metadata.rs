//! What the Angular rules read of a decorator's metadata, after `@angular-eslint/utils` 22.6.0's
//! `ast-utils.ts` and `selectors.ts`: a class's own decorator of a name, the object it is called
//! with, its properties and the key a property names.

use oxc_ast::ast::{
    Argument, Class, Decorator, Expression, ObjectExpression, ObjectProperty, ObjectPropertyKind,
    PropertyKey, TSTypeName,
};

use super::classes::{called_name, key_identifier};

/// The decorator of a class declaration that calls a name (the selectors'
/// `ClassDeclaration > Decorator[expression.callee.name=…]`).
#[must_use]
pub fn class_decorator<'b, 'a>(class: &'b Class<'a>, name: &str) -> Option<&'b Decorator<'a>> {
    if !class.is_declaration() {
        return None;
    }
    class
        .decorators
        .iter()
        .find(|decorator| called_name(decorator) == Some(name))
}

/// The first of a class declaration's decorators that calls one of some names, with that name.
#[must_use]
pub fn class_decorator_of<'b, 'a>(
    class: &'b Class<'a>,
    names: &[&str],
) -> Option<(&'b Decorator<'a>, &'a str)> {
    if !class.is_declaration() {
        return None;
    }
    class.decorators.iter().find_map(|decorator| {
        called_name(decorator)
            .filter(|name| names.contains(name))
            .map(|name| (decorator, name))
    })
}

/// The object a decorator is called with first (`getDecoratorArgument`).
#[must_use]
pub fn decorator_object<'b, 'a>(decorator: &'b Decorator<'a>) -> Option<&'b ObjectExpression<'a>> {
    let Expression::CallExpression(call) = &decorator.expression else {
        return None;
    };
    match call.arguments.first() {
        Some(Argument::ObjectExpression(object)) => Some(object),
        _ => None,
    }
}

/// The properties of the object a decorator is called with, spreads left out
/// (`getDecoratorProperties`).
#[must_use]
pub fn decorator_properties<'b, 'a>(decorator: &'b Decorator<'a>) -> Vec<&'b ObjectProperty<'a>> {
    decorator_object(decorator).map_or_else(Vec::new, |object| properties(object))
}

/// The properties of an object, spreads left out.
#[must_use]
pub fn properties<'b, 'a>(object: &'b ObjectExpression<'a>) -> Vec<&'b ObjectProperty<'a>> {
    object
        .properties
        .iter()
        .filter_map(|property| match property {
            ObjectPropertyKind::ObjectProperty(property) => Some(&**property),
            ObjectPropertyKind::SpreadProperty(_) => None,
        })
        .collect()
}

/// The property of a decorator's object whose key is an identifier of a name, computed or not
/// (`getDecoratorProperty`).
#[must_use]
pub fn decorator_property<'b, 'a>(
    decorator: &'b Decorator<'a>,
    name: &str,
) -> Option<&'b ObjectProperty<'a>> {
    decorator_properties(decorator)
        .into_iter()
        .find(|property| key_identifier(&property.key) == Some(name))
}

/// Whether a property's key names a metadata property as the selectors' `metadataProperty` read
/// it: an identifier not computed, a string, or a template whose first part is the name.
#[must_use]
pub fn names_metadata(property: &ObjectProperty, name: &str) -> bool {
    match &property.key {
        PropertyKey::StaticIdentifier(key) => !property.computed && key.name == name,
        PropertyKey::StringLiteral(key) => key.value == name,
        PropertyKey::TemplateLiteral(key) => key
            .quasis
            .first()
            .is_some_and(|quasi| quasi.value.raw == name),
        _ => false,
    }
}

/// The property of a decorator's object that names a metadata property (`metadataProperty`).
#[must_use]
pub fn metadata_property<'b, 'a>(
    decorator: &'b Decorator<'a>,
    name: &str,
) -> Option<&'b ObjectProperty<'a>> {
    decorator_properties(decorator)
        .into_iter()
        .find(|property| names_metadata(property, name))
}

/// The names of the interfaces a class implements, `B` for `ng.B` (`getDeclaredInterfaceNames`).
#[must_use]
pub fn implemented_names<'a>(class: &Class<'a>) -> Vec<&'a str> {
    class
        .implements
        .iter()
        .map(|implemented| match &implemented.expression {
            TSTypeName::IdentifierReference(name) => name.name.as_str(),
            TSTypeName::QualifiedName(name) => name.right.name.as_str(),
            TSTypeName::ThisExpression(_) => "this",
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use oxc_allocator::Allocator;
    use oxc_ast::ast::Statement;
    use oxc_parser::Parser;
    use oxc_span::SourceType;

    use super::{class_decorator, decorator_property, metadata_property};

    #[test]
    fn reads_a_class_decorator_s_metadata_by_its_keys() {
        let allocator = Allocator::default();
        let code =
            "@Component({ selector: 'a', ['host']: {}, 'styles': `b`, ...rest })\nclass A {}\n";
        let parsed = Parser::new(&allocator, code, SourceType::ts()).parse();
        let Some(Statement::ClassDeclaration(class)) = parsed.program.body.first() else {
            panic!("a class");
        };
        let decorator = class_decorator(class, "Component").expect("its decorator");

        assert!(class_decorator(class, "Directive").is_none());
        // `getDecoratorProperty` reads identifiers, computed or not, never strings.
        assert!(decorator_property(decorator, "selector").is_some());
        assert!(decorator_property(decorator, "styles").is_none());
        // `metadataProperty` reads strings and templates, never a computed identifier.
        assert!(metadata_property(decorator, "styles").is_some());
        assert!(metadata_property(decorator, "host").is_some());
    }
}
