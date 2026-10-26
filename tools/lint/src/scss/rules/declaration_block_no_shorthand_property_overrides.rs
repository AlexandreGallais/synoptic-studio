//! `declaration-block-no-shorthand-property-overrides` of Stylelint 17.16.0
//! (`lib/rules/declaration-block-no-shorthand-property-overrides/index.mjs`,
//! REF-STYLELINT-SOURCE): a shorthand written after one of its longhands (or of the properties it
//! resets) in its block, prefixes matched, reported over the shorthand once per longhand.

use super::prefixed::prefix;
use super::reference::{
    LONGHAND_SUB_PROPERTIES_OF_SHORTHAND_PROPERTIES, SHORTHAND_TO_RESET_TO_INITIAL_PROPERTY, get,
};
use super::{Context, Report, SheetRule};

/// The rule.
pub struct DeclarationBlockNoShorthandPropertyOverrides;

impl SheetRule for DeclarationBlockNoShorthandPropertyOverrides {
    fn name(&self) -> &'static str {
        "stylelint/declaration-block-no-shorthand-property-overrides"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Overridden property \"{}\" by shorthand \"{}\"")]
    }

    fn computed(&self, key: &str, args: &[String]) -> Option<String> {
        // The original's arguments are the shorthand, then the property it overrides, which its
        // message names first.
        let [shorthand, original] = args else {
            return None;
        };
        (key == "rejected")
            .then(|| format!("Overridden property \"{original}\" by shorthand \"{shorthand}\""))
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for block in sheet.declaration_blocks() {
            let mut declared: Vec<(String, String)> = Vec::new();
            for index in block {
                let node = sheet.node(index);
                let prop = &node.prop;
                let vendor = prefix(prop).to_lowercase();
                let bare = prop[prefix(prop).len()..].to_lowercase();
                let lower = prop.to_lowercase();
                match declared.iter_mut().find(|(key, _)| *key == lower) {
                    Some(entry) => entry.1.clone_from(prop),
                    None => declared.push((lower, prop.clone())),
                }
                let longhands =
                    get(LONGHAND_SUB_PROPERTIES_OF_SHORTHAND_PROPERTIES, &bare).unwrap_or_default();
                let resets = get(SHORTHAND_TO_RESET_TO_INITIAL_PROPERTY, &bare).unwrap_or_default();
                let mut union: Vec<&str> = longhands.to_vec();
                union.extend(resets.iter().filter(|each| !longhands.contains(each)));
                for property in union {
                    let wanted = format!("{vendor}{property}");
                    if let Some((_, original)) = declared.iter().find(|(key, _)| *key == wanted) {
                        let (start, end) = sheet.word_range(node, prop);
                        reports.push(Report::new(start, end, "rejected").with(&[prop, original]));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DeclarationBlockNoShorthandPropertyOverrides;
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/declaration-block-no-shorthand-property-overrides.txt",
            &|_| Ok(Box::new(DeclarationBlockNoShorthandPropertyOverrides)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (12, 20, 9),
            "{:#?}",
            tally.apart
        );
    }
}
