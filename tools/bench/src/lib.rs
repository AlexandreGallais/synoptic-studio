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
    if arguments == ["--help"] {
        return Ok(Request::Help);
    }
    let Some(separator) = arguments.iter().position(|argument| argument == "--") else {
        return Err("no `--` before the command".to_owned());
    };
    let (options, command) = arguments.split_at(separator);
    let command = command.get(1..).unwrap_or_default();
    let [flag, count] = options else {
        return Err(format!(
            "`--runs <count>` alone before `--`, not {options:?}"
        ));
    };

    if flag != "--runs" {
        return Err(format!("`--runs <count>` before `--`, not `{flag}`"));
    }
    let runs = count
        .parse::<usize>()
        .ok()
        .filter(|runs| *runs >= 1)
        .ok_or_else(|| format!("the count `{count}` is not a whole number of at least 1"))?;

    if command.is_empty() {
        return Err("no command after `--`".to_owned());
    }
    Ok(Request::Bench {
        runs,
        command: command.to_vec(),
    })
}

/// The median of durations: the middle one, or the mean of the two middle ones rounded down;
/// `None` for none.
#[must_use]
pub fn median(values: &[u64]) -> Option<u64> {
    let mut sorted = values.to_vec();
    let middle = values.len() / 2;

    sorted.sort_unstable();
    if values.len() % 2 == 1 {
        return sorted.get(middle).copied();
    }
    // `low + (high - low) / 2` rather than `(low + high) / 2`, which overflows near `u64::MAX`.
    let low = sorted.get(middle.checked_sub(1)?)?;
    let high = sorted.get(middle)?;
    Some(low + (high - low) / 2)
}

/// A duration in microseconds, written in whole milliseconds rounded to the nearest: `210 ms`.
#[must_use]
pub fn milliseconds(micros: u64) -> String {
    format!("{} ms", micros / 1000 + u64::from(micros % 1000 >= 500))
}

/// The three load averages of `/proc/loadavg` (`0.52 0.58 0.59 1/389 12345`) or of
/// `sysctl -n vm.loadavg` (`{ 2.38 6.92 6.58 }`, a decimal comma under some locales), written with
/// a decimal point; `None` when the text holds fewer than three.
#[must_use]
pub fn load_average(text: &str) -> Option<String> {
    let numbers: Vec<String> = text
        .split_whitespace()
        .map(|word| word.replace(',', "."))
        .filter(|word| {
            word.chars()
                .all(|character| character.is_ascii_digit() || character == '.')
                && word.parse::<f64>().is_ok()
        })
        .take(3)
        .collect();

    (numbers.len() == 3).then(|| numbers.join(" "))
}

/// `run` or `runs`, after a count.
const fn runs_word(count: usize) -> &'static str {
    if count == 1 { "run" } else { "runs" }
}

/// Runs the bench: the load average before, `runs` runs, each printed as it ends, the median of the
/// passing ones and the load average after; a first run that fails stops it with one error line.
/// Returns the first failing status, 0 when every run passed.
pub fn bench(
    runs: usize,
    mut run_once: impl FnMut() -> Run,
    mut load: impl FnMut() -> String,
    mut print: impl FnMut(Line),
) -> u8 {
    let mut passing = Vec::new();
    let mut first_failure = 0;

    print(Line::Out(format!("load average before: {}", load())));
    for index in 1..=runs {
        let run = run_once();
        let duration = milliseconds(run.micros);

        if run.code == 0 {
            passing.push(run.micros);
            print(Line::Out(format!("run {index}: {duration}")));
        } else if index == 1 {
            print(Line::Error(format!(
                "bench: the first run failed with exit {}: nothing measured",
                run.code
            )));
            return run.code;
        } else {
            if first_failure == 0 {
                first_failure = run.code;
            }
            print(Line::Out(format!(
                "run {index}: {duration}, failed with exit {}",
                run.code
            )));
        }
    }
    let over = if passing.len() == runs {
        format!("over {runs} {}", runs_word(runs))
    } else {
        format!(
            "over {} passing {} of {runs}",
            passing.len(),
            runs_word(passing.len())
        )
    };

    print(Line::Out(format!(
        "median: {} {over}",
        milliseconds(median(&passing).unwrap_or_default())
    )));
    print(Line::Out(format!("load average after: {}", load())));
    first_failure
}
