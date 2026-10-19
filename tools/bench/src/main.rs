//! `bench --runs <count> -- <command> [<argument>…]` (EN-141): runs the command `count` times
//! from the current folder, its input closed and its output kept, and prints each duration, their
//! median and the load average before and after; a failing run's output goes to standard error.
//! Exits 2 on a wrong call, 127 when the command cannot start, else with its first failing status.

use std::env;
use std::fs;
use std::io::{self, Write as _};
use std::process::{Command, Stdio};
use std::time::Instant;

use bench::{Line, Request, Run, USAGE, bench, load_average, parse};

/// The status of a command that cannot start, as a shell gives it.
const NOT_STARTED: u8 = 127;

/// The load average of `/proc/loadavg` (Linux), else of `sysctl -n vm.loadavg` (macOS), else
/// `unknown` (Windows has none).
fn read_load() -> String {
    fs::read_to_string("/proc/loadavg")
        .ok()
        .and_then(|text| load_average(&text))
        .or_else(|| {
            Command::new("sysctl")
                .args(["-n", "vm.loadavg"])
                .env("LC_ALL", "C")
                .stderr(Stdio::null())
                .output()
                .ok()
                .and_then(|output| load_average(&String::from_utf8_lossy(&output.stdout)))
        })
        .unwrap_or_else(|| "unknown".to_owned())
}

/// One run of the command, timed; a failing run's output written to standard error.
fn run_once(command: &[String]) -> Result<Run, String> {
    let program = command.first().map_or("", String::as_str);
    let started = Instant::now();
    let output = Command::new(program)
        .args(command.get(1..).unwrap_or_default())
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("bench: cannot run `{program}`: {error}"))?;
    let micros = u64::try_from(started.elapsed().as_micros()).unwrap_or(u64::MAX);
    let code = match output.status.code() {
        Some(0) => 0,
        // Killed by a signal, or a status beyond a byte (Windows): a failure all the same.
        Some(code) => u8::try_from(code)
            .ok()
            .filter(|code| *code != 0)
            .unwrap_or(1),
        None => 1,
    };

    if code != 0 {
        let mut error = io::stderr();
        let _ = error.write_all(&output.stdout);
        let _ = error.write_all(&output.stderr);
    }
    Ok(Run { micros, code })
}

fn main() -> std::process::ExitCode {
    tool_use::run("bench", "", || {
        let arguments: Vec<String> = env::args().skip(1).collect();

        match parse(&arguments) {
            Ok(Request::Help) => {
                print!("{USAGE}");
                0
            }
            Err(reason) => {
                eprint!("bench: {reason}\n{USAGE}");
                2
            }
            Ok(Request::Bench { runs, command }) => {
                let mut not_started = None;
                let code = bench(
                    runs,
                    || {
                        run_once(&command).unwrap_or_else(|reason| {
                            not_started = Some(reason);
                            Run {
                                micros: 0,
                                code: NOT_STARTED,
                            }
                        })
                    },
                    read_load,
                    |line| match line {
                        Line::Out(text) => println!("{text}"),
                        Line::Error(text) => eprintln!("{text}"),
                    },
                );

                if let Some(reason) = not_started {
                    eprintln!("{reason}");
                }
                code
            }
        }
    })
}
