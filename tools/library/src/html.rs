//! HTML of a documentation page turned into plain Markdown for the local source library
//! (`docs/tooling/sources.md`): headings, paragraphs, lists, code blocks and links kept; scripts,
//! styles, navigation and page chrome dropped. A simple converter, enough for documentation pages;
//! pages that build their text in the browser are left to `WebFetch`. Each step is the one
//! `library-html.ts` made with a regular expression, written here as a scan of the text, so that a
//! page is cut into the same sections as before (EN-106).

use crate::text;

/// What a page states about itself.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PageFacts {
    /// When it says it was last changed, ISO 8601, or empty.
    pub modified: String,
    /// Its `<title>`, or empty.
    pub title: String,
}

/// Named entities decoded, beyond the numeric ones.
const ENTITIES: [(&str, &str); 15] = [
    ("amp", "&"),
    ("apos", "'"),
    ("gt", ">"),
    ("hellip", "…"),
    ("laquo", "«"),
    ("ldquo", "“"),
    ("lsquo", "‘"),
    ("lt", "<"),
    ("mdash", "—"),
    ("nbsp", " "),
    ("ndash", "–"),
    ("quot", "\""),
    ("raquo", "»"),
    ("rdquo", "”"),
    ("rsquo", "’"),
];

/// Elements dropped with their content: never part of the page's text.
const DROPPED: [&str; 10] = [
    "script", "style", "noscript", "svg", "nav", "header", "footer", "aside", "form", "button",
];

/// Elements whose tags, opening or closing, become a blank line.
const BLOCKS: [&str; 12] = [
    "p",
    "div",
    "section",
    "table",
    "tr",
    "ul",
    "ol",
    "dl",
    "dd",
    "dt",
    "blockquote",
    "figure",
];

/// The character a numeric reference stands for, `#38` or `#x26`; an error, as
/// `String.fromCodePoint` throws one, for a number beyond Unicode. A lone surrogate, which a
/// JavaScript string holds and a Rust one cannot, becomes U+FFFD as SQLite would store it.
fn numeric_character(reference: &str) -> Result<char, String> {
    let digits = &reference[1..];
    let value = match digits.strip_prefix(['x', 'X']) {
        Some(hex) => u128::from_str_radix(hex, 16).ok(),
        None => digits.parse::<u128>().ok(),
    };

    match value.map(u32::try_from) {
        Some(Ok(code)) if code <= 0x0010_FFFF => Ok(char::from_u32(code).unwrap_or('\u{FFFD}')),
        _ => Err(format!(
            "RangeError: Invalid code point {}",
            code_text(digits)
        )),
    }
}

/// A code point as JavaScript writes the number it read, for the error message.
fn code_text(digits: &str) -> String {
    let value = match digits.strip_prefix(['x', 'X']) {
        Some(hex) => hex.chars().fold(0.0_f64, |sum, digit| {
            sum * 16.0 + f64::from(digit.to_digit(16).unwrap_or(0))
        }),
        None => digits.parse::<f64>().unwrap_or(f64::NAN),
    };

    if value < 1e21 {
        format!("{value:.0}")
    } else {
        let exponent = value.log10().floor();
        let mantissa = value / 10_f64.powf(exponent);

        format!("{mantissa}e+{exponent}")
    }
}

/// The length of the entity name starting a text, as `#x[\da-f]+|#\d+|[a-z]+` matches it before
/// its `;`, case-insensitively; `None` when no entity starts there.
fn entity_name(rest: &str) -> Option<usize> {
    let bytes = rest.as_bytes();
    let run = |from: usize, accepts: fn(&u8) -> bool| -> usize {
        bytes[from..]
            .iter()
            .take_while(|byte| accepts(byte))
            .count()
    };
    let ends = |length: usize| bytes.get(length) == Some(&b';');

    if bytes.first() == Some(&b'#') {
        if matches!(bytes.get(1), Some(b'x' | b'X')) {
            let hex = run(2, u8::is_ascii_hexdigit);

            if hex > 0 && ends(2 + hex) {
                return Some(2 + hex);
            }
        }

        let digits = run(1, u8::is_ascii_digit);

        return (digits > 0 && ends(1 + digits)).then_some(1 + digits);
    }

    let letters = run(0, u8::is_ascii_alphabetic);

    (letters > 0 && ends(letters)).then_some(letters)
}

