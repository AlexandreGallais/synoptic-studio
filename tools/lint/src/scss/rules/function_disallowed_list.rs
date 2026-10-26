//! `function-disallowed-list` of Stylelint 17.16.0 (`lib/rules/function-disallowed-list/index.mjs`,
//! REF-STYLELINT-SOURCE): a function of a declaration's value whose name matches one of the
//! option's names or patterns, reported over its name. The project refuses the colour functions
//! outside the tokens' files, so that every colour is a token (`color-mix()` still blends them);
//! the tokens' files are left out by the project's scope.

use super::color_named::is_standard_function;
use super::{Context, Matcher, Report, SheetRule, matches_any};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part};

/// The rule and its option.
pub struct FunctionDisallowedList {
    /// The functions refused.
    pub names: Vec<Matcher>,
}

impl FunctionDisallowedList {
    /// The project's configuration: the colour functions, in any case.
    #[must_use]
    pub fn project() -> Self {
        Self {
            names: vec![Matcher::new(
                "/^(rgba?|hsla?|hwb|lab|lch|oklab|oklch|color|device-cmyk)$/i",
            )],
        }
    }
}

impl SheetRule for FunctionDisallowedList {
    fn name(&self) -> &'static str {
        "stylelint/function-disallowed-list"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Disallowed function \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Declaration) {
            if !node.value.contains('(') {
                continue;
            }
            let start = sheet.value_start(node);
            value::parse(&node.value).walk(&mut |_, part| {
                if part.kind == Part::Function
                    && is_standard_function(part)
                    && matches_any(&self.names, &part.value)
                {
                    let at = start + part.start;
                    reports.push(
                        Report::new(at, at + part.value.len(), "rejected").with(&[&part.value]),
                    );
                }
                true
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionDisallowedList;
    use crate::scss::rules::tester::{Js, matchers, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/function-disallowed-list.txt",
            &|config| {
                let wrapped = Js::Object(vec![("names".to_owned(), config.clone())]);
                Ok(Box::new(FunctionDisallowedList {
                    names: matchers(Some(&wrapped), "names")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (16, 20, 0),
            "{:#?}",
            tally.apart
        );
        let project = FunctionDisallowedList::project();
        assert!(
            super::matches_any(&project.names, "RGBA")
                && !super::matches_any(&project.names, "color-mix")
        );
    }
}
