//! Writes the library's declaration files, one `.d.ts` per module of `src/` but its tests, each
//! from its module alone with Oxc's isolated declarations (EN-091, `REF-OXC-ISOLATED-DECLARATIONS`,
//! `REF-TS-ISOLATED-DECLARATIONS`), in place of `tsc -p tsconfig.build.json`: 0.06 s instead of
//! 0.63 s for the 589 files, the same declarations in meaning.
//!
//! Usage: `declarations <source folder> <output folder>`. A module whose declarations cannot be
//! written from it alone (an exported constant whose type is not written nor read from a plain
//! value, TypeScript's `TS9010`) is reported with its place, and nothing is written.

use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use oxc_allocator::Allocator;
use oxc_codegen::Codegen;
use oxc_isolated_declarations::{IsolatedDeclarations, IsolatedDeclarationsOptions};
use oxc_parser::Parser;
use oxc_span::SourceType;

/// The TypeScript modules under a folder, its tests left out, in a stable order.
fn modules(folder: &Path) -> io::Result<Vec<PathBuf>> {
    let mut found = Vec::new();

    for entry in fs::read_dir(folder)? {
        let path = entry?.path();

        if path.is_dir() {
            found.extend(modules(&path)?);
        } else if path.extension() == Some(OsStr::new("ts"))
            && !path.to_string_lossy().ends_with(".test.ts")
        {
            found.push(path);
        }
    }

    found.sort();
    Ok(found)
}

/// The declarations of one module, or the reasons they cannot be written from it alone.
fn declarations(source: &str) -> Result<String, Vec<String>> {
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, source, SourceType::ts()).parse();
    let options = IsolatedDeclarationsOptions {
        strip_internal: false,
    };
    let built = IsolatedDeclarations::new(&allocator, options).build(&parsed.program);
    let errors: Vec<String> = parsed
        .diagnostics
        .iter()
        .chain(&built.diagnostics)
        .map(|diagnostic| {
            // The line of the diagnostic's first label, counted from 1.
            let line = diagnostic.labels.first().map_or(0, |label| {
                source[..label.offset() as usize].matches('\n').count() + 1
            });
            format!("line {line}: {}", diagnostic.message)
        })
        .collect();

    if errors.is_empty() {
        Ok(Codegen::new().build(&built.program).code)
    } else {
        Err(errors)
    }
}

/// Writes every declaration file, or returns the errors of the modules that cannot have one.
fn run(source: &Path, output: &Path) -> io::Result<Result<usize, Vec<String>>> {
    let mut written = Vec::new();
    let mut errors = Vec::new();

    for module in modules(source)? {
        let relative = module.strip_prefix(source).unwrap_or(&module);

        match declarations(&fs::read_to_string(&module)?) {
            Ok(code) => written.push((output.join(relative.with_extension("d.ts")), code)),
            Err(found) => errors.extend(
                found
                    .into_iter()
                    .map(|error| format!("{}, {error}", module.display())),
            ),
        }
    }

    if !errors.is_empty() {
        return Ok(Err(errors));
    }

    for (path, code) in &written {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, code)?;
    }

    Ok(Ok(written.len()))
}

/// The usage, printed on `--help` or `-h` (REV-009).
const USAGE: &str = "usage: declarations <source folder> <output folder>";

fn main() -> ExitCode {
    if let Some(code) = tool_use::usage(&std::env::args().skip(1).collect::<Vec<_>>(), USAGE) {
        return code;
    }
    let args: Vec<String> = std::env::args().skip(1).collect();

    tool_use::run("declarations", "", || declare(&args))
}

/// Writes the declarations; the exit code.
fn declare(args: &[String]) -> u8 {
    let [source, output] = args else {
        eprintln!("usage: declarations <source folder> <output folder>");
        return 2;
    };

    match run(Path::new(source), Path::new(output)) {
        Ok(Ok(_)) => 0,
        Ok(Err(errors)) => {
            for error in errors {
                eprintln!("{error}");
            }
            eprintln!(
                "declarations: write the type of each exported constant above whose value is not \
                 a plain literal (TypeScript's isolatedDeclarations, EN-091)"
            );
            1
        }
        Err(error) => {
            eprintln!("declarations: {source}: {error}");
            2
        }
    }
}
