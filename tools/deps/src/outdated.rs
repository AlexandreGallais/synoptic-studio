//! `deps outdated`: every version the repository pins against its latest version at least 7 days
//! old (ADR-0038) — the npm packages of `package.json`, the GitHub Actions of the workflows,
//! Node.js's major in `.nvmrc`, the crates of `[workspace.dependencies]` — and every crate of
//! `Cargo.lock` published less than 7 days ago, which Cargo picks by itself (research note 0025,
//! The crates, checked). One line per finding, with the version and its date.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::thread;

use crate::actions::{self, ActionUse};
use crate::fetch::Answer;
use crate::{Behind, crates, node, npm};

/// The npm registry's address of a package's full metadata (`REF-NPM-PACKAGE-METADATA`).
const NPM_REGISTRY: &str = "https://registry.npmjs.org/";

/// The crates.io index over the sparse protocol (`REF-CARGO-REGISTRY-INDEX`).
const CRATES_INDEX: &str = "https://index.crates.io/";

/// Node.js's index of its releases (`REF-NODE-DIST-INDEX`).
const NODE_INDEX: &str = "https://nodejs.org/dist/index.json";

/// The source of `Cargo.lock`'s packages that come from crates.io.
const CRATES_SOURCE: &str = "registry+https://github.com/rust-lang/crates.io-index";

/// The network the check reads: addresses fetched over HTTP, and paths of GitHub's API.
pub trait Network: Sync {
    /// An answer per address, in their order.
    fn urls(&self, addresses: &[String]) -> Vec<Answer>;
    /// An answer per path of GitHub's API, in their order.
    fn github(&self, paths: &[String]) -> Vec<Answer>;
}

/// What the repository pins.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Pins {
    /// The npm packages of `package.json`, name and version.
    pub npm: Vec<(String, String)>,
    /// The actions of the workflows, each repository and major once.
    pub actions: Vec<ActionUse>,
    /// The major of `.nvmrc`.
    pub node: Option<u64>,
    /// The crates of `[workspace.dependencies]`, name and version without its `=`.
    pub crates: Vec<(String, String)>,
    /// The crates.io packages of `Cargo.lock`, name and version.
    pub locked: Vec<(String, String)>,
}

/// The findings: one line per version behind, and one per source that did not answer.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Report {
    /// The versions behind, and the locked crates too new.
    pub lines: Vec<String>,
    /// The requests without an answer, naming their source.
    pub failures: Vec<String>,
}

impl Report {
    /// The exit code: 2 when a source did not answer, 1 when a version is behind, 0 otherwise.
    #[must_use]
    pub fn exit_code(&self) -> u8 {
        if self.failures.is_empty() {
            u8::from(!self.lines.is_empty())
        } else {
            2
        }
    }
}

/// The packages of `package.json`'s `dependencies` and `devDependencies`, in their order.
#[must_use]
pub fn npm_pins(package: &serde_json::Value) -> Vec<(String, String)> {
    ["dependencies", "devDependencies"]
        .iter()
        .filter_map(|section| package[section].as_object())
        .flatten()
        .filter_map(|(name, version)| Some((name.clone(), version.as_str()?.to_owned())))
        .collect()
}

/// The quoted value of `key = "…"` inside a line of TOML, the first one.
fn quoted<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let at = line.find(&format!("{key} = \""))? + key.len() + 4;

    line[at..].split('"').next()
}

/// The crates of `Cargo.toml`'s `[workspace.dependencies]`, written `name = "=x.y.z"` or
/// `name = { version = "=x.y.z", … }`.
#[must_use]
pub fn crate_pins(manifest: &str) -> Vec<(String, String)> {
    manifest
        .lines()
        .skip_while(|line| line.trim() != "[workspace.dependencies]")
        .skip(1)
        .take_while(|line| !line.starts_with('['))
        .filter_map(|line| {
            let (name, value) = line.split_once(" = ")?;
            let version = if value.starts_with('{') {
                quoted(value, "version")?
            } else {
                value.trim().trim_matches('"')
            };

            Some((
                name.trim().to_owned(),
                version.trim_start_matches('=').to_owned(),
            ))
        })
        .collect()
}

