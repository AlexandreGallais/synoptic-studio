//! Tests of the watchdog of a background task (EN-107): the cases of `scripts/watch-task.test.ts`,
//! then the whole watch on a clock and a file of its own.

use metrics::watch::{
    Watch, WatchRules, Watching, last_line, option, seconds_ms, watch_step, watch_until,
};

/// Stall after 120 s; the watch ends on a line starting with "deployments: none".
fn rules() -> WatchRules {
    WatchRules {
        stall_ms: 120_000.0,
        until: "deployments: none".to_owned(),
    }
}

/// State after a first look at 0 s, 10 bytes read.
const FRESH: Watch = Watch {
    changed_at: 0,
    is_stalled: false,
    size: 10,
};

#[test]
fn reads_the_last_non_empty_line() {
    assert_eq!(last_line("a\nb\n\n"), "b");
    assert_eq!(last_line("  a  \n \n"), "a");
    assert_eq!(last_line(""), "");
}

#[test]
fn stays_quiet_while_the_output_grows() {
    let step = watch_step(FRESH, 5000, "runs: batch 1\n", &rules());

    assert!(!step.is_done);
    assert_eq!(step.message, "");
    // 14 bytes now, changed at 5 s.
    assert_eq!(
        step.watch,
        Watch {
            changed_at: 5000,
            is_stalled: false,
            size: 14
        }
    );
}

#[test]
fn reports_a_stall_once_then_the_output_coming_back() {
    let text = "0123456789";
    let stalled = watch_step(FRESH, 125_000, text, &rules());
    let again = watch_step(stalled.watch, 130_000, text, &rules());
    let back = watch_step(
        again.watch,
        135_000,
        &format!("{text}\nbatch 2\n"),
        &rules(),
    );

    // Silent from 0 to 125 s: 125 s.
    assert_eq!(
        stalled.message,
        "stalled: no output for 125 s — last line: 0123456789"
    );
    assert_eq!(again.message, "");
    assert_eq!(back.message, "resumed: batch 2");
    assert!(!back.watch.is_stalled);
}

#[test]
fn ends_on_any_new_line_matching_or_when_the_task_exited() {
    let text = "0123456789\ndeployments: none left\n\n[exited with code 0]\n";
    let exited = watch_step(FRESH, 5000, "0123456789\n[exited with code 1]\n", &rules());

    assert_eq!(
        watch_step(FRESH, 5000, text, &rules()).message,
        "done: deployments: none left"
    );
    assert_eq!(exited.message, "done: [exited with code 1]");
    assert!(exited.is_done);
    // Not an exit line: no code, or text after it.
    assert!(!watch_step(FRESH, 5000, "0123456789\n[exited with code ]\n", &rules()).is_done);
    assert!(
        !watch_step(
            FRESH,
            5000,
            "0123456789\n[exited with code 1] x\n",
            &rules()
        )
        .is_done
    );
}

#[test]
fn reads_only_the_lines_added_since_the_last_look() {
    // The end pattern was in the first 10 bytes, already read: not a new line.
    let old = watch_step(
        FRESH,
        5000,
        "deployments: none\nmore\n",
        &WatchRules {
            stall_ms: 120_000.0,
            until: "deploy".to_owned(),
        },
    );

    assert!(!old.is_done);
    // No end pattern: only an exit line ends the watch.
    assert!(
        !watch_step(
            FRESH,
            5000,
            "0123456789\ndeployments: none\n",
            &WatchRules {
                stall_ms: 120_000.0,
                until: String::new(),
            }
        )
        .is_done
    );
}

#[test]
fn reads_its_options_as_the_script_did() {
    let options: Vec<String> = ["out.txt", "--stall", "30", "--until"]
        .iter()
        .map(|word| (*word).to_owned())
        .collect();

    assert_eq!(option(&options, "--stall", "120"), "30");
    // An option at the end of the line, or absent, takes its default.
    assert_eq!(option(&options, "--until", ""), "");
    assert_eq!(option(&options, "--max", "1800"), "1800");
    assert!((seconds_ms("30") - 30_000.0).abs() < f64::EPSILON);
    assert!((seconds_ms("0.5") - 500.0).abs() < f64::EPSILON);
    assert!(seconds_ms("soon").is_nan());
}

/// A clock moving 5 s at each pause, over a file whose text changes at given looks.
struct Scripted {
    now: i64,
    looks: Vec<&'static str>,
    look: usize,
}

impl Watching for Scripted {
    fn now(&mut self) -> i64 {
        self.now
    }

    fn read(&mut self) -> String {
        let text = self.looks[self.look.min(self.looks.len() - 1)];

        self.look += 1;
        text.to_owned()
    }

    fn pause(&mut self) {
        self.now += 5000;
    }
}

/// The lines a watch prints over `looks`, ending at `end_at`.
fn watched(looks: Vec<&'static str>, stall_ms: f64, end_at: f64) -> String {
    let mut scripted = Scripted {
        now: 0,
        looks,
        look: 0,
    };
    let mut out = Vec::new();

    watch_until(
        &mut scripted,
        &WatchRules {
            stall_ms,
            until: "done".to_owned(),
        },
        end_at,
        &mut out,
    )
    .expect("written");
    String::from_utf8(out).expect("text")
}

#[test]
fn prints_a_stall_its_end_and_the_tasks_end_as_they_come() {
    // Looks at 5, 10, 15, 20 and 25 s: `a`, unchanged twice (silent 5 then 10 s, a stall at 10),
    // `b`, then the end pattern.
    let lines = watched(
        vec!["a\n", "a\n", "a\n", "a\nb\n", "a\nb\ndone well\n"],
        10_000.0,
        60_000.0,
    );

    assert_eq!(
        lines,
        "stalled: no output for 10 s — last line: a\nresumed: b\ndone: done well\n"
    );
}

#[test]
fn ends_when_its_time_runs_out_with_the_last_line() {
    // A limit of 12 s: looks at 5, 10 and 15 s, over at the third.
    assert_eq!(watched(vec!["x\n"], 600_000.0, 12_000.0), "watch over: x\n");
    // A limit that is not a number never passes: the watch ends on the task's end only.
    assert_eq!(
        watched(
            vec!["x\n", "x\n[exited with code 2]\n"],
            600_000.0,
            f64::NAN
        ),
        "done: [exited with code 2]\n"
    );
}
