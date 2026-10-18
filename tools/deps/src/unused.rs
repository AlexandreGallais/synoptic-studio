//! `dependencies unused`: the issue types knip reports by default (`REF-KNIP-ISSUE-TYPES`) that
//! this repository meets — unused files, unused dev packages, unlisted packages, unresolved
//! imports, unused exports and exported types — for its layout, written anew rather than ported:
//! one export per file (ADR-0019) and barrels of `export *`; imports by relative path, joined to
//! the importing file's folder (`.ts`, then `/index.ts`, tried after the path itself), or by bare
//! package name (research note 0025, no `oxc_resolver`). Its configuration is [`ENTRIES`],
//! [`PROJECT`], [`PLUGIN_USES`] and [`IGNORED`], in place of `knip.json` and knip's plugins.

use std::collections::{BTreeSet, HashMap};
use std::fmt::Write as _;
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::modules::{self, Import, Module, Taken};

/// The folders and root files whose `.ts` files are checked: knip's `project`.
pub const PROJECT: &[&str] = &[
    "*.ts",
    "docs/.vitepress/",
    "playground/",
    "scripts/",
    "src/",
    "tests/",
];

/// The entry files besides the tests (`*.test.ts`) and the files the scripts name: Vite's and
/// Vitest's configuration, the playground's page script (`playground/index.html`), the site's
/// configuration and theme.
pub const ENTRIES: &[&str] = &[
    "vite.config.ts",
    "playground/index.ts",
    "docs/.vitepress/config.ts",
    "docs/.vitepress/theme/index.ts",
];

/// The files whose words may name an entry file or a package's binary: the scripts, the hooks,
/// the workflows, the agent's settings and servers, and Vite's configuration (its `globalSetup`).
pub const SCRIPT_FILES: &[&str] = &[
    "package.json",
    ".mcp.json",
    ".claude/settings.json",
    ".github/workflows/",
    ".githooks/",
    "vite.config.ts",
];

/// A dev package used by a tool's configuration rather than by an import or a binary, as knip's
/// plugins find it: the package, the file that names it, the text that does.
pub const PLUGIN_USES: &[(&str, &str, &str)] = &[
    // Vitest's coverage provider (`test.coverage.provider`).
    ("@vitest/coverage-v8", "vite.config.ts", "provider: \"v8\""),
    // oxlint's type-aware rules, loaded by `--type-check`.
    ("oxlint-tsgolint", "package.json", "oxlint --type-check"),
    // The compiler `tsconfig.json` configures, whatever runs it.
    ("typescript", "tsconfig.json", "\"compilerOptions\""),
];

/// The comment that gives a test file Vitest's DOM, `happy-dom` (`docs/tooling/dependencies.md`).
const HAPPY_DOM: &str = "@vitest-environment happy-dom";

/// Dev packages never reported, each with its reason, as `knip.json`'s `ignoreDependencies` held
/// them: `fast-check`, the peer of `@fast-check/vitest` that the tests import it through; `oxfmt`,
/// run by `tools/format`; `secretlint` and its preset, run by the Git hooks' Rust runner and
/// through `xargs`.
pub const IGNORED: &[&str] = &[
    "@secretlint/secretlint-rule-preset-recommend",
    "fast-check",
    "oxfmt",
    "secretlint",
];

/// One finding: what, and where (`file:line:column`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Finding {
    /// The file, from the checkout's root.
    pub file: String,
    /// Its line, from 1; 0 for a whole file.
    pub line: usize,
    /// Its column, from 1.
    pub column: usize,
    /// The name, specifier or package.
    pub name: String,
    /// The kind of an export, empty otherwise.
    pub kind: &'static str,
}

/// The findings, by issue type, in knip's order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Findings {
    /// Project files no entry reaches.
    pub files: Vec<String>,
    /// Packages of `dependencies` nothing uses.
    pub dependencies: Vec<Finding>,
    /// Packages of `devDependencies` nothing uses.
    pub dev_dependencies: Vec<Finding>,
    /// Packages imported but listed nowhere.
    pub unlisted: Vec<Finding>,
    /// Relative imports that name no file.
    pub unresolved: Vec<Finding>,
    /// Exports nothing imports.
    pub exports: Vec<Finding>,
    /// Exported types nothing imports.
    pub types: Vec<Finding>,
}

