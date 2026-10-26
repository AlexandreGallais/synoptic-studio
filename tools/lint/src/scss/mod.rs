//! The reader of the style sheets (F53, EN-167): every `.scss` file is read as Stylelint reads it
//! with `postcss-scss`, by a port of `PostCSS`'s parser and `postcss-scss`'s tokenizer and parser
//! (REF-POSTCSS-SCSS-SOURCE, MIT, under ADR-0076; the choice against a parser crate, ADR-0105):
//! `tokenizer` cuts the text, `parser` builds the `tree` of rules, at-rules, declarations and
//! comments, each with its span and its raw text, and `value` reads a value into words,
//! functions, strings and separators as `postcss-value-parser` does. A sheet that cannot be read
//! is reported under `lint/parse` at its first error, as Stylelint reports `PostCSS`'s.

pub mod parser;
pub mod rules;
pub mod selector;
pub mod tokenizer;
pub mod tree;
pub mod value;

use std::fs;

use crate::PARSE_RULE;
use crate::directives::{Written, apply_comments};
use crate::finding::{Finding, unreadable};
use crate::lines::Lines;
use crate::project::Project;

/// Whether a path is a style sheet the reader reads (`*.scss`).
#[must_use]
pub fn is_sheet(path: &str) -> bool {
    std::path::Path::new(path)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("scss"))
}

/// The findings of a sheet's text: its first reading error, under `lint/parse`, else those of
/// the project's rules that run on it, its `lint-disable-next-line` directives applied. A leading
/// byte order mark is dropped before the reading, as `PostCSS`'s `Input` drops it: the places are
/// those of the text after it, as Stylelint gives them, and each fix is moved past it.
#[must_use]
pub fn lint_text(path: &str, text: &str) -> Vec<Finding> {
    let mark = if text.starts_with('\u{feff}') {
        '\u{feff}'.len_utf8()
    } else {
        0
    };
    let mut findings = lint_body(path, &text[mark..]);
    let shift = u32::try_from(mark).unwrap_or(0);
    for fix in findings.iter_mut().filter_map(|each| each.fix.as_mut()) {
        fix.start += shift;
        fix.end += shift;
    }
    findings
}

/// The findings of a sheet's text without its byte order mark.
fn lint_body(path: &str, text: &str) -> Vec<Finding> {
    match parser::parse(text) {
        Ok(sheet) => lint_sheet(path, text, &sheet),
        Err(error) => {
            let (line, column) = Lines::new(text).place(text, error.offset);
            vec![Finding {
                path: path.to_owned(),
                line,
                column,
                rule: PARSE_RULE,
                message: format!("{}.", error.message),
                fix: None,
            }]
        }
    }
}

/// The findings of the project's rules on a sheet read cleanly, in the order of their places.
fn lint_sheet(path: &str, text: &str, sheet: &tree::Sheet) -> Vec<Finding> {
    let context = rules::Context {
        path,
        text,
        lines: Lines::new(text),
        sheet,
    };
    let finding = |rule: &'static str, offset: usize, message: String| {
        let (line, column) = context.lines.place(text, offset);
        Finding {
            path: path.to_owned(),
            line,
            column,
            rule,
            message,
            fix: None,
        }
    };
    let mut findings = Vec::new();
    for rule in rules::project::RULES.iter() {
        if !rules::project::applies(rule.name(), path) {
            continue;
        }
        let mut reports = Vec::new();
        rule.check(&context, &mut reports);
        for report in reports {
            let message = rules::message(rule.as_ref(), report.key, &report.args);
            findings.push(Finding {
                fix: report.fix,
                ..finding(rule.name(), report.start, message)
            });
        }
    }
    let comments: Vec<Written> = sheet
        .nodes
        .iter()
        .filter(|node| node.is(tree::Kind::Comment))
        .map(|node| Written {
            offset: u32::try_from(node.start).unwrap_or(u32::MAX),
            end_line: context.lines.line(
                node.end
                    .unwrap_or(text.len())
                    .saturating_sub(1)
                    .max(node.start),
            ),
            value: &node.text,
        })
        .collect();
    let mut kept = apply_comments(&comments, findings, &|rule, offset, message| {
        finding(rule, offset as usize, message)
    });
    kept.sort_by_key(|each| (each.line, each.column));
    kept
}

/// The findings of a sheet read from the disk.
#[must_use]
pub fn lint_path(project: &Project, path: &str) -> Vec<Finding> {
    match fs::read_to_string(project.root.join(path)) {
        Ok(text) => lint_text(path, &text),
        Err(error) => vec![unreadable(path, &error)],
    }
}

