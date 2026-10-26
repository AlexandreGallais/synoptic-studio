//! `unicorn/no-top-level-side-effects`: in a module with a runtime export, a top-level expression
//! statement or a default-exported expression that may change something when the module is
//! imported (`eslint-plugin-unicorn` 78.0.0, `docs/rules/no-top-level-side-effects.md`).
//! Assignments are allowed, and so are the configuration helpers its documentation names
//! (`defineConfig` of `vite` among them), their arguments checked. Files with a shebang or only
//! type exports are left, and so are the files where `ESLint` turned it off: the tools, the tests
//! (`RELAXED_PURITY`) and the playground's entry. No fix.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Class, Declaration, ExportDefaultDeclarationKind, ExportSpecifier, Expression,
    ImportDeclarationSpecifier, ImportOrExportKind, ModuleExportName, Statement, UnaryOperator,
};
use oxc_ast_visit::{Visit, walk};
use oxc_semantic::ScopeFlags;
use oxc_span::GetSpan;

use crate::finding::Finding;
use crate::glob::matches_any;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{symbol_of, unwrap};
use crate::scopes::SCOPES;
use crate::source::Source;

/// The rule.
pub struct NoTopLevelSideEffects;

/// The rule's message.
const MESSAGE: &str = "Do not use top-level side effects in exported modules.";

/// The functions known to change nothing, by the module that exports them.
const PURE_EXPORTS: [(&str, &[&str]); 8] = [
    (
        "react",
        &[
            "memo",
            "forwardRef",
            "lazy",
            "createContext",
            "createRef",
            "createElement",
            "cloneElement",
            "isValidElement",
        ],
    ),
    ("vue", &["defineComponent"]),
    ("eslint/config", &["defineConfig"]),
    ("@eslint/config-helpers", &["defineConfig"]),
    ("vite", &["defineConfig"]),
    ("vitest/config", &["defineConfig", "defineProject"]),
    ("rollup", &["defineConfig"]),
    ("astro/config", &["defineConfig"]),
];

/// Whether the rule is off for a path, as `ESLint`'s scopes turned it off.
fn is_off(path: &str) -> bool {
    SCOPES
        .iter()
        .filter(|scope| matches!(scope.name, "tooling" | "tests"))
        .any(|scope| matches_any(scope.files, path))
}

/// Whether a statement exports something that remains at run time.
fn is_runtime_export(statement: &Statement) -> bool {
    match statement {
        Statement::ExportAllDeclaration(export) => export.export_kind != ImportOrExportKind::Type,
        Statement::ExportDefaultDeclaration(export) => !matches!(
            export.declaration,
            ExportDefaultDeclarationKind::TSInterfaceDeclaration(_)
        ),
        Statement::ExportDeclaration(export) => {
            !matches!(
                export.declaration,
                Declaration::TSTypeAliasDeclaration(_) | Declaration::TSInterfaceDeclaration(_)
            ) && !export.declaration.declare()
        }
        Statement::ExportNamedDeclaration(export) => {
            export.export_kind != ImportOrExportKind::Type && !all_types(&export.specifiers)
        }
        Statement::ExportFromDeclaration(export) => {
            export.export_kind != ImportOrExportKind::Type && !all_types(&export.specifiers)
        }
        _ => false,
    }
}

/// Whether specifiers are there and each exports a type, `export { type A }`.
fn all_types(specifiers: &[ExportSpecifier]) -> bool {
    !specifiers.is_empty()
        && specifiers
            .iter()
            .all(|each| each.export_kind == ImportOrExportKind::Type)
}

/// The search for a call, an assignment, an update, a `delete`, a `new`, an import, an `await`
/// or a `yield`, outside the functions an expression holds (`hasSideEffect` of
/// `@eslint-community/eslint-utils`, without its options).
#[derive(Default)]
struct Effects {
    /// Whether one was found.
    found: bool,
}

impl<'a> Visit<'a> for Effects {
    fn visit_expression(&mut self, expression: &Expression<'a>) {
        match expression {
            Expression::CallExpression(_)
            | Expression::NewExpression(_)
            | Expression::AssignmentExpression(_)
            | Expression::UpdateExpression(_)
            | Expression::ImportExpression(_)
            | Expression::AwaitExpression(_)
            | Expression::YieldExpression(_) => self.found = true,
            Expression::UnaryExpression(unary) if unary.operator == UnaryOperator::Delete => {
                self.found = true;
            }
            _ => walk::walk_expression(self, expression),
        }
    }