impl Findings {
    /// The report, a section per issue type found, as knip writes it.
    #[must_use]
    pub fn report(&self) -> String {
        let mut text = String::new();
        let mut section = |title: &str, lines: Vec<String>| {
            if !lines.is_empty() {
                let _ = writeln!(text, "{title} ({})", lines.len());
                for line in lines {
                    text.push_str(&line);
                    text.push('\n');
                }
            }
        };
        let located = |findings: &[Finding]| -> Vec<String> {
            findings
                .iter()
                .map(|finding| {
                    let kind = if finding.kind.is_empty() {
                        String::new()
                    } else {
                        format!("  {}", finding.kind)
                    };

                    format!(
                        "{}{kind}  {}:{}:{}",
                        finding.name, finding.file, finding.line, finding.column
                    )
                })
                .collect()
        };

        section("Unused files", self.files.clone());
        section("Unused dependencies", located(&self.dependencies));
        section("Unused devDependencies", located(&self.dev_dependencies));
        section("Unlisted dependencies", located(&self.unlisted));
        section("Unresolved imports", located(&self.unresolved));
        section("Unused exports", located(&self.exports));
        section("Unused exported types", located(&self.types));
        text
    }

    /// Whether nothing was found.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.report().is_empty()
    }
}

/// A checkout's text files, read once.
pub struct Tree {
    /// The checkout's root.
    pub root: PathBuf,
    /// The project's `.ts` files, from the root, sorted.
    pub files: Vec<String>,
}

/// Whether a path from the root is a project file: a `.ts` file at the root or in a project folder.
#[must_use]
pub fn is_project(path: &str) -> bool {
    Path::new(path)
        .extension()
        .is_some_and(|extension| extension == "ts")
        && PROJECT.iter().any(|entry| match entry.strip_prefix("*") {
            Some(_) => !path.contains('/'),
            None => path.starts_with(entry),
        })
}

/// A path joined to a folder, its `.` and `..` resolved, `/`-separated from the root; `None` when
/// it leaves the root.
fn joined(folder: &str, relative: &str) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();

    for component in Path::new(folder).join(relative).components() {
        match component {
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::ParentDir => {
                parts.pop()?;
            }
            _ => {}
        }
    }
    Some(parts.join("/"))
}

/// The file a relative specifier names from a file: the path itself, with `.ts`, or its
/// `/index.ts`, the first that is a file; its query (`?raw`) dropped. `None` when none is.
#[must_use]
pub fn resolve(root: &Path, from: &str, specifier: &str) -> Option<String> {
    let path = specifier.split('?').next().unwrap_or(specifier);
    let folder = Path::new(from).parent().map_or(String::new(), |parent| {
        parent.to_string_lossy().into_owned()
    });
    let base = joined(&folder, path)?;

    [
        base.clone(),
        format!("{base}.ts"),
        format!("{base}/index.ts"),
    ]
    .into_iter()
    .find(|candidate| root.join(candidate).is_file())
}

/// Whether a specifier is relative.
fn is_relative(specifier: &str) -> bool {
    specifier.starts_with("./")
        || specifier.starts_with("../")
        || specifier == "."
        || specifier == ".."
}

/// The package a bare specifier names: `vitest` of `vitest/config`, `@fast-check/vitest`; `None`
/// for a Node.js built-in (`node:fs`) or a relative or absolute path.
#[must_use]
pub fn package_of(specifier: &str) -> Option<&str> {
    if is_relative(specifier) || specifier.starts_with('/') || specifier.starts_with("node:") {
        return None;
    }
    let end = if specifier.starts_with('@') {
        specifier
            .match_indices('/')
            .nth(1)
            .map_or(specifier.len(), |(at, _)| at)
    } else {
        specifier.find('/').unwrap_or(specifier.len())
    };

    Some(&specifier[..end])
}

/// The words of a text that may be paths or binaries: runs of letters, digits and `_ . / @ -`.
fn words(text: &str) -> impl Iterator<Item = &str> {
    text.split(|character: char| {
        !(character.is_ascii_alphanumeric() || matches!(character, '_' | '.' | '/' | '@' | '-'))
    })
    .filter(|word| !word.is_empty())
    .map(|word| word.trim_start_matches("./"))
}

/// The modules of the project's files, by path; a file Oxc cannot parse is an empty module.
fn read_modules(tree: &Tree) -> HashMap<String, (String, Module)> {
    tree.files
        .iter()
        .map(|path| {
            let text = fs::read_to_string(tree.root.join(path)).unwrap_or_default();
            let module = modules::read(&text).unwrap_or_default();

            (path.clone(), (text, module))
        })
        .collect()
}

