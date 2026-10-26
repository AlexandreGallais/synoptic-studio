//! `color-no-invalid-hex` of Stylelint 17.16.0 (`lib/rules/color-no-invalid-hex/index.mjs`,
//! REF-STYLELINT-SOURCE): a word of a declaration's value starting `#` and letters or digits, not
//! 3, 4, 6 or 8 hexadecimal digits, outside `url()`, reported over its hex part.

use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part};

/// The rule.
pub struct ColorNoInvalidHex;

/// The `#` and letters or digits a word starts with (`/^#[\da-z]+/i`).
fn hex_start(word: &str) -> Option<&str> {
    let digits = word
        .strip_prefix('#')?
        .bytes()
        .take_while(u8::is_ascii_alphanumeric)
        .count();
    (digits > 0).then(|| &word[..=digits])
}

/// Whether a hex colour has 3, 4, 6 or 8 hexadecimal digits (`isValidHex`).
fn is_valid(hex: &str) -> bool {
    let digits = &hex[1..];
    matches!(digits.len(), 3 | 4 | 6 | 8) && digits.bytes().all(|byte| byte.is_ascii_hexdigit())
}

impl SheetRule for ColorNoInvalidHex {
    fn name(&self) -> &'static str {
        "stylelint/color-no-invalid-hex"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Invalid hex color \"{}\"")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Declaration) {
            if !node.value.contains('#') || node.value.contains('[') {
                continue;
            }
            let start = sheet.value_start(node);
            value::parse(&node.value).walk(&mut |_, part| {
                if part.kind == Part::Function && part.value.ends_with("url") {
                    return false;
                }
                if part.kind == Part::Word
                    && let Some(hex) = hex_start(&part.value)
                    && !is_valid(hex)
                {
                    let at = start + part.start;
                    reports.push(Report::new(at, at + hex.len(), "rejected").with(&[hex]));
                }
                true
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ColorNoInvalidHex;
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/color-no-invalid-hex.txt",
            &|_| Ok(Box::new(ColorNoInvalidHex)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (13, 4, 3),
            "{:#?}",
            tally.apart
        );
    }
}
