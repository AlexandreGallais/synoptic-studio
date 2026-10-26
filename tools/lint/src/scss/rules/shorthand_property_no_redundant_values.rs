//! `shorthand-property-no-redundant-values` of Stylelint 17.16.0
//! (`lib/rules/shorthand-property-no-redundant-values/index.mjs`, REF-STYLELINT-SOURCE): a
//! box shorthand of a plain declaration (`margin`, `padding`, `border-*`, `gap`, `inset`,
//! `overflow`…) whose values could be fewer (`1px 1px` for `1px`), `border-radius` with its `/`
//! condensed on each side, `var()` and separators left alone, reported over the value and fixed
//! by its shortest form.

use super::prefixed::prefix;
use super::{
    Context, Report, SheetRule, is_standard_declaration, is_standard_property, is_standard_value,
};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part};

/// The rule and its option.
pub struct ShorthandPropertyNoRedundantValues {
    /// Whether four edge values condensable to three are left alone
    /// (`ignore: ["four-into-three-edge-values"]`).
    pub ignore_four_into_three: bool,
}

impl ShorthandPropertyNoRedundantValues {
    /// The project's configuration: nothing left alone.
    pub const PROJECT: Self = Self {
        ignore_four_into_three: false,
    };
}

/// The shorthands checked.
const SHORTHANDS: &[&str] = &[
    "margin",
    "margin-block",
    "margin-inline",
    "padding",
    "padding-block",
    "padding-inline",
    "border-color",
    "border-style",
    "border-width",
    "border-radius",
    "border-block-color",
    "border-block-style",
    "border-block-width",
    "border-inline-color",
    "border-inline-style",
    "border-inline-width",
    "gap",
    "grid-gap",
    "overflow",
    "overscroll-behavior",
    "scroll-margin",
    "scroll-margin-block",
    "scroll-margin-inline",
    "scroll-padding",
    "scroll-padding-block",
    "scroll-padding-inline",
    "inset",
    "inset-block",
    "inset-inline",
];

/// The shorthands of four edges.
const FOUR_EDGES: &[&str] = &[
    "margin",
    "padding",
    "border-color",
    "border-style",
    "border-width",
    "scroll-margin",
    "scroll-padding",
    "inset",
];

/// Whether a part is a `var()`.
fn is_var(part: &value::Part) -> bool {
    part.kind == Part::Function && part.value.eq_ignore_ascii_case("var")
}

/// The shortest `border-radius` of a value split by one `/` and no other divider, each side
/// condensed unless it holds a `var()`; none for another value.
fn radius(parsed: &value::Value, value: &str) -> Option<String> {
    let divider = |at: &usize| parsed.part(*at).kind == Part::Div;
    let slash = parsed
        .nodes
        .iter()
        .any(|at| divider(at) && parsed.part(*at).value == "/");
    let other = parsed
        .nodes
        .iter()
        .any(|at| divider(at) && parsed.part(*at).value != "/");
    if !slash || other {
        return None;
    }
    let mut sides: [(Vec<&str>, bool); 2] = [(Vec::new(), false), (Vec::new(), false)];
    let mut side = 0;
    for &at in &parsed.nodes {
        let part = parsed.part(at);
        if part.kind == Part::Div {
            side = 1;
            continue;
        }
        if is_var(part) {
            sides[side].1 = true;
        }
        if matches!(part.kind, Part::Word | Part::Function) {
            sides[side].0.push(&value[part.start..part.end]);
        }
    }
    let condensed = |(values, var): &(Vec<&str>, bool)| {
        if *var {
            values.join(" ")
        } else {
            condense(values).join(" ")
        }
    };
    Some(
        format!("{} / {}", condensed(&sides[0]), condensed(&sides[1]))
            .trim()
            .to_owned(),
    )
}

/// The shortest form of up to four edge values (`canCondense`), in any case compared.
fn condense<'v>(values: &[&'v str]) -> Vec<&'v str> {
    let get = |at: usize| values.get(at).copied().unwrap_or("");
    let (top, right, bottom, left) = (get(0), get(1), get(2), get(3));
    let (t, r, b, l) = (
        top.to_lowercase(),
        right.to_lowercase(),
        bottom.to_lowercase(),
        left.to_lowercase(),
    );
    let one = t == r && ((t == b && (b == l || l.is_empty())) || (b.is_empty() && l.is_empty()));
    let two = (t == b && r == l) || (t == b && l.is_empty() && t != r);
    let form: Vec<&str> = if one {
        vec![top]
    } else if two {
        vec![top, right]
    } else if r == l {
        vec![top, right, bottom]
    } else {
        vec![top, right, bottom, left]
    };
    form.into_iter().filter(|each| !each.is_empty()).collect()
}

impl SheetRule for ShorthandPropertyNoRedundantValues {
    fn name(&self) -> &'static str {
        "stylelint/shorthand-property-no-redundant-values"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("expected", "Expected \"{}\" to be \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (index, node) in sheet.all(Kind::Declaration) {
            let value = &node.value;
            if !value.contains(char::is_whitespace)
                || !is_standard_declaration(sheet, index)
                || !is_standard_property(&node.prop)
                || !is_standard_value(value)
            {
                continue;
            }
            let lower = node.prop.to_lowercase();
            let property = &lower[prefix(&lower).len()..];
            if !SHORTHANDS.contains(&property) {
                continue;
            }
            let parsed = value::parse(value);
            let text_of = |at: usize| &value[parsed.part(at).start..parsed.part(at).end];
            let start = sheet.value_start(node);
            let scope = (node.start, node.end.unwrap_or(sheet.text.len()));
            let push = |shortest: String, reports: &mut Vec<Report>| {
                let raw_end = start + sheet.value(node).len();
                reports.push(
                    Report::new(start, start + value.len(), "expected")
                        .with(&[value, &shortest])
                        .fixed(start, raw_end, shortest)
                        .scoped(scope.0, scope.1),
                );
            };
            if property == "border-radius"
                && let Some(shortest) = radius(&parsed, value)
            {
                let normalized: String = value
                    .trim()
                    .to_lowercase()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");
                if normalized != shortest.to_lowercase() {
                    push(shortest, reports);
                }
                continue;
            }
            let mut values: Vec<&str> = Vec::new();
            let mut skip = false;
            for &at in &parsed.nodes {
                let part = parsed.part(at);
                if part.kind == Part::Div || is_var(part) {
                    skip = true;
                    break;
                }
                if matches!(part.kind, Part::Word | Part::Function) {
                    values.push(text_of(at));
                }
            }
            if skip || values.len() <= 1 || values.len() > 4 {
                continue;
            }
            let shortest = condense(&values);
            if self.ignore_four_into_three
                && FOUR_EDGES.contains(&property)
                && values.len() == 4
                && shortest.len() == 3
            {
                continue;
            }
            let shortest = shortest.join(" ");
            if shortest.to_lowercase() != values.join(" ").to_lowercase() {
                push(shortest, reports);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ShorthandPropertyNoRedundantValues;
    use crate::scss::rules::tester::{run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/shorthand-property-no-redundant-values.txt",
            &|config| {
                let ignore = secondary(config)
                    .and_then(|each| each.texts("ignore"))
                    .unwrap_or_default();
                Ok(Box::new(ShorthandPropertyNoRedundantValues {
                    ignore_four_into_three: ignore
                        .iter()
                        .any(|each| each == "four-into-three-edge-values"),
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (42, 54, 4),
            "{:#?}",
            tally.apart
        );
    }
}
