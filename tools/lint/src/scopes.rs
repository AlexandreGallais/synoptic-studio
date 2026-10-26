//! Which rules run on which file: the rules on for every TypeScript file, then the scopes, carried
//! from `ESLint`'s configuration, that turn this linter's rules on or off by path, in its order, the
//! later winning (EN-075, ADR-0075); and the paths never linted.

use crate::glob::matches_any;

/// Generated, vendored or copied paths, never linted (once `eslint/settings/files.ts`), the
/// packages, cache and build of the playground's npm project among them (ADR-0104).
pub const IGNORED: [&str; 14] = [
    "dist/**",
    "coverage/**",
    "node_modules/**",
    "docs/api/**",
    "docs/.vitepress/cache/**",
    "docs/.vitepress/dist/**",
    "docs/.vitepress/.temp/**",
    ".claude/worktrees/**",
    "target/**",
    "tools/*/tests/fixtures/**",
    ".git/**",
    "playground/node_modules/**",
    "playground/.angular/**",
    "playground/dist/**",
];

/// The rules on for every TypeScript file before the scopes, with the plugins' own lists
/// (`rules::functional::NAMES`, `imports::NAMES`, `stylistic::NAMES`, EN-084).
pub const ON_EVERYWHERE: [&str; 17] = [
    "@typescript-eslint/naming-convention",
    "functional/immutable-data",
    "functional/no-conditional-statements",
    "functional/no-mixed-types",
    "functional/no-return-void",
    "functional/prefer-immutable-types",
    "functional/type-declaration-immutability",
    "local/canonical-import-path",
    "local/folder-has-index",
    "local/kind-in-layer",
    "local/kind-limits",
    "local/kind-naming",
    "local/require-kind",
    "local/see-references",
    "no-restricted-syntax",
    "require-atomic-updates",
    "unicorn/no-non-function-verb-prefix",
];

/// A scope: the files it covers, those it leaves out, and the rules it turns on or off.
#[derive(Clone, Copy, Debug)]
pub struct Scope {
    /// Its name, as `ESLint`'s configuration gave it.
    pub name: &'static str,
    /// The globs of its files.
    pub files: &'static [&'static str],
    /// The globs of the files it leaves out.
    pub ignores: &'static [&'static str],
    /// Each rule and whether the scope turns it on.
    pub rules: &'static [(&'static str, bool)],
}

/// The tools and their configurations: Node.js code, where development packages may be imported.
pub const TOOLING_FILES: &[&str] = &[
    "*.config.ts",
    "playground/*.config.ts",
    "*.setup.ts",
    "*.test.ts",
    "tests/*.ts",
    "docs/.vitepress/**/*.ts",
    "scripts/**/*.ts",
    // The playground's copies of the root's fixtures and of its global setup (EN-176): tooling as
    // `tests/` is, several exports to a module, so that its tests run on its install alone.
    "playground/testing/*-fixtures.ts",
    "playground/testing/*.setup.ts",
];

/// The tests, where development packages may be imported.
pub const TEST_FILES: &[&str] = &["**/*.test.ts"];

/// The library and the playground, whose modules export one value each (ADR-0019).
const SINGLE_EXPORT_FILES: &[&str] = &["src/**/*.ts", "playground/**/*.ts"];

/// The rules off in the tests and the tools: the functional core's `no-return-void`, and the
/// rules about function kinds, which belong to the library and the playground.
const RELAXED: &[(&str, bool)] = &[
    ("functional/functional-parameters", false),
    ("functional/immutable-data", false),
    ("functional/no-conditional-statements", false),
    ("functional/no-expression-statements", false),
    ("functional/no-loop-statements", false),
    ("functional/no-return-void", false),
    ("functional/no-throw-statements", false),
    ("functional/no-try-statements", false),
    ("functional/prefer-immutable-types", false),
    ("functional/type-declaration-immutability", false),
    ("local/kind-in-layer", false),
    ("local/kind-limits", false),
    ("local/kind-naming", false),
    ("local/require-kind", false),
    ("local/see-references", false),
];

/// The playground's Angular app, which follows Angular's usage rather than the library's
/// (ADR-0104, NOTE-0036, findings 52 and 77): a component is a class, a file keeps Angular's
/// suffixed name (`app.component.ts`), a folder has no barrel and is imported by its file, and a
/// module constant keeps Angular's camelCase name (`appConfig`); `@angular-eslint`'s rules run
/// there (F51). The project's configuration files beside it, such as its test runner's, sit at
/// the project's root as Angular's do (EN-160).
pub const ANGULAR_APP_FILES: &[&str] = &["playground/src/**/*.ts", "playground/*.config.ts"];