/// Text with its HTML entities decoded: named, decimal and hexadecimal; unknown entities kept as
/// written.
///
/// # Errors
///
/// For a numeric reference beyond Unicode, with the message JavaScript gave.
pub fn decode_entities(html: &str) -> Result<String, String> {
    let mut decoded = String::with_capacity(html.len());
    let mut rest = html;

    while let Some(at) = rest.find('&') {
        decoded.push_str(&rest[..at]);
        rest = &rest[at + 1..];

        let Some(length) = entity_name(rest) else {
            decoded.push('&');
            continue;
        };
        let name = &rest[..length];

        if name.starts_with('#') {
            decoded.push(numeric_character(name)?);
        } else if let Some((_, character)) = ENTITIES
            .iter()
            .find(|(entity, _)| entity.eq_ignore_ascii_case(name))
        {
            decoded.push_str(character);
        } else {
            decoded.push('&');
            decoded.push_str(name);
            decoded.push(';');
        }

        rest = &rest[length + 1..];
    }

    decoded.push_str(rest);

    Ok(decoded)
}

/// Text with every span from an opening to a closing mark removed, by splitting: an opening mark
/// never closed stays, with what follows it (`1 < 2` in code is text).
fn without_spans(text: &str, open: &str, close: &str) -> String {
    let mut parts = text.split(open);
    let mut kept = parts.next().unwrap_or_default().to_owned();

    for part in parts {
        if let Some(end) = part.find(close) {
            kept.push_str(&part[end + close.len()..]);
        } else {
            kept.push_str(open);
            kept.push_str(part);
        }
    }

    kept
}

/// Text of an HTML fragment, its tags removed.
fn without_tags(html: &str) -> String {
    without_spans(html, "<", ">")
}

/// Position of the first ASCII-case-insensitive occurrence of a needle from a byte offset.
fn find_ignoring_case(haystack: &str, needle: &str, from: usize) -> Option<usize> {
    let bytes = haystack.as_bytes();
    let needle = needle.as_bytes();

    (from..=bytes.len().checked_sub(needle.len())?)
        .find(|&at| bytes[at..at + needle.len()].eq_ignore_ascii_case(needle))
}

/// Whether a word boundary follows a word character at a byte offset: the next character is not
/// a word character, or the text ends.
fn boundary_at(text: &str, at: usize) -> bool {
    !text[at..].chars().next().is_some_and(text::is_word)
}

/// One element found: where it starts, where its content starts and ends, where it ends.
#[derive(Clone, Copy)]
struct Element {
    start: usize,
    inner: (usize, usize),
    end: usize,
}

/// The first element `<name\b[^>]*>(content)</name>` from a byte offset, case-insensitively, its
/// content the shortest one: what the pattern `<name\b[^>]*>([\s\S]*?)<\/name>` matches.
fn element(html: &str, name: &str, from: usize) -> Option<Element> {
    let open = format!("<{name}");
    let close = format!("</{name}>");
    let mut at = from;

    while let Some(start) = find_ignoring_case(html, &open, at) {
        let after = start + open.len();

        at = start + 1;

        if !boundary_at(html, after) {
            continue;
        }

        // Without a `>` or a closing tag after this one, none after a later one either.
        let tag_end = after + html[after..].find('>')?;
        let inner_end = find_ignoring_case(html, &close, tag_end + 1)?;

        return Some(Element {
            start,
            inner: (tag_end + 1, inner_end),
            end: inner_end + close.len(),
        });
    }

    None
}

/// Every element of a name replaced, left to right, by what a function makes of the element.
fn replace_elements(
    html: &str,
    find: impl Fn(&str, usize) -> Option<Element>,
    mut replace: impl FnMut(&str, Element) -> Result<String, String>,
) -> Result<String, String> {
    let mut replaced = String::with_capacity(html.len());
    let mut done = 0;

    while let Some(found) = find(html, done) {
        replaced.push_str(&html[done..found.start]);
        replaced.push_str(&replace(html, found)?);
        done = found.end;
    }

    replaced.push_str(&html[done..]);

    Ok(replaced)
}

