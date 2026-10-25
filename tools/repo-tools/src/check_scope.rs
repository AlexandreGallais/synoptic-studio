//! What the push of a story or a bubble branch checks (decided 2026-10-25, AUD-050): only what its
//! changed files need, where every lane took 3 to 4 minutes. The pre-commit hook formatted, linted
//! and scanned each file at its commit; the push adds what reads beyond one file: TypeScript's
//! check of the whole project, the unused exports, the Vitest tests related to the change, the
//! linters again on the changed files, and for the Rust crates changed, with the crates depending
//! on them, rustfmt, Clippy and their tests. No docs build, no coverage, no Windows check, no
//! dependency check: the full `check:all` keeps them for the push of a feature branch, which its
//! pull request into `develop` relies on, and a change to a dependency, a toolchain or a
//! configuration every check reads ([`FULL_INPUTS`]) runs it on a story too.

use std::path::Path;

use stale_binaries::touches;

use crate::check_all::{PLAYGROUND_TYPES, check_named, quoted};

/// A check of a scoped push: its name in the report and the shell command it runs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Check {
    /// Its name, in its report line.
    pub name: String,
    /// The command `sh -c` runs, from the repository's root.
    pub command: String,
}

/// A crate of the workspace, read from its manifest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Crate {
    /// Its package's name, given to `cargo -p`.
    pub name: String,
    /// Its folder under `tools/`.
    pub folder: String,
    /// The folders under `tools/` of the crates it depends on, its dev-dependencies included.
    pub dependencies: Vec<String>,
}

/// What every check reads, or the dependencies and toolchains: a change to one runs the full
/// `check:all` on a story too, as before (REV-008).
pub const FULL_INPUTS: &[&str] = &[
    "Cargo.toml",
    "Cargo.lock",
    "rust-toolchain.toml",
    ".cargo/",
    "deny.toml",
    "package.json",
    "package-lock.json",
    // The playground's packages, an npm project of its own (ADR-0104).
    "playground/package.json",
    "playground/package-lock.json",
    "tsconfig.json",
    "vite.config.ts",
    ".oxlintrc.json",
    "tests/font-files-setup.ts",
    "tests/inter-regular.ttf",
];

/// What Angular's compiler reads of the playground's app: its code, templates and styles, and its
/// project's settings (EN-159).
pub const PLAYGROUND_APP_INPUTS: &[&str] = &[
    "playground/src/",
    "playground/angular.json",
    "playground/tsconfig.json",
    "playground/tsconfig.app.json",
];

/// The checks a story's push gives up whatever it changed, on the full run too: the dependency
/// updates are grouped at the review, and the Rust tests run without their coverage's floor.
pub const SKIPPED_ON_A_STORY: &[&str] = &["deps:outdated"];

/// The command a story's full run gives `rust:coverage` instead: the workspace's tests, without
/// the coverage's instrumentation.
pub const RUST_TESTS: &str = "cargo test --workspace --locked --quiet";

/// Whether a change touches an input of every check ([`FULL_INPUTS`]).
#[must_use]
pub fn needs_full_check(paths: &[String]) -> bool {
    touches(paths, FULL_INPUTS)
}

/// A crate read from its manifest: the `name` of its `[package]` and each `path = "../<folder>"`
/// of its dependencies; `None` without a name.
#[must_use]
pub fn read_crate(folder: &str, manifest: &str) -> Option<Crate> {
    let mut section = "";
    let mut name = None;
    let mut dependencies = Vec::new();

    for line in manifest.lines().map(str::trim) {
        if line.starts_with('[') {
            section = line;
            continue;
        }
        if section == "[package]" && name.is_none() {
            name = line
                .strip_prefix("name")
                .map(str::trim_start)
                .and_then(|rest| rest.strip_prefix('='))
                .map(|value| value.trim().trim_matches('"').to_owned());
        }
        if let Some((_, after)) = line.split_once("path = \"../") {
            let dependency: String = after.chars().take_while(|letter| *letter != '"').collect();

            if !dependency.is_empty() && !dependencies.contains(&dependency) {
                dependencies.push(dependency);
            }
        }
    }

    name.filter(|name| !name.is_empty()).map(|name| Crate {
        name,
        folder: folder.to_owned(),
        dependencies,
    })
}

