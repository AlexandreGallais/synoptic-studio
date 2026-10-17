//! A `TSDoc` block read into its summary and its block tags (`REF-TSDOC`): the comment's
//! decoration stripped, each `@tag` starting a block that runs to the next one.

use crate::model::{Doc, Tag};

/// The tags whose first word names what they document.
const NAMED_TAGS: [&str; 2] = ["param", "template"];

/// The lines of a `/** … */` comment, each without its leading `*` and the space after it.
fn lines(comment: &str) -> Vec<&str> {
    let inner = comment
        .trim()
        .trim_start_matches("/**")
        .trim_end_matches("*/");

    let mut lines: Vec<&str> = inner
        .lines()
        .map(|line| {
            let line = line.trim_start();
            let line = line.strip_prefix('*').unwrap_or(line);
            let line = line.strip_prefix(' ').unwrap_or(line);
            line.trim_end()
        })
        .collect();

    while lines.first().is_some_and(|line| line.is_empty()) {
        lines.remove(0);
    }
    while lines.last().is_some_and(|line| line.is_empty()) {
        lines.pop();
    }

    lines
}

/// Whether a line opens a block tag, outside a code fence.
fn tag_name(line: &str) -> Option<&str> {
    let rest = line.strip_prefix('@')?;
    let end = rest
        .find(|c: char| !c.is_ascii_alphanumeric())
        .unwrap_or(rest.len());

    (end > 0).then(|| &rest[..end])
}

/// A tag from its name and its text: the first word of a named tag is its target, the
/// `TSDoc` hyphen after it dropped.
fn tag(name: &str, text: &str) -> Tag {
    let text = text.trim();

    if NAMED_TAGS.contains(&name) {
        let (target, rest) = text.split_once(char::is_whitespace).unwrap_or((text, ""));
        let rest = rest.trim_start();
        let rest = rest.strip_prefix('-').map_or(rest, str::trim_start);

        return Tag {
            name: name.to_owned(),
            target: Some(target.to_owned()),
            text: rest.trim().to_owned(),
        };
    }

    Tag {
        name: name.to_owned(),
        target: None,
        text: text.to_owned(),
    }
}

/// A `TSDoc` comment, `/** … */`, read into its summary and its block tags.
#[must_use]
pub fn parse(comment: &str) -> Doc {
    let mut summary: Vec<&str> = Vec::new();
    let mut tags: Vec<(String, Vec<&str>)> = Vec::new();
    let mut fenced = false;

    for line in lines(comment) {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
        }

        match (fenced, tag_name(line)) {
            (false, Some(name)) => {
                let rest = &line[name.len() + 1..];
                tags.push((name.to_owned(), vec![rest]));
            }
            _ => match tags.last_mut() {
                Some((_, text)) => text.push(line),
                None => summary.push(line),
            },
        }
    }

    Doc {
        summary: summary.join("\n").trim().to_owned(),
        tags: tags
            .iter()
            .map(|(name, text)| tag(name, &text.join("\n")))
            .collect(),
    }
}

/// The names of the `{@link …}` tags of a text, each as written before any `|`.
#[must_use]
pub fn links(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut rest = text;

    while let Some(start) = rest.find("{@link") {
        let after = &rest[start + "{@link".len()..];
        let Some(end) = after.find('}') else { break };
        let inside = after[..end].trim();
        let target = inside.split('|').next().unwrap_or("").trim();
        let target = target.split_whitespace().next().unwrap_or("");
        found.push(target.to_owned());
        rest = &after[end..];
    }

    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_summary_on_several_lines_and_its_tags() {
        let doc = parse(
            "/**\n * Point moved by a vector.\n *\n * Formula: p + v.\n *\n * @kind math\n * @param point - start point\n * @param vector - displacement,\n *   on two lines\n * @returns the moved point\n * @see REF-X\n */",
        );

        assert_eq!(doc.summary, "Point moved by a vector.\n\nFormula: p + v.");
        assert_eq!(doc.first("kind"), Some("math"));
        assert_eq!(doc.named("param", "point"), Some("start point"));
        assert_eq!(
            doc.named("param", "vector"),
            Some("displacement,\n  on two lines")
        );
        assert_eq!(doc.first("returns"), Some("the moved point"));
        assert_eq!(doc.first("see"), Some("REF-X"));
        assert_eq!(doc.tags.len(), 5);
    }

    #[test]
    fn reads_a_one_line_comment() {
        let doc = parse("/** The three alignments of an axis. */");

        assert_eq!(doc.summary, "The three alignments of an axis.");
        assert_eq!(doc.tags, Vec::new());
    }

    #[test]
    fn reads_a_parameter_without_its_hyphen_and_an_empty_one() {
        let doc = parse("/**\n * @template S type of the states\n * @param x\n */");

        assert_eq!(doc.named("template", "S"), Some("type of the states"));
        assert_eq!(doc.named("param", "x"), Some(""));
        assert_eq!(doc.summary, "");
    }

    #[test]
    fn keeps_an_at_sign_inside_a_code_fence_or_a_line() {
        let doc = parse("/**\n * Text.\n * ```\n * @kind not a tag\n * ```\n * mail a@b\n */");

        assert_eq!(doc.summary, "Text.\n```\n@kind not a tag\n```\nmail a@b");
        assert_eq!(doc.tags, Vec::new());
    }

    #[test]
    fn finds_the_link_targets() {
        assert_eq!(
            links("See {@link add} and {@link Point | the point}, {@link  scale  }."),
            vec!["add", "Point", "scale"]
        );
        assert_eq!(links("no link, {@link unclosed"), Vec::<String>::new());
    }
}
