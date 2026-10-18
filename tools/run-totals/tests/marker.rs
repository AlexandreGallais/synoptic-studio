//! Tests of the run's marker (EN-092).

use run_totals::marker::{Run, closed, opened, parse_run, render_run};

fn run(start: i64, end: i64, sessions: &[&str]) -> Run {
    Run {
        start,
        end,
        sessions: sessions.iter().map(|&session| session.to_owned()).collect(),
    }
}

#[test]
fn writes_and_reads_a_marker() {
    let open = run(1_000, 0, &["a", "b"]);
    let text = render_run(&open);

    assert_eq!(
        text,
        "{\"start\":1000,\"end\":0,\"sessions\":[\"a\",\"b\"]}\n"
    );
    assert_eq!(parse_run(&text), Some(open));
}

#[test]
fn reads_no_run_from_a_text_without_a_start() {
    assert_eq!(parse_run("{}"), None);
    assert_eq!(parse_run("not json"), None);
    assert_eq!(parse_run("{\"start\":0,\"sessions\":[\"a\"]}"), None);
    // An empty session id is dropped.
    assert_eq!(
        parse_run("{\"start\":5,\"sessions\":[\"\",\"a\"]}"),
        Some(run(5, 0, &["a"]))
    );
}

#[test]
fn opens_a_run_when_none_is_open() {
    assert_eq!(opened(None, "a", 5), run(5, 0, &["a"]));
    // A run that ended is replaced by a new one.
    assert_eq!(opened(Some(&run(1, 3, &["a"])), "b", 9), run(9, 0, &["b"]));
}

#[test]
fn adds_a_new_session_to_the_open_run_without_restarting_it() {
    assert_eq!(
        opened(Some(&run(1, 0, &["a"])), "b", 9),
        run(1, 0, &["a", "b"])
    );
    // The same session again, after a compaction: nothing changes.
    assert_eq!(opened(Some(&run(1, 0, &["a"])), "a", 9), run(1, 0, &["a"]));
}

#[test]
fn closes_a_run_once() {
    assert_eq!(closed(&run(1, 0, &["a"]), 7), run(1, 7, &["a"]));
    assert_eq!(closed(&run(1, 4, &["a"]), 7), run(1, 4, &["a"]));
}

#[test]
fn measures_a_run_to_now_while_open_and_to_its_end_once_closed() {
    // 4 000 − 1 000 = 3 000 ms open; 2 500 − 1 000 = 1 500 ms once ended at 2 500.
    assert_eq!(run(1_000, 0, &["a"]).duration_ms(4_000), 3_000);
    assert_eq!(run(1_000, 2_500, &["a"]).duration_ms(4_000), 1_500);
}
