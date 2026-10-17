//! One module read with `Oxc` (`REF-OXC-CRATE`): its exports as plain data, their `TSDoc` blocks
//! and the modules it re-exports. `oxc_parser` reads the source and its comments;
//! `oxc_semantic` resolves each name of a type to the module's own symbols, so that a type
//! parameter, an import and a global of the language are told apart.

use oxc_allocator::Allocator;
use oxc_ast::ast::{
    BindingPattern, Comment, Declaration, Expression, FormalParameters, Function,
    IdentifierReference, PropertyKey, Statement, TSLiteral, TSSignature, TSTupleElement, TSType,
    TSTypeName, TSTypeOperatorOperator, TSTypeParameterDeclaration, TSTypeQueryExprName,
    UnaryOperator, VariableDeclarator,
};
use oxc_parser::Parser;
use oxc_semantic::{Scoping, SemanticBuilder};
use oxc_span::{GetSpan, SourceType};

use crate::doc;
use crate::model::{
    Body, Doc, Export, Kind, Member, Parameter, Signature, Target, Type, TypeParameter,
};

/// What one module gives the API.
#[derive(Debug, Default)]
pub struct Module {
    /// Its exports, in source order.
    pub exports: Vec<Export>,
    /// The specifiers of its `export * from` declarations.
    pub re_exports: Vec<String>,
    /// What it holds that the generator cannot read, each with its place.
    pub problems: Vec<String>,
    /// The constants whose type is the return type of a function, resolved once every module
    /// is read: (constant, function).
    pub returns_of: Vec<(String, String)>,
}

/// The source, its comments and its scopes, while one module is read.
struct Reader<'s, 'a> {
    source: &'s str,
    comments: &'s [Comment],
    scoping: &'s Scoping,
    file: &'s str,
    /// The type aliases the module declares without exporting them, which a page shows
    /// written out, as the compiler does.
    locals: Vec<(&'s str, &'s TSType<'a>)>,
    /// The local aliases being written out, so that a recursive one stops.
    expanding: Vec<String>,
    problems: Vec<String>,
}

impl<'s, 'a> Reader<'s, 'a> {
    /// The line of an offset, from 1.
    fn line(&self, offset: u32) -> usize {
        let end = usize::try_from(offset)
            .unwrap_or(usize::MAX)
            .min(self.source.len());
        self.source[..end].matches('\n').count() + 1
    }

    /// The `TSDoc` block attached to the token at an offset.
    fn doc(&self, offset: u32) -> Option<Doc> {
        self.comments
            .iter()
            .rfind(|comment| comment.is_jsdoc() && comment.attached_to == offset)
            .map(|comment| doc::parse(comment.span.source_text(self.source)))
    }

    /// A problem at an offset, named by its file and line.
    fn problem(&mut self, offset: u32, message: &str) {
        let line = self.line(offset);
        self.problems
            .push(format!("{}:{line}: {message}", self.file));
    }

    /// What a name in a type refers to.
    fn target(&self, identifier: &IdentifierReference) -> Target {
        let symbol = identifier
            .reference_id
            .get()
            .and_then(|reference| self.scoping.get_reference(reference).symbol_id());

        match symbol {
            None => Target::Global,
            Some(symbol) if self.scoping.symbol_flags(symbol).is_type_parameter() => {
                Target::TypeParameter
            }
            Some(_) => Target::Module,
        }
    }

    /// A name of a type, dotted when qualified, and what it refers to.
    fn type_name(&self, name: &TSTypeName) -> (String, Target) {
        match name {
            TSTypeName::IdentifierReference(identifier) => {
                (identifier.name.to_string(), self.target(identifier))
            }
            TSTypeName::QualifiedName(qualified) => {
                let (left, target) = self.type_name(&qualified.left);
                (format!("{left}.{}", qualified.right.name), target)
            }
            TSTypeName::ThisExpression(_) => ("this".to_owned(), Target::Global),
        }
    }

