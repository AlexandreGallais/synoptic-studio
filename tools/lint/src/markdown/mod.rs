//! The Markdown rules (EN-077): `@eslint/markdown`'s rules the project keeps, run on every
//! Markdown page of the linter's targets, since oxlint reads no Markdown (`REF-OXLINT`). A page is
//! read and parsed once into its tree, then every rule reads that tree and the text under it, as
//! `@eslint/markdown`'s rules did (`REF-ESLINT-MARKDOWN-RULES`); a rule keeps its name and its
//! messages there, so that findings compare one for one.

pub mod page;
pub mod rules;
pub mod slugger;
pub mod text;

use std::fs;
use std::path::Path;

use crate::PARSE_RULE;
use crate::finding::Finding;
use crate::glob::matches_any;
use crate::project::Project;
use page::Page;

/// A Markdown rule.
pub trait MarkdownRule: Sync {
    /// Its name, as `@eslint/markdown` writes it.
    fn name(&self) -> &'static str;

    /// Checks a parsed page.
    fn check(&self, page: &Page, findings: &mut Vec<Finding>);
}

/// Pages never linted beside the linter's ignored paths: the changelog, written by
/// `release prepare` and checked by `release check`, its entries up to 0.2.0 by release-please.
pub const IGNORED: [&str; 1] = ["CHANGELOG.md"];

/// Whether a path is a Markdown page the rules read.
#[must_use]
pub fn is_page(path: &str) -> bool {
    Path::new(path)
        .extension()
        .is_some_and(|extension| extension == "md")
        && !matches_any(&IGNORED, path)
}

/// The findings of a set of rules on a page's text.
#[must_use]
pub fn lint_text(path: &str, text: &str, rules: &[&dyn MarkdownRule]) -> Vec<Finding> {
    match Page::parse(path, text) {
        Ok(page) => {
            let mut findings = Vec::new();
            for rule in rules {
                rule.check(&page, &mut findings);
            }
            findings
        }
        Err(message) => vec![Finding {
            path: path.to_owned(),
            line: 1,
            column: 1,
            rule: PARSE_RULE,
            message,
            fix: None,
        }],
    }
}

/// The findings of every Markdown rule on a page read from the disk.
#[must_use]
pub fn lint_path(project: &Project, path: &str) -> Vec<Finding> {
    match fs::read_to_string(project.root.join(path)) {
        Ok(text) => lint_text(path, &text, rules::ALL),
        Err(error) => vec![Finding {
            path: path.to_owned(),
            line: 1,
            column: 1,
            rule: PARSE_RULE,
            message: format!("Cannot read the file: {error}."),
            fix: None,
        }],
    }
}

/// What the Markdown rules' tests share: a rule run on a page, as `RuleTester` ran it.
#[cfg(test)]
pub mod testing {
    use super::{MarkdownRule, lint_text};
    use crate::finding::Finding;

    /// The findings of one rule on a page's text.
    #[must_use]
    pub fn lint(rule: &dyn MarkdownRule, text: &str) -> Vec<Finding> {
        let mut findings = lint_text("a.md", text, &[rule]);
        findings.sort_by_key(|finding| (finding.line, finding.column));
        findings
    }

    /// Each finding's line, column and message, sorted by place.
    #[must_use]
    pub fn places(rule: &dyn MarkdownRule, text: &str) -> Vec<(usize, usize, String)> {
        lint(rule, text)
            .into_iter()
            .map(|finding| (finding.line, finding.column, finding.message))
            .collect()
    }

    /// The messages of a rule's findings on a page, sorted by place.
    #[must_use]
    pub fn messages(rule: &dyn MarkdownRule, text: &str) -> Vec<String> {
        crate::testing::messages(&lint(rule, text))
    }

    /// A page's text with the fixes of a rule applied.
    #[must_use]
    pub fn fixed(rule: &dyn MarkdownRule, text: &str) -> String {
        let findings: Vec<Finding> = lint(rule, text);
        crate::testing::fixed(text, &findings)
    }
}

#[cfg(test)]
mod tests {
    use super::page::Page;
    use super::{MarkdownRule, is_page, lint_text};
    use crate::PARSE_RULE;
    use crate::finding::Finding;

    /// Reports each heading, to see a page parsed and walked.
    struct ProbeHeadings;

    impl MarkdownRule for ProbeHeadings {
        fn name(&self) -> &'static str {
            "probe/headings"
        }

        fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
            for node in super::page::nodes(&page.root) {
                if let markdown::mdast::Node::Heading(heading) = node {
                    findings.push(page.finding(
                        self.name(),
                        super::page::start(node),
                        format!("h{}", heading.depth),
                    ));
                }
            }
        }
    }

    #[test]
    fn reads_every_page_but_the_changelog() {
        assert!(is_page("docs/a.md"));
        assert!(is_page("README.md"));
        assert!(!is_page("CHANGELOG.md"));
        assert!(!is_page("src/a.ts"));
    }

    #[test]
    fn runs_each_rule_on_the_parsed_page() {
        let findings = lint_text(
            "a.md",
            "---\ntitle: x\n---\n# A\n\n> ## B\n",
            &[&ProbeHeadings],
        );
        let found: Vec<(usize, usize, &str)> = findings
            .iter()
            .map(|each| (each.line, each.column, each.message.as_str()))
            .collect();

        // The front matter is read as such, not as a rule and a heading.
        assert_eq!(found, [(4, 1, "h1"), (6, 3, "h2")]);
        assert_ne!(findings[0].rule, PARSE_RULE);
    }
}
