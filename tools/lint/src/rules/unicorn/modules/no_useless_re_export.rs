//! `unicorn/no-useless-re-export`: a name re-exported from a module that an `export *` of the
//! file already exports whole (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/no-useless-re-export.md`). Modules are compared as written, with their
//! attributes; a file with `export *` from two modules, a renamed export and `default` are left,
//! and a value export is not covered by an `export type *`. No fix.

use oxc_ast::ast::{
    ExportAllDeclaration, ImportOrExportKind, ModuleExportName, Statement, WithClause,
};

use oxc_span::GetSpan;

use crate::expressions::text;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoUselessReExport;

/// The rule's message.
const MESSAGE: &str =
    "This export is redundant because the same module is already exported with `export *`.";

/// A name as written, an identifier or a string.
fn name_of<'a>(name: &ModuleExportName<'a>) -> &'a str {
    match name {
        ModuleExportName::IdentifierName(name) => name.name.as_str(),
        ModuleExportName::IdentifierReference(name) => name.name.as_str(),
        ModuleExportName::StringLiteral(name) => name.value.as_str(),
    }
}

/// The module a declaration names, with its attributes as written.
fn request(source: &Source, module: &str, with: Option<&WithClause>) -> String {
    let attributes = with.map_or_else(String::new, |clause| {
        clause
            .with_entries
            .iter()
            .map(|entry| text(source, entry.span))
            .collect::<Vec<_>>()
            .join("\0")
    });
    format!("{module}\0{attributes}")
}

impl Rule for NoUselessReExport {
    fn name(&self) -> &'static str {
        "unicorn/no-useless-re-export"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let wildcards: Vec<(&ExportAllDeclaration, String)> = source
            .program
            .body
            .iter()
            .filter_map(|statement| match statement {
                Statement::ExportAllDeclaration(export) if export.exported.is_none() => Some((
                    &**export,
                    request(source, &export.source.value, export.with_clause.as_deref()),
                )),
                _ => None,
            })
            .collect();
        let Some((_, first)) = wildcards.first() else {
            return;
        };
        if wildcards.iter().any(|(_, each)| each != first) {
            return;
        }
        for statement in &source.program.body {
            let Statement::ExportFromDeclaration(export) = statement else {
                continue;
            };
            let wanted = request(source, &export.source.value, export.with_clause.as_deref());
            for specifier in &export.specifiers {
                let exported = name_of(&specifier.exported);
                let renamed = specifier.local.span() != specifier.exported.span();
                if exported == "default" || name_of(&specifier.local) != exported || renamed {
                    continue;
                }
                let typed = specifier.export_kind == ImportOrExportKind::Type
                    || export.export_kind == ImportOrExportKind::Type;
                let covered = wildcards.iter().any(|(wildcard, each)| {
                    *each == wanted && (wildcard.export_kind != ImportOrExportKind::Type || typed)
                });
                if covered {
                    findings.push(source.finding(
                        self.name(),
                        specifier.span.start,
                        MESSAGE.to_owned(),
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoUselessReExport;
    use crate::testing::lint;

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&NoUselessReExport, "src/a/index.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_a_name_the_wildcard_already_exports() {
        // The documentation's case, then a type export covered by a value or a type wildcard.
        assert_eq!(
            places("export * from './foo.js';\nexport {foo} from './foo.js';"),
            [(2, 9)]
        );
        assert_eq!(
            places("export * from './foo.js';\nexport type {Foo} from './foo.js';"),
            [(2, 14)]
        );
        assert_eq!(
            places("export type * from './foo.js';\nexport {type Foo, bar} from './foo.js';"),
            [(2, 9)]
        );
        assert_eq!(
            places(
                "export * from './foo.js';\nexport * from './foo.js';\nexport {'a'} from './foo.js';"
            ),
            [(3, 9)]
        );
    }

    #[test]
    fn leaves_renames_defaults_other_modules_and_two_wildcards() {
        for code in [
            "export * from './foo.js';",
            "export * from './foo.js';\nexport {foo as bar} from './foo.js';",
            "export * from './foo.js';\nexport {default} from './foo.js';",
            "export * from './foo.js';\nexport {foo as foo} from './foo.js';",
            "export * from './foo.js';\nexport {foo} from './bar.js';",
            "export * from './foo.js';\nexport * from './bar.js';\nexport {foo} from './foo.js';",
            "export * as all from './foo.js';\nexport {foo} from './foo.js';",
            "export type * from './foo.js';\nexport {foo} from './foo.js';",
            "export * from './foo.js' with { type: 'json' };\nexport {foo} from './foo.js';",
            "import {foo} from './foo.js';\nexport * from './foo.js';\nexport {foo};",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