/// The packages of `Cargo.lock` that come from crates.io, name and version.
#[must_use]
pub fn locked_pins(lock: &str) -> Vec<(String, String)> {
    lock.split("[[package]]")
        .skip(1)
        .filter_map(|block| {
            let field = |key: &str| block.lines().find_map(|line| quoted(line, key));

            (field("source")? == CRATES_SOURCE)
                .then(|| Some((field("name")?.to_owned(), field("version")?.to_owned())))?
        })
        .collect()
}

/// What a checkout pins, read from its files; a file missing pins nothing.
#[must_use]
pub fn read_pins(root: &Path) -> Pins {
    let read = |path: &str| fs::read_to_string(root.join(path)).unwrap_or_default();
    let package = serde_json::from_str(&read("package.json")).unwrap_or_default();
    let mut workflows: Vec<_> = fs::read_dir(root.join(".github/workflows"))
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok().map(|entry| entry.path()))
                .filter(|path| path.extension().is_some_and(|extension| extension == "yml"))
                .collect()
        })
        .unwrap_or_default();

    workflows.sort();
    let actions: BTreeSet<ActionUse> = workflows
        .iter()
        .flat_map(|path| actions::uses(&fs::read_to_string(path).unwrap_or_default()))
        .collect();

    Pins {
        npm: npm_pins(&package),
        actions: actions.into_iter().collect(),
        node: read(".nvmrc").trim().trim_start_matches('v').parse().ok(),
        crates: crate_pins(&read("Cargo.toml")),
        locked: locked_pins(&read("Cargo.lock")),
    }
}

/// A date written `2026-10-11`, or `date unknown`.
#[must_use]
pub fn date(published_ms: Option<i64>) -> String {
    published_ms.map_or_else(
        || "date unknown".to_owned(),
        |ms| {
            let (year, month, day) = tool_use::civil_from_days(ms.div_euclid(86_400_000));

            format!("{year}-{month:02}-{day:02}")
        },
    )
}

/// The line of a version behind: `npm vite 8.3.4 → 8.4.0, 2026-10-01`.
fn behind_line(kind: &str, behind: &Behind) -> String {
    format!(
        "{kind} {} {} → {}, {}",
        behind.name,
        behind.current,
        behind.latest,
        date(behind.published_ms)
    )
}

/// An answer read as JSON, or why it cannot be.
fn json(answer: &Answer) -> Result<serde_json::Value, String> {
    let body = answer.as_ref().map_err(Clone::clone)?;

    serde_json::from_str(body).map_err(|error| format!("not JSON: {error}"))
}

/// The address of a crate's file in the index.
fn index_address(name: &str) -> String {
    format!("{CRATES_INDEX}{}", crates::index_path(name))
}

/// The npm packages' lines and failures.
fn npm_report(pins: &Pins, answers: &[Answer], now_ms: i64, report: &mut Report) {
    for ((name, current), answer) in pins.npm.iter().zip(answers) {
        match json(answer) {
            Ok(metadata) => report.lines.extend(
                npm::behind(name, current, &metadata, now_ms).map(|b| behind_line("npm", &b)),
            ),
            Err(error) => report
                .failures
                .push(format!("npm registry: {name}: {error}")),
        }
    }
}

/// The Node.js line or failure.
fn node_report(pinned: u64, answer: &Answer, now_ms: i64, report: &mut Report) {
    match json(answer) {
        Ok(index) => match node::latest_lts(&index, now_ms) {
            Some(lts) if lts.major != pinned => report.lines.push(format!(
                "node .nvmrc {pinned} → {} ({}), {}",
                lts.major,
                lts.version,
                date(Some(lts.published_ms))
            )),
            Some(_) => {}
            None => report
                .failures
                .push("Node.js index: no LTS release 7 days old".to_owned()),
        },
        Err(error) => report.failures.push(format!("Node.js index: {error}")),
    }
}

