//! Which rules run on which file: the rules on for every TypeScript file, then the scopes of
//! `eslint/scopes/` that turn this linter's rules on or off by path, in `ESLint`'s order, the later
//! winning (EN-075); and the paths never linted.

use crate::glob::matches_any;

/// Generated, vendored or copied paths, never linted (`eslint/settings/files.ts`).
pub const IGNORED: [&str; 12] = [
    "dist/**",
    "coverage/**",
    "node_modules/**",
    "docs/api/**",
    "docs/.vitepress/cache/**",
    "docs/.vitepress/dist/**",
    "docs/.vitepress/.temp/**",
    ".claude/worktrees/**",
    "records/**",
    "target/**",
    "tools/*/tests/fixtures/**",
    ".git/**",
];

/// The rules on for every TypeScript file before the scopes.
pub const ON_EVERYWHERE: [&str; 10] = [
    "functional/no-return-void",
    "local/canonical-import-path",
    "local/folder-has-index",
    "local/kind-in-layer",
    "local/kind-limits",
    "local/kind-naming",
    "local/require-kind",
    "local/see-references",
    "no-restricted-syntax",
    "require-atomic-updates",
];

/// A scope: the files it covers, those it leaves out, and the rules it turns on or off.
#[derive(Clone, Copy, Debug)]
pub struct Scope {
    /// Its name in `eslint/scopes/`.
    pub name: &'static str,
    /// The globs of its files.
    pub files: &'static [&'static str],
    /// The globs of the files it leaves out.
    pub ignores: &'static [&'static str],
    /// Each rule and whether the scope turns it on.
    pub rules: &'static [(&'static str, bool)],
}

/// The library and the playground, whose modules export one value each (ADR-0019).
const SINGLE_EXPORT_FILES: &[&str] = &["src/**/*.ts", "playground/**/*.ts"];

/// The rules off in the tests and the tools: the functional core's `no-return-void`, and the
/// rules about function kinds, which belong to the library and the playground.
const RELAXED: &[(&str, bool)] = &[
    ("functional/no-return-void", false),
    ("local/kind-in-layer", false),
    ("local/kind-limits", false),
    ("local/kind-naming", false),
    ("local/require-kind", false),
    ("local/see-references", false),
];

/// The scopes, in the order of `eslint/scopes/all.ts`; only those that set a rule of this linter.
pub const SCOPES: [Scope; 7] = [
    Scope {
        name: "single-export-modules",
        files: SINGLE_EXPORT_FILES,
        ignores: &["**/index.ts", "**/*.test.ts"],
        rules: &[("local/one-export-per-file", true)],
    },
    Scope {
        name: "folder-layout",
        files: SINGLE_EXPORT_FILES,
        ignores: &[],
        rules: &[("local/folder-layout", true)],
    },
    Scope {
        name: "dom-layers",
        files: &[
            "src/render/**/*.ts",
            "src/interaction/**/*.ts",
            "playground/**/*.ts",
        ],
        ignores: &[],
        rules: &[("functional/no-return-void", false)],
    },
    Scope {
        name: "barrels",
        files: &[
            "src/**/index.ts",
            "eslint/**/index.ts",
            "playground/*/**/index.ts",
        ],
        ignores: &[],
        rules: &[("local/barrel-exports", true)],
    },
    Scope {
        name: "tooling",
        files: &[
            "eslint/**/*.ts",
            "*.config.ts",
            "*.setup.ts",
            "*.test.ts",
            "tests/*.ts",
            "docs/.vitepress/**/*.ts",
            ".claude/hooks/**/*.ts",
            "scripts/**/*.ts",
        ],
        ignores: &[],
        rules: RELAXED,
    },
    Scope {
        name: "node-run",
        files: &[".claude/hooks/*.ts", "scripts/*.ts"],
        ignores: &[],
        rules: &[
            ("local/canonical-import-path", false),
            ("local/folder-has-index", false),
        ],
    },
    Scope {
        name: "tests",
        files: &["**/*.test.ts"],
        ignores: &[],
        rules: RELAXED,
    },
];

/// The library's files, where no parameter has a default or is optional (ADR-0016).
pub const LIBRARY_FILES: &[&str] = &["src/**/*.ts"];

/// Whether a path from the root is never linted.
#[must_use]
pub fn is_ignored(path: &str) -> bool {
    matches_any(&IGNORED, path)
}

/// The rules that run on a file, sorted.
#[must_use]
pub fn rules_for(path: &str) -> Vec<&'static str> {
    let mut on: Vec<&'static str> = ON_EVERYWHERE.to_vec();

    for scope in SCOPES
        .iter()
        .filter(|scope| matches_any(scope.files, path) && !matches_any(scope.ignores, path))
    {
        for (rule, enabled) in scope.rules {
            on.retain(|each| each != rule);
            if *enabled {
                on.push(rule);
            }
        }
    }
    on.sort_unstable();
    on
}

#[cfg(test)]
mod tests {
    use super::{is_ignored, rules_for};

    #[test]
    fn runs_every_rule_of_kinds_on_the_functional_core() {
        assert_eq!(
            rules_for("src/math/vector/norm.ts"),
            [
                "functional/no-return-void",
                "local/canonical-import-path",
                "local/folder-has-index",
                "local/folder-layout",
                "local/kind-in-layer",
                "local/kind-limits",
                "local/kind-naming",
                "local/one-export-per-file",
                "local/require-kind",
                "local/see-references",
                "no-restricted-syntax",
                "require-atomic-updates",
            ]
        );
    }

    #[test]
    fn checks_barrels_and_relaxes_the_shell() {
        let barrel = rules_for("playground/points/index.ts");

        assert!(barrel.contains(&"local/barrel-exports"));
        assert!(!barrel.contains(&"local/one-export-per-file"));
        assert!(!barrel.contains(&"functional/no-return-void"));
        assert!(barrel.contains(&"local/folder-layout"));
        assert!(!rules_for("playground/index.ts").contains(&"local/barrel-exports"));
    }

    #[test]
    fn leaves_kinds_to_the_library_and_imports_to_node_run_files() {
        let test = rules_for("playground/points/delete-points.test.ts");

        assert!(!test.contains(&"local/require-kind"));
        assert!(!test.contains(&"local/one-export-per-file"));
        assert!(test.contains(&"local/folder-layout"));
        assert_eq!(
            rules_for("scripts/check-all.ts"),
            ["no-restricted-syntax", "require-atomic-updates"]
        );
        assert_eq!(
            rules_for("vite.config.ts"),
            [
                "local/canonical-import-path",
                "local/folder-has-index",
                "no-restricted-syntax",
                "require-atomic-updates",
            ]
        );
    }

    #[test]
    fn checks_the_folder_layout_of_the_library_the_playground_and_their_tests() {
        for path in [
            "src/math/vector/norm.ts",
            "playground/points/index.ts",
            "playground/points/delete-points.test.ts",
        ] {
            assert!(rules_for(path).contains(&"local/folder-layout"), "{path}");
        }
        assert!(!rules_for("tests/folders.test.ts").contains(&"local/folder-layout"));
    }

    #[test]
    fn ignores_generated_and_copied_paths() {
        assert!(is_ignored("docs/api/index.ts"));
        assert!(is_ignored("tools/api-pages/tests/fixtures/a.ts"));
        assert!(is_ignored("node_modules/a/b.ts"));
        assert!(!is_ignored("src/index.ts"));
    }
}
