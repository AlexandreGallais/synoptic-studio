//! Tests of the `bench` binary (EN-141) on commands of the shell, each counting its runs in a file
//! of its own; the durations themselves are the machine's, so only the lines' shape is checked.
//! Unix only: the commands are `sh`'s, and Windows has no load average to read.
#![cfg(unix)]

use std::fs;
use std::path::PathBuf;
use std::process::Output;

/// A fresh file counting the runs of a test's command.
fn counter(name: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("bench-{name}-{}", std::process::id()));
    let _ = fs::remove_file(&path);
    path
}

/// The runs a command counted, one line each.
fn counted(path: &PathBuf) -> usize {
    fs::read_to_string(path).map_or(0, |text| text.lines().count())
}

/// `bench` run with its arguments.
fn bench(arguments: &[&str]) -> Output {
    test_support::command(env!("CARGO_BIN_EXE_bench"))
        .args(arguments)
        .output()
        .expect("bench runs")
}

#[test]
fn f33_ac3_runs_a_command_5_times_and_prints_each_duration_the_median_and_the_loads() {
    let path = counter("passing");
    let output = bench(&[
        "--runs",
        "5",
        "--",
        "sh",
        "-c",
        "echo run >> \"$0\"",
        path.to_str().expect("a path"),
    ]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    assert_eq!(output.status.code(), Some(0), "{stdout}");
    assert_eq!(counted(&path), 5);
    assert_eq!(lines.len(), 8, "{stdout}");
    assert!(lines[0].starts_with("load average before: "), "{stdout}");
    for (index, line) in lines[1..6].iter().enumerate() {
        assert!(
            line.starts_with(&format!("run {}: ", index + 1)),
            "{stdout}"
        );
        assert!(line.ends_with(" ms"), "{stdout}");
    }
    assert!(lines[6].starts_with("median: "), "{stdout}");
    assert!(lines[6].ends_with(" ms over 5 runs"), "{stdout}");
    assert!(lines[7].starts_with("load average after: "), "{stdout}");
    // The load is read on this machine, `sysctl` or `/proc/loadavg`: three numbers.
    assert_ne!(lines[0], "load average before: unknown", "{stdout}");
    assert_eq!(lines[0].split(' ').count(), 6, "{stdout}");
}

#[test]
fn f33_ac3_stops_at_a_first_run_that_fails_and_shows_its_output() {
    let path = counter("failing");
    let output = bench(&[
        "--runs",
        "5",
        "--",
        "sh",
        "-c",
        "echo run >> \"$0\"; echo its output; echo its error >&2; exit 5",
        path.to_str().expect("a path"),
    ]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(5));
    assert_eq!(counted(&path), 1);
    assert_eq!(stdout.lines().count(), 1, "{stdout}");
    assert!(stdout.starts_with("load average before: "), "{stdout}");
    assert!(stderr.contains("its output\n"), "{stderr}");
    assert!(stderr.contains("its error\n"), "{stderr}");
    assert!(
        stderr.ends_with("bench: the first run failed with exit 5: nothing measured\n"),
        "{stderr}"
    );
}

#[test]
fn f33_ac3_goes_on_after_a_later_failure_and_exits_with_its_status() {
    let path = counter("second");
    // The second run, its counter at 2 lines, fails with 7.
    let output = bench(&[
        "--runs",
        "3",
        "--",
        "sh",
        "-c",
        "echo run >> \"$0\"; test \"$(wc -l < \"$0\")\" -ne 2 || exit 7",
        path.to_str().expect("a path"),
    ]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert_eq!(output.status.code(), Some(7), "{stdout}");
    assert_eq!(counted(&path), 3);
    assert!(stdout.contains(" ms, failed with exit 7\n"), "{stdout}");
    assert!(
        stdout.contains(" ms over 2 passing runs of 3\n"),
        "{stdout}"
    );
}

#[test]
fn says_a_command_that_cannot_start_and_exits_127() {
    let output = bench(&["--runs", "3", "--", "/nonexistent/bench-command"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(127));
    assert!(
        stderr.contains("bench: the first run failed with exit 127: nothing measured\n"),
        "{stderr}"
    );
    assert!(
        stderr.contains("bench: cannot run `/nonexistent/bench-command`: "),
        "{stderr}"
    );
}

#[test]
fn prints_its_usage_on_help_and_on_a_wrong_call() {
    let help = bench(&["--help"]);
    assert_eq!(help.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&help.stdout).starts_with("usage: bench --runs <count> -- "));

    let wrong = bench(&["--runs", "0", "--", "true"]);
    let stderr = String::from_utf8_lossy(&wrong.stderr);
    assert_eq!(wrong.status.code(), Some(2));
    assert!(stderr.starts_with("bench: "), "{stderr}");
    assert!(
        stderr.contains("usage: bench --runs <count> -- "),
        "{stderr}"
    );
    assert_eq!(String::from_utf8_lossy(&wrong.stdout), "");
}
