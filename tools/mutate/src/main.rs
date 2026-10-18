//! `mutate [--all] <files…> [-- <test command…>]` (`npm run mutate -- src/math/clamp.ts`): the first
//! mutation of each kind in each file, every one with `--all`, tried in a copy of the repository
//! against `npx vitest related {file} --run` or the command given — `-- npx vitest run {test}` runs
//! only the file's own test file, much faster where a barrel relates most of the suite to it;
//! exits 1 when one survives.

use std::env;
use std::process::ExitCode;
use std::time::Duration;

use mutate::{Outcome, Request, VITEST_RELATED, Verdict, run, summary};

/// Time a mutated run may last beyond four times the unchanged one.
const GRACE: Duration = Duration::from_secs(30);

fn main() -> ExitCode {
    let argv: Vec<String> = env::args().skip(1).collect();
    let mode = if argv.iter().any(|argument| argument == "--all") {
        "--all"
    } else {
        ""
    };

    tool_use::run("mutate", mode, || mutate(&argv))
}

/// Tries the mutations the arguments ask for; its exit code.
fn mutate(argv: &[String]) -> u8 {
    let (options, command) = argv
        .iter()
        .position(|argument| argument == "--")
        .map_or((argv, None), |at| (&argv[..at], Some(&argv[at + 1..])));
    let all = options.iter().any(|option| option == "--all");
    let files: Vec<String> = options.iter().filter(|o| *o != "--all").cloned().collect();
    let default: Vec<String> = VITEST_RELATED
        .iter()
        .map(|part| (*part).to_owned())
        .collect();
    let command = command
        .filter(|given| !given.is_empty())
        .unwrap_or(&default);

    if files.is_empty() {
        eprintln!(
            "usage: npm run mutate -- [--all] <files…> [-- <test command with {{file}} or {{test}}>]"
        );
        return 2;
    }
    let Ok(dir) = env::current_dir() else {
        eprintln!("mutate: no current folder");
        return 2;
    };
    let request = Request {
        dir: &dir,
        files: &files,
        command,
        all,
        grace: GRACE,
    };

    match run(&request, &mut |outcome| println!("{}", outcome.line())) {
        Ok(outcomes) => {
            println!("{}", summary(&outcomes));
            let survived = outcomes.iter().any(|outcome| {
                matches!(
                    outcome,
                    Outcome::Tried {
                        verdict: Verdict::Survived,
                        ..
                    }
                )
            });

            u8::from(survived)
        }
        Err(error) => {
            eprintln!("mutate: {error}");
            2
        }
    }
}
