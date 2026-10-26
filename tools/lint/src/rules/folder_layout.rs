//! `local/folder-layout`: a code folder holds modules or domain folders, never both — only the
//! barrel sits beside folders — and a folder past the signal of modules asks for a domain to
//! split it, unless it is listed with its reason (ADR-0043).

use crate::finding::Finding;
use crate::rules::Rule;
use crate::settings::{FOLDER_SIGNAL, FOLDERS_PAST_SIGNAL};
use crate::source::Source;

/// The rule.
pub struct FolderLayout;

/// Whether a file name is a module: TypeScript, neither a test nor the barrel.
fn is_module(name: &str) -> bool {
    name.rsplit_once('.')
        .is_some_and(|(_, extension)| extension == "ts")
        && !name.ends_with(".test.ts")
        && name != "index.ts"
}

/// Whether a folder name is a domain: neither hidden (`.cache`) nor a tool's (`__snapshots__`).
fn is_domain(name: &str) -> bool {
    !name.starts_with('.') && !name.starts_with("__")
}

/// The findings of a file's place, with the folders listed past a signal.
fn check(source: &Source, signal: usize, listed: &[&str], rule: &'static str) -> Vec<Finding> {
    let name = source.name();
    let path = source
        .path
        .rsplit_once('/')
        .map_or("", |(folder, _)| folder);
    let entries = source.project.entries(&source.folder());
    let mut found = Vec::new();

    if name != "index.ts"
        && entries
            .iter()
            .any(|entry| entry.is_folder && is_domain(&entry.name))
    {
        found.push(source.finding(
            rule,
            0,
            format!("Folder `{path}` holds domain folders: move `{name}` into one of them."),
        ));
    }
    let modules = entries
        .iter()
        .filter(|entry| entry.is_file && is_module(&entry.name))
        .count();
    if is_module(name) && modules > signal && !listed.contains(&path) {
        found.push(source.finding(
            rule,
            0,
            format!(
                "Folder `{path}` holds {modules} modules, past {signal}: split it by domain, or list it with its reason in FOLDERS_PAST_SIGNAL of tools/lint/src/settings.rs."
            ),
        ));
    }
    found
}

impl Rule for FolderLayout {
    fn name(&self) -> &'static str {
        "local/folder-layout"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        findings.extend(check(
            source,
            FOLDER_SIGNAL,
            &FOLDERS_PAST_SIGNAL,
            self.name(),
        ));
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use oxc_allocator::Allocator;

    use super::{FolderLayout, check};
    use crate::project::Project;
    use crate::rules::Rule;
    use crate::source::Source;
    use crate::testing::{NONE, fixture, lint_at, messages, write_files};

    /// Three modules, a test and a barrel: past a signal of 2.
    const PAST: [&str; 5] = ["m1.ts", "m2.ts", "m3.ts", "m1.test.ts", "index.ts"];

    /// A folder of domains, a mixed one, full ones, listed ones, one at the signal.
    fn tree() -> PathBuf {
        let root = fixture("folder-layout");
        let mut files: Vec<String> = [
            "domains/a/x.ts",
            "domains/index.ts",
            "mixed/a/x.ts",
            "mixed/b.ts",
        ]
        .iter()
        .map(|each| (*each).to_owned())
        .collect();
        for folder in ["full", "listed", "a/listed", "b/listed"] {
            files.extend(PAST.iter().map(|name| format!("{folder}/{name}")));
        }
        // Two modules, at the signal and not past it, once the test and the barrel are left out.
        files.extend(
            [
                "even/m1.ts",
                "even/m2.ts",
                "even/m1.test.ts",
                "even/index.ts",
            ]
            .map(str::to_owned),
        );
        files.extend(
            [
                "snapshots/m1.test.ts",
                "snapshots/__snapshots__/a.snap",
                "snapshots/.cache/a",
            ]
            .map(str::to_owned),
        );
        let names: Vec<&str> = files.iter().map(String::as_str).collect();
        write_files(&root, &names);
        root
    }

    /// The messages on a file of the tree, with a signal of 2 and two folders listed.
    fn found(root: &Path, path: &str) -> Vec<String> {
        let allocator = Allocator::default();
        let parsed =
            oxc_parser::Parser::new(&allocator, "export {};", oxc_span::SourceType::ts()).parse();
        let semantic = oxc_semantic::SemanticBuilder::new()
            .build(&parsed.program)
            .semantic;
        let project = Project::new(root);
        let source = Source::new(path, &parsed.program, &semantic, &[], &project);

        messages(&check(
            &source,
            2,
            &["listed", "a/listed"],
            FolderLayout.name(),
        ))
    }

    #[test]
    fn reports_a_file_beside_domain_folders() {
        let root = tree();

        assert_eq!(
            found(&root, "mixed/b.ts"),
            ["Folder `mixed` holds domain folders: move `b.ts` into one of them."]
        );
        assert_eq!(
            found(&root, "mixed/b.test.ts"),
            ["Folder `mixed` holds domain folders: move `b.test.ts` into one of them."]
        );
    }

    #[test]
    fn reports_a_folder_past_the_signal_unless_listed() {
        let root = tree();
        let past = |path: &str| {
            format!(
                "Folder `{path}` holds 3 modules, past 2: split it by domain, or list it with its reason in FOLDERS_PAST_SIGNAL of tools/lint/src/settings.rs."
            )
        };

        assert_eq!(found(&root, "full/m1.ts"), [past("full")]);
        assert_eq!(found(&root, "b/listed/m1.ts"), [past("b/listed")]);
    }

    #[test]
    fn accepts_barrels_tests_listed_folders_and_tool_folders() {
        let root = tree();
        for path in [
            "domains/index.ts",
            "domains/a/x.ts",
            "full/m1.test.ts",
            "full/index.ts",
            "listed/m1.ts",
            "a/listed/m1.ts",
            "even/m1.ts",
            "snapshots/m1.test.ts",
        ] {
            assert_eq!(found(&root, path), NONE, "{path}");
        }
    }

    #[test]
    fn uses_the_project_signal_and_list() {
        let root = fixture("folder-layout-project");
        let modules: Vec<String> = (0..16)
            .map(|index| format!("playground/testing/m{index}.ts"))
            .collect();
        let names: Vec<&str> = modules.iter().map(String::as_str).collect();
        write_files(&root, &names);

        assert_eq!(
            messages(&lint_at(
                &FolderLayout,
                &root,
                "playground/testing/m1.ts",
                "export {};"
            )),
            NONE
        );
    }
}
