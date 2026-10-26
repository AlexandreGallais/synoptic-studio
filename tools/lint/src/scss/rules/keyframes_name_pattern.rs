//! `keyframes-name-pattern` of Stylelint 17.16.0 (`lib/rules/keyframes-name-pattern/index.mjs`,
//! REF-STYLELINT-SOURCE): a `@keyframes` name (prefixed or not) not matching the pattern, an
//! interpolated one left out, reported over its name; the project's pattern
//! stylelint-config-standard's kebab-case.

use super::{Context, Pattern, Report, SheetRule, has_interpolation};
use crate::scss::tree::Kind;

/// The rule and its pattern.
pub struct KeyframesNamePattern {
    /// The pattern.
    pub pattern: Pattern,
}

impl KeyframesNamePattern {
    /// The project's configuration: kebab-case.
    #[must_use]
    pub fn project() -> Self {
        Self {
            pattern: Pattern::kebab(),
        }
    }
}

/// Whether an at-rule's name is `keyframes`, prefixed or not (`/^(-(o|moz|ms|webkit)-)?keyframes$/i`).
#[must_use]
pub fn is_keyframes(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    let bare = ["-o-", "-moz-", "-ms-", "-webkit-"]
        .iter()
        .find_map(|prefix| lower.strip_prefix(prefix))
        .unwrap_or(&lower);
    bare == "keyframes"
}

impl SheetRule for KeyframesNamePattern {
    fn name(&self) -> &'static str {
        "stylelint/keyframes-name-pattern"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("expected", "Expected \"{}\" to match pattern \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::AtRule) {
            let name = &node.params;
            if !is_keyframes(&node.name) || has_interpolation(name) || self.pattern.matches(name) {
                continue;
            }
            let start = sheet.params_start(node);
            reports.push(
                Report::new(start, start + name.len(), "expected")
                    .with(&[name, &self.pattern.text]),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{KeyframesNamePattern, is_keyframes};
    use crate::scss::rules::tester::{pattern, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/keyframes-name-pattern.txt",
            &|config| {
                Ok(Box::new(KeyframesNamePattern {
                    pattern: pattern(config)?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (6, 5, 0),
            "{:#?}",
            tally.apart
        );
        assert!(is_keyframes("-WEBKIT-Keyframes") && !is_keyframes("-x-keyframes"));
        assert!(KeyframesNamePattern::project().pattern.matches("fade-in"));
    }
}