/// The main part of a page: its `<main>` or `<article>` when it has one, else its `<body>`, else
/// the whole page.
fn main_part(html: &str) -> &str {
    ["main", "article", "body"]
        .iter()
        .find_map(|name| element(html, name, 0))
        .map_or(html, |found| &html[found.inner.0..found.inner.1])
}

/// Code blocks as fenced Markdown, their text untouched by the other rules.
fn fenced_code(html: &str) -> Result<String, String> {
    replace_elements(
        html,
        |html, from| element(html, "pre", from),
        |html, found| {
            let code = decode_entities(&without_tags(&html[found.inner.0..found.inner.1]))?;

            Ok(format!("\n\n```\n{}\n```\n\n", text::trim(&code)))
        },
    )
}

/// The first heading `<h1>` to `<h6>` from a byte offset, closed by the same level.
fn heading(html: &str, from: usize) -> Option<Element> {
    (from..html.len())
        .filter(|&at| html.as_bytes()[at] == b'<')
        .find_map(|at| {
            let bytes = html.as_bytes();
            let level = *bytes.get(at + 2)?;

            if !matches!(bytes.get(at + 1), Some(b'h' | b'H'))
                || !(b'1'..=b'6').contains(&level)
                || !boundary_at(html, at + 3)
            {
                return None;
            }

            let tag_end = at + 3 + html[at + 3..].find('>')?;
            let close = format!("</h{}>", char::from(level));
            let inner_end = find_ignoring_case(html, &close, tag_end + 1)?;

            Some(Element {
                start: at,
                inner: (tag_end + 1, inner_end),
                end: inner_end + close.len(),
            })
        })
}

/// The first link `<a\b[^>]*href="(href)"[^>]*>(label)</a>` from a byte offset, and its address:
/// the last `href="` of the opening tag tried first, as the pattern's greedy `[^>]*` does.
fn link(html: &str, from: usize) -> Option<(Element, (usize, usize))> {
    let mut at = from;

    while let Some(start) = find_ignoring_case(html, "<a", at) {
        at = start + 1;

        if !boundary_at(html, start + 2) {
            continue;
        }

        let region_end = html[start + 2..]
            .find('>')
            .map_or(html.len(), |offset| start + 2 + offset);
        let mut hrefs: Vec<usize> = Vec::new();
        let mut search = start + 2;

        while let Some(href) = find_ignoring_case(&html[..region_end], "href=\"", search) {
            hrefs.push(href);
            search = href + 1;
        }

        let found = hrefs.iter().rev().find_map(|&href| {
            let value_start = href + "href=\"".len();
            let value_end = value_start + html[value_start..].find('"')?;
            let tag_end = value_end + 1 + html[value_end + 1..].find('>')?;
            let inner_end = find_ignoring_case(html, "</a>", tag_end + 1)?;

            Some((
                Element {
                    start,
                    inner: (tag_end + 1, inner_end),
                    end: inner_end + "</a>".len(),
                },
                (value_start, value_end),
            ))
        });

        if found.is_some() {
            return found;
        }
    }

    None
}

/// Headings and links as Markdown: `<h1>`–`<h6>` as `#` lines, absolute links as `[label](url)`,
/// other links as their label.
fn headings_and_links(html: &str) -> Result<String, String> {
    let headed = replace_elements(html, heading, |html, found| {
        let level = usize::from(html.as_bytes()[found.start + 2] - b'0');
        let title = without_tags(&html[found.inner.0..found.inner.1]);

        Ok(format!(
            "\n\n{} {}\n\n",
            "#".repeat(level),
            text::trim(&title)
        ))
    })?;
    let mut written = String::with_capacity(headed.len());
    let mut done = 0;

    while let Some((found, (href_start, href_end))) = link(&headed, done) {
        let href = &headed[href_start..href_end];
        let label = &headed[found.inner.0..found.inner.1];

        written.push_str(&headed[done..found.start]);

        if href.starts_with("http") {
            written.push('[');
            written.push_str(label);
            written.push_str("](");
            written.push_str(href);
            written.push(')');
        } else {
            written.push_str(label);
        }

        done = found.end;
    }

    written.push_str(&headed[done..]);

    Ok(written)
}