/// The crates' lines and failures: `[workspace.dependencies]` behind, `Cargo.lock` too new.
fn crate_report(
    pins: &Pins,
    names: &[String],
    answers: &[Answer],
    now_ms: i64,
    report: &mut Report,
) {
    let releases: Vec<_> = names
        .iter()
        .zip(answers)
        .map(|(name, answer)| match answer {
            Ok(index) => Some((name, crates::releases(index))),
            Err(error) => {
                report
                    .failures
                    .push(format!("crates.io index: {name}: {error}"));
                None
            }
        })
        .collect();
    let releases_of = |name: &str| {
        releases
            .iter()
            .flatten()
            .find(|(known, _)| *known == name)
            .map(|(_, r)| r)
    };

    for (name, current) in &pins.crates {
        if let Some(behind) =
            releases_of(name).and_then(|r| crates::behind(name, current, r, now_ms))
        {
            report.lines.push(behind_line("crate", &behind));
        }
    }
    for (name, locked) in &pins.locked {
        if let Some(new) = releases_of(name).and_then(|r| crates::too_new(name, locked, r, now_ms))
        {
            report.lines.push(format!(
                "lock {name} {locked}, {}, under 7 days: {}",
                date(Some(new.published_ms)),
                new.pin.map_or_else(
                    || "no compatible version 7 days old".to_owned(),
                    |pin| format!("cargo update -p {name}@{locked} --precise {pin}")
                )
            ));
        }
    }
}

/// The actions' lines and failures: a next major whose first tag is 7 days old.
fn action_report(
    pins: &Pins,
    network: &impl Network,
    references: &[Answer],
    now_ms: i64,
    report: &mut Report,
) {
    let mut firsts = Vec::new();

    for (action, answer) in pins.actions.iter().zip(references) {
        match json(answer) {
            Ok(references) => {
                let tags = actions::major_tags(&references, action.major + 1);

                if let Some(tag) = actions::first_tag(&tags) {
                    firsts.push((action, tag.clone()));
                }
            }
            Err(error) => report
                .failures
                .push(format!("GitHub: {}: {error}", action.repository)),
        }
    }
    let paths: Vec<String> = firsts
        .iter()
        .map(|(action, tag)| format!("repos/{}/commits/tags/{tag}", action.repository))
        .collect();

    for ((action, tag), answer) in firsts.iter().zip(network.github(&paths)) {
        match json(&answer).map(|commit| actions::commit_ms(&commit)) {
            Ok(Some(ms)) if now_ms - ms < crate::COOLDOWN_MS => {}
            Ok(published) => report.lines.push(format!(
                "action {} v{} → v{} or later ({tag}), {}",
                action.repository,
                action.major,
                action.major + 1,
                date(published)
            )),
            Err(error) => report
                .failures
                .push(format!("GitHub: {}: {error}", action.repository)),
        }
    }
}

/// The check of every pin at `now_ms`, the network read once for each package, crate and action.
#[must_use]
pub fn outdated(pins: &Pins, network: &impl Network, now_ms: i64) -> Report {
    let names: Vec<String> = pins
        .crates
        .iter()
        .chain(&pins.locked)
        .map(|(name, _)| name.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let mut addresses: Vec<String> = pins
        .npm
        .iter()
        .map(|(name, _)| format!("{NPM_REGISTRY}{name}"))
        .collect();

    addresses.extend(names.iter().map(|name| index_address(name)));
    addresses.extend(pins.node.map(|_| NODE_INDEX.to_owned()));
    let paths: Vec<String> = pins
        .actions
        .iter()
        .map(|action| {
            format!(
                "repos/{}/git/matching-refs/tags/v{}",
                action.repository,
                action.major + 1
            )
        })
        .collect();
    let (answers, references) = thread::scope(|scope| {
        let github = scope.spawn(|| network.github(&paths));

        (network.urls(&addresses), github.join().unwrap_or_default())
    });
    let (npm_answers, rest) = answers.split_at(pins.npm.len().min(answers.len()));
    let (crate_answers, node_answers) = rest.split_at(names.len().min(rest.len()));
    let mut report = Report::default();

    npm_report(pins, npm_answers, now_ms, &mut report);
    action_report(pins, network, &references, now_ms, &mut report);
    if let (Some(pinned), Some(answer)) = (pins.node, node_answers.first()) {
        node_report(pinned, answer, now_ms, &mut report);
    }
    crate_report(pins, &names, crate_answers, now_ms, &mut report);
    report
}