/// The old page's folders inside the Angular app, kept unchanged as its utils (EN-160): they keep
/// the library's rules, as `tests/code-folders.ts` says, until F57 to F59 move them.
pub const ANGULAR_APP_IGNORES: &[&str] = &["playground/src/app/shared/utils/**"];

/// The scopes, in the order `ESLint`'s configuration applied them; only those that set a rule of
/// this linter.
pub const SCOPES: [Scope; 8] = [
    Scope {
        name: "single-export-modules",
        files: SINGLE_EXPORT_FILES,
        ignores: &[
            "**/index.ts",
            "**/*.test.ts",
            // Several exports to a module, as their originals in `tests/` (EN-176).
            "playground/testing/*-fixtures.ts",
            "playground/testing/*.setup.ts",
        ],
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
        rules: &[
            ("functional/functional-parameters", false),
            ("functional/immutable-data", false),
            ("functional/no-conditional-statements", false),
            ("functional/no-expression-statements", false),
            ("functional/no-loop-statements", false),
            ("functional/no-return-void", false),
            ("functional/no-throw-statements", false),
            ("functional/no-try-statements", false),
            ("functional/prefer-immutable-types", false),
            ("functional/type-declaration-immutability", false),
        ],
    },
    Scope {
        name: "barrels",
        files: &["src/**/index.ts", "playground/*/**/index.ts"],
        ignores: &[],
        rules: &[("local/barrel-exports", true)],
    },
    Scope {
        name: "tooling",
        files: TOOLING_FILES,
        ignores: &[],
        rules: RELAXED,
    },
    Scope {
        name: "node-run",
        files: &["scripts/*.ts"],
        ignores: &[],
        rules: &[
            ("local/canonical-import-path", false),
            ("local/folder-has-index", false),
        ],
    },
    Scope {
        name: "tests",
        files: TEST_FILES,
        ignores: &[],
        rules: RELAXED,
    },
    Scope {
        name: "angular-app",
        files: ANGULAR_APP_FILES,
        ignores: ANGULAR_APP_IGNORES,
        rules: &[
            ("@angular-eslint/contextual-lifecycle", true),
            ("@angular-eslint/no-async-lifecycle-method", true),
            ("@angular-eslint/no-duplicates-in-metadata-arrays", true),
            ("@angular-eslint/no-lifecycle-call", true),
            ("@angular-eslint/no-pipe-impure", true),
            ("@angular-eslint/require-lifecycle-on-prototype", true),
            ("@typescript-eslint/naming-convention", false),
            ("functional/no-classes", false),
            ("functional/no-this-expressions", false),
            ("local/canonical-import-path", false),
            ("local/folder-has-index", false),
            ("local/folder-layout", false),
            ("local/one-export-per-file", false),
        ],
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
    let mut on: Vec<&'static str> = ON_EVERYWHERE
        .iter()
        .chain(crate::rules::functional::NAMES)
        .chain(crate::rules::imports::NAMES)
        .chain(crate::rules::stylistic::NAMES)
        .copied()
        .collect();

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

/// The files where a DOM exists: the shell that renders and listens, the playground and the
/// documentation's theme; the library's core never touches it (ADR-0005, ADR-0025).
pub const DOM_FILES: &[&str] = &[
    "src/render/**/*.ts",
    "src/interaction/**/*.ts",
    "playground/**/*.ts",
    "docs/.vitepress/theme/**/*.ts",
];

/// The rules that run on a file: those its scopes set, then Unicorn's, which no scope turns
/// off (EN-078), those of `eslint-plugin-jsdoc` (EN-083), Unicorn's web rules, and its DOM rules
/// where a DOM exists (EN-082); `unicorn/no-useless-recursion` left where
/// `functional/no-loop-statements` runs (EN-084).
#[must_use]
pub fn linted_by(path: &str) -> Vec<&'static str> {
    use crate::rules::unicorn::{dom, web};

    let mut on = rules_for(path);
    on.extend(crate::rules::unicorn::NAMES);
    on.extend(crate::rules::jsdoc::NAMES);
    on.extend(web::NAMES);
    if matches_any(DOM_FILES, path) {
        on.extend(dom::NAMES);
        on.extend(web::DOM_NAMES);
    }
    if on.contains(&"functional/no-loop-statements") {
        // It suggests a loop, which the functional core may not write (EN-084).
        on.retain(|each| *each != "unicorn/no-useless-recursion");
    }
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
                "@stylistic/multiline-comment-style",
                "@stylistic/padding-line-between-statements",
                "@stylistic/spaced-comment",
                "@typescript-eslint/naming-convention",
                "functional/functional-parameters",
                "functional/immutable-data",
                "functional/no-classes",
                "functional/no-conditional-statements",
                "functional/no-expression-statements",
                "functional/no-let",
                "functional/no-loop-statements",
                "functional/no-mixed-types",
                "functional/no-promise-reject",
                "functional/no-return-void",
                "functional/no-this-expressions",
                "functional/no-throw-statements",
                "functional/no-try-statements",
                "functional/prefer-immutable-types",
                "functional/readonly-type",
                "functional/type-declaration-immutability",
                "import-x/no-extraneous-dependencies",
                "import-x/no-restricted-paths",
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
                "unicorn/no-non-function-verb-prefix",
            ]
        );
    }

    #[test]
    fn relaxes_the_functional_core_s_rules_in_the_shell_the_tools_and_the_tests() {
        for path in [
            "src/render/a.ts",
            "playground/a/b.ts",
            "scripts/a.ts",
            "src/math/a.test.ts",
        ] {
            let rules = rules_for(path);
            for rule in [
                "functional/functional-parameters",
                "functional/immutable-data",
                "functional/no-conditional-statements",
                "functional/no-expression-statements",
                "functional/no-loop-statements",
                "functional/no-throw-statements",
                "functional/no-try-statements",
                "functional/prefer-immutable-types",
                "functional/type-declaration-immutability",
            ] {
                assert!(!rules.contains(&rule), "{path} {rule}");
            }
            assert!(rules.contains(&"functional/no-mixed-types"), "{path}");
            assert!(
                rules.contains(&"@typescript-eslint/naming-convention"),
                "{path}"
            );
        }
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
            rules_for("scripts/default-font-metrics.ts"),
            [
                "@stylistic/multiline-comment-style",
                "@stylistic/padding-line-between-statements",
                "@stylistic/spaced-comment",
                "@typescript-eslint/naming-convention",
                "functional/no-classes",
                "functional/no-let",
                "functional/no-mixed-types",
                "functional/no-promise-reject",
                "functional/no-this-expressions",
                "functional/readonly-type",
                "import-x/no-extraneous-dependencies",
                "import-x/no-restricted-paths",
                "no-restricted-syntax",
                "require-atomic-updates",
                "unicorn/no-non-function-verb-prefix",
            ]
        );
        assert_eq!(
            rules_for("vite.config.ts"),
            [
                "@stylistic/multiline-comment-style",
                "@stylistic/padding-line-between-statements",
                "@stylistic/spaced-comment",
                "@typescript-eslint/naming-convention",
                "functional/no-classes",
                "functional/no-let",
                "functional/no-mixed-types",
                "functional/no-promise-reject",
                "functional/no-this-expressions",
                "functional/readonly-type",
                "import-x/no-extraneous-dependencies",
                "import-x/no-restricted-paths",
                "local/canonical-import-path",
                "local/folder-has-index",
                "no-restricted-syntax",
                "require-atomic-updates",
                "unicorn/no-non-function-verb-prefix",
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
    fn lints_the_playground_s_copied_fixtures_as_the_root_s_tests() {
        // Its copies of the root's fixtures and global setup (EN-176): several exports, no kinds.
        for path in [
            "playground/testing/font-fixtures.ts",
            "playground/testing/font-files.setup.ts",
        ] {
            let rules = rules_for(path);

            assert!(!rules.contains(&"local/one-export-per-file"), "{path}");
            assert!(!rules.contains(&"local/require-kind"), "{path}");
            assert!(rules.contains(&"local/folder-layout"), "{path}");
        }
        // Its actions keep the playground's rules.
        let action = rules_for("playground/testing/press-key.ts");

        assert!(action.contains(&"local/one-export-per-file"));
        assert!(action.contains(&"local/require-kind"));
    }

    #[test]
    fn ignores_generated_and_copied_paths() {
        assert!(is_ignored("docs/api/index.ts"));
        assert!(is_ignored("tools/api-pages/tests/fixtures/a.ts"));
        assert!(is_ignored("node_modules/a/b.ts"));
        assert!(is_ignored("playground/node_modules/a/b.d.ts"));
        assert!(is_ignored("playground/.angular/cache/a.ts"));
        assert!(is_ignored("playground/dist/a.ts"));
        assert!(!is_ignored("src/index.ts"));
        assert!(!is_ignored("playground/points/index.ts"));
        assert!(!is_ignored("playground/src/app/app.component.ts"));
    }

    #[test]
    fn f50_ac3_lints_the_angular_app_by_angular_s_usage() {
        let app = rules_for("playground/src/app/app.component.ts");

        // A class with Angular's suffixed name, in a folder without a barrel, imported by its file.
        for rule in [
            "@typescript-eslint/naming-convention",
            "functional/no-classes",
            "functional/no-this-expressions",
            "local/canonical-import-path",
            "local/folder-has-index",
            "local/folder-layout",
            "local/one-export-per-file",
        ] {
            assert!(!app.contains(&rule), "{rule}");
        }
        // The rest of the playground's rules stay on.
        assert!(app.contains(&"import-x/no-restricted-paths"));
        assert!(app.contains(&"local/require-kind"));
        // The old page's folders, the app's utils, keep the library's rules (EN-160).
        let util = rules_for("playground/src/app/shared/utils/points/show-delete-button.ts");
        let barrel = rules_for("playground/src/app/shared/utils/points/index.ts");
        for rule in [
            "@typescript-eslint/naming-convention",
            "functional/no-classes",
            "functional/no-this-expressions",
            "local/canonical-import-path",
            "local/folder-has-index",
            "local/folder-layout",
            "local/one-export-per-file",
        ] {
            assert!(util.contains(&rule), "{rule}");
        }
        assert!(barrel.contains(&"local/barrel-exports"));
    }

    #[test]
    fn f51_ac1_runs_angular_s_rules_on_the_angular_app_only() {
        let app = rules_for("playground/src/app/app.component.ts");

        for rule in crate::rules::angular::NAMES {
            assert!(app.contains(rule), "{rule}");
        }
        // The app's utils, the old page, the library and the tools are plain TypeScript.
        for path in [
            "playground/src/app/shared/utils/points/show-delete-button.ts",
            "playground/points/delete-points.ts",
            "src/render/a.ts",
            "scripts/a.ts",
        ] {
            let rules = rules_for(path);
            for rule in crate::rules::angular::NAMES {
                assert!(!rules.contains(rule), "{path} {rule}");
            }
        }
    }

    #[test]
    fn leaves_useless_recursion_where_loops_are_forbidden() {
        // It suggests a loop: the functional core may not write one (EN-084).
        for path in ["src/math/a.ts", "src/io/b/c.ts", "src/index.ts"] {
            assert!(
                !super::linted_by(path).contains(&"unicorn/no-useless-recursion"),
                "{path}"
            );
        }
        for path in [
            "src/render/a.ts",
            "playground/a/b.ts",
            "src/math/a.test.ts",
            "scripts/a.ts",
        ] {
            assert!(
                super::linted_by(path).contains(&"unicorn/no-useless-recursion"),
                "{path}"
            );
        }
    }

    #[test]
    fn runs_unicorn_s_rules_after_the_scoped_ones_on_every_file() {
        for path in ["src/render/a.ts", "playground/a/b.test.ts", "scripts/a.ts"] {
            let on = super::linted_by(path);
            let scoped = rules_for(path);

            assert_eq!(on[..scoped.len()], scoped);
            assert_eq!(
                on[scoped.len()..scoped.len() + crate::rules::unicorn::NAMES.len()],
                *crate::rules::unicorn::NAMES,
                "{path}"
            );
            let jsdoc = scoped.len() + crate::rules::unicorn::NAMES.len();
            assert_eq!(
                on[jsdoc..jsdoc + crate::rules::jsdoc::NAMES.len()],
                *crate::rules::jsdoc::NAMES,
                "{path}"
            );
        }
    }

    #[test]
    fn runs_the_dom_rules_only_where_a_dom_exists() {
        use crate::rules::unicorn::{dom, web};

        for path in [
            "src/render/a.ts",
            "playground/a/b.test.ts",
            "docs/.vitepress/theme/a.ts",
        ] {
            let on = super::linted_by(path);

            assert!(on.ends_with(web::DOM_NAMES), "{path}");
            assert!(dom::NAMES.iter().all(|name| on.contains(name)), "{path}");
            assert!(web::NAMES.iter().all(|name| on.contains(name)), "{path}");
        }
        for path in [
            "src/math/a.ts",
            "src/model/a.test.ts",
            "scripts/a.ts",
            "vite.config.ts",
        ] {
            let on = super::linted_by(path);

            assert!(dom::NAMES.iter().all(|name| !on.contains(name)), "{path}");
            assert!(
                web::DOM_NAMES.iter().all(|name| !on.contains(name)),
                "{path}"
            );
            assert!(web::NAMES.iter().all(|name| on.contains(name)), "{path}");
        }
    }
}
