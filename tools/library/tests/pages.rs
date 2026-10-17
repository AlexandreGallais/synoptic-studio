//! Tests of the pages read through the library, on the cases of `library-pages.test.ts`, and of
//! the fetch by `curl` against a server of the test on the loopback address (EN-106). Every
//! library is in memory.

use std::cell::Cell;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::path::Path;
use std::thread;

use library::chunks::page_chunks;
use library::pages::{
    LibraryPage, Origin, Response, curl_arguments, curl_response, library_page, library_page_with,
    page_from_response, request_headers, table_of_contents,
};
use library::store::{StoredPage, open_library, save_page};
use library::time::{day_of, epoch_of, iso_date};
use rusqlite::Connection;

/// Markdown served for the tests.
const MARKDOWN: &str = "# Hooks\n\nIntro.\n\n## Input\n\nThe `duration_ms` field.";

/// A served Markdown page, as `curl` would give it.
fn markdown_response() -> Response {
    Response {
        status: 200,
        content_type: "text/markdown".to_owned(),
        body: MARKDOWN.to_owned(),
        ..Response::default()
    }
}

/// A response of a status alone.
fn status(code: u16) -> Response {
    Response {
        status: code,
        ..Response::default()
    }
}

fn memory() -> Connection {
    open_library(Path::new(":memory:")).unwrap()
}

#[test]
fn a_date_is_an_instant_a_calendar_date_at_midnight_utc_or_nothing() {
    assert_eq!(epoch_of("1970-01-02T00:00:00Z"), Ok(86_400_000));
    assert_eq!(epoch_of("1970-01-03"), Ok(172_800_000));
    assert_eq!(epoch_of(""), Ok(0));
    // Read by Temporal on Node.js 24: 10:00 at +02:00 is 08:00 UTC; the fraction cut to
    // milliseconds; `t`, a space, annotations, the basic format; a leap second as :59.
    assert_eq!(epoch_of("2026-09-01T10:00:00+02:00"), Ok(1_788_249_600_000));
    assert_eq!(epoch_of("2026-09-01T10:00:00.1239Z"), Ok(1_788_256_800_123));
    assert_eq!(epoch_of("2026-09-01 10:00:00Z[UTC]"), Ok(1_788_256_800_000));
    assert_eq!(epoch_of("20260901T1000Z"), Ok(1_788_256_800_000));
    assert_eq!(epoch_of("2026-06-30T23:59:60Z"), Ok(1_782_863_999_000));
    assert_eq!(epoch_of("2024-02-29"), Ok(1_709_164_800_000));
    assert_eq!(epoch_of("20260901"), Ok(1_788_220_800_000));
    // `t` is no time for the test of a time: the date alone is read.
    assert_eq!(epoch_of("2026-09-01t10:00Z"), Ok(1_788_220_800_000));
}

#[test]
fn a_date_temporal_refused_is_an_error() {
    for date in [
        "2026-09-01T10:00:00",
        "2026-02-30T10:00:00Z",
        "2026-09-01T10:00:00+2",
        "2026-02-30",
        "2026-13-01",
        "Sep 2026",
        "Sep 1, 2026",
    ] {
        assert_eq!(
            epoch_of(date),
            Err("RangeError: Invalid time value for Temporal".to_owned()),
            "{date}"
        );
    }
}

#[test]
fn a_day_is_written_from_epoch_milliseconds() {
    assert_eq!(day_of(1_760_000_000_000), "2025-10-09");
    assert_eq!(day_of(-1), "1969-12-31");
    assert_eq!(day_of(0), "not stated");
    assert_eq!(iso_date(0), "1970-01-01");
}

#[test]
fn markdown_is_kept_as_served_its_title_from_its_first_heading() {
    let response = Response {
        etag: "\"e\"".to_owned(),
        ..markdown_response()
    };

    assert_eq!(
        page_from_response("https://x.org/h.md", &response, 7).unwrap(),
        StoredPage {
            etag: "\"e\"".to_owned(),
            fetched_at: 7,
            last_modified: String::new(),
            markdown: MARKDOWN.to_owned(),
            modified_at: 0,
            title: "Hooks".to_owned(),
            url: "https://x.org/h.md".to_owned(),
        }
    );
}

