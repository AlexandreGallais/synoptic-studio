//! `at-rule-no-unknown` of stylelint-scss 7.3.0 (`src/rules/at-rule-no-unknown/index.js`,
//! REF-STYLELINT-SOURCE): Stylelint's `at-rule-no-unknown` run with Sass's at-rules and those its
//! option names known: an at-rule neither CSS's, Sass's nor prefixed, reported over its `@name`.

use super::prefixed::prefix;
use super::reference::{AT_KEYWORDS, has};
use super::{Context, Matcher, Report, SheetRule, is_standard_at_rule, matches_any};
use crate::scss::tree::Kind;

/// The rule and its option.
pub struct AtRuleNoUnknown {
    /// The at-rules known beside CSS's and Sass's (`ignoreAtRules`).
    pub ignore: Vec<Matcher>,
}

impl AtRuleNoUnknown {
    /// The project's configuration: none beside.
    pub const PROJECT: Self = Self { ignore: Vec::new() };
}

/// Sass's at-rules.
const SASS_AT_RULES: &[&str] = &[
    "at-root", "content", "debug", "each", "else", "else if", "error", "extend", "for", "forward",
    "function", "if", "import", "include", "media", "mixin", "return", "use", "warn", "while",
];

impl SheetRule for AtRuleNoUnknown {
    fn name(&self) -> &'static str {
        "scss/at-rule-no-unknown"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Unexpected unknown at-rule \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::AtRule) {
            let name = &node.name;
            if !is_standard_at_rule(node)
                || SASS_AT_RULES.contains(&name.as_str())
                || matches_any(&self.ignore, name)
                || !prefix(name).is_empty()
                || has(AT_KEYWORDS, &name.to_lowercase())
            {
                continue;
            }
            let word = format!("@{name}");
            let (start, end) = sheet.word_range(node, &word);
            reports.push(Report::new(start, end, "rejected").with(&[&word]));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AtRuleNoUnknown;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/at-rule-no-unknown.txt",
            &|config| {
                Ok(Box::new(AtRuleNoUnknown {
                    ignore: matchers(secondary(config), "ignoreAtRules")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (34, 9, 1),
            "{:#?}",
            tally.apart
        );
    }
}
