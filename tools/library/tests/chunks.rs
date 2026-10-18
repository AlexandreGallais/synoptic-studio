//! Tests of the sections a page is cut into and of its freshness, on the cases of
//! `library-chunks.test.ts` and the edges of the cut (EN-106).

use calendar::DAY_MS;
use library::chunks::{PageAge, is_stale, page_chunks};

/// Index and heading path of each chunk.
fn headings(markdown: &str) -> Vec<(i64, String)> {
    page_chunks(markdown)
        .into_iter()
        .map(|chunk| (chunk.index, chunk.heading))
        .collect()
}

#[test]
fn a_page_is_split_at_its_headings_the_path_kept_and_code_blocks_respected() {
    let markdown = [
        "Intro.",
        "# Guide",
        "Text.",
        "## Install",
        "```sh",
        "# not a heading",
        "```",
        "### Linux",
        "Steps.",
        "## Use",
        "Run it.",
    ]
    .join("\n");

    assert_eq!(
        headings(&markdown),
        [
            (0, String::new()),
            (1, "Guide".to_owned()),
            (2, "Guide › Install".to_owned()),
            (3, "Guide › Install › Linux".to_owned()),
            (4, "Guide › Use".to_owned()),
        ]
    );
    assert_eq!(
        page_chunks(&markdown)[2].text,
        "## Install\n```sh\n# not a heading\n```"
    );
}

#[test]
fn a_long_section_is_cut_between_paragraphs() {
    // Each paragraph has 500 × 5 − 1 = 2499 characters: two do not fit under 4000.
    let paragraph = "word ".repeat(500).trim().to_owned();
    let chunks = page_chunks(&format!("# Long\n\n{paragraph}\n\n{paragraph}"));

    assert_eq!(chunks.len(), 2);
    assert!(chunks.iter().all(|chunk| chunk.heading == "Long"));
    assert_eq!(chunks[1].text, paragraph);
}

#[test]
fn a_length_is_counted_in_utf16_code_units_as_javascript_counts_it() {
    // "😀" is one character and two UTF-16 units: 1000 of them are 2000 units, so a part of 1999
    // units and one of 2000 make 3999, under 4000, and stay one chunk; with 2001 they make 4000.
    let smileys = "😀".repeat(1000);
    let short = "a".repeat(1999);
    let long = "a".repeat(2000);

    assert_eq!(page_chunks(&format!("{short}\n\n{smileys}")).len(), 1);
    assert_eq!(page_chunks(&format!("{long}\n\n{smileys}")).len(), 2);
}

#[test]
fn a_heading_needs_one_to_six_hashes_and_a_space() {
    assert_eq!(
        headings("#NoSpace\n####### Seven\n###### Six"),
        [(0, String::new()), (1, "Six".to_owned())]
    );
}

#[test]
fn a_skipped_level_keeps_the_path_above_it() {
    // `### C` after `# A` keeps `A` and adds `C`; `## D` cuts the path at level 1.
    assert_eq!(
        headings("# A\n### C\n## D"),
        [
            (0, "A".to_owned()),
            (1, "A › C".to_owned()),
            (2, "A › D".to_owned())
        ]
    );
}

#[test]
fn an_empty_page_or_white_space_has_no_chunk() {
    assert_eq!(page_chunks(""), []);
    assert_eq!(page_chunks(" \n\u{FEFF}\n"), []);
}

#[test]
fn a_young_page_is_read_again_after_a_day() {
    let fetched_at = 100 * DAY_MS;
    let young = |now| PageAge {
        fetched_at,
        modified_at: fetched_at - 3 * DAY_MS,
        now,
    };

    assert!(!is_stale(young(fetched_at + DAY_MS / 2)));
    assert!(is_stale(young(fetched_at + 2 * DAY_MS)));
}

#[test]
fn any_other_page_is_read_again_after_90_days() {
    let fetched_at = 100 * DAY_MS;
    let old = |modified_at, now| PageAge {
        fetched_at,
        modified_at,
        now,
    };

    assert!(!is_stale(old(
        fetched_at - 400 * DAY_MS,
        fetched_at + 30 * DAY_MS
    )));
    assert!(is_stale(old(
        fetched_at - 400 * DAY_MS,
        fetched_at + 91 * DAY_MS
    )));
    assert!(is_stale(old(0, fetched_at + 91 * DAY_MS)));
    // Exactly 90 days is not over 90 days; changed exactly 14 days before is not young.
    assert!(!is_stale(old(0, fetched_at + 90 * DAY_MS)));
    assert!(!is_stale(old(
        fetched_at - 14 * DAY_MS,
        fetched_at + 2 * DAY_MS
    )));
}
