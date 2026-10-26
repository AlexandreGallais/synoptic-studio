//! `no-global-function-names` of stylelint-scss 7.3.0 (`src/rules/no-global-function-names/index.js`,
//! REF-STYLELINT-SOURCE): a call of a global built-in function Sass deprecates for its modules'
//! (`map-get()` for `map.get()`), in a declaration's value or an at-rule's parameters, reported
//! over its name with the module's function to call instead.

use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part};

/// The rule.
pub struct NoGlobalFunctionNames;

/// Each global function with its module.
const MODULES: &[(&str, &str)] = &[
    ("red", "color"),
    ("blue", "color"),
    ("green", "color"),
    ("mix", "color"),
    ("hue", "color"),
    ("saturation", "color"),
    ("lightness", "color"),
    ("complement", "color"),
    ("adjust-color", "color"),
    ("scale-color", "color"),
    ("change-color", "color"),
    ("ie-hex-str", "color"),
    ("map-get", "map"),
    ("map-merge", "map"),
    ("map-remove", "map"),
    ("map-keys", "map"),
    ("map-values", "map"),
    ("map-has-key", "map"),
    ("unquote", "string"),
    ("quote", "string"),
    ("str-length", "string"),
    ("str-insert", "string"),
    ("str-index", "string"),
    ("str-slice", "string"),
    ("to-upper-case", "string"),
    ("to-lower-case", "string"),
    ("unique-id", "string"),
    ("percentage", "math"),
    ("ceil", "math"),
    ("floor", "math"),
    ("abs", "math"),
    ("random", "math"),
    ("unit", "math"),
    ("unitless", "math"),
    ("comparable", "math"),
    ("length", "list"),
    ("nth", "list"),
    ("set-nth", "list"),
    ("join", "list"),
    ("append", "list"),
    ("zip", "list"),
    ("index", "list"),
    ("list-separator", "list"),
    ("feature-exists", "meta"),
    ("variable-exists", "meta"),
    ("global-variable-exists", "meta"),
    ("function-exists", "meta"),
    ("mixin-exists", "meta"),
    ("inspect", "meta"),
    ("get-function", "meta"),
    ("type-of", "meta"),
    ("call", "meta"),
    ("content-exists", "meta"),
    ("keywords", "meta"),
    ("selector-nest", "selector"),
    ("selector-append", "selector"),
    ("selector-replace", "selector"),
    ("selector-unify", "selector"),
    ("is-superselector", "selector"),
    ("simple-selectors", "selector"),
    ("selector-parse", "selector"),
    ("selector-extend", "selector"),
    ("lighten", "color"),
    ("adjust-hue", "color"),
    ("darken", "color"),
    ("desaturate", "color"),
    ("opacify", "color"),
    ("transparentize", "color"),
];

/// The functions renamed in their module.
const RENAMED: &[(&str, &str)] = &[
    ("adjust-color", "adjust"),
    ("scale-color", "scale"),
    ("change-color", "change"),
    ("map-get", "get"),
    ("map-merge", "merge"),
    ("map-remove", "remove"),
    ("map-keys", "keys"),
    ("map-values", "values"),
    ("map-has-key", "has-key"),
    ("str-length", "length"),
    ("str-insert", "insert"),
    ("str-index", "index"),
    ("str-slice", "slice"),
    ("unitless", "is-unitless"),
    ("comparable", "compatible"),
    ("list-separator", "separator"),
    ("selector-nest", "nest"),
    ("selector-append", "append"),
    ("selector-replace", "replace"),
    ("selector-unify", "unify"),
    ("selector-parse", "parse"),
    ("selector-extend", "extend"),
    ("lighten", "adjust"),
    ("adjust-hue", "adjust"),
    ("darken", "adjust"),
    ("desaturate", "adjust"),
    ("opacify", "adjust"),
    ("saturate", "adjust"),
    ("transparentize", "adjust"),
];

/// The functions whose call changes shape, the old call and the new.
const REWRITTEN: &[(&str, &str, &str)] = &[
    (
        "lighten",
        "lighten($color, $amount)",
        "adjust($color, $lightness: $amount)",
    ),
    (
        "adjust-hue",
        "adjust-hue($color, $amount)",
        "adjust($color, $hue: $amount)",
    ),
    (
        "darken",
        "darken($color, $amount)",
        "adjust($color, $lightness: -$amount)",
    ),
    (
        "desaturate",
        "desaturate($color, $amount)",
        "adjust($color, $saturation: -$amount)",
    ),
    (
        "opacify",
        "opacify($color, $amount)",
        "adjust($color, $alpha: -$amount)",
    ),
    (
        "saturate",
        "saturate($color, $amount)",
        "adjust($color, $saturation: $amount)",
    ),
    (
        "transparentize",
        "transparentize($color, $amount)",
        "adjust($color, $alpha: -$amount)",
    ),
];

/// The message of a function (`errorMessage`).
fn advice(name: &str) -> String {
    let module = MODULES
        .iter()
        .find(|(each, _)| *each == name)
        .map_or("undefined", |(_, module)| module);
    match RENAMED.iter().find(|(each, _)| *each == name) {
        Some((_, renamed)) => match REWRITTEN.iter().find(|(each, _, _)| *each == name) {
            Some((_, old, new)) => format!("Expected {module}.{new} instead of {old}"),
            None => format!("Expected {module}.{renamed} instead of {name}"),
        },
        None => format!("Expected {module}.{name} instead of {name}"),
    }
}

impl NoGlobalFunctionNames {
    /// Reports the global functions of a text of a node.
    fn check_text(sheet: &Context, index: usize, text: &str, reports: &mut Vec<Report>) {
        value::parse(text).walk(&mut |_, part| {
            let name = part
                .value
                .strip_prefix("#{")
                .map_or(part.value.as_str(), str::trim_start);
            if part.kind == Part::Function
                && !name.is_empty()
                && MODULES.iter().any(|(each, _)| *each == name)
            {
                let (start, end) = sheet.word_range(sheet.node(index), name);
                reports.push(Report::new(start, end, "rejected").with(&[name]));
            }
            true
        });
    }
}

impl SheetRule for NoGlobalFunctionNames {
    fn name(&self) -> &'static str {
        "scss/no-global-function-names"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejectedFullMessage", "{}")]
    }

    fn computed(&self, key: &str, args: &[String]) -> Option<String> {
        (key == "rejected").then(|| advice(args.first().map_or("", String::as_str)))
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (index, node) in sheet.all(Kind::Declaration) {
            Self::check_text(sheet, index, &node.value, reports);
        }
        for (index, node) in sheet.all(Kind::AtRule) {
            Self::check_text(sheet, index, sheet.params(node), reports);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NoGlobalFunctionNames, advice};
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/no-global-function-names.txt",
            &|_| Ok(Box::new(NoGlobalFunctionNames)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (38, 20, 0),
            "{:#?}",
            tally.apart
        );
        // Own cases: the three forms of the message.
        assert_eq!(advice("red"), "Expected color.red instead of red");
        assert_eq!(advice("map-get"), "Expected map.get instead of map-get");
        assert_eq!(
            advice("darken"),
            "Expected color.adjust($color, $lightness: -$amount) instead of darken($color, $amount)"
        );
    }
}
