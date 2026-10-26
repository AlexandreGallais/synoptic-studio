//! `declaration-block-no-duplicate-custom-properties` of Stylelint 17.16.0
//! (`lib/rules/declaration-block-no-duplicate-custom-properties/index.mjs`, REF-STYLELINT-SOURCE):
//! a custom property declared again in its block, but those its option ignores, reported over its
//! name at the repeat.

use super::{Context, Matcher, Report, SheetRule, is_standard_property, matches_any};

/// The rule and its option.
pub struct DeclarationBlockNoDuplicateCustomProperties {
    /// The properties left out (`ignoreProperties`).
    pub ignore_properties: Vec<Matcher>,
}

impl DeclarationBlockNoDuplicateCustomProperties {
    /// The project's configuration: none left out.
    pub const PROJECT: Self = Self {
        ignore_properties: Vec::new(),
    };
}

impl SheetRule for DeclarationBlockNoDuplicateCustomProperties {
    fn name(&self) -> &'static str {
        "stylelint/declaration-block-no-duplicate-custom-properties"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Duplicate custom property \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for block in sheet.declaration_blocks() {
            let mut seen: Vec<&str> = Vec::new();
            for index in block {
                let node = sheet.node(index);
                let prop = node.prop.as_str();
                if !is_standard_property(prop)
                    || !prop.starts_with("--")
                    || matches_any(&self.ignore_properties, prop)
                {
                    continue;
                }
                if seen.contains(&prop) {
                    let (start, end) = sheet.word_range(node, prop);
                    reports.push(Report::new(start, end, "rejected").with(&[prop]));
                    continue;
                }
                seen.push(prop);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DeclarationBlockNoDuplicateCustomProperties;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/declaration-block-no-duplicate-custom-properties.txt",
            &|config| {
                Ok(Box::new(DeclarationBlockNoDuplicateCustomProperties {
                    ignore_properties: matchers(secondary(config), "ignoreProperties")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (10, 9, 13),
            "{:#?}",
            tally.apart
        );
    }
}