    /// The type of a local alias a reference names, when it takes no argument.
    fn local(&self, name: &str, target: Target, bare: bool) -> Option<&'s TSType<'a>> {
        if target != Target::Module || !bare || self.expanding.iter().any(|each| each == name) {
            return None;
        }
        self.locals
            .iter()
            .find(|(local, _)| *local == name)
            .map(|(_, ty)| *ty)
    }

    /// A literal type as the compiler prints it.
    fn literal(&mut self, literal: &TSLiteral, offset: u32) -> Type {
        match literal {
            TSLiteral::BooleanLiteral(boolean) => Type::Literal(boolean.value.to_string()),
            TSLiteral::NumericLiteral(number) => Type::Literal(number_text(number.value)),
            TSLiteral::StringLiteral(string) => Type::Literal(format!("\"{}\"", string.value)),
            TSLiteral::UnaryExpression(unary) => match (&unary.operator, &unary.argument) {
                (UnaryOperator::UnaryNegation, Expression::NumericLiteral(number)) => {
                    Type::Literal(format!("-{}", number_text(number.value)))
                }
                _ => self.unsupported(offset, "this literal type"),
            },
            _ => self.unsupported(offset, "this literal type"),
        }
    }

    /// A problem for a syntax the pages cannot show, and a placeholder type.
    fn unsupported(&mut self, offset: u32, what: &str) -> Type {
        self.problem(offset, &format!("the generator cannot read {what}"));
        Type::Intrinsic("unknown".to_owned())
    }

    /// A type as written.
    fn ty(&mut self, ty: &TSType) -> Type {
        let intrinsic = |name: &str| Type::Intrinsic(name.to_owned());

        match ty {
            TSType::TSAnyKeyword(_) => intrinsic("any"),
            TSType::TSBigIntKeyword(_) => intrinsic("bigint"),
            TSType::TSBooleanKeyword(_) => intrinsic("boolean"),
            TSType::TSNeverKeyword(_) => intrinsic("never"),
            TSType::TSNullKeyword(_) => intrinsic("null"),
            TSType::TSNumberKeyword(_) => intrinsic("number"),
            TSType::TSObjectKeyword(_) => intrinsic("object"),
            TSType::TSStringKeyword(_) => intrinsic("string"),
            TSType::TSSymbolKeyword(_) => intrinsic("symbol"),
            TSType::TSUndefinedKeyword(_) => intrinsic("undefined"),
            TSType::TSUnknownKeyword(_) => intrinsic("unknown"),
            TSType::TSVoidKeyword(_) => intrinsic("void"),
            TSType::TSArrayType(array) => Type::Array(Box::new(self.ty(&array.element_type))),
            TSType::TSFunctionType(function) => Type::Function(Box::new(Signature {
                type_parameters: self.type_parameters(function.type_parameters.as_deref()),
                parameters: self.parameters(&function.params),
                returns: self.ty(&function.return_type.type_annotation),
            })),
            TSType::TSIndexedAccessType(access) => Type::IndexedAccess(
                Box::new(self.ty(&access.object_type)),
                Box::new(self.ty(&access.index_type)),
            ),
            TSType::TSIntersectionType(intersection) => Type::Intersection(
                intersection
                    .types
                    .iter()
                    .map(|each| self.ty(each))
                    .collect(),
            ),
            TSType::TSUnionType(union) => {
                Type::Union(union.types.iter().map(|each| self.ty(each)).collect())
            }
            TSType::TSLiteralType(literal) => self.literal(&literal.literal, literal.span.start),
            TSType::TSParenthesizedType(inner) => self.ty(&inner.type_annotation),
            TSType::TSTupleType(tuple) => Type::Tuple(
                tuple
                    .element_types
                    .iter()
                    .map(|element| match element {
                        TSTupleElement::TSOptionalType(_) | TSTupleElement::TSRestType(_) => {
                            self.unsupported(element.span().start, "an optional or rest element")
                        }
                        other => self.ty(other.to_ts_type()),
                    })
                    .collect(),
            ),
            TSType::TSTypeLiteral(literal) => Type::Object(self.members(&literal.members)),
            TSType::TSNamedTupleMember(member) => match &member.element_type {
                TSTupleElement::TSOptionalType(_) | TSTupleElement::TSRestType(_) => {
                    self.unsupported(member.span.start, "an optional or rest element")
                }
                other => self.ty(other.to_ts_type()),
            },
            TSType::TSTypePredicate(predicate) => {
                Type::Literal(predicate.span.source_text(self.source).to_owned())
            }
            TSType::TSTypeOperatorType(operator) => Type::Operator(
                match operator.operator {
                    TSTypeOperatorOperator::Keyof => "keyof",
                    TSTypeOperatorOperator::Unique => "unique",
                    TSTypeOperatorOperator::Readonly => "readonly",
                },
                Box::new(self.ty(&operator.type_annotation)),
            ),
            TSType::TSTypeQuery(query) => match &query.expr_name {
                TSTypeQueryExprName::TSImportType(_) => {
                    self.unsupported(query.span.start, "an import type")
                }
                other => {
                    let (name, target) = self.type_name(other.to_ts_type_name());
                    Type::Query { name, target }
                }
            },
            TSType::TSTypeReference(reference) => {
                let (name, target) = self.type_name(&reference.type_name);
                if let Some(local) = self.local(&name, target, reference.type_arguments.is_none()) {
                    self.expanding.push(name);
                    let written = self.ty(local);
                    self.expanding.pop();
                    return written;
                }
                let arguments = reference
                    .type_arguments
                    .as_ref()
                    .map(|arguments| arguments.params.iter().map(|each| self.ty(each)).collect())
                    .unwrap_or_default();

                Type::Reference {
                    name,
                    target,
                    arguments,
                }
            }
            other => self.unsupported(other.span().start, "this kind of type"),
        }
    }

