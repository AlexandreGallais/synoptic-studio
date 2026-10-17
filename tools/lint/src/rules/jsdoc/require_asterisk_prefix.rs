//! `jsdoc/require-asterisk-prefix` with `always`: every line of a block but its `*/` line opens
//! with `*` (`eslint-plugin-jsdoc` 65.3.1, `src/rules/requireAsteriskPrefix.js`). Autofixed; the
//! first line without it is reported.

use crate::finding::Finding;
use crate::jsdoc::{self, Doc};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct RequireAsteriskPrefix;

impl RequireAsteriskPrefix {
    /// The finding of one block, if any.
    fn check(&self, source: &Source, doc: &Doc) -> Option<Finding> {
        let mut block = doc.block.clone();
        let index = block.source.iter().position(|line| {
            let tokens = &line.tokens;
            (tokens.end.is_empty() || !tokens.tag.is_empty()) && tokens.delimiter.is_empty()
        })?;
        let tokens = &mut block.source[index].tokens;
        if tokens.start.is_empty() {
            tokens.start = format!("{} ", doc.indent);
        }
        "*".clone_into(&mut tokens.delimiter);
        tokens.post_delimiter = if tokens.tag.is_empty() && tokens.description.is_empty() {
            String::new()
        } else {
            " ".to_owned()
        };
        let mut finding = doc.finding_at(
            source,
            self.name(),
            index,
            Some(0),
            "Expected JSDoc line to have the prefix.".to_owned(),
        );
        finding.fix = Some(doc.rewrite(&block));
        Some(finding)
    }
}

impl Rule for RequireAsteriskPrefix {
    fn name(&self) -> &'static str {
        "jsdoc/require-asterisk-prefix"
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
    use super::RequireAsteriskPrefix;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(
            &RequireAsteriskPrefix,
            "require-asterisk-prefix",
            "src/a.ts",
            &[],
        );
    }
}
