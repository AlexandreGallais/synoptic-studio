//! `function-url-no-scheme-relative` of Stylelint 17.16.0
//! (`lib/rules/function-url-no-scheme-relative/index.mjs`, REF-STYLELINT-SOURCE): a `url()` of a
//! declaration or an `@import` whose address starts `//`, reported over its argument.

use super::{Context, Report, SheetRule, function_arguments, has_interpolation};
use crate::scss::tree::Kind;

/// The rule.
pub struct FunctionUrlNoSchemeRelative;

/// Whether an address is plain CSS: no interpolation, no Less or Sass variable
/// (`isStandardSyntaxUrl`).
#[must_use]
pub fn is_standard_url(url: &str) -> bool {
    if url.is_empty() {
        return true;
    }
    if has_interpolation(url) || url.contains("${") {
        return false;
    }
    let quoted = (url.starts_with('\'') && url.ends_with('\''))
        || (url.starts_with('"') && url.ends_with('"'));
    if quoted {
        return !(url.contains("@{") && url.contains('}'));
    }
    let less = url.starts_with('@')
        && url.trim_start_matches('@').len() + 2 >= url.len()
        && url
            .trim_start_matches('@')
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
        && !url.trim_start_matches('@').is_empty();
    if less {
        return false;
    }
    let sass = url.contains('$')
        && url.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || b"$ \t\n\r\x0c_+-,./*'\"@#?".contains(&byte)
        })
        && !url.ends_with('/');
    !sass
}

impl SheetRule for FunctionUrlNoSchemeRelative {
    fn name(&self) -> &'static str {
        "stylelint/function-url-no-scheme-relative"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Disallowed scheme-relative url")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for index in sheet.sheet.walk() {
            let node = sheet.node(index);
            let applies = node.is(Kind::Declaration)
                || (node.is(Kind::AtRule) && node.name.eq_ignore_ascii_case("import"));
            if !applies {
                continue;
            }
            let source = sheet.source(node).to_lowercase();
            for (argument, at, _) in function_arguments(&source, &|name| name == "url") {
                let url = argument.trim().trim_matches(|c| c == '\'' || c == '"');
                if is_standard_url(url) && url.starts_with("//") {
                    let from = node.start + at;
                    reports.push(Report::new(from, from + argument.len(), "rejected"));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionUrlNoSchemeRelative, is_standard_url};
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/function-url-no-scheme-relative.txt",
            &|_| Ok(Box::new(FunctionUrlNoSchemeRelative)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (24, 9, 4),
            "{:#?}",
            tally.apart
        );
        assert!(
            is_standard_url("")
                && is_standard_url("'@{a}")
                && !is_standard_url("@a")
                && !is_standard_url("$a/b.png")
        );
    }
}
