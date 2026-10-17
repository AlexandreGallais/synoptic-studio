//! `jsdoc/check-indentation`: no line of a block holds text indented by two blanks or more after
//! its `*`, nor text right against it (`eslint-plugin-jsdoc` 65.3.1,
//! `src/rules/checkIndentation.js`, its default options). The lines of a code fence and of an
//! `@example` tag are left out; the first fault of a block is reported.

use crate::finding::Finding;
use crate::jsdoc;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct CheckIndentation;

/// Whether a character is a space or a tab.
fn is_blank(character: u8) -> bool {
    character == b' ' || character == b'\t'
}

/// The end of a margin, blanks then `*`, starting at an offset (`([ \t]+\*)`).
fn margin_end(text: &[u8], at: usize) -> Option<usize> {
    let blanks = text[at..]
        .iter()
        .take_while(|each| is_blank(**each))
        .count();
    (blanks > 0 && text.get(at + blanks) == Some(&b'*')).then_some(at + blanks + 1)
}

/// Whether a line starting at an offset reads blanks, `*`, then the end of a masked part:
/// `*/`, a tag (`@`, word characters, a blank), or a fence when it may end one.
fn closes_masked_part(text: &[u8], at: usize, fence: bool) -> bool {
    let mut position = at
        + text[at..]
            .iter()
            .take_while(|each| is_blank(**each))
            .count();
    if text.get(position) != Some(&b'*') {
        return false;
    }
    position += 1;
    if text.get(position) == Some(&b'/') {
        return true;
    }
    position += text[position..]
        .iter()
        .take_while(|each| is_blank(**each))
        .count();
    if fence && text[position..].starts_with(b"```") {
        return true;
    }
    if text.get(position) != Some(&b'@') {
        return false;
    }
    let word = text[position + 1..]
        .iter()
        .take_while(|each| each.is_ascii_alphanumeric() || **each == b'_')
        .count();
    word > 0
        && text
            .get(position + 1 + word)
            .is_some_and(|each| each.is_ascii_whitespace() || *each == 0x0b)
}

/// A text whose parts opened by a margin then an opener are replaced by the margin on each of
/// their lines, up to the line that closes them (`maskCodeBlocks`, `maskExcludedContent`).
fn masked(text: &str, opener: &[u8], fence: bool) -> String {
    let bytes = text.as_bytes();
    let mut result = String::new();
    let mut copied = 0;
    let mut at = 0;

    while at < bytes.len() {
        let Some(after_margin) = margin_end(bytes, at) else {
            at += 1;
            continue;
        };
        let starts_part = bytes.get(after_margin).is_some_and(|each| is_blank(*each))
            && bytes[after_margin + 1..].starts_with(opener)
            && (fence
                || matches!(
                    bytes.get(after_margin + 1 + opener.len()),
                    Some(b' ' | b'\n')
                ));
        let close = starts_part
            .then(|| {
                (after_margin + 1 + opener.len()..bytes.len())
                    .filter(|each| bytes[*each] == b'\n')
                    .find(|each| closes_masked_part(bytes, each + 1, fence))
            })
            .flatten();
        let Some(close) = close else {
            at += 1;
            continue;
        };
        let lines = text[after_margin..=close].matches('\n').count();
        result.push_str(&text[copied..at]);
        result.push_str(&format!("{}\n", &text[at..after_margin]).repeat(lines));
        copied = close + 1;
        at = close + 1;
    }
    result.push_str(&text[copied..]);
    result
}

/// The end of the first match, on a line, of `^(?:\/?\**|[ \t]*)\*` then a test on what follows,
/// tried in the order a regular expression tries them.
fn prefixed_match(line: &[u8], follows: impl Fn(&[u8]) -> Option<usize>) -> Option<usize> {
    let try_at = |position: usize| {
        (line.get(position) == Some(&b'*'))
            .then(|| follows(&line[position + 1..]).map(|length| position + 1 + length))
            .flatten()
    };
    let slashes: &[usize] = if line.first() == Some(&b'/') {
        &[1, 0]
    } else {
        &[0]
    };
    for &slash in slashes {
        let stars = line[slash..]
            .iter()
            .take_while(|each| **each == b'*')
            .count();
        if let Some(end) = (0..=stars).rev().find_map(|count| try_at(slash + count)) {
            return Some(end);
        }
    }
    let blanks = line.iter().take_while(|each| is_blank(**each)).count();
    (0..=blanks).rev().find_map(try_at)
}

/// The first line, from 0, where a test matches, and the offset after its match.
fn first_match(text: &str, follows: &dyn Fn(&[u8]) -> Option<usize>) -> Option<(usize, usize)> {
    let mut start = 0;
    for (index, line) in text.split('\n').enumerate() {
        if let Some(end) = prefixed_match(line.as_bytes(), follows) {
            return Some((index, start + end));
        }
        start += line.len() + 1;
    }
    None
}

impl Rule for CheckIndentation {
    fn name(&self) -> &'static str {
        "jsdoc/check-indentation"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for doc in jsdoc::all(source) {
            let text = masked(&masked(doc.text, b"```", true), b"@example", false);
            let no_space = first_match(&text, &|rest| {
                std::str::from_utf8(rest)
                    .ok()
                    .and_then(|rest| rest.chars().next())
                    .filter(|each| !jsdoc::block::is_space(*each) && *each != '*' && *each != '/')
                    .map(char::len_utf8)
            });
            let indented = first_match(&text, &|rest| {
                (rest.len() >= 2 && is_blank(rest[0]) && is_blank(rest[1])).then_some(2)
            });
            let found = match (no_space, indented) {
                (Some(space), Some(indent)) if space.1 <= indent.1 => {
                    Some((space.0, "There must be a space after the asterisk."))
                }
                (Some(space), None) => Some((space.0, "There must be a space after the asterisk.")),
                (_, Some(indent)) => Some((indent.0, "There must be no indentation.")),
                (None, None) => None,
            };
            if let Some((line, message)) = found {
                findings.push(doc.finding_at(source, self.name(), line, None, message.to_owned()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CheckIndentation;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&CheckIndentation, "check-indentation", "src/a.ts", &[]);
    }
}