    fn visit_call_expression(&mut self, _: &oxc_ast::ast::CallExpression<'a>) {
        self.found = true;
    }

    fn visit_function(&mut self, _: &oxc_ast::ast::Function<'a>, _: ScopeFlags) {}

    fn visit_arrow_function_expression(&mut self, _: &oxc_ast::ast::ArrowFunctionExpression<'a>) {}
}

/// Whether an expression may change something, outside the functions it holds.
fn changes(expression: &Expression) -> bool {
    let mut effects = Effects::default();
    effects.visit_expression(expression);
    effects.found
}

/// Whether a call is of a function a module named in `PURE_EXPORTS` exports, imported directly.
fn is_pure_call(source: &Source, expression: &Expression) -> bool {
    let Expression::CallExpression(call) = expression else {
        return false;
    };
    if call.optional {
        return false;
    }
    let (reference, method) = match unwrap(&call.callee) {
        Expression::StaticMemberExpression(member) if !member.optional => {
            (unwrap(&member.object), Some(member.property.name.as_str()))
        }
        Expression::Identifier(_) => (unwrap(&call.callee), None),
        _ => return false,
    };
    let Expression::Identifier(reference) = reference else {
        return false;
    };
    let Some(symbol) = symbol_of(source, reference) else {
        return false;
    };
    let nodes = source.semantic.nodes();
    let declaration = source.semantic.scoping().symbol_declaration(symbol);
    let AstKind::ImportDeclaration(import) = nodes.parent_kind(declaration) else {
        return false;
    };
    let Some(names) = PURE_EXPORTS
        .iter()
        .find(|(module, _)| import.source.value == *module)
        .map(|(_, names)| *names)
    else {
        return false;
    };
    if import.import_kind == ImportOrExportKind::Type {
        return false;
    }
    let specifier = import.specifiers.iter().flatten().find(|each| {
        let local = match each {
            ImportDeclarationSpecifier::ImportSpecifier(specifier) => &specifier.local,
            ImportDeclarationSpecifier::ImportDefaultSpecifier(specifier) => &specifier.local,
            ImportDeclarationSpecifier::ImportNamespaceSpecifier(specifier) => &specifier.local,
        };
        local.symbol_id.get() == Some(symbol)
    });
    match (specifier, method) {
        (
            Some(
                ImportDeclarationSpecifier::ImportDefaultSpecifier(_)
                | ImportDeclarationSpecifier::ImportNamespaceSpecifier(_),
            ),
            Some(method),
        ) => names.contains(&method),
        (Some(ImportDeclarationSpecifier::ImportDefaultSpecifier(_)), None) => {
            names.contains(&"default")
        }
        (Some(ImportDeclarationSpecifier::ImportSpecifier(specifier)), None) => {
            specifier.import_kind != ImportOrExportKind::Type
                && match &specifier.imported {
                    ModuleExportName::IdentifierName(name) => names.contains(&name.name.as_str()),
                    ModuleExportName::StringLiteral(name) => name
                        .value
                        .as_str()
                        .is_some_and(|name| names.contains(&name)),
                    ModuleExportName::IdentifierReference(_) => false,
                }
        }
        _ => false,
    }
}

/// Whether an expression may change something, a pure helper's call judged by its arguments.
fn has_effect(source: &Source, expression: &Expression) -> bool {
    let expression = unwrap(expression);
    if let Expression::CallExpression(call) = expression
        && is_pure_call(source, expression)
    {
        return call.arguments.iter().any(|argument| {
            argument
                .as_expression()
                .is_none_or(|argument| has_effect(source, argument))
        });
    }
    matches!(expression, Expression::TaggedTemplateExpression(_)) || changes(expression)
}

/// Whether a class may change something when it is evaluated: only its parent class is read.
fn class_has_effect(source: &Source, class: &Class) -> bool {
    class
        .heritage
        .as_ref()
        .is_some_and(|parent| top_level_effect(source, &parent.expression))
}

/// Whether a top-level expression may change something.
fn top_level_effect(source: &Source, expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::ClassExpression(class) => class_has_effect(source, class),
        other => has_effect(source, other),
    }
}

impl Rule for NoTopLevelSideEffects {
    fn name(&self) -> &'static str {
        "unicorn/no-top-level-side-effects"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let body = &source.program.body;
        if is_off(source.path)
            || source.text.starts_with("#!")
            || !body.iter().any(is_runtime_export)
        {
            return;
        }
        for statement in body {
            let reported = match statement {
                Statement::ExpressionStatement(statement) => {
                    let expression = &statement.expression;
                    (!matches!(unwrap(expression), Expression::AssignmentExpression(_))
                        && top_level_effect(source, expression))
                    .then_some(statement.span.start)
                }
                Statement::ExportDefaultDeclaration(export) => {
                    let declaration = &export.declaration;
                    declaration.as_expression().and_then(|expression| {
                        (!matches!(unwrap(expression), Expression::AssignmentExpression(_))
                            && top_level_effect(source, expression))
                        .then(|| declaration.span().start)
                    })
                }
                _ => None,
            };
            if let Some(at) = reported {
                findings.push(source.finding(self.name(), at, MESSAGE.to_owned()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoTopLevelSideEffects;
    use crate::testing::lint;

    /// The places of the findings in a module of the library.
    fn places(code: &str) -> Vec<(usize, usize)> {
        places_at("src/a/b.ts", code)
    }

    /// The places of the findings in a file at a path.
    fn places_at(path: &str, code: &str) -> Vec<(usize, usize)> {
        lint(&NoTopLevelSideEffects, path, code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_effects_at_the_top_of_an_exporting_module() {
        // The documentation's cases.
        assert_eq!(places("export {};\ninit();"), [(2, 1)]);
        assert_eq!(places("export {};\nnew App();"), [(2, 1)]);
        assert_eq!(
            places("import React from 'react';\nexport default React.memo<Props>(initialize());"),
            [(2, 16)]
        );
        // Other effects the documentation names.
        assert_eq!(places("export const a = 1;\ncounter++;"), [(2, 1)]);
        assert_eq!(
            places("export const a = 1;\ndelete object.property;"),
            [(2, 1)]
        );
        assert_eq!(places("export const a = 1;\ntag`x`;"), [(2, 1)]);
        assert_eq!(places("export const a = 1;\nawait a;"), [(2, 1)]);
        assert_eq!(places("export const a = 1;\na?.b();"), [(2, 1)]);
        assert_eq!(places("export default init();"), [(1, 16)]);
        assert_eq!(
            places("export * from './a';\n(class extends f() {});"),
            [(2, 1)]
        );
        assert_eq!(
            places("import { defineConfig } from 'vite';\nexport default defineConfig(read());"),
            [(2, 16)]
        );
    }

    #[test]
    fn leaves_declarations_assignments_pure_helpers_and_modules_without_exports() {
        for code in [
            "export {};\nfunction init() {\n  document.title = 'gone';\n}",
            "new App();",
            "export {};\nconst response = fetch();",
            "export {};\ndocument.title = 'gone';",
            "import React from 'react';\nexport default React.memo<Props>(Link);",
            "import { defineConfig } from 'vite';\nexport default defineConfig({ a: 1 });",
            "import { memo as m } from 'react';\nexport default m(() => f());",
            "export type A = string;\ninit();",
            "export interface A {}\ninit();",
            "export declare const a: number;\ninit();",
            "export { type A } from './a';\ninit();",
            "#!/usr/bin/env node\nexport {};\ninit();",
            "export const a = 1;\n() => f();",
            "export const a = 1;\n(class {});",
            "export const a = 1;\n[a, { b: 1 }];",
            "export default function f() { g(); }",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
        for code in [
            "import { defineConfig } from 'other';\nexport default defineConfig({});",
            "import type { defineConfig } from 'vite';\nexport default defineConfig({});",
            "const defineConfig = f;\nexport default defineConfig({});",
            "import * as React from 'react';\nexport default React['memo'](a);",
        ] {
            assert_eq!(places(code).len(), 1, "{code}");
        }
    }

    #[test]
    fn stays_off_where_eslint_turned_it_off() {
        let code = "export {};\ninit();";

        for path in [
            "scripts/a.ts",
            "src/a/b.test.ts",
            "vite.config.ts",
            "tests/a.ts",
        ] {
            assert_eq!(
                places_at(path, code),
                Vec::<(usize, usize)>::new(),
                "{path}"
            );
        }
        assert_eq!(places_at("playground/a/b.ts", code), [(2, 1)]);
        // The old page's entry is gone (EN-160): the app's `src/main.ts` exports nothing.
        assert_eq!(places_at("playground/index.ts", code), [(2, 1)]);
        assert_eq!(
            places_at("playground/src/main.ts", "await bootstrapApplication(App);"),
            Vec::<(usize, usize)>::new()
        );
    }
}
