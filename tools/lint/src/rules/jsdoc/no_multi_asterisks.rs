//! `jsdoc/no-multi-asterisks`: a block's lines open with one `*`, and its `*/` follows no other
//! asterisk (`eslint-plugin-jsdoc` 65.3.1, `src/rules/noMultiAsterisks.js`, its default options).
//! Autofixed; the first faulty line of a block is reported.

use crate::finding::Finding;
use crate::jsdoc::{self, Doc};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoMultiAsterisks;

/// The length of a description's opening run of blanks and lone asterisks, an asterisk followed
/// by another ending it (`^([\t ]|\*(?!\*))+`).
fn middle_run(description: &str) -> usize {
    let bytes = description.as_bytes();
    let mut length = 0;
    while let Some(byte) = bytes.get(length) {
        let lone_star = *byte == b'*' && bytes.get(length + 1) != Some(&b'*');
        if *byte == b' ' || *byte == b'\t' || lone_star {
            length += 1;
        } else {
            break;
        }
    }
    length
}

/// Where the ending run of asterisks and blanks starts in a text ending with `*`, and where its
/// captured part starts: from the run's first asterisk on a one-line block
/// (`\*((?:\*| |\t)*)\*$`), from the run's start otherwise (`((?:\*| |\t)*)\*$`).
fn end_run(text: &str, single_line: bool) -> Option<(usize, usize)> {
    if !text.ends_with('*') {
        return None;
    }
    let body = &text[..text.len() - 1];
    let run = body.len() - body.trim_end_matches(['*', ' ', '\t']).len();
    let start = body.len() - run;
    if single_line {
        let star = body[start..].find('*')?;
        Some((start + star, start + star + 1))
    } else {
        Some((start, start))
    }
}

impl NoMultiAsterisks {
    /// The finding of one block, if any.
    fn check(&self, source: &Source, doc: &Doc) -> Option<Finding> {
        let mut block = doc.block.clone();
        for index in 0..block.source.len() {
            let tokens = &mut block.source[index].tokens;
            let run = middle_run(&tokens.description);
            if tokens.end.is_empty()
                && tokens.tag.is_empty()
                && tokens.r#type.is_empty()
                && tokens.name.is_empty()
                && run > 0
            {
                tokens.description.drain(..run);
                let mut finding = doc.finding_at(
                    source,
                    self.name(),
                    index,
                    None,
                    "Should be no multiple asterisks on middle lines.".to_owned(),
                );
                finding.fix = Some(doc.rewrite(&block));
                return Some(finding);
            }
            if tokens.end.is_empty() {
                continue;
            }
            let single_line = tokens.delimiter == "/**";
            let delimiter = if single_line {
                "*"
            } else {
                tokens.delimiter.as_str()
            };
            let text = format!("{}{delimiter}", tokens.description);
            let (start, captured) = end_run(&text, single_line)?;
            let group = &text[captured..text.len() - 1];
            if !single_line && !group.is_empty() && group.trim().is_empty() {
                return None;
            }
            if !single_line {
                tokens.delimiter.clear();
            }
            text[..start].clone_into(&mut tokens.description);
            let mut finding = doc.finding_at(
                source,
                self.name(),
                index,
                None,
                "Should be no multiple asterisks on end lines.".to_owned(),
            );
            finding.fix = Some(doc.rewrite(&block));
            return Some(finding);
        }
        None
    }
}

impl Rule for NoMultiAsterisks {
    fn name(&self) -> &'static str {
        "jsdoc/no-multi-asterisks"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        findings.extend(
            jsdoc::all(source)
                .iter()
                .filter_map(|doc| self.check(source, doc)),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::NoMultiAsterisks;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&NoMultiAsterisks, "no-multi-asterisks", "src/a.ts", &[]);
    }
}
