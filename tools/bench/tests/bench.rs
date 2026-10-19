//! Tests of the bench's decisions (EN-141): the call read, the median, the durations and load
//! averages written, and the runs, given fake runs and loads, so that no test reads the clock.
//! `f33_ac3_…` marks a test proving F33's third criterion.

use std::cell::Cell;

use bench::{Line, Request, Run, bench, load_average, median, milliseconds, parse};

/// The arguments, as owned strings.
fn arguments(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| (*word).to_owned()).collect()
}

/// A standard output line.
fn out(text: &str) -> Line {
    Line::Out(text.to_owned())
}

/// A run passed after `millis` milliseconds.
const fn passed(millis: u64) -> Run {
    Run {
        micros: millis * 1000,
        code: 0,
    }
}

/// The lines printed, the status returned, the runs made and the loads read by a bench of `runs`
/// over the given results, the loads read being `load 1`, `load 2`…
fn bench_of(runs: usize, results: &[Run]) -> (Vec<Line>, u8, usize, usize) {
    let made = Cell::new(0);
    let read = Cell::new(0);
    let mut lines = Vec::new();
    let code = bench(
        runs,
        || {
            made.set(made.get() + 1);
            results[made.get() - 1]
        },
        || {
            read.set(read.get() + 1);
            format!("load {}", read.get())
        },
        |line| lines.push(line),
    );

    (lines, code, made.get(), read.get())
}

#[test]
fn reads_the_runs_and_the_command_after_the_separator() {
    assert_eq!(
        parse(&arguments(&[
            "--runs", "5", "--", "npx", "vitest", "run", "a b.ts"
        ])),
        Ok(Request::Bench {
            runs: 5,
            command: arguments(&["npx", "vitest", "run", "a b.ts"])
        })
    );
    // After `--`, every word is the command's, its own `--runs` and `--` included.
    assert_eq!(
        parse(&arguments(&["--runs", "1", "--", "x", "--runs", "--"])),
        Ok(Request::Bench {
            runs: 1,
            command: arguments(&["x", "--runs", "--"])
        })
    );
    assert_eq!(parse(&arguments(&["--help"])), Ok(Request::Help));
}

#[test]
fn refuses_a_call_without_a_count_of_at_least_one_or_a_command() {
    for words in [
        &[][..],
        &["--runs"],
        &["--runs", "5"],
        &["--runs", "5", "--"],
        &["--runs", "0", "--", "x"],
        &["--runs", "-1", "--", "x"],
        &["--runs", "five", "--", "x"],
        &["--runs", "2.5", "--", "x"],
        &["--", "x"],
        &["5", "--", "x"],
        &["--runs", "5", "--runs", "6", "--", "x"],
        &["--runs", "5", "--quiet", "--", "x"],
        &["--help", "--runs", "5", "--", "x"],
    ] {
        let parsed = parse(&arguments(words));
        assert!(
            parsed.as_ref().is_err_and(|reason| !reason.is_empty()),
            "{words:?}: {parsed:?}"
        );
    }
}

#[test]
fn takes_the_middle_duration_or_the_mean_of_the_two_middle_ones() {
    assert_eq!(median(&[]), None);
    assert_eq!(median(&[7]), Some(7));
    // Sorted 1, 3, 5: the middle one.
    assert_eq!(median(&[5, 1, 3]), Some(3));
    // Sorted 1, 2, 3, 4: (2 + 3) / 2 = 2.5, rounded down to 2.
    assert_eq!(median(&[4, 1, 3, 2]), Some(2));
    // Sorted 2, 6: (2 + 6) / 2 = 4.
    assert_eq!(median(&[6, 2]), Some(4));
    // Two of the largest: their mean, without overflowing their sum.
    assert_eq!(median(&[u64::MAX, u64::MAX]), Some(u64::MAX));
    assert_eq!(median(&[u64::MAX - 1, u64::MAX]), Some(u64::MAX - 1));
}

