//! `dependencies unused` on planted trees, against knip 6.41.0's verdicts on the same cases: each
//! case was planted in a copy of this repository on 2026-10-18 and knip, with `knip.json`, gave
//! the findings asserted here, file, line and column included.

use std::fs;
use std::path::{Path, PathBuf};

use deps::modules::{self, Taken};
use deps::unused::{self, Findings, Tree};

/// A planted checkout in the temporary folder, its files written from `(path, text)`.
fn plant(name: &str, files: &[(&str, &str)]) -> PathBuf {
    let root = std::env::temp_dir().join(format!("deps-unused-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);

    for (path, text) in files {
        let path = root.join(path);

        fs::create_dir_all(path.parent().expect("a folder")).expect("a folder");
        fs::write(path, text).expect("a file");
    }
    root
}

/// The tree of a planted checkout, every `.ts` file of the project listed.
fn tree(root: &Path, files: &[(&str, &str)]) -> Tree {
    let listed: Vec<&str> = files.iter().map(|(path, _)| *path).collect();

    Tree {
        root: root.to_path_buf(),
        files: unused::project_files(root, &listed.join("\0")),
    }
}

/// What every planted tree holds: the scripts, the compiler's configuration, Vite's, the
/// library's barrel and a test of it.
const BASE: &[(&str, &str)] = &[
    (
        "package.json",
        "{\n  \"scripts\": {\n    \"test\": \"vitest run\",\n    \"lint\": \"oxlint src\",\n    \"font\": \"node scripts/font.ts\"\n  },\n  \"devDependencies\": {\n    \"@types/node\": \"26.6.5\",\n    \"@vitest/coverage-v8\": \"5.0.4\",\n    \"fast-check\": \"4.10.2\",\n    \"happy-dom\": \"21.0.1\",\n    \"is-number\": \"7.0.0\",\n    \"oxlint\": \"1.87.0\",\n    \"oxlint-tsgolint\": \"7.0.2004\",\n    \"typescript\": \"7.0.2\",\n    \"vite\": \"8.3.4\",\n    \"vitest\": \"5.0.4\"\n  }\n}\n",
    ),
    (
        ".oxlintrc.json",
        "{\n  \"options\": {\n    \"typeAware\": true,\n    \"typeCheck\": true\n  }\n}\n",
    ),
    (
        "tsconfig.json",
        "{\n  // Comments are allowed here.\n  \"compilerOptions\": {\n    \"types\": [\"node\", \"vite/client\"]\n  }\n}\n",
    ),
    (
        "vite.config.ts",
        "import { defineConfig } from \"vitest/config\";\n\nexport default defineConfig({ test: { coverage: { provider: \"v8\" }, globalSetup: [\"tests/setup.ts\"] } });\n",
    ),
    (
        "scripts/font.ts",
        "import { readFileSync } from \"node:fs\";\n\nexport const FONT = readFileSync;\n",
    ),
    ("tests/setup.ts", "export function setup(): void {}\n"),
    ("src/index.ts", "export * from \"./math\";\n"),
    ("src/math/index.ts", "export * from \"./add\";\n"),
    (
        "src/math/add.ts",
        "export function add(a: number, b: number): number {\n  return a + b;\n}\n",
    ),
    (
        "src/math/add.test.ts",
        "// @vitest-environment happy-dom\nimport { expect, it } from \"vitest\";\n\nimport { add } from \"..\";\n\nit(\"adds\", () => {\n  expect(add(1, 2)).toBe(3);\n});\n",
    ),
];

fn check(name: &str, planted: &[(&str, &str)]) -> Findings {
    let files: Vec<(&str, &str)> = BASE.iter().chain(planted).copied().collect();
    let root = plant(name, &files);
    let findings = unused::check(&tree(&root, &files));

    fs::remove_dir_all(&root).expect("removed");
    findings
}

#[test]
fn f21_ac4_finds_only_the_dev_package_nothing_names_on_the_base_tree() {
    // Every package is named: by an import (`vitest`), `tsconfig.json`'s types (`@types/node`,
    // `vite`), a script's binary (`oxlint`), Vitest's coverage provider, the DOM comment, oxlint's
    // `typeAware` in `.oxlintrc.json` (EN-121), the compiler's configuration; `fast-check` is
    // ignored, as `knip.json` did.
    // Knip reported `is-number  package.json:12:6`: line 12, its name after the quote at column 5.
    assert_eq!(
        check("base", &[]).report(),
        "Unused devDependencies (1)\nis-number  package.json:12:6\n"
    );
}

#[test]
fn f21_ac4_reports_an_unused_file_export_and_type_as_knip_does() {
    let findings = check(
        "planted",
        &[
            // A file nothing imports: unused, its exports not reported.
            (
                "src/math/orphan.ts",
                "export function orphan(): number {\n  return 1;\n}\n",
            ),
            // Reached through the barrel, imported by nobody: column 17, after `export function `.
            (
                "src/math/export.ts",
                "export function plantedExport(): number {\n  return 2;\n}\n",
            ),
            ("src/math/type.ts", "export type PlantedType = number;\n"),
            (
                "src/math/index.ts",
                "export * from \"./add\";\nexport * from \"./export\";\nexport * from \"./type\";\n",
            ),
        ],
    );

    assert_eq!(
        findings.report(),
        "Unused files (1)\nsrc/math/orphan.ts\nUnused devDependencies (1)\nis-number  package.json:12:6\nUnused exports (1)\nplantedExport  function  src/math/export.ts:1:17\nUnused exported types (1)\nPlantedType  type  src/math/type.ts:1:13\n"
    );
}

#[test]
fn f21_ac4_reports_unlisted_packages_and_unresolved_imports() {
    let findings = check(
        "imports",
        &[(
            "tests/imports.test.ts",
            "import { pad } from \"left-pad-planted\";\nimport { gone } from \"./missing-planted\";\nimport page from \"../src/math/add.ts?raw\";\n\nexport const USED = [pad, gone, page];\n",
        )],
    );

    // Knip: `left-pad-planted  tests/…:1:21` and `./missing-planted  tests/…:2:22`, the column of
    // the specifier's opening quote; `?raw` is dropped before the path is resolved.
    assert_eq!(findings.unlisted.len(), 1);
    assert_eq!(
        (
            findings.unlisted[0].name.as_str(),
            findings.unlisted[0].line,
            findings.unlisted[0].column
        ),
        ("left-pad-planted", 1, 21)
    );
    assert_eq!(findings.unresolved.len(), 1);
    assert_eq!(
        (
            findings.unresolved[0].name.as_str(),
            findings.unresolved[0].line,
            findings.unresolved[0].column
        ),
        ("./missing-planted", 2, 22)
    );
}

#[test]
fn f21_ac4_follows_named_re_exports_and_types_used_by_an_export() {
    let findings = check(
        "re-exports",
        &[
            (
                "src/math/many.ts",
                "export const PLANTED_CONST = 3;\nexport interface PlantedShape {\n  readonly a: number;\n}\nexport const SELF_USED = 4;\nexport const twice = SELF_USED * 2;\n",
            ),
            (
                "src/math/barrel.ts",
                "export { PLANTED_CONST, twice } from \"./many\";\nexport type { PlantedShape } from \"./many\";\n",
            ),
            (
                "src/math/default.ts",
                "import { twice } from \"./barrel\";\nexport default twice;\n",
            ),
            (
                "src/math/self.ts",
                "export type ByExported = number;\nexport type ByLocal = string;\nexport type ByNothingUsed = boolean;\nconst local: ByLocal = \"a\";\nexport function usesIt(value: ByExported): ByExported {\n  return value + local.length;\n}\nexport function unusedFn(value: ByNothingUsed): boolean {\n  return value;\n}\n",
            ),
            (
                "tests/default.test.ts",
                "import used from \"../src/math/default\";\nimport { usesIt } from \"../src/math/self\";\n\nexport const Z = [used, usesIt];\n",
            ),
        ],
    );
    let names = |list: &[unused::Finding]| -> Vec<String> {
        list.iter()
            .map(|finding| {
                format!(
                    "{} {}:{}:{}",
                    finding.name, finding.file, finding.line, finding.column
                )
            })
            .collect()
    };

    // `twice` is used through the barrel, so in `many.ts` too; `SELF_USED`, used only in its own
    // file, is reported; a re-export unused is reported in the barrel and at its origin. A type
    // the signature of a used export names is used; one a local or an unused export names is not.
    assert_eq!(
        names(&findings.exports),
        [
            "PLANTED_CONST src/math/barrel.ts:1:10",
            "PLANTED_CONST src/math/many.ts:1:14",
            "SELF_USED src/math/many.ts:5:14",
            "unusedFn src/math/self.ts:8:17",
        ]
    );
    assert_eq!(
        names(&findings.types),
        [
            "PlantedShape src/math/barrel.ts:2:15",
            "PlantedShape src/math/many.ts:2:18",
            "ByLocal src/math/self.ts:2:13",
            "ByNothingUsed src/math/self.ts:3:13",
        ]
    );
    assert_eq!(findings.types[1].kind, "interface");
    assert_eq!(findings.types[0].kind, "type");
}

#[test]
fn a_namespace_import_uses_every_export_and_a_default_re_export_is_one() {
    let findings = check(
        "namespace",
        &[
            (
                "src/math/type.ts",
                "export type PlantedType = number;\nexport const ALSO = 1;\n",
            ),
            (
                "src/math/ns.ts",
                "export { default as twiceDefault } from \"./default\";\nexport * as everything from \"./type\";\n",
            ),
            ("src/math/default.ts", "export default 2;\n"),
            (
                "tests/ns.test.ts",
                "import { everything } from \"../src/math/ns\";\nimport(\"../src/math/add\");\n\nexport const E: everything.PlantedType = everything.ALSO;\n",
            ),
        ],
    );

    // Knip: `twiceDefault  src/math/ns.ts:1:21`; `everything` takes all of `type.ts`; the
    // default export of `default.ts` is the origin of the unused re-export, so unused too.
    assert_eq!(
        findings
            .exports
            .iter()
            .map(|finding| format!(
                "{} {}:{}:{}",
                finding.name, finding.file, finding.line, finding.column
            ))
            .collect::<Vec<_>>(),
        [
            "default src/math/default.ts:1:8",
            "twiceDefault src/math/ns.ts:1:21"
        ]
    );
    assert_eq!(findings.types, []);
    assert_eq!(findings.files, Vec::<String>::new());
}

#[test]
fn a_package_named_by_a_workflow_or_a_hook_is_used() {
    let findings = check(
        "workflow",
        &[
            (
                ".github/workflows/ci.yml",
                "jobs:\n  a:\n    steps:\n      - run: npx is-number\n",
            ),
            (
                ".githooks/pre-commit",
                "#!/bin/sh\nexec target/release/git-hooks pre-commit\n",
            ),
        ],
    );

    assert_eq!(findings.report(), "");
    assert!(findings.is_empty());
}

#[test]
fn reads_the_imports_and_exports_of_a_module() {
    let module = modules::read(
        "import type { A } from \"./a\";\nimport * as b from \"./b\";\nimport \"./side\";\nexport { c } from \"./c\";\nexport * from \"./d\";\nexport const enum E { X }\nexport abstract class F {}\nexport async function g(): Promise<void> {}\nconst h = import(`./h`);\nconst i = import(`./${h}`);\n",
    )
    .expect("parsed");
    let mut imports: Vec<(String, Taken)> = module
        .imports
        .iter()
        .map(|import| (import.specifier.clone(), import.taken.clone()))
        .collect();

    imports.sort_by_key(|import| format!("{import:?}"));
    imports.dedup();
    assert_eq!(
        imports,
        [
            ("./a".to_owned(), Taken::Name("A".to_owned())),
            ("./a".to_owned(), Taken::Nothing),
            ("./b".to_owned(), Taken::All),
            ("./b".to_owned(), Taken::Nothing),
            ("./c".to_owned(), Taken::Nothing),
            ("./d".to_owned(), Taken::Nothing),
            ("./h".to_owned(), Taken::All),
            ("./side".to_owned(), Taken::Nothing),
        ]
    );
    assert_eq!(module.stars, ["./d"]);
    assert_eq!(
        module
            .exports
            .iter()
            .map(|export| (export.name.as_str(), export.kind, export.is_type))
            .collect::<Vec<_>>(),
        [
            ("E", "enum", true),
            ("F", "class", false),
            ("g", "function", false),
            ("c", "", false)
        ]
    );
    assert_eq!(modules::read("export const = ;"), None);
    assert_eq!(modules::line_column("ab\ncdé\nf", 8), (3, 1));
    assert_eq!(modules::line_column("ab\ncdé\nf", 5), (2, 3));
}

#[test]
fn resolves_relative_specifiers_and_names_packages() {
    let root = plant(
        "resolve",
        &[("src/a.ts", ""), ("src/b/index.ts", ""), ("page.html", "")],
    );

    assert_eq!(
        unused::resolve(&root, "src/c.ts", "./a"),
        Some("src/a.ts".to_owned())
    );
    assert_eq!(
        unused::resolve(&root, "src/c.ts", "./a.ts"),
        Some("src/a.ts".to_owned())
    );
    assert_eq!(
        unused::resolve(&root, "src/c.ts", "./b"),
        Some("src/b/index.ts".to_owned())
    );
    assert_eq!(unused::resolve(&root, "src/b/x.ts", ".."), None);
    assert_eq!(
        unused::resolve(&root, "src/c.ts", "../page.html?raw"),
        Some("page.html".to_owned())
    );
    assert_eq!(unused::resolve(&root, "src/c.ts", "../../outside"), None);
    assert_eq!(unused::resolve(&root, "src/c.ts", "./missing"), None);
    fs::remove_dir_all(&root).expect("removed");
    assert_eq!(unused::package_of("vitest/config"), Some("vitest"));
    assert_eq!(
        unused::package_of("@fast-check/vitest"),
        Some("@fast-check/vitest")
    );
    assert_eq!(unused::package_of("@scope/name/deep"), Some("@scope/name"));
    assert_eq!(unused::package_of("node:fs"), None);
    assert_eq!(unused::package_of("./x"), None);
    assert_eq!(unused::package_of("/abs"), None);
}

#[test]
fn lists_the_project_files_git_lists() {
    let root = plant(
        "project",
        &[
            ("vite.config.ts", ""),
            ("src/a.ts", ""),
            ("tools/x.ts", ""),
            ("src/b.css", ""),
        ],
    );
    let listed = "vite.config.ts\0src/a.ts\0tools/x.ts\0src/b.css\0src/deleted.ts\0src/a.ts\0";

    assert_eq!(
        unused::project_files(&root, listed),
        ["src/a.ts", "vite.config.ts"]
    );
    assert!(unused::is_project("docs/.vitepress/theme/index.ts"));
    assert!(!unused::is_project(".claude/hooks/x.ts"));
    // The whole playground is a project of its own (EN-175), listed for its own check.
    assert!(!unused::is_project("playground/testing/press-key.ts"));
    assert!(!unused::is_project("playground/src/main.ts"));
    assert!(!unused::is_project("playground/vitest-base.config.ts"));
    assert!(unused::ls_files_arguments().contains(&":(glob)*.ts".to_owned()));
    assert!(unused::ls_files_arguments().contains(&"playground/".to_owned()));
    fs::remove_dir_all(&root).expect("removed");
}

#[test]
fn f21_ac4_the_binary_reports_a_planted_file_in_a_git_checkout() {
    let files: Vec<(&str, &str)> = BASE
        .iter()
        .copied()
        .chain([
            ("src/math/orphan.ts", "export const ORPHAN = 1;\n"),
            (".gitignore", "ignored.ts\n"),
            ("ignored.ts", ""),
        ])
        .collect();
    let root = plant("binary", &files);
    let git = |arguments: &[&str]| {
        test_support::command("git")
            .args(arguments)
            .current_dir(&root)
            .output()
            .expect("git runs")
    };

    assert!(git(&["init", "--quiet"]).status.success());
    let output = test_support::command(env!("CARGO_BIN_EXE_dependencies"))
        .arg("unused")
        .current_dir(&root)
        .output()
        .expect("the binary runs");

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "Unused files (1)\nsrc/math/orphan.ts\nUnused devDependencies (1)\nis-number  package.json:12:6\n"
    );
    fs::remove_dir_all(&root).expect("removed");
}

/// The playground's npm project of a planted checkout: an Angular app whose `src/main.ts` imports
/// `@angular/core`, which asks `rxjs` as a peer and `zone.js` as an optional one; `@angular/build`
/// named by `angular.json`'s builder, `sass` by its SCSS, `@angular/cli` by the `ng` of a script,
/// `vitest` by its runner configuration; a test using a helper of `testing/`, the library by its name
/// `editor` and a fixture of the root's `tests/` (EN-175); `left-pad`, a file and a helper unused.
const PLAYGROUND: &[(&str, &str)] = &[
    (
        "playground/package.json",
        "{\n  \"scripts\": { \"start\": \"ng serve\" },\n  \"dependencies\": {\n    \"@angular/core\": \"22.2.3\",\n    \"left-pad\": \"1.3.0\",\n    \"rxjs\": \"7.8.2\"\n  },\n  \"devDependencies\": {\n    \"@angular/build\": \"22.2.3\",\n    \"@angular/cli\": \"22.2.3\",\n    \"sass\": \"1.105.2\",\n    \"vitest\": \"5.0.5\"\n  }\n}\n",
    ),
    (
        "playground/angular.json",
        "{\n  \"builder\": \"@angular/build:application\",\n  \"style\": \"scss\"\n}\n",
    ),
    (
        "playground/node_modules/@angular/core/package.json",
        "{ \"peerDependencies\": { \"rxjs\": \"^7.4.0\", \"zone.js\": \"~0.16.0\" }, \"peerDependenciesMeta\": { \"zone.js\": { \"optional\": true } } }",
    ),
    (
        "playground/node_modules/@angular/cli/package.json",
        "{ \"bin\": { \"ng\": \"bin/ng.js\" } }",
    ),
    (
        "playground/src/main.ts",
        "import { Component } from \"@angular/core\";\n\nimport { AppComponent } from \"./app/app.component\";\n\nexport const APP = [Component, AppComponent];\n",
    ),
    (
        "playground/src/app/app.component.ts",
        "export class AppComponent {}\n",
    ),
    ("playground/src/app/orphan.ts", "export const ORPHAN = 1;\n"),
    (
        "playground/vitest-base.config.ts",
        "import { defineConfig } from \"vitest/config\";\n\nexport default defineConfig({});\n",
    ),
    (
        "playground/src/app/app.component.test.ts",
        "import { SCALE } from \"editor\";\nimport { expect, it } from \"vitest\";\n\nimport { FIXTURE } from \"../../../tests/fixture\";\nimport { pressKey } from \"../../testing/press-key\";\n\nit(\"a\", () => expect([SCALE, FIXTURE, pressKey]).toHaveLength(3));\n",
    ),
    (
        "playground/testing/press-key.ts",
        "export const pressKey = 1;\n",
    ),
    (
        "playground/testing/orphan-helper.ts",
        "export const HELPER = 1;\n",
    ),
    // The root's side: a fixture and an export of the library only the playground imports.
    (
        "tests/fixture.ts",
        "export const FIXTURE = 1;\nexport const UNUSED_FIXTURE = 2;\n",
    ),
    (
        "src/math/index.ts",
        "export * from \"./add\";\nexport * from \"./scale\";\n",
    ),
    ("src/math/scale.ts", "export const SCALE = 2;\n"),
];

#[test]
fn f50_ac3_the_binary_checks_the_playground_as_a_project_of_its_own() {
    let files: Vec<(&str, &str)> = BASE.iter().chain(PLAYGROUND).copied().collect();
    let root = plant("playground", &files);

    assert!(
        test_support::command("git")
            .args(["init", "--quiet"])
            .current_dir(&root)
            .status()
            .expect("git runs")
            .success()
    );
    let output = test_support::command(env!("CARGO_BIN_EXE_dependencies"))
        .arg("unused")
        .current_dir(&root)
        .output()
        .expect("the binary runs");

    fs::remove_dir_all(&root).expect("removed");
    // The playground's findings named from the root: its file, and `left-pad` at line 5, column
    // 6 of its `package.json` (four spaces, the quote, then the name); `rxjs` used through
    // `@angular/core`, `zone.js` not listed; the root's `is-number` beside them. The playground's
    // helper unused; the root's `SCALE` and `FIXTURE` used by the playground's test, not the
    // fixture it leaves (line 2, column 14 after `export const `).
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "Unused files (2)\nplayground/src/app/orphan.ts\nplayground/testing/orphan-helper.ts\nUnused dependencies (1)\nleft-pad  playground/package.json:5:6\nUnused devDependencies (1)\nis-number  package.json:12:6\nUnused exports (1)\nUNUSED_FIXTURE  tests/fixture.ts:2:14\n"
    );
}

