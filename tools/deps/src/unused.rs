//! `dependencies unused`: the issue types knip reports by default (`REF-KNIP-ISSUE-TYPES`) that
//! this repository meets — unused files, unused dev packages, unlisted packages, unresolved
//! imports, unused exports and exported types — for its layout, written anew rather than ported:
//! one export per file (ADR-0019) and barrels of `export *`; imports by relative path, joined to
//! the importing file's folder (`.ts`, then `/index.ts`, tried after the path itself), or by bare
//! package name (research note 0025, no `oxc_resolver`). Its configuration is [`PROJECT`] and a
//! [`Config`] per npm project, [`ROOT`] and [`PLAYGROUND`] (ADR-0104), in place of `knip.json`
//! and knip's plugins; each project read from its own folder, its `package.json` and its
//! `node_modules`.

use std::collections::{BTreeSet, HashMap};
use std::fmt::Write as _;
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::modules::{self, Import, Module, Taken};

/// The folders and root files whose `.ts` files are checked: knip's `project`.
pub const PROJECT: &[&str] = &["*.ts", "docs/.vitepress/", "scripts/", "src/", "tests/"];

/// The playground, a second npm project with its own `package.json` (ADR-0104): its Angular app,
/// the helpers of its tests and its runner configurations, left out of [`PROJECT`] and checked from
/// its own folder by [`PLAYGROUND`] (EN-159, EN-175).
pub const SECOND_PROJECT: &str = "playground/";

/// The folder of the playground's npm project, its `package.json` and `node_modules` inside.
pub const PLAYGROUND_FOLDER: &str = "playground";

/// What the check reads of one npm project, in place of knip's configuration: its entry files
/// besides its tests, the files whose words name an entry or a binary, the packages a tool's
/// configuration uses, the packages never reported, and the names its `tsconfig.json`'s `paths`
/// map to a source file; each path from the project's folder.
#[derive(Debug, Clone, Copy)]
pub struct Config {
    /// The entry files besides the tests (`*.test.ts`).
    pub entries: &'static [&'static str],
    /// The files whose words may name an entry file or a package's binary, a folder by its `/`.
    pub script_files: &'static [&'static str],
    /// Each package a tool's configuration uses: the package, the file, the text naming it.
    pub plugin_uses: &'static [(&'static str, &'static str, &'static str)],
    /// The packages never reported.
    pub ignored: &'static [&'static str],
    /// Its folder from the checkout's root, `""` for the root's.
    pub folder: &'static str,
    /// Each bare name imported as a source file through the `paths` of `tsconfig.json`, no
    /// package, with that file from the checkout's root: a [`Use`] of it, neither listed nor
    /// unlisted.
    pub aliases: &'static [(&'static str, &'static str)],
}

/// The root's project: the library, its tests and its tools.
pub const ROOT: Config = Config {
    entries: ENTRIES,
    script_files: SCRIPT_FILES,
    plugin_uses: PLUGIN_USES,
    ignored: IGNORED,
    folder: "",
    aliases: &[],
};

/// The playground, from `playground/`: `src/main.ts`, the `browser` entry of `angular.json`, and
/// `vitest-base.config.ts`, the runner configuration of its `ng test`; its scripts and
/// `angular.json`, whose builders, binaries and runner configurations name packages and files;
/// `@angular/build` named by those builders, `sass` by the components' SCSS, `vite` by the
/// override that pins it; the library imported by its name `editor` through the tsconfig's
/// `paths` (ADR-0104). Not reported until their code comes: `@ngrx/signals`, the stores (F55).
pub const PLAYGROUND: Config = Config {
    entries: &["src/main.ts", "vitest-base.config.ts"],
    script_files: &["package.json", "angular.json"],
    plugin_uses: &[
        ("@angular/build", "angular.json", "\"@angular/build:"),
        ("sass", "angular.json", "\"style\": \"scss\""),
        ("vite", "package.json", "\"$vite\""),
    ],
    ignored: &["@ngrx/signals"],
    folder: PLAYGROUND_FOLDER,
    aliases: &[("editor", "src/index.ts")],
};

