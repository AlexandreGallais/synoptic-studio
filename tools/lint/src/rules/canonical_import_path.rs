//! `local/canonical-import-path`: a relative import takes the shortest legal path (ADR-0012) —
//! the file in its own folder (`./vector`), another folder through its barrel (`../geometry`),
//! never `index` nor `.ts`; importing the barrel of its own folder or of an ancestor is a cycle.
//! Autofixed.

use std::path::{Component, Path, PathBuf};

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, StringLiteral};
use oxc_semantic::AstNode;

use crate::finding::{Finding, Fix};
use crate::project::Project;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct CanonicalImportPath;

/// What a relative specifier should be.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Canonical {
    /// The shortest legal specifier.
    Specifier(String),
    /// It names the barrel of the importer's folder or of an ancestor.
    AncestorBarrel,
    /// It names nothing on the disk.
    Unresolved,
}

/// A path with its `.` and `..` folded, without reading the disk.
fn normalized(path: &Path) -> PathBuf {
    let mut parts: Vec<Component> = Vec::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir if matches!(parts.last(), Some(Component::Normal(_))) => {
                parts.pop();
            }
            Component::ParentDir if matches!(parts.last(), Some(Component::RootDir)) => {}
            other => parts.push(other),
        }
    }
    parts.iter().collect()
}

/// The module a relative specifier names: a source file, or a folder with a barrel.
fn module_of(project: &Project, folder: &Path, specifier: &str) -> Option<PathBuf> {
    let target = normalized(&folder.join(specifier.strip_suffix(".ts").unwrap_or(specifier)));
    let is_index = target.file_name().is_some_and(|name| name == "index");
    let mut file = target.clone().into_os_string();
    file.push(".ts");
    if !is_index && project.is_file(Path::new(&file)) {
        return Some(target);
    }
    let as_folder = if is_index {
        target
            .parent()
            .map_or_else(|| target.clone(), Path::to_path_buf)
    } else {
        target
    };
    project.has_index(&as_folder).then_some(as_folder)
}

/// The shortest legal specifier for a relative import: the child of the common ancestor that
/// holds the target — the target file itself when it sits in that ancestor, its folder through
/// the barrel otherwise.
#[must_use]
pub fn canonical(project: &Project, folder: &Path, specifier: &str) -> Canonical {
    let Some(target) = module_of(project, folder, specifier) else {
        return Canonical::Unresolved;
    };
    let from: Vec<Component> = folder.components().collect();
    let to: Vec<Component> = target.components().collect();
    let common = from
        .iter()
        .zip(&to)
        .take_while(|(left, right)| left == right)
        .count();
    let levels = from.len() - common;

    match to.get(common) {
        None => Canonical::AncestorBarrel,
        Some(entry) => {
            let prefix = if levels == 0 {
                "./".to_owned()
            } else {
                "../".repeat(levels)
            };
            Canonical::Specifier(format!("{prefix}{}", entry.as_os_str().to_string_lossy()))
        }
    }
}

impl CanonicalImportPath {
    /// Checks a module specifier written as a string.
    fn check(&self, literal: &StringLiteral, source: &Source, findings: &mut Vec<Finding>) {
        let Some(value) = literal.value.as_str() else {
            return;
        };
        if !value.starts_with('.') {
            return;
        }
        match canonical(source.project, &source.folder(), value) {
            Canonical::AncestorBarrel => findings.push(source.finding(
                self.name(),
                literal.span.start,
                "Importing the barrel of this folder or of an ancestor creates a cycle: import the file."
                    .to_owned(),
            )),
            Canonical::Specifier(expected) if expected != value => {
                let quote = &source.text[literal.span.start as usize..=literal.span.start as usize];
                let fix = Fix {
                    start: literal.span.start,
                    end: literal.span.end,
                    text: format!("{quote}{expected}{quote}"),
                };
                let message = format!("Use the canonical import path `{expected}`.");
                findings.push(source.fixed(self.name(), literal.span.start, message, fix));
            }
            Canonical::Specifier(_) | Canonical::Unresolved => {}
        }
    }
}