#[test]
fn html_is_converted_and_an_untitled_page_named_by_its_address() {
    let response = Response {
        status: 200,
        content_type: "text/html".to_owned(),
        body: "<main><p>Body</p></main>".to_owned(),
        ..Response::default()
    };
    let page = page_from_response("https://x.org/p", &response, 7).unwrap();

    assert_eq!(
        (page.title.as_str(), page.markdown.as_str()),
        ("https://x.org/p", "Body")
    );
}

#[test]
fn a_page_stating_a_date_temporal_refuses_is_not_read() {
    let response = Response {
        status: 200,
        content_type: "text/html".to_owned(),
        body: "<meta property=\"article:modified_time\" content=\"2026-09-01T10:00:00\">"
            .to_owned(),
        ..Response::default()
    };

    assert!(page_from_response("https://x.org/p", &response, 7).is_err());
}

#[test]
fn a_change_only_is_asked_when_the_page_is_stored_with_a_tag_or_a_date() {
    let stored = StoredPage {
        etag: "\"e\"".to_owned(),
        last_modified: "Mon".to_owned(),
        ..StoredPage::default()
    };
    let names = |page: &StoredPage| -> Vec<&'static str> {
        request_headers(page)
            .into_iter()
            .map(|(name, _)| name)
            .collect()
    };

    assert_eq!(
        names(&stored),
        ["user-agent", "if-none-match", "if-modified-since"]
    );
    assert_eq!(names(&StoredPage::default()), ["user-agent"]);
}

#[test]
fn a_missing_page_is_fetched_once_then_read_from_the_library() {
    let database = memory();
    let calls = Cell::new(0);
    let fetch = |_: &str, _: &StoredPage| {
        calls.set(calls.get() + 1);
        Ok(markdown_response())
    };
    let first = library_page_with(&database, "https://x.org/h.md", false, &fetch).unwrap();
    let second = library_page_with(&database, "https://x.org/h.md", false, &fetch).unwrap();

    assert_eq!(
        (first.origin, second.origin),
        (Origin::Online, Origin::Library)
    );
    assert_eq!(calls.get(), 1);
    assert!(table_of_contents(&second).contains("1. Hooks › Input — "));
}

#[test]
fn the_stored_page_is_kept_when_the_site_says_it_did_not_change_or_refuses_it() {
    let database = memory();
    let stored = StoredPage {
        fetched_at: 1,
        markdown: MARKDOWN.to_owned(),
        title: "Hooks".to_owned(),
        url: "u".to_owned(),
        ..StoredPage::default()
    };

    save_page(&database, &stored, &page_chunks(MARKDOWN)).unwrap();

    let unchanged = library_page_with(&database, "u", true, &|_, _| Ok(status(304))).unwrap();
    let refused = library_page_with(&database, "u", true, &|_, _| Ok(status(404))).unwrap();
    let missing = library_page_with(&memory(), "v", false, &|_, _| Ok(status(404))).unwrap();

    assert_eq!(
        (unchanged.origin, unchanged.sections.len()),
        (Origin::Online, 2)
    );
    assert!(unchanged.page.fetched_at > 1);
    assert_eq!(
        (refused.origin, refused.sections.len()),
        (Origin::Failed, 2)
    );
    assert_eq!(
        (missing.page.title.as_str(), missing.page.url.as_str()),
        ("HTTP 404", "v")
    );
}

#[test]
fn a_fetch_that_gives_no_response_is_an_error() {
    let result = library_page_with(&memory(), "https://x.invalid/a", false, &|_, _| {
        Err("Error: curl: (6) Could not resolve host: x.invalid".to_owned())
    });

    assert_eq!(
        result,
        Err("Error: curl: (6) Could not resolve host: x.invalid".to_owned())
    );
}

#[test]
fn a_table_of_contents_gives_the_dates_the_origin_and_each_section_length_in_utf16_units() {
    let entry = LibraryPage {
        origin: Origin::Library,
        page: StoredPage {
            fetched_at: 1_760_000_000_000,
            title: "T".to_owned(),
            url: "https://x.org".to_owned(),
            ..StoredPage::default()
        },
        sections: page_chunks("Top 😀\n# A\nb"),
    };

    // "Top 😀" is 4 characters and the smiley's two UTF-16 units: 6.
    assert_eq!(
        table_of_contents(&entry),
        "# T\nhttps://x.org — read 2025-10-09 (library), changed not stated\n\n0. (top) — 6 characters\n1. A — 5 characters"
    );
}