/// The texts of the script files, the files of a listed folder included, sorted.
fn script_texts(root: &Path) -> Vec<(String, String)> {
    let mut texts = Vec::new();

    for entry in SCRIPT_FILES {
        if entry.ends_with('/') {
            let mut paths: Vec<PathBuf> = fs::read_dir(root.join(entry))
                .map(|entries| {
                    entries
                        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
                        .collect()
                })
                .unwrap_or_default();

            paths.sort();
            for path in paths.into_iter().filter(|path| path.is_file()) {
                texts.push((
                    path.strip_prefix(root)
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .into_owned(),
                    fs::read_to_string(&path).unwrap_or_default(),
                ));
            }
        } else if let Ok(text) = fs::read_to_string(root.join(entry)) {
            // Of `package.json`, its scripts only: its lists name every package.
            let text = if *entry == "package.json" {
                serde_json::from_str::<serde_json::Value>(&text)
                    .ok()
                    .and_then(|package| {
                        Some(
                            package["scripts"]
                                .as_object()?
                                .values()
                                .filter_map(|script| script.as_str())
                                .collect::<Vec<_>>()
                                .join("\n"),
                        )
                    })
                    .unwrap_or_default()
            } else {
                text
            };

            texts.push(((*entry).to_owned(), text));
        }
    }
    texts
}

/// The entry files: [`ENTRIES`], the tests, and every project file a script file names.
fn entries(tree: &Tree, scripts: &[(String, String)]) -> BTreeSet<String> {
    let files: BTreeSet<&str> = tree.files.iter().map(String::as_str).collect();
    let mut entries: BTreeSet<String> = ENTRIES
        .iter()
        .filter(|entry| files.contains(**entry))
        .map(|entry| (*entry).to_owned())
        .collect();

    entries.extend(
        tree.files
            .iter()
            .filter(|path| path.ends_with(".test.ts"))
            .cloned(),
    );
    for (_, text) in scripts {
        entries.extend(
            words(text)
                .filter(|word| files.contains(word))
                .map(str::to_owned),
        );
    }
    entries
}

/// The analysis: the files reached from the entries, and which export of which file is used.
struct Graph<'a> {
    modules: &'a HashMap<String, (String, Module)>,
    root: &'a Path,
    used: BTreeSet<(String, String)>,
    all_used: BTreeSet<String>,
}

impl Graph<'_> {
    /// The project file an import of `from` names, when it names one.
    fn target(&self, from: &str, specifier: &str) -> Option<String> {
        if !is_relative(specifier) {
            return None;
        }
        resolve(self.root, from, specifier).filter(|path| self.modules.contains_key(path))
    }

    /// Marks an export of a file used, following its re-exports and `export *`.
    fn mark(&mut self, file: &str, name: &str) {
        if !self.used.insert((file.to_owned(), name.to_owned())) {
            return;
        }
        let modules = self.modules;
        let Some((_, module)) = modules.get(file) else {
            return;
        };

        match module.exports.iter().find(|export| export.name == name) {
            Some(export) => {
                if let Some((specifier, taken)) = &export.from
                    && let Some(target) = self.target(file, specifier)
                {
                    self.take(&target, taken);
                }
                // An exported type its declaration names is used with it, as knip counts it.
                for named in &export.names {
                    if module
                        .exports
                        .iter()
                        .any(|other| other.name == *named && other.is_type && other.from.is_none())
                    {
                        self.mark(file, named);
                    }
                }
            }
            None => {
                for star in &module.stars {
                    if let Some(target) = self.target(file, star) {
                        self.mark(&target, name);
                    }
                }
            }
        }
    }

    /// Marks every export of a file used, `export *` included.
    fn mark_all(&mut self, file: &str) {
        if !self.all_used.insert(file.to_owned()) {
            return;
        }
        let modules = self.modules;
        let Some((_, module)) = modules.get(file) else {
            return;
        };

        for export in &module.exports {
            self.mark(file, &export.name);
        }
        for star in &module.stars {
            if let Some(target) = self.target(file, star) {
                self.mark_all(&target);
            }
        }
    }

    /// Marks what an import takes of a file.
    fn take(&mut self, file: &str, taken: &Taken) {
        match taken {
            Taken::Name(name) => self.mark(file, name),
            Taken::All => self.mark_all(file),
            Taken::Nothing => {}
        }
    }
}

