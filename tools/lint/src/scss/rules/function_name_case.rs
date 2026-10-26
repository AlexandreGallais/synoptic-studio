//! `function-name-case` of Stylelint 17.16.0 (`lib/rules/function-name-case/index.mjs`,
//! REF-STYLELINT-SOURCE): a function of a plain declaration's value not in lower case (`"lower"`,
//! the project's; the camel-case ones such as `translateX` kept so) or in upper case (`"upper"`),
//! but those its option ignores, reported over its name and fixed by rewriting it.

use super::color_named::is_standard_function;
use super::reference::CAMEL_CASE_FUNCTIONS;
use super::{Context, Matcher, Report, SheetRule, is_standard_value, matches_any};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part};

/// The rule and its options.
pub struct FunctionNameCase {
    /// Whether names are lower case (`"lower"`), else upper case (`"upper"`).
    pub lower: bool,
    /// The functions left out (`ignoreFunctions`).
    pub ignore_functions: Vec<Matcher>,
}

impl FunctionNameCase {
    /// The project's configuration: `"lower"`.
    pub const PROJECT: Self = Self {
        lower: true,
        ignore_functions: Vec::new(),
    };

    /// The name a function should have.
    fn expected(&self, name: &str) -> String {
        if !self.lower {
            return name.to_uppercase();
        }
        let lower = name.to_lowercase();
        CAMEL_CASE_FUNCTIONS
            .iter()
            .flat_map(|list| list.iter())
            .find(|camel| camel.to_lowercase() == lower)
            .map_or(lower, |camel| (*camel).to_owned())
    }
}

impl SheetRule for FunctionNameCase {
    fn name(&self) -> &'static str {
        "stylelint/function-name-case"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("expected", "Expected \"{}\" to be \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Declaration) {
            if !node.value.contains('(') || !is_standard_value(&node.value) {
                continue;
            }
            let start = sheet.value_start(node);
            let end = node.end.unwrap_or(sheet.text.len());
            value::parse(sheet.value(node)).walk(&mut |_, part| {
                if part.kind != Part::Function
                    || !is_standard_function(part)
                    || matches_any(&self.ignore_functions, &part.value)
                {
                    return true;
                }
                let expected = self.expected(&part.value);
                if expected != part.value {
                    let at = start + part.start;
                    reports.push(
                        Report::new(at, at + part.value.len(), "expected")
                            .with(&[&part.value, &expected])
                            .fixed(at, at + part.value.len(), expected)
                            .scoped(node.start, end),
                    );
                }
                true
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionNameCase;
    use crate::scss::rules::tester::{Js, matchers, primary, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/function-name-case.txt",
            &|config| {
                Ok(Box::new(FunctionNameCase {
                    lower: primary(config) == &Js::Text("lower".to_owned()),
                    ignore_functions: matchers(secondary(config), "ignoreFunctions")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (41, 55, 0),
            "{:#?}",
            tally.apart
        );
    }
}