#[test]
fn curl_is_asked_for_http_only_its_facts_after_a_mark() {
    let arguments = curl_arguments("-x", &[("user-agent", "a".to_owned())]);

    assert!(
        arguments
            .windows(2)
            .any(|pair| pair == ["--proto", "=http,https"])
    );
    assert!(
        arguments
            .windows(2)
            .any(|pair| pair == ["--header", "user-agent: a"])
    );
    // The address is given as the value of `--url`, never read as an option.
    assert_eq!(arguments[arguments.len() - 2..], ["--url", "-x"]);
}

#[test]
fn the_response_curl_gave_is_read_from_its_body_and_its_facts() {
    let facts = "\nlibrary-curl-facts\n200\ntext/html; charset=utf-8\n\"v1\"\nMon, 12 Oct 2026 10:00:00 GMT\n";
    let response = curl_response(true, b"\xEF\xBB\xBF<p>x</p>", facts).unwrap();

    assert_eq!(
        response,
        Response {
            status: 200,
            content_type: "text/html; charset=utf-8".to_owned(),
            etag: "\"v1\"".to_owned(),
            last_modified: "Mon, 12 Oct 2026 10:00:00 GMT".to_owned(),
            body: "<p>x</p>".to_owned(),
        }
    );
    assert_eq!(
        curl_response(
            false,
            b"",
            "curl: (6) Could not resolve host: x.invalid\n\nlibrary-curl-facts\n000\n\n\n\n"
        ),
        Err("Error: curl: (6) Could not resolve host: x.invalid".to_owned())
    );
    assert_eq!(
        curl_response(false, b"", ""),
        Err("Error: curl failed".to_owned())
    );
    assert!(curl_response(true, b"", "").is_err());
}

/// A server on the loopback address answering each request with the next response, its requests
/// given back when it is done.
fn serve(responses: Vec<&'static str>) -> (String, thread::JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());
    let server = thread::spawn(move || {
        responses
            .into_iter()
            .map(|response| {
                let (mut stream, _) = listener.accept().unwrap();
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut request = String::new();

                loop {
                    let mut line = String::new();

                    reader.read_line(&mut line).unwrap();

                    if line == "\r\n" || line.is_empty() {
                        break;
                    }

                    request.push_str(&line);
                }

                stream.write_all(response.as_bytes()).unwrap();

                request
            })
            .collect()
    });

    (address, server)
}

#[test]
fn a_page_is_fetched_by_curl_then_asked_again_conditionally() {
    let (address, server) = serve(vec![
        "HTTP/1.1 200 OK\r\nContent-Type: text/markdown\r\nETag: \"v1\"\r\nConnection: close\r\n\r\n# Hooks\n\nIntro.\n\n## Input\n\nThe `duration_ms` field.",
        "HTTP/1.1 304 Not Modified\r\nConnection: close\r\n\r\n",
        "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
    ]);
    let database = memory();
    let url = format!("{address}/h.md");
    let first = library_page(&database, &url, false).unwrap();
    let second = library_page(&database, &url, true).unwrap();
    let third = library_page(&database, &url, true).unwrap();
    let requests = server.join().unwrap();

    assert_eq!(
        (first.origin, first.page.etag.as_str(), first.sections.len()),
        (Origin::Online, "\"v1\"", 2)
    );
    assert_eq!((second.origin, second.sections.len()), (Origin::Online, 2));
    assert_eq!((third.origin, third.sections.len()), (Origin::Failed, 2));
    assert!(requests[0].contains("synoptic-studio-library/1"));
    assert!(!requests[0].to_lowercase().contains("if-none-match"));
    assert!(requests[1].to_lowercase().contains("if-none-match: \"v1\""));
}

#[test]
fn a_connection_refused_is_an_error_with_curl_message() {
    // A port just freed: nothing listens on it.
    let port = TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let result = library_page(&memory(), &format!("http://127.0.0.1:{port}/a"), false);

    assert!(result.unwrap_err().starts_with("Error: curl: (7)"));
}

#[test]
fn a_scheme_other_than_http_is_refused_by_curl() {
    let result = library_page(&memory(), "file:///etc/hosts", false);

    assert!(result.unwrap_err().starts_with("Error: curl: (1)"));
}