/// The crates a change reaches: those whose folder it changed, then every crate depending on one
/// of them, directly or not; their names, in the order of `crates`.
#[must_use]
pub fn affected_crates(paths: &[String], crates: &[Crate]) -> Vec<String> {
    let mut reached: Vec<&str> = crates
        .iter()
        .filter(|krate| {
            let prefix = format!("tools/{}/", krate.folder);

            paths.iter().any(|path| path.starts_with(&prefix))
        })
        .map(|krate| krate.folder.as_str())
        .collect();

    loop {
        let more: Vec<&str> = crates
            .iter()
            .filter(|krate| !reached.contains(&krate.folder.as_str()))
            .filter(|krate| {
                krate
                    .dependencies
                    .iter()
                    .any(|dependency| reached.contains(&dependency.as_str()))
            })
            .map(|krate| krate.folder.as_str())
            .collect();

        if more.is_empty() {
            break;
        }
        reached.extend(more);
    }

    crates
        .iter()
        .filter(|krate| reached.contains(&krate.folder.as_str()))
        .map(|krate| krate.name.clone())
        .collect()
}

/// Whether a path's extension is one of those given, without case.
fn has_extension(path: &str, extensions: &[&str]) -> bool {
    Path::new(path).extension().is_some_and(|extension| {
        extensions
            .iter()
            .any(|known| extension.eq_ignore_ascii_case(known))
    })
}

/// A check of the given name and command.
fn check(name: &str, command: String) -> Check {
    Check {
        name: name.to_owned(),
        command,
    }
}

/// Paths as words of `sh`, separated by spaces.
fn words(paths: &[&str]) -> String {
    paths
        .iter()
        .map(|path| quoted(path))
        .collect::<Vec<_>>()
        .join(" ")
}

/// What a scoped push runs on a change.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scope {
    /// Lanes of checks: in order within a lane, the lanes at the same time.
    pub lanes: Vec<Vec<Check>>,
    /// The timing tests related to the change, run alone once the lanes ended.
    pub after: Option<Check>,
}

/// The checks of a story's push on its change (decided 2026-10-25, AUD-050): `paths` every file
/// changed against the base, `present` those of them still in the working tree, `crates` the
/// workspace's, `repository_tests` the test files of `tests/` and `scripts/`, which read the
/// repository's files beside their imports.
///
/// - the playground's app or its settings changed: Angular's type check of it (EN-159);
/// - TypeScript changed: oxlint on its files, `tsc --noEmit` on the project, the unused exports;
/// - TypeScript or Markdown changed: the project's linter on their files;
/// - a file changed outside `tools/`: the Vitest tests related to it, and the repository's own;
/// - a crate changed: rustfmt on it, Clippy and the tests of it and of the crates depending on it;
/// - TypeScript changed: its related timing tests after the lanes.
#[must_use]
pub fn scope(
    paths: &[String],
    present: &[String],
    crates: &[Crate],
    repository_tests: &[String],
) -> Scope {
    let typescript: Vec<&str> = present
        .iter()
        .map(String::as_str)
        .filter(|path| has_extension(path, &["ts"]))
        .collect();
    let linted: Vec<&str> = present
        .iter()
        .map(String::as_str)
        .filter(|path| has_extension(path, &["ts", "md"]))
        .collect();
    let mut related: Vec<&str> = paths
        .iter()
        .map(String::as_str)
        .filter(|path| !path.starts_with("tools/"))
        .collect();
    let rust = affected_crates(paths, crates);
    let mut lanes = Vec::new();

    if !linted.is_empty() {
        let mut lane = Vec::new();

        if !typescript.is_empty() {
            lane.push(check(
                "oxlint",
                format!(
                    "oxlint --deny-warnings --report-unused-disable-directives-severity=error {}",
                    words(&typescript)
                ),
            ));
        }
        lane.push(check(
            "lint",
            format!("target/release/lint {}", words(&linted)),
        ));
        lanes.push(lane);
    }
    if touches(paths, PLAYGROUND_APP_INPUTS) {
        lanes.push(vec![check_named(PLAYGROUND_TYPES)]);
    }
    if !typescript.is_empty() {
        lanes.push(vec![
            check("tsc", "tsc --noEmit".to_owned()),
            check(
                "deps:unused",
                "target/release/dependencies unused".to_owned(),
            ),
        ]);
    }
    if !related.is_empty() {
        for test in repository_tests {
            if !related.contains(&test.as_str()) {
                related.push(test);
            }
        }
        lanes.push(vec![check(
            "test",
            format!(
                "vitest related --run --passWithNoTests --tags-filter='!timing' {}",
                words(&related)
            ),
        )]);
    }
    if !rust.is_empty() {
        let packages = rust
            .iter()
            .map(|name| format!("-p {name}"))
            .collect::<Vec<_>>()
            .join(" ");

        lanes.push(vec![
            check("rustfmt", format!("cargo fmt --check {packages}")),
            check(
                "clippy",
                format!("cargo clippy --all-targets --locked --quiet {packages}"),
            ),
            check(
                "cargo test",
                format!("cargo test --locked --quiet {packages}"),
            ),
        ]);
    }

    let after = (!typescript.is_empty()).then(|| {
        check(
            "test:timings",
            format!(
                "vitest related --run --passWithNoTests --tags-filter=timing {}",
                words(&typescript)
            ),
        )
    });

    Scope { lanes, after }
}

