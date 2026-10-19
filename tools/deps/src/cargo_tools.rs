//! The cargo tools the Rust checks run, `cargo-llvm-cov` and `cargo-deny` (EN-121): installed by
//! `cargo install`, so in neither `Cargo.toml` nor `Cargo.lock`. Their versions are read where they
//! are written — the `CARGO_<TOOL>_VERSION` of each workflow's `env`, the
//! `cargo install --locked <tool>@<version>` lines of `package.json`, the workflows and
//! `docs/tooling/`, and the rows of `docs/tooling/dependencies.md`, Rust tools — then held to one
//! version per tool, its row there, and its latest release on crates.io 7 days old.

use std::collections::{BTreeMap, BTreeSet};

use crate::version::Version;

/// The page whose Rust tools table gives each tool its row.
pub const DEPENDENCIES_PAGE: &str = "docs/tooling/dependencies.md";

/// The heading of that table's section.
const RUST_TOOLS_HEADING: &str = "## Rust tools";

/// What closes a word of an install line in a page or a script: a code span, a quote, a comma.
const WORD_END: &[char] = &['`', '\'', '"', ',', ';', ')', '.'];

/// One version of a tool, where it is written.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ToolPin {
    /// The crate of the tool: `cargo-deny`.
    pub name: String,
    /// Its version as written: `0.20.2`.
    pub version: String,
    /// The file, from the checkout's root.
    pub place: String,
}

/// A pin, its version checked to be a version number.
fn pin(name: &str, version: &str, place: &str) -> Option<ToolPin> {
    Version::parse(version)?;

    Some(ToolPin {
        name: name.to_owned(),
        version: version.to_owned(),
        place: place.to_owned(),
    })
}

/// The tools of a workflow's `env`: each `CARGO_<TOOL>_VERSION: x.y.z`, its key read as the
/// crate's name in lowercase with dashes (`CARGO_LLVM_COV_VERSION` is `cargo-llvm-cov`).
#[must_use]
pub fn env_pins(workflow: &str, place: &str) -> Vec<ToolPin> {
    workflow
        .lines()
        .filter_map(|line| {
            let (key, value) = line.trim().split_once(": ")?;
            let tool = key.strip_prefix("CARGO_")?.strip_suffix("_VERSION")?;

            pin(
                &format!("cargo-{}", tool.to_lowercase().replace('_', "-")),
                value.trim(),
                place,
            )
        })
        .collect()
}

/// The tools of each `cargo install --locked <tool>@<version>` of a text, every word after
/// `--locked` that is a name, `@` and a version number; `@${{ env.… }}` is no version.
#[must_use]
pub fn install_pins(text: &str, place: &str) -> Vec<ToolPin> {
    text.split("cargo install ")
        .skip(1)
        .flat_map(|rest| {
            rest.split_whitespace()
                .skip_while(|word| word.starts_with('-'))
                .map_while(|word| {
                    let (name, version) = word.trim_end_matches(WORD_END).split_once('@')?;

                    pin(name, version, place)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

/// The tools of the Rust tools table of `docs/tooling/dependencies.md`: each row whose second
/// cell opens with a version number (`| cargo-deny | 0.20.2, 2026-07-09 | …`).
#[must_use]
pub fn row_pins(page: &str) -> Vec<ToolPin> {
    page.lines()
        .skip_while(|line| *line != RUST_TOOLS_HEADING)
        .skip(1)
        .take_while(|line| !line.starts_with("## "))
        .filter_map(|line| {
            let mut cells = line.strip_prefix('|')?.split('|').map(str::trim);
            let name = cells.next()?.trim_matches('`');
            let version = cells.next()?.split(',').next()?;

            pin(name, version, DEPENDENCIES_PAGE)
        })
        .collect()
}

/// The lines of the tools whose places disagree: one written at two versions, naming each
/// version's places, and one written without its row in `docs/tooling/dependencies.md`.
#[must_use]
pub fn disagreements(pins: &[ToolPin]) -> Vec<String> {
    let mut tools: BTreeMap<&str, BTreeMap<&str, BTreeSet<&str>>> = BTreeMap::new();

    for pin in pins {
        tools
            .entry(&pin.name)
            .or_default()
            .entry(&pin.version)
            .or_default()
            .insert(&pin.place);
    }
    tools
        .into_iter()
        .flat_map(|(name, versions)| {
            let has_row = versions
                .values()
                .any(|places| places.contains(DEPENDENCIES_PAGE));
            let mut by_precedence: Vec<_> = versions.into_iter().collect();

            by_precedence.sort_by_key(|(version, _)| Version::parse(version));
            let two = (by_precedence.len() > 1).then(|| {
                let each: Vec<String> = by_precedence
                    .iter()
                    .map(|(version, places)| {
                        format!(
                            "at {version} in {}",
                            places.iter().copied().collect::<Vec<_>>().join(", ")
                        )
                    })
                    .collect();

                format!("tool {name} {}", each.join(", "))
            });
            let rowless = (!has_row).then(|| {
                format!("tool {name} without its row in {DEPENDENCIES_PAGE} (Rust tools)")
            });

            two.into_iter().chain(rowless)
        })
        .collect()
}
