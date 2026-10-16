//! `val-prepare <repository root> <feature>` (EN-056): writes the feature's lines in its epic's
//! checklist and prints the address that opens it. `npm run val:prepare -- F18`.

use std::fs;
use std::path::Path;
use std::process::ExitCode;

use val_prepare::{feature_id, plan};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (Some(root), Some(id)) = (args.first(), args.get(1).and_then(|arg| feature_id(arg))) else {
        eprintln!("usage: npm run val:prepare -- <feature, F18>");
        return ExitCode::from(2);
    };
    let root = Path::new(root);

    let prepared = match plan(root, &id) {
        Ok(prepared) => prepared,
        Err(message) => {
            eprintln!("val:prepare: {message}");
            return ExitCode::FAILURE;
        }
    };

    for write in &prepared.writes {
        if let Err(error) = fs::write(root.join(&write.path), &write.text) {
            eprintln!("val:prepare: {}: {error}", write.path.display());
            return ExitCode::FAILURE;
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

    ExitCode::SUCCESS
}
