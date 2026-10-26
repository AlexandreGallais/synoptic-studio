//! `declaration-property-value-keyword-no-deprecated` of Stylelint 17.16.0
//! (`lib/rules/declaration-property-value-keyword-no-deprecated/index.mjs`, REF-STYLELINT-SOURCE):
//! a deprecated keyword of a plain declaration's value for its property (`overflow: overlay`,
//! `word-break: break-word`, the deprecated system colours…), outside functions but colour ones,
//! values holding Sass and the values its option ignores, reported over it; one with a standard
//! form fixed by rewriting it. The map's keys are looked up in lower case, as the original looks
//! them up, so that `image-rendering`'s camel-case ones never match.

use super::reference::{COLOR_FUNCTIONS, SINGLE_VALUE_COLOR_PROPERTIES, has};
use super::{
    Context, Matcher, Report, SheetRule, is_standard_declaration, is_standard_value, matches_any,
};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part};

/// The rule and its option.
pub struct DeclarationPropertyValueKeywordNoDeprecated {
    /// The values left out (`ignoreKeywords`).
    pub ignore: Vec<Matcher>,
}

impl DeclarationPropertyValueKeywordNoDeprecated {
    /// The project's configuration: none left out.
    pub const PROJECT: Self = Self { ignore: Vec::new() };
}

/// A property's deprecated keywords: with their standard forms, or refused.
enum Keywords {
    /// Each keyword and its standard form.
    Mapped(&'static [(&'static str, &'static str)]),
    /// The keywords refused.
    Refused(&'static [&'static str]),
}

/// The deprecated system colours and their standard forms.
const DEPRECATED_COLORS: &[(&str, &str)] = &[
    ("activecaption", "canvas"),
    ("appworkspace", "canvas"),
    ("background", "canvas"),
    ("inactivecaption", "canvas"),
    ("infobackground", "canvas"),
    ("menu", "canvas"),
    ("scrollbar", "canvas"),
    ("window", "canvas"),
    ("activeborder", "ButtonBorder"),
    ("inactiveborder", "ButtonBorder"),
    ("threeddarkshadow", "ButtonBorder"),
    ("threedhighlight", "ButtonBorder"),
    ("threedlightshadow", "ButtonBorder"),
    ("threedshadow", "ButtonBorder"),
    ("windowframe", "ButtonBorder"),
    ("captiontext", "CanvasText"),
    ("infotext", "CanvasText"),
    ("menutext", "CanvasText"),
    ("windowtext", "CanvasText"),
    ("buttonhighlight", "ButtonFace"),
    ("buttonshadow", "ButtonFace"),
    ("threedface", "ButtonFace"),
    ("inactivecaptiontext", "GrayText"),
];

/// The intrinsic sizes refused.
const INTRINSIC: &[&str] = &["intrinsic", "min-intrinsic"];

/// A property's deprecated keywords (`PROPERTY_NAME_TO_KEYWORDS`).
fn keywords(property: &str) -> Option<Keywords> {
    Some(match property {
        "appearance" => Keywords::Mapped(&[
            ("button", "auto"),
            ("checkbox", "auto"),
            ("listbox", "auto"),
            ("menulist", "auto"),
            ("meter", "auto"),
            ("progress-bar", "auto"),
            ("push-button", "auto"),
            ("radio", "auto"),
            ("searchfield", "auto"),
            ("slider-horizontal", "auto"),
            ("square-button", "auto"),
            ("textarea", "auto"),
        ]),
        "image-rendering" => Keywords::Mapped(&[
            ("optimizeQuality", "smooth"),
            ("optimizeSpeed", "pixelated"),
        ]),
        "overflow" | "overflow-x" | "overflow-y" => Keywords::Mapped(&[("overlay", "auto")]),
        "text-justify" => Keywords::Mapped(&[("distribute", "inter-character")]),
        "text-orientation" => Keywords::Mapped(&[("sideways-right", "sideways")]),
        "user-select" => Keywords::Mapped(&[("element", "contain")]),
        "zoom" => Keywords::Mapped(&[("reset", "1")]),
        "border-color" | "scrollbar-color" => Keywords::Mapped(DEPRECATED_COLORS),
        "text-decoration" | "text-decoration-line" => Keywords::Refused(&["blink"]),
        "box-sizing" => Keywords::Refused(&["padding-box"]),
        "image-orientation" => Keywords::Refused(&["flip"]),
        "min-height" | "min-width" | "max-height" | "max-width" | "height" | "width" => {
            Keywords::Refused(INTRINSIC)
        }
        "word-break" => Keywords::Refused(&["break-word"]),
        _ if has(SINGLE_VALUE_COLOR_PROPERTIES, property) => Keywords::Mapped(DEPRECATED_COLORS),
        _ => return None,
    })
}

impl SheetRule for DeclarationPropertyValueKeywordNoDeprecated {
    fn name(&self) -> &'static str {
        "stylelint/declaration-property-value-keyword-no-deprecated"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("expected", "Expected \"{}\" to be \"{}\""),
            ("rejected", "Deprecated keyword \"{}\" for property \"{}\""),
        ]
    }

    fn computed(&self, key: &str, args: &[String]) -> Option<String> {
        // The original's arguments are the property, then the keyword, which its message names
        // first.
        let [property, keyword] = args else {
            return None;
        };
        (key == "rejected")
            .then(|| format!("Deprecated keyword \"{keyword}\" for property \"{property}\""))
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (index, node) in sheet.all(Kind::Declaration) {
            if !node.value.bytes().any(|byte| byte.is_ascii_alphabetic())
                || !is_standard_declaration(sheet, index)
                || matches_any(&self.ignore, &node.value)
            {
                continue;
            }
            let Some(keywords) = keywords(&node.prop.to_lowercase()) else {
                continue;
            };
            let parsed = value::parse(sheet.value(node));
            let mut standard = true;
            parsed.walk(&mut |_, part| {
                if part.kind != Part::Comment && !is_standard_value(&part.value) {
                    standard = false;
                }
                true
            });
            if !standard {
                continue;
            }
            let start = sheet.value_start(node);
            let scope = (node.start, node.end.unwrap_or(sheet.text.len()));
            parsed.walk(&mut |_, part| {
                let lower = part.value.to_lowercase();
                if part.kind == Part::Function && !has(COLOR_FUNCTIONS, &lower) {
                    return false;
                }
                if part.kind != Part::Word {
                    return true;
                }
                let at = start + part.start;
                let end = at + part.value.len();
                match &keywords {
                    Keywords::Refused(refused) if refused.contains(&lower.as_str()) => {
                        reports.push(
                            Report::new(at, end, "rejected").with(&[&node.prop, &part.value]),
                        );
                    }
                    Keywords::Mapped(mapped) => {
                        if let Some((_, expected)) = mapped.iter().find(|(key, _)| *key == lower) {
                            reports.push(
                                Report::new(at, end, "expected")
                                    .with(&[&part.value, expected])
                                    .fixed(at, end, *expected)
                                    .scoped(scope.0, scope.1),
                            );
                        }
                    }
                    Keywords::Refused(_) => {}
                }
                true
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DeclarationPropertyValueKeywordNoDeprecated;
    use crate::scss::rules::tester::{matchers, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/declaration-property-value-keyword-no-deprecated.txt",
            &|config| {
                Ok(Box::new(DeclarationPropertyValueKeywordNoDeprecated {
                    ignore: matchers(secondary(config), "ignoreKeywords")?,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (8, 14, 0),
            "{:#?}",
            tally.apart
        );
    }
}
