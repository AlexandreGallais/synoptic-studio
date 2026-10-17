//! `jsdoc/require-hyphen-before-param-description` with `always`: a `@param` description opens
//! with a hyphen, and no hyphen ends the tag's first line alone (`eslint-plugin-jsdoc` 65.3.1,
//! `src/rules/requireHyphenBeforeParamDescription.js`). Autofixed.

use crate::finding::Finding;
use crate::jsdoc::block::{Block, Spec, is_space, trim, trim_start};
use crate::jsdoc::{self, Doc};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct RequireHyphenBeforeParamDescription;

/// A tag's description, its lines joined, the space after its tag or its `*` left out
/// (`getTagDescription`).
#[must_use]
pub fn tag_description(block: &Block, tag: &Spec) -> String {
    let mut descriptions = Vec::new();
    for index in &tag.lines {
        let tokens = &block.source[*index].tokens;
        let before = if !tokens.tag.is_empty() && !tokens.post_tag.is_empty() {
            tokens.post_tag.as_str()
        } else if tokens.tag.is_empty()
            && tokens.name.is_empty()
            && tokens.r#type.is_empty()
            && !tokens.post_delimiter.is_empty()
        {
            tokens.post_delimiter.as_str()
        } else {
            ""
        };
        let description = format!(
            "{}{}{}",
            before.get(1..).unwrap_or_default(),
            tokens.description,
            tokens.line_end
        );
        if !tokens.end.is_empty() {
            if !description.is_empty() {
                descriptions.push(description);
            }
            break;
        }
        descriptions.push(description);
    }
    descriptions.join("\n")
}

impl RequireHyphenBeforeParamDescription {
    /// The findings of one block.
    fn check(&self, source: &Source, doc: &Doc, findings: &mut Vec<Finding>) {
        let mut block = doc.block.clone();
        let tags: Vec<Spec> = block
            .tags
            .iter()
            .filter(|tag| tag.tag == "param")
            .cloned()
            .collect();
        for tag in &tags {
            let description = tag_description(&block, tag);
            if trim(&description).is_empty() {
                continue;
            }
            let after_hyphen = trim_start(&description).strip_prefix('-');
            let hyphen_newline = description
                .split_once('-')
                .is_some_and(|(before, after)| trim(before).is_empty() && after.starts_with('\n'));
            let empty = tag
                .lines
                .iter()
                .take_while(|index| block.source[**index].tokens.description.is_empty())
                .count();
            let line = tag.lines[0] + empty;
            let message = if hyphen_newline {
                "There must be no hyphen followed by newline after the @param name."
            } else if after_hyphen.is_none() {
                "There must be a hyphen before @param description."
            } else {
                continue;
            };
            let mut report = true;
            for index in &tag.lines {
                let tokens = &mut block.source[*index].tokens;
                if !tokens.description.is_empty() {
                    if hyphen_newline {
                        let rest = trim_start(&tokens.description);
                        let rest = rest.strip_prefix('-').unwrap_or(rest);
                        tokens.description = rest.trim_start_matches(is_space).to_owned();
                        tokens.post_name.clear();
                    } else {
                        let blank =
                            tokens.description.len() - trim_start(&tokens.description).len();
                        tokens.description.insert_str(blank, "- ");
                    }
                    break;
                }
                if !hyphen_newline && !tokens.name.is_empty() {
                    report = false;
                    break;
                }
            }
            if report {
                let mut finding =
                    doc.finding_at(source, self.name(), line, None, message.to_owned());
                finding.fix = Some(doc.rewrite(&block));
                findings.push(finding);
            }
        }
    }
}

impl Rule for RequireHyphenBeforeParamDescription {
    fn name(&self) -> &'static str {
        "jsdoc/require-hyphen-before-param-description"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for doc in jsdoc::all(source) {
            self.check(source, &doc, findings);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RequireHyphenBeforeParamDescription;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(
            &RequireHyphenBeforeParamDescription,
            "require-hyphen-before-param-description",
            "src/a.ts",
            &[],
        );
    }
}
