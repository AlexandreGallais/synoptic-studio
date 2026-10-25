//! The module paths a file names, as `import-x`'s `moduleVisitor` reads them with `commonjs` on:
//! an `import`, an `export … from`, an `import()` and a `require()` of a written string; with
//! where each is written and whether it names types only.

use oxc_ast::AstKind;
use oxc_ast::ast::{Argument, Expression, ImportDeclarationSpecifier};

use crate::source::Source;

/// A module path a file names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModuleSource<'a> {
    /// The path as written.
    pub specifier: &'a str,
    /// The start of the string that writes it.
    pub literal: u32,
    /// The start of the statement or call that names it.
    pub node: u32,
    /// Whether it imports or exports types only: `import type`, `export type`, or an import
    /// whose every specifier is a `type` one.
    pub types_only: bool,
}

/// The module paths of a file, in the order of its nodes.
#[must_use]
pub fn module_sources<'a>(source: &Source<'_, 'a>) -> Vec<ModuleSource<'a>> {
    source
        .semantic
        .nodes()
        .iter()
        .filter_map(|node| match node.kind() {
            AstKind::ImportDeclaration(declaration) => {
                let specifiers: &[ImportDeclarationSpecifier] = declaration
                    .specifiers
                    .as_ref()
                    .map_or(&[], |each| each.as_slice());
                let every_type = !specifiers.is_empty()
                    && specifiers.iter().all(|each| {
                        matches!(each, ImportDeclarationSpecifier::ImportSpecifier(specifier) if specifier.import_kind.is_type())
                    });
                Some(ModuleSource {
                    specifier: declaration.source.value.as_str(),
                    literal: declaration.source.span.start,
                    node: declaration.span.start,
                    types_only: declaration.import_kind.is_type() || every_type,
                })
            }
            AstKind::ExportAllDeclaration(declaration) => Some(ModuleSource {
                specifier: declaration.source.value.as_str(),
                literal: declaration.source.span.start,
                node: declaration.span.start,
                types_only: declaration.export_kind.is_type(),
            }),
            AstKind::ExportFromDeclaration(declaration) => Some(ModuleSource {
                specifier: declaration.source.value.as_str(),
                literal: declaration.source.span.start,
                node: declaration.span.start,
                types_only: declaration.export_kind.is_type(),
            }),
            AstKind::ImportExpression(call) => match &call.source {
                Expression::StringLiteral(literal) => Some(ModuleSource {
                    specifier: literal.value.as_str(),
                    literal: literal.span.start,
                    node: call.span.start,
                    types_only: false,
                }),
                _ => None,
            },
            AstKind::CallExpression(call) => match (&call.callee, call.arguments.as_slice()) {
                (Expression::Identifier(callee), [Argument::StringLiteral(literal)])
                    if callee.name == "require" =>
                {
                    Some(ModuleSource {
                        specifier: literal.value.as_str(),
                        literal: literal.span.start,
                        node: call.span.start,
                        types_only: false,
                    })
                }
                _ => None,
            },
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use oxc_allocator::Allocator;
    use oxc_parser::Parser;
    use oxc_semantic::SemanticBuilder;
    use oxc_span::SourceType;

    use super::module_sources;
    use crate::project::Project;
    use crate::source::Source;

    /// The specifiers of a piece of code, each with whether it names types only.
    fn read(code: &str) -> Vec<(String, bool)> {
        let allocator = Allocator::default();
        let program = Parser::new(&allocator, code, SourceType::ts())
            .parse()
            .program;
        let semantic = SemanticBuilder::new()
            .with_build_nodes(true)
            .build(&program)
            .semantic;
        let project = Project::new(Path::new("/project"));
        let source = Source::new("a.ts", &program, &semantic, &[], &project);

        module_sources(&source)
            .iter()
            .map(|each| (each.specifier.to_owned(), each.types_only))
            .collect()
    }

    #[test]
    fn reads_imports_exports_dynamic_imports_and_requires() {
        assert_eq!(
            read(
                "import a from \"a\";\nexport * from \"b\";\nexport { c } from \"c\";\nconst d = import(\"d\");\nconst e = require(\"e\");"
            ),
            [
                ("a".into(), false),
                ("b".into(), false),
                ("c".into(), false),
                ("d".into(), false),
                ("e".into(), false)
            ]
        );
    }

    #[test]
    fn tells_imports_of_types_only() {
        assert_eq!(
            read(
                "import type { A } from \"a\";\nimport { type B } from \"b\";\nimport { type C, d } from \"c\";\nexport type * from \"e\";\nimport \"f\";"
            ),
            [
                ("a".into(), true),
                ("b".into(), true),
                ("c".into(), false),
                ("e".into(), true),
                ("f".into(), false)
            ]
        );
    }

    #[test]
    fn leaves_computed_paths() {
        assert_eq!(
            read("const a = import(name);\nconst b = require(name, 1);"),
            []
        );
    }
}