#[cfg(test)]
mod tests {
    use super::{is_sheet, lint_path, lint_text};

    #[test]
    fn f53_ac2_reports_a_sheet_it_cannot_read_at_its_first_error() {
        let findings = lint_text("a.scss", "a {\n  color: red;\n  b\n}\n");

        // `b` then `}`: neither a declaration nor a rule, line 3, column 3.
        assert_eq!(
            findings
                .iter()
                .map(|each| (each.line, each.column, each.rule, each.message.as_str()))
                .collect::<Vec<_>>(),
            [(3, 3, "lint/parse", "Unknown word b.")]
        );
        let clean =
            "@use \"x\";\n\na {\n  display: block;\n\n  &:hover {\n    display: none;\n  }\n}\n";
        assert_eq!(lint_text("a.scss", clean), []);
    }

    /// The rule, line and column of each finding.
    fn places(findings: &[crate::finding::Finding]) -> Vec<(&str, usize, usize)> {
        findings
            .iter()
            .map(|each| (each.rule, each.line, each.column))
            .collect()
    }

    #[test]
    fn f53_ac2_refuses_colours_imports_important_unknowns_duplicates_and_deep_nesting() {
        // `.f` nests four deep, one past the project's limit.
        let deep = ".b {\n  .c {\n    .d {\n      .e {\n        .f {\n          display: none;\n        }\n\n        display: none;\n      }\n\n      display: none;\n    }\n\n    display: none;\n  }\n\n  display: none;\n}\n";
        let text = format!(
            "@import \"a\";\n\na {{\n  color: #fff;\n  colr: red;\n  width: 1pz;\n  color: blue !important;\n}}\n\na {{\n  display: none;\n}}\n\n{deep}\n@inclde x;\n"
        );
        let findings = lint_text("a.scss", &text);

        // Own case: one of each the acceptance criteria name, each where its rule reports it.
        assert_eq!(
            places(&findings),
            [
                ("stylelint/at-rule-disallowed-list", 1, 1),
                // Stylelint reports the first of two `color`s, the one its fix removes.
                ("stylelint/declaration-block-no-duplicate-properties", 4, 3),
                ("stylelint/color-no-hex", 4, 10),
                ("scss/property-no-unknown", 5, 3),
                ("stylelint/color-named", 5, 9),
                // The unit, after its number.
                // `width` (Recess's 74th) after `color` (179th), `colr` unplaced (EN-168).
                ("order/properties-order", 6, 3),
                ("stylelint/unit-no-unknown", 6, 11),
                ("stylelint/color-named", 7, 10),
                ("stylelint/declaration-no-important", 7, 15),
                ("stylelint/no-duplicate-selectors", 10, 1),
                ("stylelint/max-nesting-depth", 18, 9),
                // Each declaration after the rule nested before it (EN-168).
                ("order/order", 22, 9),
                ("order/order", 25, 7),
                ("order/order", 28, 5),
                ("order/order", 31, 3),
                ("scss/at-rule-no-unknown", 34, 1),
            ]
        );
    }

    #[test]
    fn f53_ac2_lets_a_justified_directive_turn_a_rule_off_on_the_next_line() {
        let text = "a {\n  /* lint-disable-next-line stylelint/color-no-hex -- a reason */\n  color: #fff;\n\n  /* lint-disable-next-line stylelint/color-no-hex */\n  background: #000;\n}\n";
        let findings = lint_text("a.scss", text);

        // The first directive silences its line; the second gives no reason, so it is reported
        // and silences its line still, as in the TypeScript code.
        assert_eq!(places(&findings), [("lint/disable-directive", 5, 3)]);
    }

