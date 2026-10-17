//! `lint [--fix] <files or folders…>` (EN-075): the project's linter, run from the root by
//! `npm run lint` after oxlint, on the same targets. Prints each finding as
//! `file:line:column rule message`, then a count; fails when it finds anything. `--fix` writes
//! the fixes of the autofixed rules and prints the findings left.

use std::process::ExitCode;

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let root = std::env::current_dir().unwrap_or_default();
    let (report, clean) = lint::run(&root, &arguments);

    print!("{report}");
    if clean {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