    /// The type parameters of a declaration.
    fn type_parameters(
        &mut self,
        declaration: Option<&TSTypeParameterDeclaration>,
    ) -> Vec<TypeParameter> {
        declaration.map_or_else(Vec::new, |declaration| {
            declaration
                .params
                .iter()
                .map(|parameter| TypeParameter {
                    name: parameter.name.name.to_string(),
                    constraint: parameter.constraint.as_ref().map(|each| self.ty(each)),
                })
                .collect()
        })
    }

    /// The parameters of a function or a function type.
    fn parameters(&mut self, parameters: &FormalParameters) -> Vec<Parameter> {
        let mut read = Vec::new();

        for parameter in &parameters.items {
            let value = match &parameter.type_annotation {
                Some(annotation) => self.ty(&annotation.type_annotation),
                None => self.unsupported(parameter.span.start, "a parameter without its type"),
            };
            read.push(Parameter {
                name: binding_name(&parameter.pattern),
                value,
                optional: parameter.optional || parameter.initializer.is_some(),
                rest: false,
            });
        }

        if let Some(rest) = &parameters.rest {
            let value = match &rest.type_annotation {
                Some(annotation) => self.ty(&annotation.type_annotation),
                None => self.unsupported(rest.span.start, "a parameter without its type"),
            };
            read.push(Parameter {
                name: binding_name(&rest.rest.argument),
                value,
                optional: false,
                rest: true,
            });
        }

        read
    }

    /// The properties of an object type.
    fn members(&mut self, members: &[TSSignature]) -> Vec<Member> {
        let mut read = Vec::new();

        for member in members {
            let TSSignature::TSPropertySignature(property) = member else {
                self.unsupported(member.span().start, "a member other than a property");
                continue;
            };
            let name = match &property.key {
                PropertyKey::StaticIdentifier(identifier) => identifier.name.to_string(),
                PropertyKey::StringLiteral(string) => string.value.to_string(),
                _ => {
                    self.unsupported(property.span.start, "a computed property name");
                    continue;
                }
            };
            let value = match &property.type_annotation {
                Some(annotation) => self.ty(&annotation.type_annotation),
                None => self.unsupported(property.span.start, "a property without its type"),
            };
            read.push(Member {
                name,
                readonly: property.readonly,
                optional: property.optional,
                value,
                doc: self.doc(property.span.start),
                line: self.line(property.key.span().start),
            });
        }

        read
    }

