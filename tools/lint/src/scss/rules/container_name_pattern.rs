//! `container-name-pattern` of Stylelint 17.16.0 (`lib/rules/container-name-pattern/index.mjs`,
//! REF-STYLELINT-SOURCE): a container name not matching the pattern, in a `container` or
//! `container-name` declaration (before its `/`, CSS-wide keywords and Sass values left out) or an
//! `@container`'s parameters (`and`, `or`, `none`, `not` left out), reported over it; the project's
//! pattern stylelint-config-standard's kebab-case, an optional `--` first.

use super::reference::{BASIC_KEYWORDS, has};
use super::{
    Context, Pattern, Report, SheetRule, is_standard_at_rule, is_standard_declaration,
    is_standard_value,
};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part};

/// The rule and its pattern.
pub struct ContainerNamePattern {
    /// The pattern.
    pub pattern: Pattern,
}

impl ContainerNamePattern {
    /// The project's configuration: `^(--)?([a-z][a-z0-9]*)(-[a-z0-9]+)*$`.
    #[must_use]
    pub fn project() -> Self {
        Self {
            pattern: Pattern::new("^(--)?([a-z][a-z0-9]*)(-[a-z0-9]+)*$"),
        }
    }
}

impl SheetRule for ContainerNamePattern {
    fn name(&self) -> &'static str {
        "stylelint/container-name-pattern"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("expected", "Expected \"{}\" to match pattern \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let mut report = |at: usize, name: &str| {
            reports.push(
                Report::new(at, at + name.len(), "expected").with(&[name, &self.pattern.text]),
            );
        };
        for index in sheet.sheet.walk() {
            let node = sheet.node(index);
            let property = node.prop.to_ascii_lowercase();
            if node.is(Kind::Declaration)
                && (property == "container" || property == "container-name")
                && is_standard_declaration(sheet, index)
            {
                let start = sheet.value_start(node);
                let parsed = value::parse(&node.value);
                let mut container_type = false;
                parsed.walk(&mut |_, part| {
                    if container_type {
                        return true;
                    }
                    if part.kind == Part::Div && part.value == "/" {
                        container_type = true;
                    }
                    if part.kind != Part::Word {
                        return false;
                    }
                    if !has(BASIC_KEYWORDS, &part.value.to_lowercase())
                        && is_standard_value(&part.value)
                        && !self.pattern.matches(&part.value)
                    {
                        report(start + part.start, &part.value);
                    }
                    true
                });
            } else if node.is(Kind::AtRule)
                && node.name.eq_ignore_ascii_case("container")
                && is_standard_at_rule(node)
            {
                let start = sheet.params_start(node);
                value::parse(&node.params).walk(&mut |_, part| {
                    if part.kind != Part::Word {
                        return false;
                    }
                    let keyword = matches!(
                        part.value.to_lowercase().as_str(),
                        "and" | "or" | "none" | "not"
                    );
                    if !keyword && !self.pattern.matches(&part.value) {
                        report(start + part.start, &part.value);
                    }
                    true
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ContainerNamePattern;
    use crate::scss::rules::tester::{pattern, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/container-name-pattern.txt",
            &|config| {
                Ok(Box::new(ContainerNamePattern {
                    pattern: pattern(config)?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (21, 9, 2),
            "{:#?}",
            tally.apart
        );
        assert!(ContainerNamePattern::project().pattern.matches("--sidebar"));
    }
}