/// The length of a tag `<name\b[^>]*>`, or `</name\b[^>]*>` when closing ones count, starting a
/// text, for one of the names, case-insensitively; `None` when none starts there.
fn tag_length(rest: &str, names: &[&str], closing_too: bool) -> Option<usize> {
    let after = 1 + usize::from(closing_too && rest.as_bytes().get(1) == Some(&b'/'));
    let name_length = rest[after..]
        .chars()
        .take_while(|c| text::is_word(*c))
        .count();
    let name = &rest[after..after + name_length];

    if name.is_empty() || !names.iter().any(|known| known.eq_ignore_ascii_case(name)) {
        return None;
    }

    rest[after + name_length..]
        .find('>')
        .map(|offset| after + name_length + offset + 1)
}

/// Every tag of some names replaced by a mark.
fn replace_tags(html: &str, names: &[&str], closing_too: bool, mark: &str) -> String {
    replace_at_tags(html, mark, |rest| tag_length(rest, names, closing_too))
}

/// Every tag a function measures at a `<` replaced by a mark, left to right.
fn replace_at_tags(html: &str, mark: &str, measure: impl Fn(&str) -> Option<usize>) -> String {
    let mut replaced = String::with_capacity(html.len());
    let mut done = 0;
    let mut at = 0;

    while let Some(offset) = html[at..].find('<') {
        let start = at + offset;

        if let Some(length) = measure(&html[start..]) {
            replaced.push_str(&html[done..start]);
            replaced.push_str(mark);
            done = start + length;
            at = done;
        } else {
            at = start + 1;
        }
    }

    replaced.push_str(&html[done..]);

    replaced
}

/// The length of a line break `<br\s*\/?>` starting a text, case-insensitively.
fn line_break_length(rest: &str) -> Option<usize> {
    if !rest.get(1..3)?.eq_ignore_ascii_case("br") {
        return None;
    }

    let spaces: usize = rest[3..]
        .chars()
        .take_while(|c| text::is_space(*c))
        .map(char::len_utf8)
        .sum();
    let slash = usize::from(rest[3 + spaces..].starts_with('/'));

    rest[3 + spaces + slash..]
        .starts_with('>')
        .then_some(3 + spaces + slash + 1)
}

/// Every element of a name dropped with its content: `<name\b` to the first `</name>` after it.
fn drop_elements(html: &str, name: &str) -> String {
    let open = format!("<{name}");
    let close = format!("</{name}>");
    let mut kept = String::with_capacity(html.len());
    let mut done = 0;
    let mut at = 0;

    while let Some(start) = find_ignoring_case(html, &open, at) {
        at = start + 1;

        if !boundary_at(html, start + open.len()) {
            continue;
        }

        // Without a closing tag after this one, none after a later one either.
        let Some(close_at) = find_ignoring_case(html, &close, start + open.len()) else {
            break;
        };

        kept.push_str(&html[done..start]);
        done = close_at + close.len();
        at = done;
    }

    kept.push_str(&html[done..]);

    kept
}

