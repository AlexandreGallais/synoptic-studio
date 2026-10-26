//! `dollar-variable-no-missing-interpolation` of stylelint-scss 7.3.0
//! (`src/rules/dollar-variable-no-missing-interpolation/index.js` and
//! `src/utils/isNativeCssFunction.js`, REF-STYLELINT-SOURCE): a `$variable` the sheet declares,
//! written without `#{}` where Sass reads text, not values: in a custom property's value, in
//! `@keyframes`, `@counter-style` or `@supports`, or (a variable holding a string) in a property
//! naming an identifier (`animation-name`); reported over the first such variable of the node and
//! fixed by interpolating each.

use super::{Context, Report, SheetRule};
use crate::scss::tree::{Kind, Node};
use crate::scss::value::{self, Kind as Part};

/// The rule.
pub struct DollarVariableNoMissingInterpolation;

/// The properties naming an identifier.
const IDENT_PROPERTIES: &[&str] = &[
    "animation",
    "animation-name",
    "counter-reset",
    "counter-increment",
    "list-style-type",
    "will-change",
];

/// The at-rules naming an identifier.
const IDENT_AT_RULES: &[&str] = &["counter-style", "keyframes", "supports"];

/// CSS's own functions (`isNativeCssFunction`).
const NATIVE_FUNCTIONS: &[&str] = &[
    "abs",
    "acos",
    "anchor",
    "anchor-size",
    "annotation",
    "asin",
    "atan",
    "atan2",
    "attr",
    "blur",
    "brightness",
    "calc",
    "character-variant",
    "calc-size",
    "circle",
    "clamp",
    "color",
    "color-contrast",
    "color-mix",
    "conic-gradient",
    "contrast",
    "cos",
    "counter",
    "counters",
    "cross-fade",
    "cubic-bezier",
    "device-cmyk",
    "drop-shadow",
    "element",
    "ellipse",
    "env",
    "exp",
    "fit-content",
    "format",
    "frames",
    "grayscale",
    "hsl",
    "hsla",
    "hue-rotate",
    "hwb",
    "hypot",
    "image",
    "image-set",
    "inset",
    "invert",
    "lab",
    "layer",
    "lch",
    "leader",
    "light-dark",
    "linear",
    "linear-gradient",
    "local",
    "log",
    "matrix",
    "matrix3d",
    "max",
    "min",
    "minmax",
    "mod",
    "oklab",
    "oklch",
    "opacity",
    "ornaments",
    "paint",
    "palette-mix",
    "path",
    "perspective",
    "polygon",
    "pow",
    "radial-gradient",
    "ray",
    "rect",
    "rem",
    "repeat",
    "repeating-conic-gradient",
    "repeating-linear-gradient",
    "repeating-radial-gradient",
    "rgb",
    "rgba",
    "rotate",
    "rotate3d",
    "rotateX",
    "rotatex",
    "rotateY",
    "rotatey",
    "rotateZ",
    "rotatez",
    "round",
    "saturate",
    "scale",
    "scale3d",
    "scaleX",
    "scalex",
    "scaleY",
    "scaley",
    "scaleZ",
    "scalez",
    "scroll",
    "sepia",
    "shape",
    "sign",
    "sin",
    "skew",
    "skewX",
    "skewx",
    "skewY",
    "skewy",
    "sqrt",
    "steps",
    "styleset",
    "stylistic",
    "swash",
    "symbols",
    "tan",
    "target-counter",
    "target-counters",
    "target-text",
    "translate",
    "translate3d",
    "translateX",
    "translatex",
    "translateY",
    "translatey",
    "translateZ",
    "translatez",
    "url",
    "var",
    "view",
    "xywh",
];

/// Whether a function is CSS's own.
#[must_use]
pub fn is_native(name: &str) -> bool {
    NATIVE_FUNCTIONS.contains(&name)
}

/// Whether a word is a variable, namespaced or not (`/^(?:[\w-]+\.)?\$[\w-]+$/`).
fn is_reference(word: &str) -> bool {
    let is_name = |text: &str| {
        !text.is_empty()
            && text
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
    };
    let local = match word.split_once('.') {
        Some((module, rest)) if is_name(module) => rest,
        _ => word,
    };
    local.strip_prefix('$').is_some_and(is_name)
}

/// Whether a byte of a text sits inside `#{…}` (`isInsideInterpolationBlock`).
fn is_interpolated(text: &str, at: usize) -> bool {
    let bytes = text.as_bytes();
    let (mut depth, mut inside, mut index) = (0_i32, false, 0);
    while index < at.min(bytes.len()) {
        if bytes[index] == b'#' && bytes.get(index + 1) == Some(&b'{') {
            depth += 1;
            inside = true;
            index += 1;
        } else if bytes[index] == b'}' {
            depth -= 1;
            if depth == 0 {
                inside = false;
            }
        }
        index += 1;
    }
    inside && depth > 0
}

/// The ranges to interpolate: each variable, or with `functions` the Sass function holding it
/// (`getInterpolationRanges`).
fn ranges(text: &str, functions: bool) -> Vec<(usize, usize)> {
    let parsed = value::parse(text);
    let mut found = Vec::new();
    // The functions around the part read, and the parts to read with how many of them hold
    // each: a walk in the original's order, without recursion, for values nested deep.
    let mut around: Vec<usize> = Vec::new();
    let mut pending: Vec<(usize, usize)> =
        parsed.nodes.iter().rev().map(|&each| (each, 0)).collect();
    while let Some((each, depth)) = pending.pop() {
        around.truncate(depth);
        let part = parsed.part(each);
        if part.kind == Part::Word
            && is_reference(&part.value)
            && !is_interpolated(text, part.start)
        {
            let boundary = around
                .iter()
                .rposition(|at| is_native(&parsed.part(*at).value))
                .map_or(0, |at| at + 1);
            let sass = around[boundary..]
                .iter()
                .find(|at| !is_native(&parsed.part(**at).value))
                .filter(|_| functions);
            found.push(sass.map_or((part.start, part.end), |at| {
                (parsed.part(*at).start, parsed.part(*at).end)
            }));
        }
        if !part.nodes.is_empty() {
            let inner = if part.kind == Part::Function {
                around.push(each);
                depth + 1
            } else {
                depth
            };
            pending.extend(part.nodes.iter().rev().map(|&child| (child, inner)));
        }
    }
    found
}

