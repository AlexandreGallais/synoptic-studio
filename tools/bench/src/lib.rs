//! A command's duration over runs (EN-141, ADR-0069): `bench --runs 5 -- <command>` runs it 5
//! times, prints each duration, then their median, between the load average before and after, so
//! that a story setting a limit from a measure (`alone: 210 ms`, `.claude/rules/tooling.md`) writes
//! neither its runs nor its median by hand. A first run that fails stops the bench, measuring
//! nothing: the command is broken, not slow. A later run that fails is printed with its status and
//! left out of the median; the bench exits with the first failing status.
//!
//! The decisions live here, without the clock nor a process: the binary runs the command, times it
//! and reads the load average (functional core, ADR-0014).

/// The usage, printed by `--help` on standard output and on a wrong call on standard error.
pub const USAGE: &str = "usage: bench --runs <count> -- <command> [<argument>…]\n\
runs the command <count> times, at least 1, and prints each duration, their median and the load\n\
average before and after; a first run that fails stops it, measuring nothing, and it exits with\n\
the command's first failing status\n";

/// What a call asks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    /// `--help`: the usage, exit 0.
    Help,
    /// The runs and the command, its program first.
    Bench {
        /// How many times the command runs, at least 1.
        runs: usize,
        /// The program and its arguments, as given after `--`.
        command: Vec<String>,
    },
}

/// One run of the command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Run {
    /// Its duration, in microseconds.
    pub micros: u64,
    /// Its exit status, 0 when it passed.
    pub code: u8,
}

/// A line the bench prints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Line {
    /// On standard output: a duration, the median, a load average.
    Out(String),
    /// On standard error: why nothing was measured.
    Error(String),
}

/// The request of the arguments after the binary's name; the reason, a wrong call.
///
/// # Errors
///
/// No `--runs`, a count that is not a whole number of at least 1, no `--`, no command after it, or
/// an argument before `--` other than `--runs <count>`.
pub fn parse(arguments: &[String]) -> Result<Request, String> {
    let _ = arguments;
    Err(String::new())
}

/// The median of durations: the middle one, or the mean of the two middle ones rounded down;
/// `None` for none.
#[must_use]
pub fn median(values: &[u64]) -> Option<u64> {
    let _ = values;
    None
}

/// A duration in microseconds, written in whole milliseconds rounded to the nearest: `210 ms`.
#[must_use]
pub fn milliseconds(micros: u64) -> String {
    let _ = micros;
    String::new()
}

/// The three load averages of `/proc/loadavg` (`0.52 0.58 0.59 1/389 12345`) or of
/// `sysctl -n vm.loadavg` (`{ 2.38 6.92 6.58 }`, a decimal comma under some locales), written with
/// a decimal point; `None` when the text holds fewer than three.
#[must_use]
pub fn load_average(text: &str) -> Option<String> {
    let _ = text;
    None
}

/// Runs the bench: the load average before, `runs` runs, each printed as it ends, the median of the
/// passing ones and the load average after; a first run that fails stops it with one error line.
/// Returns the first failing status, 0 when every run passed.
pub fn bench(
    runs: usize,
    run_once: impl FnMut() -> Run,
    load: impl FnMut() -> String,
    print: impl FnMut(Line),
) -> u8 {
    let _ = (runs, run_once, load, print);
    0
}
