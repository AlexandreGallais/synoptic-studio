//! `val-prepare <repository root> <feature>` (EN-056): writes the feature's lines in its epic's
//! checklist and prints the address that opens it, the feature read from the records by
//! `knowledge sql` (EN-154). `target/release/val-prepare . F18`.

use std::fs;
use std::path::Path;
use std::process::ExitCode;

use val_prepare::{feature_id, feature_statement, feature_text_of, plan};

/// The usage, printed on `--help` or `-h` (REV-009).
const USAGE: &str = "usage: target/release/val-prepare . <feature, F18>";

fn main() -> ExitCode {
    if let Some(code) = tool_use::usage(&std::env::args().skip(1).collect::<Vec<_>>(), USAGE) {
        return code;
    }
    let args: Vec<String> = std::env::args().skip(1).collect();

    tool_use::run("val-prepare", "", || prepare(&args))
}

/// Writes the feature's checklist lines; its exit code.
fn prepare(args: &[String]) -> u8 {
    let (Some(root), Some(id)) = (args.first(), args.get(1).and_then(|arg| feature_id(arg))) else {
        eprintln!("usage: target/release/val-prepare . <feature, F18>");
        return 2;
    };
    let root = Path::new(root);

    let text = feature_statement(&id)
        .ok_or_else(|| format!("{id}: not a feature id"))
        .and_then(|statement| knowledge_call::rows(root, &statement))
        .and_then(|rows| feature_text_of(&id, &rows));
    let prepared = match text.and_then(|text| plan(root, &id, &text)) {
        Ok(prepared) => prepared,
        Err(message) => {
            eprintln!("val-prepare: {message}");
            return 1;
        }
    };

    for write in &prepared.writes {
        if let Err(error) = fs::write(root.join(&write.path), &write.text) {
            eprintln!("val-prepare: {}: {error}", write.path.display());
            return 1;
        }
        println!("written {}", write.path.display());
    }

    if let Some(warning) = &prepared.warning {
        println!("{warning}");
    }

    println!(
        "Rewrite each line as what to try and what to see, run `npm run fix`, then open {}",
        prepared.address
    );

    0
}