#[test]
fn f50_ac3_lists_the_playground_s_files_from_its_folder() {
    let root = plant(
        "playground-files",
        &[
            ("playground/src/main.ts", ""),
            ("playground/src/app/a.component.ts", ""),
            ("playground/src/app/a.component.html", ""),
            ("playground/testing/b.ts", ""),
            ("playground/vitest-base.config.ts", ""),
        ],
    );
    let listed = "playground/src/main.ts\0playground/src/app/a.component.ts\0playground/src/app/a.component.html\0playground/testing/b.ts\0playground/vitest-base.config.ts\0playground/src/gone.ts\0src/a.ts\0";

    // Its app, the helpers of its tests and its runner configuration (EN-175).
    assert_eq!(
        unused::playground_files(&root, listed),
        [
            "src/app/a.component.ts",
            "src/main.ts",
            "testing/b.ts",
            "vitest-base.config.ts"
        ]
    );
    fs::remove_dir_all(&root).expect("removed");
}

#[test]
fn the_binary_exits_2_outside_a_checkout() {
    let root = plant("no-git", &[("package.json", "{}")]);
    let output = test_support::command(env!("CARGO_BIN_EXE_dependencies"))
        .arg("unused")
        .current_dir(&root)
        .env("GIT_CEILING_DIRECTORIES", std::env::temp_dir())
        .output()
        .expect("the binary runs");

    assert_eq!(output.status.code(), Some(2));
    fs::remove_dir_all(&root).expect("removed");
}

#[test]
fn f50_ac3_names_from_the_root_the_file_a_project_imports_outside_its_folder() {
    // `playground/src/a/b.test.ts` up three folders: the root's `tests/fixture`, its query dropped.
    assert_eq!(
        unused::outside(
            "playground",
            "src/a/b.test.ts",
            "../../../tests/fixture?raw"
        )
        .as_deref(),
        Some("tests/fixture")
    );
    // Inside the project, from the root's project, or out of the checkout: none.
    assert_eq!(unused::outside("playground", "src/a/b.ts", "../c"), None);
    assert_eq!(unused::outside("", "src/a.ts", "../../x"), None);
    assert_eq!(unused::outside("playground", "a.ts", "../../x"), None);
}
