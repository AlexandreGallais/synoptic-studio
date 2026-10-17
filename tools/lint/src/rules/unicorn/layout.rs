//! The layout of a fix's new text, shared by Unicorn's rules: the file's indentation step, the
//! blank text before a node on its line, and a text moved from one indentation to another
//! (`eslint-plugin-unicorn`'s `getIndentUnit`, `getIndentString`, `reindentText`).

use crate::source::Source;

/// The most common step of indentation in a file: spaces, or a tab when tabs indent more lines
/// or nothing is indented (`getIndentUnit`).
#[must_use]
pub fn indent_unit(text: &str) -> String {
    let mut steps: Vec<(usize, usize)> = Vec::new();
    let (mut tabs, mut spaces, mut previous) = (0, 0, 0);
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        let indent = &line[..line.len() - line.trim_start().len()];
        if indent.starts_with('\t') {
            tabs += 1;
            continue;
        }
        let width = indent.len();
        if width > 0 {
            spaces += 1;
        }
        let step = width.abs_diff(previous);
        if step > 0 {
            match steps.iter_mut().find(|(each, _)| *each == step) {
                Some((_, count)) => *count += 1,
                None => steps.push((step, 1)),
            }
        }
        previous = width;
    }
    let most = steps
        .iter()
        .max_by_key(|(step, count)| (*count, std::cmp::Reverse(*step)));
    match most {
        Some((step, _)) if spaces > tabs => " ".repeat(*step),
        _ => "\t".to_owned(),
    }
}

/// The blank text before a node on its line.
#[must_use]
pub fn indent_before(source: &Source, start: u32) -> String {
    let before = source.text.get(..start as usize).unwrap_or_default();
    let line = before.rsplit('\n').next().unwrap_or_default();
    line[line.trim_end().len()..].to_owned()
}

/// The text with its lines after the first moved from one indentation to another
/// (`reindentText`).
#[must_use]
pub fn reindent(text: &str, from: &str, to: &str) -> String {
    let mut lines = text.split('\n');
    let mut out = lines.next().unwrap_or_default().to_owned();
    for line in lines {
        out.push('\n');
        if line.trim().is_empty() {
            continue;
        }
        match line.strip_prefix(from) {
            Some(rest) => {
                out.push_str(to);
                out.push_str(rest);
            }
            None => out.push_str(line),
        }
    }
    out
}