/// The entry files besides the tests (`*.test.ts`) and the files the scripts name: Vite's and
/// Vitest's configuration, the library's entry (`exports` of `package.json`), which the playground
/// imports by its name `editor` (ADR-0104), the site's configuration and theme.
pub const ENTRIES: &[&str] = &[
    "vite.config.ts",
    "src/index.ts",
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
    // oxlint's type-aware rules and type check, set in its configuration since EN-121.
    ("oxlint-tsgolint", ".oxlintrc.json", "\"typeAware\": true"),
    // The compiler `tsconfig.json` configures, whatever runs it.
    ("typescript", "tsconfig.json", "\"compilerOptions\""),
];

/// The comment that gives a test file Vitest's DOM, `happy-dom` (its row of the records'
/// `dependencies`).
const HAPPY_DOM: &str = "@vitest-environment happy-dom";

/// Dev packages never reported, each with its reason, as `knip.json`'s `ignoreDependencies` held
/// them: `fast-check`, the peer of `@fast-check/vitest` that the tests import it through; `oxfmt`,
/// run by `tools/format`.
pub const IGNORED: &[&str] = &["fast-check", "oxfmt"];

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

    /// The findings of another project added, each of its paths read from `folder`: the
    /// playground's `src/main.ts` becomes `playground/src/main.ts`.
    pub fn add(&mut self, other: Findings, folder: &str) {
        let moved = |finding: Finding| Finding {
            file: format!("{folder}/{}", finding.file),
            ..finding
        };

        self.files.extend(
            other
                .files
                .into_iter()
                .map(|file| format!("{folder}/{file}")),
        );
        for (target, found) in [
            (&mut self.dependencies, other.dependencies),
            (&mut self.dev_dependencies, other.dev_dependencies),
            (&mut self.unlisted, other.unlisted),
            (&mut self.unresolved, other.unresolved),
            (&mut self.exports, other.exports),
            (&mut self.types, other.types),
        ] {
            target.extend(found.into_iter().map(moved));
        }
    }
}

/// A checkout's text files, read once.
pub struct Tree {
    /// The checkout's root.
    pub root: PathBuf,
    /// The project's `.ts` files, from the root, sorted.
    pub files: Vec<String>,
}

/// Whether a path from the root is a project file: a `.ts` file at the root or in a project folder,
/// outside [`SECOND_PROJECT`].
#[must_use]
pub fn is_project(path: &str) -> bool {
    Path::new(path)
        .extension()
        .is_some_and(|extension| extension == "ts")
        && !path.starts_with(SECOND_PROJECT)
        && PROJECT.iter().any(|entry| match entry.strip_prefix("*") {
            Some(_) => !path.contains('/'),
            None => path.starts_with(entry),
        })
}

/// What a project takes of a file outside its folder (ADR-0104): the playground's import of the
/// library by its name `editor`, or of a fixture of the root's `tests/` by a relative path. The
/// root's check counts it as an import of that file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Use {
    /// The path imported, from the checkout's root, without its extension when written without.
    pub path: String,
    /// What the import takes.
    pub taken: Taken,
}