    /// A function's signature, its return type written.
    fn signature(&mut self, function: &Function) -> Signature {
        let returns = match &function.return_type {
            Some(annotation) => self.ty(&annotation.type_annotation),
            None => self.unsupported(function.span.start, "a function without its return type"),
        };

        Signature {
            type_parameters: self.type_parameters(function.type_parameters.as_deref()),
            parameters: self.parameters(&function.params),
            returns,
        }
    }

    /// A constant's type: written, or read from a literal value; the function whose return
    /// type it takes when its value is a call.
    fn constant(
        &mut self,
        declarator: &VariableDeclarator,
    ) -> (Type, Option<String>, Option<String>) {
        if let Some(annotation) = &declarator.type_annotation {
            return (self.ty(&annotation.type_annotation), None, None);
        }

        let raw = |span: oxc_span::Span| span.source_text(self.source).to_owned();

        match &declarator.init {
            Some(Expression::NumericLiteral(number)) => (
                Type::Literal(number_text(number.value)),
                Some(raw(number.span)),
                None,
            ),
            Some(Expression::StringLiteral(string)) => (
                Type::Literal(format!("\"{}\"", string.value)),
                Some(raw(string.span)),
                None,
            ),
            Some(Expression::BooleanLiteral(boolean)) => (
                Type::Literal(boolean.value.to_string()),
                Some(raw(boolean.span)),
                None,
            ),
            Some(Expression::BinaryExpression(binary)) if binary.operator.is_arithmetic() => {
                (Type::Intrinsic("number".to_owned()), None, None)
            }
            Some(Expression::CallExpression(call)) => match &call.callee {
                Expression::Identifier(callee) => (
                    Type::Intrinsic("unknown".to_owned()),
                    None,
                    Some(callee.name.to_string()),
                ),
                _ => (self.untyped(declarator), None, None),
            },
            Some(Expression::TSAsExpression(cast)) => {
                match (&cast.expression, &cast.type_annotation) {
                    (Expression::ArrayExpression(array), TSType::TSTypeReference(reference))
                        if matches!(&reference.type_name, TSTypeName::IdentifierReference(name) if name.name == "const") =>
                    {
                        let elements = array
                            .elements
                            .iter()
                            .map(|element| match element.as_expression() {
                                Some(Expression::StringLiteral(string)) => {
                                    Type::Literal(format!("\"{}\"", string.value))
                                }
                                Some(Expression::NumericLiteral(number)) => {
                                    Type::Literal(number_text(number.value))
                                }
                                _ => self.unsupported(
                                    array.span.start,
                                    "this element of a constant array",
                                ),
                            })
                            .collect();
                        (
                            Type::Operator("readonly", Box::new(Type::Tuple(elements))),
                            None,
                            None,
                        )
                    }
                    _ => (self.untyped(declarator), None, None),
                }
            }
            _ => (self.untyped(declarator), None, None),
        }
    }

    /// A problem for a constant whose type cannot be read without the compiler.
    fn untyped(&mut self, declarator: &VariableDeclarator) -> Type {
        self.unsupported(
            declarator.span.start,
            "this constant's type: write its type annotation",
        )
    }
}

/// A number as JavaScript prints it, for the integers and the plain decimals of the library.
#[must_use]
pub fn number_text(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e21 {
        format!("{value:.0}")
    } else if value != 0.0 && (value.abs() < 1e-6 || value.abs() >= 1e21) {
        let text = format!("{value:e}");
        match text.split_once('e') {
            Some((mantissa, exponent)) if !exponent.starts_with('-') => {
                format!("{mantissa}e+{exponent}")
            }
            _ => text,
        }
    } else {
        value.to_string()
    }
}

/// The name a parameter binds, or its pattern's text for a destructuring.
fn binding_name(pattern: &BindingPattern) -> String {
    pattern
        .get_identifier_name()
        .map_or_else(|| "__namedParameters".to_owned(), |name| name.to_string())
}

/// One module read: its exports, its re-exports and what the generator cannot read in it.
///
/// `file` names it in the pages and the problems: its path relative to the entry's folder.
#[must_use]
pub fn module(source: &str, file: &str) -> Module {
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, source, SourceType::ts()).parse();
    let mut module = Module::default();

