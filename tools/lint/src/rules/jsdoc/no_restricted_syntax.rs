//! `jsdoc/no-restricted-syntax` with the project's one context: no block holds `@todo`,
//! `@author`, `@version` nor `@since`, since work and history are tracked in `docs/` and Git
//! (`eslint-plugin-jsdoc` 65.3.1, `src/rules/noRestrictedSyntax.js`, the selector
//! `JsdocBlock:has(JsdocTag[tag=/^(todo|author|version|since)$/])` on any block).

use crate::finding::Finding;
use crate::jsdoc;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoRestrictedSyntax;

/// The tags refused.
const RESTRICTED: [&str; 4] = ["todo", "author", "version", "since"];

impl Rule for NoRestrictedSyntax {
    fn name(&self) -> &'static str {
        "jsdoc/no-restricted-syntax"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for doc in jsdoc::all(source) {
            if doc
                .block
                .tags
                .iter()
                .any(|tag| RESTRICTED.contains(&tag.tag.as_str()))
            {
                findings.push(doc.finding(
                    source,
                    self.name(),
                    "Track work and history in docs/ and git, not in JSDoc tags.".to_owned(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoRestrictedSyntax;
    use crate::jsdoc::cases::check;
    use crate::testing::{lint, messages};

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&NoRestrictedSyntax, "no-restricted-syntax", "src/a.ts");
    }

    #[test]
    fn refuses_the_tags_of_work_and_history() {
        let code = "/**\n * Text.\n *\n * @todo later\n */\nconst a = 1;\n/** @since 1.0 */\nconst b = 2;\n/** Text, @author in prose. */\nconst c = 3;\n/** @todos plural */\nconst d = 4;\n";
        let findings = lint(&NoRestrictedSyntax, "src/a.ts", code);

        // `@todo` and `@since` are block tags; `@author` mid-line is no tag, `@todos` another one.
        assert_eq!(
            findings.iter().map(|each| each.line).collect::<Vec<_>>(),
            [1, 7]
        );
        assert_eq!(
            messages(&findings)[0],
            "Track work and history in docs/ and git, not in JSDoc tags."
        );
    }
}