/// A text with its variables interpolated (`wrapVariablesWithInterpolation`).
fn interpolate(text: &str, functions: bool) -> String {
    let mut fixed = text.to_owned();
    for (start, end) in ranges(text, functions).into_iter().rev() {
        fixed = format!(
            "{}#{{{}}}{}",
            &fixed[..start],
            &fixed[start..end],
            &fixed[end..]
        );
    }
    fixed
}

/// The variables a sheet declares, and those holding a quoted string (`collectVariables`).
fn variables(sheet: &Context) -> (Vec<String>, Vec<String>) {
    let (mut all, mut strings) = (Vec::new(), Vec::new());
    for (_, node) in sheet.all(Kind::Declaration) {
        let (prop, value) = (&node.prop, &node.value);
        let dollar = (value.len() > 1 && value.starts_with('$')) || value.contains(".$");
        if !all.contains(value) && prop.starts_with("--") && dollar {
            all.push(value.clone());
            continue;
        }
        if !prop.starts_with('$') || all.contains(prop) {
            continue;
        }
        let quoted = value.len() >= 2
            && value.starts_with(['"', '\''])
            && value.ends_with(['"', '\''])
            && !value.contains(['\n', '\r']);
        if quoted {
            strings.push(prop.clone());
        }
        all.push(prop.clone());
    }
    (all, strings)
}

impl DollarVariableNoMissingInterpolation {
    /// Reports a node's first variable missing its interpolation (`checkValueForVariables`).
    fn check_node(
        sheet: &Context,
        node: &Node,
        known: &(Vec<String>, Vec<String>),
        reports: &mut Vec<Report>,
    ) {
        let (all, strings) = known;
        let at_rule = node.is(Kind::AtRule);
        let text = if at_rule { &node.params } else { &node.value };
        let custom = node.prop.starts_with("--");
        let mut first = None;
        value::parse(text).walk(&mut |_, part| {
            if part.kind != Part::Word
                || part.value.is_empty()
                || part.value.starts_with("#{")
                || first.is_some()
            {
                return true;
            }
            let reported = if (at_rule && node.name == "supports")
                || (!at_rule && IDENT_PROPERTIES.contains(&node.prop.as_str()))
            {
                strings.contains(&part.value)
            } else {
                (at_rule && IDENT_AT_RULES.contains(&node.name.as_str())) || (!at_rule && custom)
            } && (strings.contains(&part.value) || all.contains(&part.value));
            if reported && !is_interpolated(text, part.start) {
                first = Some(part.value.clone());
            }
            true
        });
        let Some(variable) = first else {
            return;
        };
        let shown = if at_rule {
            format!("@{}", node.name)
        } else {
            node.prop.clone()
        };
        let (start, end) = sheet.word_range(node, &variable);
        let (from, length) = if at_rule {
            (sheet.params_start(node), sheet.params(node).len())
        } else {
            (sheet.value_start(node), sheet.value(node).len())
        };
        reports.push(
            Report::new(start, end, "rejected")
                .with(&[&shown, &variable])
                .fixed(from, from + length, interpolate(text, !at_rule && custom)),
        );
    }
}

impl SheetRule for DollarVariableNoMissingInterpolation {
    fn name(&self) -> &'static str {
        "scss/dollar-variable-no-missing-interpolation"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "rejected",
            "Expected variable {} to be interpolated when using it with {}",
        )]
    }

    fn computed(&self, key: &str, args: &[String]) -> Option<String> {
        // The original's message names the variable second: its arguments are the node's name,
        // then the variable.
        let [name, variable] = args else {
            return None;
        };
        (key == "rejected").then(|| {
            format!("Expected variable {variable} to be interpolated when using it with {name}")
        })
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let known = variables(sheet);
        if known.0.is_empty() {
            return;
        }
        let mut reported: Vec<usize> = Vec::new();
        let mut each = |index: usize, reports: &mut Vec<Report>| {
            if !reported.contains(&index) {
                let before = reports.len();
                Self::check_node(sheet, sheet.node(index), &known, reports);
                if reports.len() > before {
                    reported.push(index);
                }
            }
        };
        for (index, node) in sheet.all(Kind::Declaration) {
            if IDENT_PROPERTIES.iter().any(|name| node.prop.contains(name)) {
                each(index, reports);
            }
        }
        for (index, node) in sheet.all(Kind::AtRule) {
            if IDENT_AT_RULES.iter().any(|name| node.name.contains(name)) {
                each(index, reports);
            }
        }
        for (index, node) in sheet.all(Kind::Declaration) {
            if node.prop.starts_with("--") {
                each(index, reports);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DollarVariableNoMissingInterpolation, interpolate, is_reference};
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/dollar-variable-no-missing-interpolation.txt",
            &|_| Ok(Box::new(DollarVariableNoMissingInterpolation)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (25, 34, 0),
            "{:#?}",
            tally.apart
        );
        assert!(
            is_reference("m.$a")
                && is_reference("$a-b")
                && !is_reference("$")
                && !is_reference("a")
        );
        assert_eq!(interpolate("f($a) $b", true), "#{f($a)} #{$b}");
    }
}