/// A documentation page as Markdown: its main text with headings, lists, code blocks and links;
/// blank lines collapsed.
///
/// # Errors
///
/// For a numeric reference beyond Unicode, as the TypeScript converter threw.
pub fn html_to_markdown(html: &str) -> Result<String, String> {
    let kept = DROPPED.iter().fold(
        without_spans(main_part(html), "<!--", "-->"),
        |part, name| drop_elements(&part, name),
    );
    let marked = headings_and_links(&fenced_code(&kept)?)?;
    let marked = replace_tags(&marked, &["li"], false, "\n- ");
    let marked = replace_tags(&marked, &BLOCKS, true, "\n\n");
    let marked = replace_at_tags(&marked, "\n", line_break_length);
    let marked = replace_tags(&marked, &["code", "kbd", "samp"], true, "`");
    let marked = replace_tags(&marked, &["strong", "b"], true, "**");
    let marked = replace_tags(&marked, &["td", "th"], true, " | ");
    let lines = decode_entities(&without_tags(&marked))?
        .split('\n')
        .map(|line| text::trim_end(&collapse_blanks(line)).to_owned())
        .collect::<Vec<_>>()
        .join("\n");

    Ok(text::trim(&collapse_line_breaks(&lines)).to_owned())
}

/// A line with each run of spaces and tabs written as one space.
fn collapse_blanks(line: &str) -> String {
    let mut collapsed = String::with_capacity(line.len());
    let mut is_blank = false;

    for character in line.chars() {
        if character == ' ' || character == '\t' {
            if !is_blank {
                collapsed.push(' ');
            }

            is_blank = true;
        } else {
            collapsed.push(character);
            is_blank = false;
        }
    }

    collapsed
}

/// A text with each run of three line breaks or more written as two.
fn collapse_line_breaks(text: &str) -> String {
    let mut collapsed = String::with_capacity(text.len());
    let mut breaks = 0;

    for character in text.chars() {
        if character == '\n' {
            breaks += 1;

            if breaks <= 2 {
                collapsed.push('\n');
            }
        } else {
            breaks = 0;
            collapsed.push(character);
        }
    }

    collapsed
}

/// The first value of `property="article:modified_time"\s+content="([^"]+)"`, case-insensitively.
fn meta_modified(html: &str) -> Option<&str> {
    let mut at = 0;

    while let Some(start) = find_ignoring_case(html, "property=\"article:modified_time\"", at) {
        at = start + 1;

        let after = start + "property=\"article:modified_time\"".len();
        let spaces = space_length(&html[after..]);

        if spaces == 0 {
            continue;
        }

        if let Some(value) = quoted_after(html, after + spaces, "content=\"") {
            return Some(value);
        }
    }

    None
}

/// The first value of `"dateModified"\s*:\s*"([^"]+)"`, case-insensitively.
fn json_modified(html: &str) -> Option<&str> {
    let mut at = 0;

    while let Some(start) = find_ignoring_case(html, "\"dateModified\"", at) {
        at = start + 1;

        let after = start + "\"dateModified\"".len();
        let colon = after + space_length(&html[after..]);

        if !html[colon..].starts_with(':') {
            continue;
        }

        let quote = colon + 1 + space_length(&html[colon + 1..]);

        if let Some(value) = quoted_after(html, quote, "\"") {
            return Some(value);
        }
    }

    None
}

/// The byte length of the white space starting a text.
fn space_length(text: &str) -> usize {
    text.chars()
        .take_while(|c| text::is_space(*c))
        .map(char::len_utf8)
        .sum()
}

/// The non-empty value between a prefix ending in `"`, at a byte offset, and the next `"`.
fn quoted_after<'a>(html: &'a str, at: usize, prefix: &str) -> Option<&'a str> {
    let start = at + prefix.len();

    if !html.get(at..start)?.eq_ignore_ascii_case(prefix) {
        return None;
    }

    let length = html[start..].find('"')?;

    (length > 0).then(|| &html[start..start + length])
}

/// Title and last change a page states about itself: its `<title>`, and its modification date
/// from its meta tags or its JSON-LD; each decoded and trimmed, or empty.
///
/// # Errors
///
/// For a numeric reference beyond Unicode in a value, as the TypeScript reader threw.
pub fn page_facts(html: &str) -> Result<PageFacts, String> {
    let modified = meta_modified(html)
        .or_else(|| json_modified(html))
        .unwrap_or_default();
    let title = element(html, "title", 0).map_or("", |found| &html[found.inner.0..found.inner.1]);

    Ok(PageFacts {
        modified: text::trim(&decode_entities(modified)?).to_owned(),
        title: text::trim(&decode_entities(title)?).to_owned(),
    })
}