#[test]
fn writes_a_duration_in_whole_milliseconds_rounded_to_the_nearest() {
    assert_eq!(milliseconds(0), "0 ms");
    // 0.499 ms rounds down, 0.5 ms up.
    assert_eq!(milliseconds(499), "0 ms");
    assert_eq!(milliseconds(500), "1 ms");
    // 210.4 ms.
    assert_eq!(milliseconds(210_400), "210 ms");
    // 1,234.567 ms.
    assert_eq!(milliseconds(1_234_567), "1235 ms");
    assert_eq!(milliseconds(u64::MAX), "18446744073709552 ms");
}

#[test]
fn reads_the_three_load_averages_of_linux_and_of_macos() {
    assert_eq!(
        load_average("0.52 0.58 0.59 1/389 12345\n").as_deref(),
        Some("0.52 0.58 0.59")
    );
    assert_eq!(
        load_average("{ 2.38 6.92 6.58 }\n").as_deref(),
        Some("2.38 6.92 6.58")
    );
    // `sysctl` under a French locale.
    assert_eq!(
        load_average("{ 2,38 6,92 6,58 }\n").as_deref(),
        Some("2.38 6.92 6.58")
    );
    assert_eq!(load_average(""), None);
    assert_eq!(load_average("{ 1.00 2.00 }"), None);
    assert_eq!(load_average("not a load"), None);
}

#[test]
fn f33_ac3_runs_the_command_5_times_and_prints_each_duration_the_median_and_the_loads() {
    let (lines, code, made, read) = bench_of(
        5,
        &[
            passed(210),
            passed(190),
            passed(250),
            passed(200),
            passed(205),
        ],
    );

    // Sorted 190, 200, 205, 210, 250: the median is 205 ms.
    assert_eq!(
        lines,
        [
            out("load average before: load 1"),
            out("run 1: 210 ms"),
            out("run 2: 190 ms"),
            out("run 3: 250 ms"),
            out("run 4: 200 ms"),
            out("run 5: 205 ms"),
            out("median: 205 ms over 5 runs"),
            out("load average after: load 2"),
        ]
    );
    assert_eq!((code, made, read), (0, 5, 2));
}

#[test]
fn f33_ac3_exits_with_the_first_failing_status_its_median_of_the_passing_runs() {
    let (lines, code, made, read) = bench_of(
        5,
        &[
            passed(100),
            Run {
                micros: 300_000,
                code: 3,
            },
            passed(120),
            Run {
                micros: 90_000,
                code: 4,
            },
            passed(110),
        ],
    );

    // The passing runs, sorted 100, 110, 120: the median is 110 ms.
    assert_eq!(
        lines,
        [
            out("load average before: load 1"),
            out("run 1: 100 ms"),
            out("run 2: 300 ms, failed with exit 3"),
            out("run 3: 120 ms"),
            out("run 4: 90 ms, failed with exit 4"),
            out("run 5: 110 ms"),
            out("median: 110 ms over 3 passing runs of 5"),
            out("load average after: load 2"),
        ]
    );
    assert_eq!((code, made, read), (3, 5, 2));
}

#[test]
fn f33_ac3_stops_at_a_first_run_that_fails_measuring_nothing() {
    let (lines, code, made, read) = bench_of(
        5,
        &[
            Run {
                micros: 5_000,
                code: 2,
            },
            passed(100),
        ],
    );

    assert_eq!(
        lines,
        [
            out("load average before: load 1"),
            Line::Error("bench: the first run failed with exit 2: nothing measured".to_owned()),
        ]
    );
    assert_eq!((code, made, read), (2, 1, 1));
}

#[test]
fn measures_a_single_run() {
    let (lines, code, made, read) = bench_of(1, &[passed(42)]);

    assert_eq!(
        lines,
        [
            out("load average before: load 1"),
            out("run 1: 42 ms"),
            out("median: 42 ms over 1 run"),
            out("load average after: load 2"),
        ]
    );
    assert_eq!((code, made, read), (0, 1, 2));
}

#[test]
fn says_when_every_run_after_the_first_failed() {
    let (lines, code, _, _) = bench_of(
        2,
        &[
            passed(10),
            Run {
                micros: 20_000,
                code: 1,
            },
        ],
    );

    assert_eq!(lines[3], out("median: 10 ms over 1 passing run of 2"));
    assert_eq!(code, 1);
}