impl Rule for CanonicalImportPath {
    fn name(&self) -> &'static str {
        "local/canonical-import-path"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let literal = match node.kind() {
            AstKind::ImportDeclaration(import) => &import.source,
            AstKind::ExportAllDeclaration(export) => &export.source,
            AstKind::ExportFromDeclaration(export) => &export.source,
            AstKind::ImportExpression(import) => match &import.source {
                Expression::StringLiteral(literal) => literal,
                _ => return,
            },
            _ => return,
        };
        self.check(literal, source, findings);
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use std::path::Path;

    use super::{CanonicalImportPath, normalized};
    use crate::testing::{NONE, fixed, fixture, lint_at, messages, write_files};

    /// Two layers, one with a sub-folder; every folder has a barrel.
    fn tree() -> PathBuf {
        let root = fixture("canonical-import");
        write_files(
            &root,
            &[
                "src/index.ts",
                "src/geometry/index.ts",
                "src/geometry/vector.ts",
                "src/geometry/arc/index.ts",
                "src/geometry/arc/fillet.ts",
                "src/model/index.ts",
                "src/model/shape.ts",
                "src/model/port.ts",
            ],
        );
        root
    }

    /// The importer in the `model` layer.
    const SHAPE: &str = "src/model/shape.ts";
    /// The importer at the root of the `geometry` layer.
    const GEOMETRY: &str = "src/geometry/vector.ts";
    /// The importer in a sub-folder of the `geometry` layer.
    const FILLET: &str = "src/geometry/arc/fillet.ts";

    /// The code fixed, after the one expected finding.
    fn fixes(path: &str, code: &str, message: &str) -> String {
        let findings = lint_at(&CanonicalImportPath, &tree(), path, code);
        assert_eq!(messages(&findings), [message], "{code}");
        fixed(code, &findings)
    }

    #[test]
    fn folds_dots_without_climbing_above_the_root() {
        assert_eq!(normalized(Path::new("/a/./b/../../../c")), Path::new("/c"));
        assert_eq!(
            normalized(Path::new("../../a/./b/..")),
            Path::new("../../a")
        );
    }

    #[test]
    fn drops_the_extension_and_the_index() {
        assert_eq!(
            fixes(
                SHAPE,
                "import \"./port.ts\";",
                "Use the canonical import path `./port`."
            ),
            "import \"./port\";"
        );
        assert_eq!(
            fixes(
                SHAPE,
                "import '../geometry/index';",
                "Use the canonical import path `../geometry`."
            ),
            "import '../geometry';"
        );
    }

    #[test]
    fn imports_another_folder_through_its_barrel() {
        assert_eq!(
            fixes(
                SHAPE,
                "import \"../geometry/arc/fillet\";",
                "Use the canonical import path `../geometry`."
            ),
            "import \"../geometry\";"
        );
        assert_eq!(
            fixes(
                GEOMETRY,
                "export * from \"./arc/fillet\";",
                "Use the canonical import path `./arc`."
            ),
            "export * from \"./arc\";"
        );
    }

    #[test]
    fn checks_named_reexports_and_dynamic_imports() {
        let code = "export { a } from \"./arc/fillet\";\nconst b = import(\"./arc/fillet\");";
        let findings = lint_at(&CanonicalImportPath, &tree(), GEOMETRY, code);

        assert_eq!(findings.len(), 2);
        assert_eq!(
            fixed(code, &findings),
            "export { a } from \"./arc\";\nconst b = import(\"./arc\");"
        );
    }

    #[test]
    fn reports_the_barrel_of_an_ancestor_or_of_its_own_folder() {
        for code in ["import \"../index\";", "import \".\";"] {
            assert_eq!(
                messages(&lint_at(&CanonicalImportPath, &tree(), GEOMETRY, code)),
                [
                    "Importing the barrel of this folder or of an ancestor creates a cycle: import the file."
                ]
            );
        }
    }

    #[test]
    fn accepts_canonical_paths_packages_and_what_it_cannot_resolve() {
        let cases = [
            (SHAPE, "import \"./port\";"),
            (SHAPE, "import \"../geometry\";"),
            (GEOMETRY, "import \"./arc\";"),
            (FILLET, "import \"../vector\";"),
            (SHAPE, "import \"node:fs\";"),
            (SHAPE, "import \"./missing\";"),
            (SHAPE, "const a = import(`./port.ts`);"),
        ];
        for (path, code) in cases {
            assert_eq!(
                messages(&lint_at(&CanonicalImportPath, &tree(), path, code)),
                NONE,
                "{code}"
            );
        }
    }
}
