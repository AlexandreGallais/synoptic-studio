//! Tests of the HTML converter, on the cases of `library-html.test.ts` and on the edges of each
//! pattern it replaces (EN-106).

use library::html::{PageFacts, decode_entities, html_to_markdown, page_facts};

fn markdown(html: &str) -> String {
    html_to_markdown(html).unwrap()
}

#[test]
fn named_decimal_and_hexadecimal_references_are_decoded_and_unknown_ones_kept() {
    assert_eq!(
        decode_entities("a &amp; b &#38; c &#x26; &lt;p&gt; &mdash; &unknown;").unwrap(),
        "a & b & c & <p> — &unknown;"
    );
    // Case-insensitive names and `X`; no `;`, no digit, or a letter after the digits: kept.
    assert_eq!(
        decode_entities("&AMP; &#X41; &amp &#; &#x; &#12a; &").unwrap(),
        "& A &amp &#; &#x; &#12a; &"
    );
}

#[test]
fn a_reference_beyond_unicode_is_an_error_as_javascript_threw_one() {
    // 0x110000 = 1114112, one past the last code point.
    assert_eq!(
        decode_entities("&#x110000;"),
        Err("RangeError: Invalid code point 1114112".to_owned())
    );
    // A lone surrogate becomes the replacement character.
    assert_eq!(decode_entities("&#xD800;").unwrap(), "\u{FFFD}");
}

#[test]
fn nested_tags_and_comments_are_removed_keeping_a_less_than_that_opens_no_tag() {
    assert_eq!(
        markdown("<body><p><<b>i>x<!<!-- a -->-- b --></p></body>"),
        "x"
    );
}

#[test]
fn the_main_part_is_kept_as_markdown_and_the_page_chrome_dropped() {
    let html = [
        "<html><head><title>T</title><script>x()</script></head><body>",
        "<nav>Menu</nav><main><h1>Title</h1><p>One <strong>bold</strong> <code>x</code>.</p>",
        "<ul><li>first</li><li><a href=\"https://a.org\">second</a></li><li><a href=\"/local\">third</a></li></ul>",
        "<pre><code>let a = 1 &lt; 2;\n</code></pre><!-- note --><footer>Foot</footer></main></body></html>",
    ]
    .concat();

    assert_eq!(
        markdown(&html),
        [
            "# Title",
            "",
            "One **bold** `x`.",
            "",
            "- first",
            "- [second](https://a.org)",
            "- third",
            "",
            "```",
            "let a = 1 < 2;",
            "```",
        ]
        .join("\n")
    );
}

#[test]
fn the_article_then_the_body_are_the_main_part_when_there_is_no_main() {
    assert_eq!(
        markdown("<body><article><p>A</p></article><p>B</p></body>"),
        "A"
    );
    assert_eq!(markdown("<body><p>B</p></body>"), "B");
    assert_eq!(markdown("<p>C</p>"), "C");
    // A `<main>` never closed is no main part.
    assert_eq!(markdown("<main><p>D</p><body><p>E</p></body>"), "E");
}

#[test]
fn tags_are_read_case_insensitively_and_only_at_a_word_boundary() {
    assert_eq!(
        markdown("<BODY><H2 id=x>Big</H2><P>One<BR/>Two<br >Three</P></BODY>"),
        "## Big\n\nOne\nTwo\nThree"
    );
    // `<pre>` is no `<p>`, `<bdi>` no `<b>`, `<navigation>` no `<nav>`.
    assert_eq!(
        markdown("<navigation>kept</navigation><bdi>also</bdi>"),
        "keptalso"
    );
}

#[test]
fn a_heading_is_closed_by_its_own_level_only() {
    assert_eq!(markdown("<h1>A</h2>B</h1>"), "# AB");
    // Never closed: its tags are removed as any tag.
    assert_eq!(markdown("<h3>C"), "C");
}

#[test]
fn a_link_takes_the_last_href_of_its_tag_and_keeps_only_absolute_addresses() {
    assert_eq!(
        markdown("<a data-href=\"/x\" href=\"https://b.org\">B</a> <a href=\"#top\">top</a>"),
        "[B](https://b.org) top"
    );
    // An address holding `>` is read to its closing quote.
    assert_eq!(
        markdown("<a href=\"http://c.org/?a>b\">C</a>"),
        "[C](http://c.org/?a>b)"
    );
    // No href: the tags are removed.
    assert_eq!(markdown("<a name=\"x\">D</a>"), "D");
}

#[test]
fn tables_become_pipes_and_spaces_and_blank_lines_are_collapsed() {
    assert_eq!(
        markdown("<table><tr><th>A</th><td>1</td></tr></table>\n\n\n\n<p>x  \t y   </p>"),
        "| A | | 1 |\n\nx y"
    );
}

#[test]
fn every_dropped_element_goes_with_its_content() {
    for name in [
        "script", "style", "noscript", "svg", "nav", "header", "footer", "aside", "form", "button",
    ] {
        assert_eq!(
            markdown(&format!("<p>a<{name} x>b</{name}>c</p>")),
            "ac",
            "{name}"
        );
    }
    // Never closed: only its tag goes.
    assert_eq!(markdown("<p>a<script>b</p>"), "ab");
}

#[test]
fn a_code_block_is_fenced_its_entities_decoded_once_more_with_the_page() {
    // `&amp;lt;` is decoded in the block, then again with the page: `<`, as the converter did.
    assert_eq!(
        markdown("<pre class=x>a &amp;lt; b</pre>"),
        "```\na < b\n```"
    );
}

#[test]
fn the_title_and_the_modification_date_are_read_from_meta_tags_or_json_ld() {
    assert_eq!(
        page_facts(
            "<title>Doc &amp; more</title><meta property=\"article:modified_time\" content=\"2026-09-01T10:00:00Z\">"
        )
        .unwrap(),
        PageFacts {
            modified: "2026-09-01T10:00:00Z".to_owned(),
            title: "Doc & more".to_owned()
        }
    );
    assert_eq!(
        page_facts("<script>{\"dateModified\": \"2026-08-02\"}</script>")
            .unwrap()
            .modified,
        "2026-08-02"
    );
    assert_eq!(page_facts("<p>nothing</p>").unwrap(), PageFacts::default());
    // An empty value is skipped for the next one; the meta tag needs a space before `content`.
    assert_eq!(
        page_facts("<meta property=\"article:modified_time\"content=\"x\">\"dateModified\":\"\" \"DATEMODIFIED\" : \"2026-01-02\"")
            .unwrap()
            .modified,
        "2026-01-02"
    );
}
