//! `watch-task <output file> [--stall 120] [--until <start of a line>] [--max 1800]` (EN-107): the
//! watchdog of a background task, run under Claude Code's Monitor (`docs/tooling/metrics.md`,
//! Watching a long task). It looks at the output every 5 seconds and prints a line only when the
//! output stalled for `--stall` seconds, came back, has a new line starting with `--until` or
//! saying the task exited, or when `--max` seconds passed. Its lines are those of `watch-task.ts`.

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::ExitCode;
use std::thread;
use std::time::Duration;

use calendar::now_ms;
use metrics::watch::{WatchRules, Watching, option, seconds_ms, watch_until};

/// Seconds between two looks at the file.
const PERIOD: Duration = Duration::from_secs(5);

/// The real clock and file, a look every 5 seconds.
struct Watched {
    file: PathBuf,
}

impl Watching for Watched {
    fn now(&mut self) -> i64 {
        now_ms()
    }

    fn read(&mut self) -> String {
        fs::read(&self.file)
            .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
            .unwrap_or_default()
    }

    fn pause(&mut self) {
        thread::sleep(PERIOD);
    }
}

fn main() -> ExitCode {
    let options: Vec<String> = env::args().skip(1).collect();

    tool_use::run("watch-task", "", || {
        let rules = WatchRules {
            stall_ms: seconds_ms(option(&options, "--stall", "120")),
            until: option(&options, "--until", "").to_owned(),
        };
        let mut watched = Watched {
            file: PathBuf::from(options.first().map_or("", String::as_str)),
        };
        // Times far below 2^53 milliseconds.
        #[allow(clippy::cast_precision_loss)]
        let end_at = now_ms() as f64 + seconds_ms(option(&options, "--max", "1800"));

        match watch_until(&mut watched, &rules, end_at, &mut io::stdout()) {
            Ok(()) => 0,
            Err(error) => {
                eprintln!("watch-task: {error}");
                1
            }
        }
    })
}