    if let Some(error) = parsed.diagnostics.errors().next() {
        module
            .problems
            .push(format!("{file}: cannot be parsed: {error}"));
        return module;
    }

    let program = parsed.program;
    let semantic = SemanticBuilder::new().build(&program).semantic;
    let locals = program
        .body
        .iter()
        .filter_map(|statement| match statement {
            Statement::TSTypeAliasDeclaration(alias) if alias.type_parameters.is_none() => {
                Some((alias.id.name.as_str(), &alias.type_annotation))
            }
            _ => None,
        })
        .collect();
    let mut reader = Reader {
        source,
        comments: &program.comments,
        scoping: semantic.scoping(),
        file,
        locals,
        expanding: Vec::new(),
        problems: Vec::new(),
    };

    for statement in &program.body {
        match statement {
            Statement::ExportAllDeclaration(all) if all.exported.is_none() => {
                module.re_exports.push(all.source.value.to_string());
            }
            Statement::ExportDeclaration(export) => {
                let doc = reader.doc(export.span.start);
                read_declaration(&mut reader, &mut module, &export.declaration, doc);
            }
            Statement::ExportAllDeclaration(_)
            | Statement::ExportNamedDeclaration(_)
            | Statement::ExportFromDeclaration(_)
            | Statement::ExportDefaultDeclaration(_)
            | Statement::TSExportAssignment(_) => {
                reader.problem(
                    statement.span().start,
                    "the generator reads `export` declarations and `export * from` only",
                );
            }
            _ => {}
        }
    }

    module.problems.append(&mut reader.problems);
    module
}