/// The path from the checkout's root that a relative specifier of a project's file names when it
/// leaves the project's folder; `None` when it stays inside, for the root's project (folder `""`),
/// or when it leaves the checkout too.
#[must_use]
pub fn outside(folder: &str, from: &str, specifier: &str) -> Option<String> {
    let parent = Path::new(from).parent().map_or(String::new(), |parent| {
        parent.to_string_lossy().into_owned()
    });
    let path = specifier.split('?').next().unwrap_or(specifier);

    if folder.is_empty() || joined(&parent, path).is_some() {
        return None;
    }
    joined(&format!("{folder}/{parent}"), path)
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

/// The modules of the project's files, by path, read on every core; a file Oxc cannot parse is an
/// empty module.
fn read_modules(tree: &Tree) -> HashMap<String, (String, Module)> {
    let threads = std::thread::available_parallelism().map_or(1, std::num::NonZero::get);
    let size = tree.files.len().div_ceil(threads).max(1);

    std::thread::scope(|scope| {
        let handles: Vec<_> = tree
            .files
            .chunks(size)
            .map(|chunk| {
                scope.spawn(move || {
                    chunk
                        .iter()
                        .map(|path| {
                            let text = fs::read_to_string(tree.root.join(path)).unwrap_or_default();
                            let module = modules::read(&text).unwrap_or_default();

                            (path.clone(), (text, module))
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();

        handles
            .into_iter()
            .flat_map(|handle| handle.join().unwrap_or_default())
            .collect()
    })
}

/// The texts of the script files, the files of a listed folder included, sorted.
fn script_texts(root: &Path, config: &Config) -> Vec<(String, String)> {
    let mut texts = Vec::new();

    for entry in config.script_files {
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

/// The entry files: the configuration's, the tests, and every project file a script file names.
fn entries(tree: &Tree, scripts: &[(String, String)], config: &Config) -> BTreeSet<String> {
    let files: BTreeSet<&str> = tree.files.iter().map(String::as_str).collect();
    let mut entries: BTreeSet<String> = config
        .entries
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
    /// Each relative specifier of each file, resolved once: the file system is asked once per
    /// import, not at each name a barrel passes on.
    targets: HashMap<(String, String), Option<String>>,
    used: BTreeSet<(String, String)>,
    all_used: BTreeSet<String>,
}

impl<'a> Graph<'a> {
    /// The graph of the modules, every relative specifier resolved.
    fn new(modules: &'a HashMap<String, (String, Module)>, root: &'a Path) -> Self {
        let targets = modules
            .iter()
            .flat_map(|(file, (_, module))| {
                module
                    .imports
                    .iter()
                    .filter(|import| is_relative(&import.specifier))
                    .map(move |import| (file.clone(), import.specifier.clone()))
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|(file, specifier)| {
                let target =
                    resolve(root, &file, &specifier).filter(|path| modules.contains_key(path));

                ((file, specifier), target)
            })
            .collect();

        Self {
            modules,
            root,
            targets,
            used: BTreeSet::new(),
            all_used: BTreeSet::new(),
        }
    }

    /// The project file an import of `from` names, when it names one.
    fn target(&self, from: &str, specifier: &str) -> Option<String> {
        self.targets
            .get(&(from.to_owned(), specifier.to_owned()))
            .cloned()
            .flatten()
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
    config: &Config,
    used_packages: &mut BTreeSet<String>,
    findings: &mut Findings,
    uses: &mut Vec<Use>,
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
            if let Some((_, path)) = config.aliases.iter().find(|(name, _)| *name == specifier) {
                uses.push(Use {
                    path: (*path).to_owned(),
                    taken: taken.clone(),
                });
            } else if let Some(package) = package_of(specifier) {
                used_packages.insert(package.to_owned());
                if !listed.contains(package) && seen.insert((file.clone(), *offset)) {
                    findings
                        .unlisted
                        .push(finding(file, text, *offset, package, ""));
                }
            } else if is_relative(specifier) {
                if let Some(target) = graph.target(file, specifier) {
                    graph.take(&target, taken);
                } else if let Some(path) = outside(config.folder, file, specifier) {
                    uses.push(Use {
                        path,
                        taken: taken.clone(),
                    });
                } else if resolve(graph.root, file, specifier).is_none()
                    && seen.insert((file.clone(), *offset))
                {
                    findings
                        .unresolved
                        .push(finding(file, text, *offset, specifier, ""));
                }
            }
        }
    }
}

/// The packages a used package asks as peers, its optional peers left out (`peerDependencies`
/// and `peerDependenciesMeta` of its installed `package.json`), and in turn theirs: such a package
/// is used through the one asking it, as `@angular/core` asks `rxjs` (EN-159).
fn add_peers(root: &Path, used_packages: &mut BTreeSet<String>) {
    let mut pending: Vec<String> = used_packages.iter().cloned().collect();

    while let Some(package) = pending.pop() {
        let manifest: serde_json::Value = fs::read_to_string(
            root.join("node_modules")
                .join(&package)
                .join("package.json"),
        )
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default();
        let Some(peers) = manifest["peerDependencies"].as_object() else {
            continue;
        };

        for peer in peers.keys() {
            let is_optional = manifest["peerDependenciesMeta"][peer]["optional"] == true;

            if !is_optional && used_packages.insert(peer.clone()) {
                pending.push(peer.clone());
            }
        }
    }
}

/// The listed packages nothing uses: no import, no `tsconfig.json` type, no plugin, no package
/// asking it as a peer, no binary a script file names, and not ignored by the configuration.
fn unused_packages(
    root: &Path,
    scripts: &[(String, String)],
    mut used_packages: BTreeSet<String>,
    package_json: &str,
    config: &Config,
    findings: &mut Findings,
) {
    let script_words: BTreeSet<&str> = scripts.iter().flat_map(|(_, text)| words(text)).collect();

    used_packages.extend(tsconfig_types(root));
    for (package, file, needle) in config.plugin_uses {
        if fs::read_to_string(root.join(file)).is_ok_and(|text| text.contains(needle)) {
            used_packages.insert((*package).to_owned());
        }
    }
    add_peers(root, &mut used_packages);
    let is_unused = |name: &str| {
        !used_packages.contains(name)
            && !config.ignored.contains(&name)
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

/// The findings of the root's tree.
#[must_use]
pub fn check(tree: &Tree) -> Findings {
    check_project(tree, &ROOT)
}

/// The findings of a project's tree, read by its configuration.
#[must_use]
pub fn check_project(tree: &Tree, config: &Config) -> Findings {
    check_with_uses(tree, config, &[]).0
}

/// The findings of a project's tree, read by its configuration, another project's [`Use`]s of its
/// files counted as imports from outside; with the project's own uses of files outside its folder.
#[must_use]
pub fn check_with_uses(tree: &Tree, config: &Config, given: &[Use]) -> (Findings, Vec<Use>) {
    let modules = read_modules(tree);
    let scripts = script_texts(&tree.root, config);
    let entries = entries(tree, &scripts, config);
    let mut graph = Graph::new(&modules, &tree.root);
    let imported: Vec<(String, &Taken)> = given
        .iter()
        .filter_map(|each| {
            resolve(&tree.root, "", &format!("./{}", each.path))
                .filter(|path| modules.contains_key(path))
                .map(|path| (path, &each.taken))
        })
        .collect();
    let starts: BTreeSet<String> = entries
        .iter()
        .cloned()
        .chain(imported.iter().map(|(path, _)| path.clone()))
        .collect();
    let reached = reached(&graph, &starts);
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
    let mut uses = Vec::new();

    read_imports(
        &mut graph,
        &reached,
        &names,
        config,
        &mut used_packages,
        &mut findings,
        &mut uses,
    );
    for (path, taken) in &imported {
        graph.take(path, taken);
    }
    unused_packages(
        &tree.root,
        &scripts,
        used_packages,
        &package_json,
        config,
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
    (findings, uses)
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

/// The files of the playground's Angular app among those `git ls-files` lists, that exist and are
/// `.ts` files of [`SECOND_PROJECT`]: each from the playground's folder (`src/main.ts`), sorted.
#[must_use]
pub fn playground_files(root: &Path, listed: &str) -> Vec<String> {
    let prefix = format!("{PLAYGROUND_FOLDER}/");

    listed
        .split('\0')
        .filter(|path| {
            path.starts_with(SECOND_PROJECT)
                && Path::new(path)
                    .extension()
                    .is_some_and(|extension| extension == "ts")
                && root.join(path).is_file()
        })
        .filter_map(|path| path.strip_prefix(&prefix).map(str::to_owned))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
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
    // The playground's files, checked as a project of its own (ADR-0104).
    arguments.push(SECOND_PROJECT.to_owned());
    arguments
}
