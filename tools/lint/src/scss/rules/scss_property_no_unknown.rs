//! `property-no-unknown` of stylelint-scss 7.3.0 (`src/rules/property-no-unknown/index.js` and
//! its utils `isStandardSyntaxProperty`, `isStandardSyntaxDeclaration`, `hasInterpolation`,
//! REF-STYLELINT-SOURCE): a property known-css-properties 0.37.0 does not list, nested properties
//! read whole (`font: { family: x }` is `font-family`), a module's namespace dropped; prefixed
//! properties, custom properties and Sass's own left alone; reported over the property.

use super::reference::KNOWN_PROPERTIES;
use super::{Context, Matcher, Report, SheetRule, has_any_interpolation, matches_any};
use crate::scss::tree::Kind;

/// The rule and its options.
pub struct PropertyNoUnknown {
    /// Properties left alone (`ignoreProperties`).
    pub ignore_properties: Vec<Matcher>,
    /// Rules whose declarations are left alone, by selector (`ignoreSelectors`).
    pub ignore_selectors: Vec<Matcher>,
    /// At-rules whose declarations are left alone, by name (`ignoreAtRules`).
    pub ignore_at_rules: Vec<Matcher>,
    /// Whether prefixed properties are checked too (`checkPrefixed`).
    pub check_prefixed: bool,
}

impl PropertyNoUnknown {
    /// The project's configuration: the original's defaults.
    pub const PROJECT: Self = Self {
        ignore_properties: Vec::new(),
        ignore_selectors: Vec::new(),
        ignore_at_rules: Vec::new(),
        check_prefixed: false,
    };
}

/// Whether a property is plain for stylelint-scss (`isStandardSyntaxProperty` of its utils).
fn is_plain(property: &str) -> bool {
    !property.starts_with('$')
        && !property.starts_with('@')
        && !property.ends_with('+')
        && !property.ends_with("+_")
        && !has_any_interpolation(property)
}

/// Whether a property starts with a vendor prefix (`/^(-\w+-)/`).
fn is_prefixed(property: &str) -> bool {
    property.strip_prefix('-').is_some_and(|rest| {
        let word = rest
            .bytes()
            .take_while(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
            .count();
        // `\w+` backtracks: `-a_b-` and `-ab-c-` both match, so any `-` after a first word
        // character, with only word characters before it, does.
        word > 0
            && rest[1..]
                .bytes()
                .position(|byte| byte == b'-')
                .is_some_and(|at| at < word)
    })
}

impl SheetRule for PropertyNoUnknown {
    fn name(&self) -> &'static str {
        "scss/property-no-unknown"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Unexpected unknown property \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (index, node) in sheet.all(Kind::Declaration) {
            let mut prop = node
                .prop
                .split_once('.')
                .map_or(node.prop.as_str(), |(_, rest)| rest)
                .to_owned();
            let parent = sheet.node(node.parent.unwrap_or(0));
            // `isStandardSyntaxDeclaration` of stylelint-scss.
            let standard = node.prop.starts_with('$')
                || node.prop.contains(".$")
                || !((node.prop.starts_with('@') && !node.prop.starts_with("@{"))
                    || (parent.is(Kind::AtRule) && parent.after_name == ":")
                    || (parent.is(Kind::Rule)
                        && parent.selector.starts_with('#')
                        && parent.selector.ends_with("()")));
            if !is_plain(&prop)
                || !standard
                || prop.starts_with("--")
                || (!self.check_prefixed && is_prefixed(&prop))
                || matches_any(&self.ignore_properties, &prop)
                || (parent.is(Kind::Rule) && matches_any(&self.ignore_selectors, &parent.selector))
                || sheet.ancestors(index).iter().any(|each| {
                    each.is(Kind::AtRule) && matches_any(&self.ignore_at_rules, &each.name)
                })
            {
                continue;
            }
            let mut pointer = node.parent;
            while let Some(at) = pointer {
                let each = sheet.node(at);
                if !(each.is(Kind::Rule)
                    && each.selector.ends_with(':')
                    && !each.selector.starts_with("--"))
                {
                    break;
                }
                prop = format!("{}-{prop}", each.selector.replacen(':', "", 1));
                pointer = each.parent;
            }
            while let Some(at) = pointer {
                let each = sheet.node(at);
                if !(each.is(Kind::Declaration) && each.is_nested() && !each.prop.is_empty()) {
                    break;
                }
                prop = format!("{}-{prop}", each.prop);
                pointer = each.parent;
            }
            if KNOWN_PROPERTIES.contains(&prop.to_lowercase().as_str()) {
                continue;
            }
            let (start, end) = sheet.word_range(node, &prop);
            reports.push(Report::new(start, end, "rejected").with(&[&prop]));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PropertyNoUnknown, is_prefixed};
    use crate::scss::rules::tester::{Js, matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/property-no-unknown.txt",
            &|config| {
                let options = secondary(config);
                Ok(Box::new(PropertyNoUnknown {
                    ignore_properties: matchers(options, "ignoreProperties")?,
                    ignore_selectors: matchers(options, "ignoreSelectors")?,
                    ignore_at_rules: matchers(options, "ignoreAtRules")?,
                    check_prefixed: options.and_then(|each| each.get("checkPrefixed"))
                        == Some(&Js::Bool(true)),
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (32, 17, 5),
            "{:#?}",
            tally.apart
        );
        assert!(
            is_prefixed("-webkit-a")
                && is_prefixed("-a_b-c")
                && !is_prefixed("-a")
                && !is_prefixed("a-b")
                && !is_prefixed("--a")
        );
    }
}