/// The exports of one `export` declaration.
fn read_declaration(
    reader: &mut Reader,
    module: &mut Module,
    declaration: &Declaration,
    doc: Option<Doc>,
) {
    let file = reader.file.to_owned();

    match declaration {
        Declaration::FunctionDeclaration(function) => {
            let Some(id) = &function.id else { return };
            module.exports.push(Export {
                name: id.name.to_string(),
                kind: Kind::Function,
                file,
                line: reader.line(id.span.start),
                doc,
                body: Body::Function(reader.signature(function)),
            });
        }
        Declaration::TSTypeAliasDeclaration(alias) => {
            module.exports.push(Export {
                name: alias.id.name.to_string(),
                kind: Kind::TypeAlias,
                file,
                line: reader.line(alias.id.span.start),
                doc,
                body: Body::TypeAlias {
                    type_parameters: reader.type_parameters(alias.type_parameters.as_deref()),
                    value: reader.ty(&alias.type_annotation),
                },
            });
        }
        Declaration::VariableDeclaration(variable) => {
            for declarator in &variable.declarations {
                let name = binding_name(&declarator.id);
                let (value, literal, returns_of) = reader.constant(declarator);
                if let Some(function) = returns_of {
                    module.returns_of.push((name.clone(), function));
                }
                module.exports.push(Export {
                    name,
                    kind: Kind::Variable,
                    file: file.clone(),
                    line: reader.line(declarator.span.start),
                    doc: doc.clone(),
                    body: Body::Variable { value, literal },
                });
            }
        }
        other => reader.problem(
            other.span().start,
            "the generator reads exported functions, type aliases and constants only",
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn only(source: &str) -> Export {
        let module = module(source, "a.ts");
        assert!(module.problems.is_empty(), "{:?}", module.problems);
        assert_eq!(module.exports.len(), 1);
        module.exports.into_iter().next().unwrap()
    }

    fn reference(name: &str, target: Target) -> Type {
        Type::Reference {
            name: name.to_owned(),
            target,
            arguments: Vec::new(),
        }
    }

    #[test]
    fn reads_a_function_its_line_and_its_doc() {
        let export = only(
            "import type { Point } from \"./point\";\n\n/**\n * Moved.\n *\n * @param point - start\n * @returns moved\n */\nexport function add(point: Point, by: number): Point {\n  return point;\n}\n",
        );

        assert_eq!(export.name, "add");
        assert_eq!(export.kind, Kind::Function);
        assert_eq!(export.line, 9);
        assert_eq!(export.doc.as_ref().unwrap().summary, "Moved.");
        let Body::Function(signature) = export.body else {
            panic!()
        };
        assert_eq!(signature.parameters.len(), 2);
        assert_eq!(
            signature.parameters[0].value,
            reference("Point", Target::Module)
        );
        assert_eq!(
            signature.parameters[1].value,
            Type::Intrinsic("number".to_owned())
        );
        assert_eq!(signature.returns, reference("Point", Target::Module));
    }

    #[test]
    fn tells_type_parameters_imports_and_globals_apart() {
        let export = only(
            "/** S. */\nexport function scan<S extends ArrayBufferLike>(items: readonly (readonly S[])[], step: (state: S) => S): Readonly<Uint8Array<S>> | undefined {\n  return undefined;\n}\n",
        );
        let Body::Function(signature) = export.body else {
            panic!()
        };

        assert_eq!(
            signature.type_parameters[0].constraint,
            Some(reference("ArrayBufferLike", Target::Global))
        );
        assert_eq!(
            signature.parameters[0].value,
            Type::Operator(
                "readonly",
                Box::new(Type::Array(Box::new(Type::Operator(
                    "readonly",
                    Box::new(Type::Array(Box::new(reference("S", Target::TypeParameter))))
                ))))
            )
        );
        let Type::Function(step) = &signature.parameters[1].value else {
            panic!()
        };
        assert_eq!(step.parameters[0].name, "state");
        let Type::Union(types) = &signature.returns else {
            panic!()
        };
        assert_eq!(types[1], Type::Intrinsic("undefined".to_owned()));
    }

    #[test]
    fn reads_a_type_alias_with_its_members_and_their_docs() {
        let export = only(
            "import type { Corner } from \"./corner\";\n/** A corner. */\nexport type Curve<T> = Corner & {\n  /** Handles. */\n  readonly handles: T;\n  size?: [number, \"a\", -1, true];\n};\n",
        );
        let Body::TypeAlias {
            type_parameters,
            value: Type::Intersection(parts),
        } = export.body
        else {
            panic!()
        };

        assert_eq!(type_parameters[0].name, "T");
        let Type::Object(members) = &parts[1] else {
            panic!()
        };
        assert_eq!(members[0].name, "handles");
        assert!(members[0].readonly);
        assert_eq!(members[0].line, 5);
        assert_eq!(members[0].doc.as_ref().unwrap().summary, "Handles.");
        assert!(members[1].optional);
        assert!(members[1].doc.is_none());
        assert_eq!(
            members[1].value,
            Type::Tuple(vec![
                Type::Intrinsic("number".to_owned()),
                Type::Literal("\"a\"".to_owned()),
                Type::Literal("-1".to_owned()),
                Type::Literal("true".to_owned()),
            ])
        );
    }

    #[test]
    fn reads_a_type_query_an_indexed_access_and_keyof() {
        let export = only(
            "import type { A } from \"./a\";\n/** B. */\nexport type B = (typeof A)[number] | keyof A | A.b;\n",
        );
        let Body::TypeAlias {
            value: Type::Union(types),
            ..
        } = export.body
        else {
            panic!()
        };

        assert_eq!(
            types[0],
            Type::IndexedAccess(
                Box::new(Type::Query {
                    name: "A".to_owned(),
                    target: Target::Module
                }),
                Box::new(Type::Intrinsic("number".to_owned()))
            )
        );
        assert_eq!(
            types[1],
            Type::Operator("keyof", Box::new(reference("A", Target::Module)))
        );
        assert_eq!(types[2], reference("A.b", Target::Module));
    }

    #[test]
    fn reads_the_constants_of_literal_values() {
        let module = module(
            "/** A. */\nexport const A = 65_536;\n/** B. */\nexport const B = \"x\";\n/** C. */\nexport const C = [\"min\", 2] as const;\n/** D. */\nexport const D = A / 2;\n/** E. */\nexport const E = half(A);\n/** F. */\nexport const F: number = 1, G = false;\n",
            "a.ts",
        );
        assert!(module.problems.is_empty(), "{:?}", module.problems);
        let bodies: Vec<_> = module
            .exports
            .iter()
            .map(|each| each.body.clone())
            .collect();

        assert_eq!(
            bodies[0],
            Body::Variable {
                value: Type::Literal("65536".to_owned()),
                literal: Some("65_536".to_owned())
            }
        );
        assert_eq!(
            bodies[1],
            Body::Variable {
                value: Type::Literal("\"x\"".to_owned()),
                literal: Some("\"x\"".to_owned())
            }
        );
        assert_eq!(
            bodies[2],
            Body::Variable {
                value: Type::Operator(
                    "readonly",
                    Box::new(Type::Tuple(vec![
                        Type::Literal("\"min\"".to_owned()),
                        Type::Literal("2".to_owned())
                    ]))
                ),
                literal: None
            }
        );
        assert_eq!(
            bodies[3],
            Body::Variable {
                value: Type::Intrinsic("number".to_owned()),
                literal: None
            }
        );
        assert_eq!(module.returns_of, vec![("E".to_owned(), "half".to_owned())]);
        assert_eq!(
            bodies[6],
            Body::Variable {
                value: Type::Literal("false".to_owned()),
                literal: Some("false".to_owned())
            }
        );
        assert_eq!(module.exports[6].doc.as_ref().unwrap().summary, "F.");
    }

    #[test]
    fn names_what_it_cannot_read() {
        let module = module(
            "export const A = { x: 1 };\nexport interface I { x: number }\nexport { B } from \"./b\";\nexport function f(x) { return x; }\nexport type T = `a${string}`;\nexport * as all from \"./c\";\n",
            "a.ts",
        );

        assert_eq!(
            module.problems,
            vec![
                "a.ts:1: the generator cannot read this constant's type: write its type annotation",
                "a.ts:2: the generator reads exported functions, type aliases and constants only",
                "a.ts:3: the generator reads `export` declarations and `export * from` only",
                "a.ts:4: the generator cannot read a function without its return type",
                "a.ts:4: the generator cannot read a parameter without its type",
                "a.ts:5: the generator cannot read this kind of type",
                "a.ts:6: the generator reads `export` declarations and `export * from` only",
            ]
        );
    }

    #[test]
    fn names_a_file_it_cannot_parse() {
        let module = module("export function (", "a.ts");

        assert_eq!(module.problems.len(), 1);
        assert!(module.problems[0].starts_with("a.ts: cannot be parsed"));
    }

    #[test]
    fn collects_the_re_exports() {
        let module = module(
            "export * from \"./a\";\nexport type * from \"./b\";\n",
            "index.ts",
        );

        assert_eq!(module.re_exports, vec!["./a", "./b"]);
        assert_eq!(module.exports, Vec::new());
    }

    #[test]
    fn prints_numbers_as_javascript_does() {
        assert_eq!(number_text(360.0), "360");
        assert_eq!(number_text(1e-9), "1e-9");
        assert_eq!(number_text(0.5), "0.5");
        assert_eq!(number_text(1e21), "1e+21");
        assert_eq!(number_text(0.0), "0");
    }
}