/// The files reached from the entries through relative imports, the entries included.
fn reached(graph: &Graph, entries: &BTreeSet<String>) -> BTreeSet<String> {
    let mut reached: BTreeSet<String> = BTreeSet::new();
    let mut pending: Vec<String> = entries.iter().cloned().collect();

    while let Some(file) = pending.pop() {
        if !reached.insert(file.clone()) {
            continue;
        }
        if let Some((_, module)) = graph.modules.get(&file) {
            pending.extend(
                module
                    .imports
                    .iter()
                    .filter_map(|import| graph.target(&file, &import.specifier)),
            );
        }
    }
    reached
}

/// A located finding in a file's text.
fn finding(file: &str, text: &str, offset: u32, name: &str, kind: &'static str) -> Finding {
    let (line, column) = modules::line_column(text, offset);

    Finding {
        file: file.to_owned(),
        line,
        column,
        name: name.to_owned(),
        kind,
    }
}

/// The binaries of a listed package, from its installed `package.json`: its `bin` names, or its
/// own name without its scope when it is not installed.
fn binaries(root: &Path, package: &str) -> Vec<String> {
    let manifest: serde_json::Value =
        fs::read_to_string(root.join("node_modules").join(package).join("package.json"))
            .ok()
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default();
    let own = package.rsplit('/').next().unwrap_or(package).to_owned();

    match &manifest["bin"] {
        serde_json::Value::Object(bins) => bins.keys().cloned().collect(),
        serde_json::Value::String(_) => vec![own],
        _ if manifest.is_null() => vec![own],
        _ => Vec::new(),
    }
}

/// The packages `tsconfig.json`'s `types` names: `node` is `@types/node`, `vite/client` `vite`.
fn tsconfig_types(root: &Path) -> Vec<String> {
    let text = fs::read_to_string(root.join("tsconfig.json")).unwrap_or_default();
    let Some(start) = text.find("\"types\"") else {
        return Vec::new();
    };
    let list = text[start..]
        .split_once('[')
        .and_then(|(_, rest)| rest.split_once(']'))
        .map_or("", |(list, _)| list);

    list.split(',')
        .map(|item| item.trim().trim_matches('"'))
        .filter(|item| !item.is_empty())
        .map(|item| match package_of(item) {
            Some(package) if package != item => package.to_owned(),
            _ => format!("@types/{item}"),
        })
        .collect()
}