    #[test]
    fn reads_a_sheet_after_its_byte_order_mark_as_postcss_does() {
        // Own cases: PostCSS's `Input` drops the mark, so nothing is reported of it and the
        // places are those of the text after it. `.b { color: #fff; }`: `#fff` is the 13th
        // character after the mark; `@import` at 1:1; a mark alone an empty source.
        assert_eq!(
            lint_text("a.scss", "\u{feff}a {\n  display: block;\n}\n"),
            []
        );
        assert_eq!(
            places(&lint_text("a.scss", "\u{feff}.b { color: #fff; }\n")),
            [("stylelint/color-no-hex", 1, 13)]
        );
        assert_eq!(
            places(&lint_text("a.scss", "\u{feff}@import \"x\";\n")),
            [("stylelint/at-rule-disallowed-list", 1, 1)]
        );
        assert_eq!(
            places(&lint_text("a.scss", "\u{feff}")),
            [("stylelint/no-empty-source", 1, 1)]
        );
        // `margin` at 1:5 after the mark, the unit of its `0px` at 1:14 (`a { margin: 0` is 13
        // characters), fixed three bytes later in the file: the mark is kept.
        let text = "\u{feff}a { margin: 0px; }\n";
        let findings = lint_text("a.scss", text);
        assert_eq!(
            places(&findings),
            [("stylelint/length-zero-no-unit", 1, 14)]
        );
        assert_eq!(
            crate::testing::fixed(text, &findings),
            "\u{feff}a { margin: 0; }\n"
        );
    }

    #[test]
    fn f53_ac2_fixes_a_crlf_sheet_without_corrupting_it() {
        let text = "a {\r\n  color: blue;\r\n}\r\nb {\r\n  margin: 0px 0px;\r\n}\r\n";
        let mut fixed = text.to_owned();
        // As `lint --fix` does, one pass after another until nothing is fixed.
        for _ in 0..crate::FIX_PASSES {
            let findings = lint_text("a.scss", &fixed);
            if findings.iter().all(|each| each.fix.is_none()) {
                break;
            }
            fixed = crate::testing::fixed(&fixed, &findings);
        }

        assert_eq!(
            fixed,
            "a {\r\n  color: blue;\r\n}\r\n\r\nb {\r\n  margin: 0;\r\n}\r\n"
        );
        assert_eq!(
            places(&lint_text("a.scss", &fixed)),
            [("stylelint/color-named", 2, 10)]
        );
    }

    #[test]
    fn f53_ac3_fix_puts_a_crlf_sheet_in_the_recess_order_and_a_second_run_changes_nothing() {
        let text = "a {\r\n  width: 1px; /* why */\r\n  position: relative;\r\n}\r\n";
        let mut fixed = text.to_owned();
        let mut passes = 0;
        // As `lint --fix` does, every rule of the project, one pass after another.
        for _ in 0..crate::FIX_PASSES {
            let findings = lint_text("a.scss", &fixed);
            if findings.iter().all(|each| each.fix.is_none()) {
                break;
            }
            fixed = crate::testing::fixed(&fixed, &findings);
            passes += 1;
        }

        // Own case: `position` (Recess's 3rd) before `width` (74th), the comment on `width`'s
        // line moving with it, each line still ending in CR LF; one pass, then nothing to report.
        assert_eq!(
            fixed,
            "a {\r\n  position: relative;\r\n  width: 1px; /* why */\r\n}\r\n"
        );
        assert_eq!(passes, 1);
        assert_eq!(places(&lint_text("a.scss", &fixed)), []);
    }

    #[test]
    fn f53_ac2_reads_a_deep_sheet_without_overflowing_its_stack() {
        let deep = 100_000;
        let blocks = format!("{}{}", "a {".repeat(deep), "}".repeat(deep));
        let value = format!(
            "a {{ --b: {}$c{}; d: {}1{}; }}",
            "f(".repeat(deep),
            ")".repeat(deep),
            "(".repeat(deep),
            ")".repeat(deep)
        );
        let selector = format!("{}a{} {{}}", ":not(".repeat(deep), ")".repeat(deep));

        assert_eq!(
            places(&lint_text("a.scss", &blocks)),
            [("lint/parse", 1, 769)]
        );
        assert!(lint_text("a.scss", &format!("$c: x;\n{value}")).len() < 10);
        assert!(lint_text("a.scss", &selector).len() < 10);
    }

    #[test]
    fn reads_only_the_scss_files_from_the_disk() {
        let root = crate::testing::fixture("sheets");
        let written = std::fs::write(root.join("a.scss"), "a {\n");
        let project = crate::project::Project::new(&root);

        assert!(written.is_ok());
        assert!(is_sheet("a.component.scss") && !is_sheet("a.css"));
        let found = lint_path(&project, "a.scss");
        assert_eq!(
            found
                .iter()
                .map(|each| (each.line, each.column, each.message.as_str()))
                .collect::<Vec<_>>(),
            [(1, 1, "Unclosed block.")]
        );
        let missing = lint_path(&project, "missing.scss");
        assert!(missing[0].message.starts_with("Cannot read the file"));
    }
}
