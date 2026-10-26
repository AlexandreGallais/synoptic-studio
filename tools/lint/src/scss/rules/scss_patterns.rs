//! The naming patterns of stylelint-scss 7.3.0 (`src/rules/at-function-pattern`,
//! `at-mixin-pattern`, `dollar-variable-pattern` and `percent-placeholder-pattern`,
//! REF-STYLELINT-SOURCE): a function's, a mixin's, a variable's or a placeholder's name not
//! matching the pattern, reported over the name; the project's pattern
//! stylelint-config-standard-scss's kebab-case, a leading `-` allowed.

use super::scss_selectors::{
    has_interpolating_ampersand, is_standard_rule, is_standard_selector, resolve,
};
use super::{Context, Pattern, Report, SheetRule};
use crate::scss::selector::{self, Kind as Part};
use crate::scss::tree::Kind;

/// What a pattern rule names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Named {
    /// `@function` names.
    Function,
    /// `@mixin` names.
    Mixin,
    /// `$variables`, those at the root or inside blocks left out when told.
    Variable {
        /// Whether those at the root are left out (`ignore: ["global"]`).
        ignore_global: bool,
        /// Whether those inside blocks are left out (`ignore: ["local"]`).
        ignore_local: bool,
    },
    /// `%placeholders`.
    Placeholder,
}

/// A pattern rule.
pub struct NamePattern {
    /// What it names.
    pub named: Named,
    /// The pattern.
    pub pattern: Pattern,
}

impl NamePattern {
    /// The project's configuration of a kind: `^(-?[a-z][a-z0-9]*)(-[a-z0-9]+)*$`.
    #[must_use]
    pub fn project(named: Named) -> Self {
        Self {
            named,
            pattern: Pattern::new("^(-?[a-z][a-z0-9]*)(-[a-z0-9]+)*$"),
        }
    }

    /// A name reported over a word of a node.
    fn report(
        &self,
        sheet: &Context,
        index: usize,
        name: &str,
        word: &str,
        reports: &mut Vec<Report>,
    ) {
        if self.pattern.matches(name) {
            return;
        }
        let (start, end) = sheet.word_range(sheet.node(index), word);
        reports.push(Report::new(start, end, "expected").with(&[name, &self.pattern.text]));
    }
}

/// An at-rule's name without its parameters (`replace(/(\s*)\([\s\S]*\)/g, "")`).
fn without_parameters(params: &str) -> &str {
    match (params.find('('), params.rfind(')')) {
        (Some(open), Some(close)) if close > open => {
            let name = &params[..open];
            if close + 1 == params.len() {
                name.trim_end()
            } else {
                params
            }
        }
        _ => params,
    }
}

impl SheetRule for NamePattern {
    fn name(&self) -> &'static str {
        match self.named {
            Named::Function => "scss/at-function-pattern",
            Named::Mixin => "scss/at-mixin-pattern",
            Named::Variable { .. } => "scss/dollar-variable-pattern",
            Named::Placeholder => "scss/percent-placeholder-pattern",
        }
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("expected", "Expected \"{}\" to match pattern \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for index in sheet.sheet.walk() {
            let node = sheet.node(index);
            match self.named {
                Named::Function | Named::Mixin => {
                    let wanted = if self.named == Named::Function {
                        "function"
                    } else {
                        "mixin"
                    };
                    if node.is(Kind::AtRule) && node.name == wanted {
                        let name = without_parameters(&node.params);
                        self.report(sheet, index, name, name, reports);
                    }
                }
                Named::Variable {
                    ignore_global,
                    ignore_local,
                } => {
                    if !node.is(Kind::Declaration) || !node.prop.starts_with('$') {
                        continue;
                    }
                    let global = node.parent == Some(0);
                    if (ignore_global && global) || (ignore_local && !global) {
                        continue;
                    }
                    self.report(sheet, index, &node.prop[1..], &node.prop, reports);
                }
                Named::Placeholder => {
                    if !node.is(Kind::Rule)
                        || !is_standard_rule(sheet, index)
                        || !is_standard_selector(&node.selector)
                    {
                        continue;
                    }
                    let selectors = if has_interpolating_ampersand(&node.selector) {
                        resolve(sheet, &node.selector, index)
                    } else {
                        vec![node.selector.clone()]
                    };
                    for text in selectors {
                        let Ok(parsed) = selector::parse(&text) else {
                            continue;
                        };
                        for each in parsed.walk() {
                            let part = parsed.part(each);
                            if part.kind == Part::Tag && part.value.starts_with('%') {
                                self.report(sheet, index, &part.value[1..], &part.value, reports);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NamePattern, Named};
    use crate::scss::rules::tester::{pattern, run, secondary};

    /// Runs a kind's rule on its file.
    fn tally(file: &str, named: Named) -> crate::scss::rules::tester::Tally {
        run(
            file,
            &|config| {
                let ignore = secondary(config)
                    .and_then(|each| each.texts("ignore"))
                    .unwrap_or_default();
                let named = match named {
                    Named::Variable { .. } => Named::Variable {
                        ignore_global: ignore.iter().any(|each| each == "global"),
                        ignore_local: ignore.iter().any(|each| each == "local"),
                    },
                    other => other,
                };
                Ok(Box::new(NamePattern {
                    named,
                    pattern: pattern(config)?,
                }))
            },
            &[],
        )
    }

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss_at_function_pattern() {
        let tally = tally("stylelint-scss/at-function-pattern.txt", Named::Function);
        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (20, 12, 0),
            "{:#?}",
            tally.apart
        );
    }

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss_at_mixin_pattern() {
        let tally = tally("stylelint-scss/at-mixin-pattern.txt", Named::Mixin);
        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (21, 12, 0),
            "{:#?}",
            tally.apart
        );
    }

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss_dollar_variable_pattern() {
        let named = Named::Variable {
            ignore_global: false,
            ignore_local: false,
        };
        let tally = tally("stylelint-scss/dollar-variable-pattern.txt", named);
        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (15, 13, 0),
            "{:#?}",
            tally.apart
        );
    }

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss_percent_placeholder_pattern() {
        let tally = tally(
            "stylelint-scss/percent-placeholder-pattern.txt",
            Named::Placeholder,
        );
        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (17, 14, 2),
            "{:#?}",
            tally.apart
        );
    }
}
