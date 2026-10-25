//! `stale-binaries [<checkout>]` (the last step of the `check:rust` check, EN-104, ADR-0078): prints each release
//! binary older than its sources, or missing, with the rebuild that refreshes it, and exits 1;
//! prints nothing and exits 0 when every binary is fresh. The checkout is the current folder's
//! when none is named.

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

/// The usage, printed on `--help` or `-h` (REV-009).
const USAGE: &str = "usage: target/release/stale-binaries [<checkout>]";

fn main() -> ExitCode {
    if let Some(code) = tool_use::usage(&std::env::args().skip(1).collect::<Vec<_>>(), USAGE) {
        return code;
    }
    tool_use::run("stale-binaries", "", || {
        let root = env::args()
            .nth(1)
            .map(PathBuf::from)
            .or_else(|| env::current_dir().ok())
            .unwrap_or_default();
        let stale = stale_binaries::stale_binaries(&root);

        for line in &stale {
            println!("{line}");
        }
        if stale.is_empty() {
            0
        } else {
            eprintln!(
                "stale-binaries: {} release binaries older than their sources: run `cargo build --release --workspace --locked`",
                stale.len()
            );
            1
        }
    })
}
