//! The cargo tools the Rust checks run, `cargo-llvm-cov` and `cargo-deny` (EN-121): installed by
//! `cargo install`, so in neither `Cargo.toml` nor `Cargo.lock`. Their versions are read where they
//! are written — the `CARGO_<TOOL>_VERSION` of each workflow's `env`, the
//! `cargo install --locked <tool>@<version>` lines of `package.json`, the workflows and
//! `docs/tooling/`, and the rows of `docs/tooling/dependencies.md`, Rust tools — then held to one
//! version per tool, its row there, and its latest release on crates.io 7 days old.

/// The page whose Rust tools table gives each tool its row.
pub const DEPENDENCIES_PAGE: &str = "docs/tooling/dependencies.md";

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

/// The tools of a workflow's `env`: each `CARGO_<TOOL>_VERSION: x.y.z`, its key read as the
/// crate's name in lowercase with dashes (`CARGO_LLVM_COV_VERSION` is `cargo-llvm-cov`).
#[must_use]
pub fn env_pins(workflow: &str, place: &str) -> Vec<ToolPin> {
    let _ = (workflow, place);

    Vec::new()
}

/// The tools of each `cargo install --locked <tool>@<version>` of a text, every word after
/// `--locked` that is a name, `@` and a version number; `@${{ env.… }}` is no version.
#[must_use]
pub fn install_pins(text: &str, place: &str) -> Vec<ToolPin> {
    let _ = (text, place);

    Vec::new()
}

/// The tools of the Rust tools table of `docs/tooling/dependencies.md`: each row whose second
/// cell opens with a version number (`| cargo-deny | 0.20.2, 2026-07-09 | …`).
#[must_use]
pub fn row_pins(page: &str) -> Vec<ToolPin> {
    let _ = page;

    Vec::new()
}

/// The lines of the tools whose places disagree: one written at two versions, naming each
/// version's places, and one written without its row in `docs/tooling/dependencies.md`.
#[must_use]
pub fn disagreements(pins: &[ToolPin]) -> Vec<String> {
    let _ = pins;

    Vec::new()
}
