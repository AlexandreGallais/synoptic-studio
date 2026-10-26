//! `function-color-channel` and `function-color-relative` of stylelint-scss 7.3.0
//! (`src/rules/function-color-channel/index.js`, `src/rules/function-color-relative/index.js`,
//! REF-STYLELINT-SOURCE): the colour functions Sass deprecates, reported over their names: those
//! reading a channel (`red()`, `color.alpha()`) for `color.channel()`, those changing a colour by
//! an amount (`darken()`, `fade-in()`) for `color.scale()`, outside `filter` but inside its
//! `drop-shadow()`.

use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part, Value};

/// Which of the two rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Colour {
    /// `function-color-channel`.
    Channel,
    /// `function-color-relative`.
    Relative,
}

/// The rule.
pub struct ColorFunctions {
    /// Which.
    pub colour: Colour,
}

/// The functions reading a channel.
const CHANNEL: &[&str] = &[
    "color.alpha",
    "color.blackness",
    "color.blue",
    "color.green",
    "color.hue",
    "color.lightness",
    "color.red",
    "color.saturation",
    "color.whiteness",
    "alpha",
    "blackness",
    "blue",
    "green",
    "hue",
    "lightness",
    "opacity",
    "red",
    "saturation",
];

/// The functions changing a colour by an amount.
const RELATIVE: &[&str] = &[
    "saturate",
    "desaturate",
    "darken",
    "lighten",
    "opacify",
    "fade-in",
    "transparentize",
    "fade-out",
];

impl ColorFunctions {
    /// Whether a part is one of the rule's functions.
    fn is_refused(&self, parsed: &Value, index: usize) -> bool {
        let part = parsed.part(index);
        let names = match self.colour {
            Colour::Channel => CHANNEL,
            Colour::Relative => RELATIVE,
        };
        part.kind == Part::Function && names.contains(&part.value.as_str())
    }
}

impl SheetRule for ColorFunctions {
    fn name(&self) -> &'static str {
        match self.colour {
            Colour::Channel => "scss/function-color-channel",
            Colour::Relative => "scss/function-color-relative",
        }
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        match self.colour {
            Colour::Channel => &[("expected", "Expected the color.channel function to be used")],
            Colour::Relative => &[("expected", "Expected the scale-color function to be used")],
        }
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Declaration) {
            let parsed = value::parse(&node.value);
            let start = sheet.value_start(node);
            let filter = node.prop == "filter";
            parsed.walk(&mut |index, part| {
                if part.kind != Part::Function || part.value.is_empty() {
                    return true;
                }
                let found: Vec<usize> = if !filter && self.is_refused(&parsed, index) {
                    vec![index]
                } else if filter && self.colour == Colour::Relative && part.value == "drop-shadow" {
                    part.nodes
                        .iter()
                        .copied()
                        .filter(|each| self.is_refused(&parsed, *each))
                        .collect()
                } else {
                    Vec::new()
                };
                for each in found {
                    let function = parsed.part(each);
                    let at = start + function.start;
                    reports.push(Report::new(at, at + function.value.len(), "expected"));
                }
                true
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ColorFunctions, Colour};
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss_channel() {
        let tally = run(
            "stylelint-scss/function-color-channel.txt",
            &|_| {
                Ok(Box::new(ColorFunctions {
                    colour: Colour::Channel,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (4, 18, 0),
            "{:#?}",
            tally.apart
        );
    }

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss_relative() {
        let tally = run(
            "stylelint-scss/function-color-relative.txt",
            &|_| {
                Ok(Box::new(ColorFunctions {
                    colour: Colour::Relative,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (3, 12, 0),
            "{:#?}",
            tally.apart
        );
    }
}