/// The listed packages of a `package.json` section, with where each is written.
fn listed(package_json: &str, section: &str) -> Vec<(String, u32)> {
    let package: serde_json::Value = serde_json::from_str(package_json).unwrap_or_default();
    let start = package_json.find(&format!("\"{section}\"")).unwrap_or(0);

    package[section]
        .as_object()
        .map(|names| {
            names
                .keys()
                .map(|name| {
                    let at = package_json[start..]
                        .find(&format!("\"{name}\""))
                        .map_or(0, |at| start + at + 1);

                    (name.clone(), u32::try_from(at).unwrap_or(0))
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Reads the imports of the reached files: marks what they take, and records the packages they
/// name, the packages listed nowhere and the relative imports that name no file.
fn read_imports(
    graph: &mut Graph,
    reached: &BTreeSet<String>,
    listed: &BTreeSet<&str>,
    used_packages: &mut BTreeSet<String>,
    findings: &mut Findings,
) {
    let modules = graph.modules;
    let mut seen: BTreeSet<(String, u32)> = BTreeSet::new();

    for file in reached {
        let Some((text, module)) = modules.get(file) else {
            continue;
        };

        if text.contains(HAPPY_DOM) {
            used_packages.insert("happy-dom".to_owned());
        }
        for Import {
            specifier,
            taken,
            offset,
        } in &module.imports
        {
            if let Some(package) = package_of(specifier) {
                used_packages.insert(package.to_owned());
                if !listed.contains(package) && seen.insert((file.clone(), *offset)) {
                    findings
                        .unlisted
                        .push(finding(file, text, *offset, package, ""));
                }
            } else if is_relative(specifier) {
                match graph.target(file, specifier) {
                    Some(target) => graph.take(&target, taken),
                    None if resolve(graph.root, file, specifier).is_none()
                        && seen.insert((file.clone(), *offset)) =>
                    {
                        findings
                            .unresolved
                            .push(finding(file, text, *offset, specifier, ""));
                    }
                    None => {}
                }
            }
        }
    }
}

/// The listed packages nothing uses: no import, no `tsconfig.json` type, no plugin, no binary a
/// script file names, and not [`IGNORED`].
fn unused_packages(
    root: &Path,
    scripts: &[(String, String)],
    mut used_packages: BTreeSet<String>,
    package_json: &str,
    findings: &mut Findings,
) {
    let script_words: BTreeSet<&str> = scripts.iter().flat_map(|(_, text)| words(text)).collect();

    used_packages.extend(tsconfig_types(root));
    for (package, file, needle) in PLUGIN_USES {
        if fs::read_to_string(root.join(file)).is_ok_and(|text| text.contains(needle)) {
            used_packages.insert((*package).to_owned());
        }
    }
    let is_unused = |name: &str| {
        !used_packages.contains(name)
            && !IGNORED.contains(&name)
            && !binaries(root, name)
                .iter()
                .any(|binary| script_words.contains(binary.as_str()))
    };

    for (target, section) in [
        (&mut findings.dependencies, "dependencies"),
        (&mut findings.dev_dependencies, "devDependencies"),
    ] {
        target.extend(
            listed(package_json, section)
                .iter()
                .filter(|(name, _)| is_unused(name))
                .map(|(name, offset)| finding("package.json", package_json, *offset, name, "")),
        );
    }
}

/// The exports and exported types of the reached files, entries left out, that nothing uses.
fn unused_exports(
    graph: &Graph,
    reached: &BTreeSet<String>,
    entries: &BTreeSet<String>,
    findings: &mut Findings,
) {
    for file in reached.iter().filter(|file| !entries.contains(*file)) {
        let Some((text, module)) = graph.modules.get(file) else {
            continue;
        };

        for export in &module.exports {
            if !graph.used.contains(&(file.clone(), export.name.clone())) {
                let found = finding(file, text, export.offset, &export.name, export.kind);

                if export.is_type {
                    findings.types.push(found);
                } else {
                    findings.exports.push(found);
                }
            }
        }
    }
}

/// The findings of a tree.
#[must_use]
pub fn check(tree: &Tree) -> Findings {
    let modules = read_modules(tree);
    let scripts = script_texts(&tree.root);
    let entries = entries(tree, &scripts);
    let mut graph = Graph {
        modules: &modules,
        root: &tree.root,
        used: BTreeSet::new(),
        all_used: BTreeSet::new(),
    };
    let reached = reached(&graph, &entries);
    let mut findings = Findings {
        files: tree
            .files
            .iter()
            .filter(|file| !reached.contains(*file))
            .cloned()
            .collect(),
        ..Findings::default()
    };
    let package_json = fs::read_to_string(tree.root.join("package.json")).unwrap_or_default();
    let all_listed: Vec<(String, u32)> = listed(&package_json, "dependencies")
        .into_iter()
        .chain(listed(&package_json, "devDependencies"))
        .collect();
    let names: BTreeSet<&str> = all_listed.iter().map(|(name, _)| name.as_str()).collect();
    let mut used_packages = BTreeSet::new();

    read_imports(
        &mut graph,
        &reached,
        &names,
        &mut used_packages,
        &mut findings,
    );
    unused_packages(
        &tree.root,
        &scripts,
        used_packages,
        &package_json,
        &mut findings,
    );
    unused_exports(&graph, &reached, &entries, &mut findings);
    for list in [
        &mut findings.unlisted,
        &mut findings.unresolved,
        &mut findings.exports,
        &mut findings.types,
    ] {
        list.sort();
    }
    findings
}

/// The project files of a checkout: tracked or untracked and not ignored, as `git ls-files` lists
/// them, that exist and are `.ts` files of [`PROJECT`].
#[must_use]
pub fn project_files(root: &Path, listed: &str) -> Vec<String> {
    let mut files: Vec<String> = listed
        .split('\0')
        .filter(|path| is_project(path) && root.join(path).is_file())
        .map(str::to_owned)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    files.sort();
    files
}

/// The Git arguments that list the project's files with `-z`.
#[must_use]
pub fn ls_files_arguments() -> Vec<String> {
    let mut arguments: Vec<String> = [
        "ls-files",
        "--cached",
        "--others",
        "--exclude-standard",
        "-z",
        "--",
    ]
    .map(str::to_owned)
    .to_vec();

    arguments.extend(PROJECT.iter().map(|entry| {
        if entry.starts_with('*') {
            format!(":(glob){entry}")
        } else {
            (*entry).to_owned()
        }
    }));
    arguments
}