/// The line saying a push runs only the checks its change needs, naming them.
#[must_use]
pub fn scope_line(base: &str, scope: &Scope) -> String {
    let names: Vec<&str> = scope
        .lanes
        .iter()
        .flatten()
        .chain(&scope.after)
        .map(|check| check.name.as_str())
        .collect();
    let checks = if names.is_empty() {
        "none".to_owned()
    } else {
        names.join(", ")
    };

    format!(
        "- scoped push: only what the change against {base} needs ({checks}); the full check:all \
         on a feature branch"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn owned(paths: &[&str]) -> Vec<String> {
        paths.iter().map(|&path| path.to_owned()).collect()
    }

    /// Three crates: `b` depends on `a`, `c` on `b` by a dev-dependency, `d` on none.
    fn crates() -> Vec<Crate> {
        let manifests = [
            ("a", "[package]\nname = \"a\"\n"),
            (
                "b",
                "[package]\nname = \"bee\"\n[dependencies]\na = { path = \"../a\" }\n",
            ),
            (
                "c",
                "[package]\nname = \"c\"\n[dev-dependencies]\nb = { path = \"../b\" }\n",
            ),
            ("d", "[package]\nname = \"d\"\n"),
        ];

        manifests
            .iter()
            .filter_map(|(folder, text)| read_crate(folder, text))
            .collect()
    }

    #[test]
    fn reads_a_crates_name_and_the_folders_it_depends_on() {
        let manifest = "[package]\nname = \"repo-tools\"\nversion = \"0.1.0\"\n\n[dependencies]\n\
                        calendar = { path = \"../calendar\" }\nmarkdown.workspace = true\n\
                        [dev-dependencies]\ntest-support = { path = \"../test-support\" }\n\
                        calendar = { path = \"../calendar\" }\n[lints]\nworkspace = true\n";

        assert_eq!(
            read_crate("repo-tools", manifest),
            Some(Crate {
                name: "repo-tools".to_owned(),
                folder: "repo-tools".to_owned(),
                dependencies: owned(&["calendar", "test-support"]),
            })
        );
        // A `name` outside `[package]` names no crate; no name, no crate.
        assert_eq!(read_crate("x", "[bin]\nname = \"x\"\n"), None);
        assert_eq!(read_crate("x", "[package]\nname = \"\"\n"), None);
        assert_eq!(read_crate("x", ""), None);
    }

    #[test]
    fn reaches_the_crates_changed_and_every_crate_depending_on_them() {
        let crates = crates();

        // a changed: b depends on it, c on b; d not.
        assert_eq!(
            affected_crates(&owned(&["tools/a/src/lib.rs"]), &crates),
            owned(&["a", "bee", "c"])
        );
        assert_eq!(
            affected_crates(
                &owned(&["tools/c/tests/x.rs", "tools/d/Cargo.toml"]),
                &crates
            ),
            owned(&["c", "d"])
        );
        // Outside a crate's folder, or a folder only starting like one: none.
        assert_eq!(
            affected_crates(&owned(&["tools/ab/x.rs", "docs/a.md"]), &crates),
            Vec::<String>::new()
        );
        assert_eq!(affected_crates(&[], &crates), Vec::<String>::new());
    }

    #[test]
    fn runs_the_full_check_for_a_dependency_a_toolchain_or_a_configuration_changed() {
        for input in [
            "Cargo.lock",
            "Cargo.toml",
            "rust-toolchain.toml",
            ".cargo/config.toml",
            "package.json",
            "package-lock.json",
            "tsconfig.json",
            "vite.config.ts",
            ".oxlintrc.json",
        ] {
            assert!(needs_full_check(&owned(&["docs/x.md", input])), "{input}");
        }
        assert!(!needs_full_check(&owned(&[
            "tools/a/Cargo.toml",
            "src/x.ts"
        ])));
        assert!(!needs_full_check(&[]));
    }

    #[test]
    fn f50_ac3_runs_the_full_check_for_the_playground_s_packages() {
        for input in ["playground/package.json", "playground/package-lock.json"] {
            assert!(needs_full_check(&owned(&[input])), "{input}");
        }
        assert!(!needs_full_check(&owned(&[
            "playground/src/main.ts",
            "playground/angular.json"
        ])));
    }

    #[test]
    fn f50_ac3_checks_the_playground_s_app_by_angular_s_compiler() {
        let types = check_named(PLAYGROUND_TYPES);

        // A template: Angular's check alone, then the related tests.
        let template = owned(&["playground/src/app/app.component.html"]);
        let scope_of_template = scope(&template, &template, &crates(), &[]);

        assert_eq!(scope_of_template.lanes[0], vec![types.clone()]);
        assert_eq!(scope_of_template.lanes.len(), 2);
        // Its settings, even removed from the working tree.
        for input in [
            "playground/angular.json",
            "playground/tsconfig.json",
            "playground/tsconfig.app.json",
        ] {
            let change = owned(&[input]);

            assert_eq!(
                scope(&change, &[], &crates(), &[]).lanes[0],
                vec![types.clone()]
            );
        }
        // The old page and a folder only starting like the app's: not Angular's.
        for path in ["playground/points/a.ts", "playground/srcx/a.ts"] {
            let change = owned(&[path]);

            assert!(
                !scope(&change, &change, &crates(), &[])
                    .lanes
                    .iter()
                    .flatten()
                    .any(|check| check.name == PLAYGROUND_TYPES),
                "{path}"
            );
        }
    }

    #[test]
    fn aud050_checks_a_markdown_change_by_its_lint_and_the_repositorys_tests() {
        let change = owned(&["docs/a.md"]);
        let scope = scope(&change, &change, &crates(), &owned(&["tests/r.test.ts"]));

        assert_eq!(
            scope.lanes,
            [
                vec![check("lint", "target/release/lint 'docs/a.md'".to_owned())],
                vec![check(
                    "test",
                    "vitest related --run --passWithNoTests --tags-filter='!timing' 'docs/a.md' \
                     'tests/r.test.ts'"
                        .to_owned()
                )],
            ]
        );
        assert_eq!(scope.after, None);
        assert_eq!(
            scope_line("origin/feature/f48-x", &scope),
            "- scoped push: only what the change against origin/feature/f48-x needs (lint, \
             test); the full check:all on a feature branch"
        );
    }

    #[test]
    fn aud050_checks_a_typescript_change_by_oxlint_tsc_its_related_tests_and_timings() {
        let paths = owned(&["src/x.ts", "src/gone.ts", "tests/r.test.ts"]);
        let present = owned(&["src/x.ts", "tests/r.test.ts"]);
        let scope = scope(&paths, &present, &crates(), &owned(&["tests/r.test.ts"]));

        assert_eq!(
            scope.lanes,
            [
                vec![
                    check(
                        "oxlint",
                        "oxlint --deny-warnings --report-unused-disable-directives-severity=error \
                         'src/x.ts' 'tests/r.test.ts'"
                            .to_owned()
                    ),
                    check(
                        "lint",
                        "target/release/lint 'src/x.ts' 'tests/r.test.ts'".to_owned()
                    ),
                ],
                vec![
                    check("tsc", "tsc --noEmit".to_owned()),
                    check(
                        "deps:unused",
                        "target/release/dependencies unused".to_owned()
                    ),
                ],
                // The deleted file too, its importers' tests related; a repository test once.
                vec![check(
                    "test",
                    "vitest related --run --passWithNoTests --tags-filter='!timing' 'src/x.ts' \
                     'src/gone.ts' 'tests/r.test.ts'"
                        .to_owned()
                )],
            ]
        );
        assert_eq!(
            scope.after,
            Some(check(
                "test:timings",
                "vitest related --run --passWithNoTests --tags-filter=timing 'src/x.ts' \
                 'tests/r.test.ts'"
                    .to_owned()
            ))
        );
    }

    #[test]
    fn aud050_checks_a_crate_change_by_rustfmt_clippy_and_the_tests_of_it_and_its_dependents() {
        let change = owned(&["tools/b/src/lib.rs"]);
        let scope = scope(&change, &change, &crates(), &owned(&["tests/r.test.ts"]));

        // No Vitest for a change inside `tools/`; the crates by their package names.
        assert_eq!(
            scope.lanes,
            [vec![
                check("rustfmt", "cargo fmt --check -p bee -p c".to_owned()),
                check(
                    "clippy",
                    "cargo clippy --all-targets --locked --quiet -p bee -p c".to_owned()
                ),
                check(
                    "cargo test",
                    "cargo test --locked --quiet -p bee -p c".to_owned()
                ),
            ]]
        );
        assert_eq!(scope.after, None);
    }

    #[test]
    fn aud050_checks_nothing_for_a_change_no_check_reads() {
        // A file of `tools/` outside every crate, and no change at all.
        for change in [owned(&["tools/README.md"]), Vec::new()] {
            let scope = scope(&change, &[], &crates(), &owned(&["tests/r.test.ts"]));

            assert!(scope.lanes.is_empty(), "{change:?}");
            assert_eq!(scope.after, None);
            assert!(scope_line("b", &scope).contains("needs (none)"));
        }
    }
}
