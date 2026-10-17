//! `local/folder-has-index`: every folder holding source files has its barrel, `index.ts`
//! (ADR-0012); the root, the site's theme and the repository's tests are exempt.

use crate::finding::Finding;
use crate::rules::Rule;
use crate::settings::FOLDERS_WITHOUT_INDEX;
use crate::source::Source;

/// The rule.
pub struct FolderHasIndex;

impl Rule for FolderHasIndex {
    fn name(&self) -> &'static str {
        "local/folder-has-index"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let path = source
            .path
            .rsplit_once('/')
            .map_or(".", |(folder, _)| folder);
        if !FOLDERS_WITHOUT_INDEX.contains(&path) && !source.project.has_index(&source.folder()) {
            findings.push(source.finding(
                self.name(),
                0,
                format!("Folder `{path}` has no `index.ts`: create its barrel."),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FolderHasIndex;
    use crate::testing::{NONE, fixture, lint_at, messages, write_files};

    #[test]
    fn reports_a_folder_without_barrel_unless_exempt() {
        let root = fixture("folder-index");
        write_files(
            &root,
            &["with/index.ts", "without/a.ts", "tests/a.ts", "a.ts"],
        );
        let found = |path: &str| messages(&lint_at(&FolderHasIndex, &root, path, "export {};"));

        assert_eq!(
            found("without/a.ts"),
            ["Folder `without` has no `index.ts`: create its barrel."]
        );
        assert_eq!(found("with/a.ts"), NONE);
        assert_eq!(found("tests/a.ts"), NONE);
        assert_eq!(found("a.ts"), NONE);
    }
}
