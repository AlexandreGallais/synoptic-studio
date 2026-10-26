//! `function-url-quotes` of Stylelint 17.16.0 (`lib/rules/function-url-quotes/index.mjs`,
//! REF-STYLELINT-SOURCE): a `url()` of a plain declaration or an at-rule whose argument is not
//! quoted where its option wants quotes (`"always"`, the project's, stylelint-config-standard's),
//! or quoted where it refuses them, a URL that needs its quotes, interpolations and modifiers left
//! alone, reported over the argument and fixed by adding or removing the quotes. Whether a URL
//! reads unquoted is decided by a reading of ours of CSS's `url` token.

use super::function_url_no_scheme_relative::is_standard_url;
use super::{Context, Report, SheetRule, function_arguments, is_standard_declaration};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part};

/// The rule and its options.
pub struct FunctionUrlQuotes {
    /// Whether quotes are wanted (`"always"`), else refused (`"never"`).
    pub always: bool,
    /// Whether an empty argument takes the other way (`except: ["empty"]`).
    pub except_empty: bool,
}

impl FunctionUrlQuotes {
    /// The project's configuration: `"always"`.
    pub const PROJECT: Self = Self {
        always: true,
        except_empty: false,
    };
}

/// Whether a quoted URL needs its quotes: unquoted, it would not be one `url` token
/// (`requiresQuotes`).
fn requires_quotes(url: &str) -> bool {
    let quote = url.chars().next().unwrap_or_default();
    let content = match url[1..].rfind(quote) {
        Some(end) => &url[1..=end],
        None => url,
    };
    let trimmed = content.trim_matches([' ', '\t', '\n', '\r', '\u{c}']);
    let mut characters = trimmed.chars().peekable();
    while let Some(character) = characters.next() {
        match character {
            '"' | '\'' | '(' | ')' | ' ' | '\t' | '\n' | '\r' | '\u{c}' => return true,
            '\\' => {
                if matches!(characters.next(), None | Some('\n' | '\r' | '\u{c}')) {
                    return true;
                }
            }
            c if (c as u32) < 0x20 || c == '\u{7f}' => return true,
            _ => {}
        }
    }
    false
}

/// Whether a `url()` holds something but a string, a space or a word, or a word with an
/// unescaped space (`hasUrlModifier`).
fn has_modifier(source: &str, start: usize) -> bool {
    let parsed = value::parse(source);
    let mut found = false;
    parsed.walk(&mut |_, part| {
        if part.kind == Part::Function && part.start + part.value.len() + 1 == start {
            found = part.nodes.iter().any(|&each| {
                let argument = parsed.part(each);
                match argument.kind {
                    Part::String | Part::Space => false,
                    Part::Word => argument.value.char_indices().any(|(at, character)| {
                        character.is_whitespace()
                            && (at == 0 || !argument.value[..at].ends_with('\\'))
                    }),
                    _ => true,
                }
            });
            return false;
        }
        true
    });
    found
}

impl FunctionUrlQuotes {
    /// Checks a value or parameters starting at a byte.
    fn check_source(
        &self,
        source: &str,
        offset: usize,
        scope: (usize, usize),
        reports: &mut Vec<Report>,
    ) {
        for (argument, start, name) in
            function_arguments(source, &|name| name.eq_ignore_ascii_case("url"))
        {
            if has_modifier(source, start) {
                continue;
            }
            let left_trimmed = argument.trim_start();
            if !is_standard_url(left_trimmed) {
                continue;
            }
            let empty = matches!(argument.trim(), "" | "''" | "\"\"");
            let expect = self.always != (self.except_empty && empty);
            let quoted = left_trimmed.starts_with(['\'', '"']);
            if (quoted && requires_quotes(left_trimmed)) || expect == quoted {
                continue;
            }
            let from = offset + start + argument.len() - left_trimmed.len();
            let to = offset + start + argument.len();
            let inner = left_trimmed.trim_end();
            let (key, replacement) = if expect {
                ("expected", format!("\"{inner}\""))
            } else {
                (
                    "rejected",
                    inner[1..inner.len().saturating_sub(1).max(1)].to_owned(),
                )
            };
            reports.push(
                Report::new(from, to, key)
                    .with(&[&name.to_lowercase()])
                    .fixed(from, from + inner.len(), replacement)
                    .scoped(scope.0, scope.1),
            );
        }
    }
}

impl SheetRule for FunctionUrlQuotes {
    fn name(&self) -> &'static str {
        "stylelint/function-url-quotes"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            (
                "expected",
                "Expected quotes around \"{}\" function argument",
            ),
            (
                "rejected",
                "Expected no quotes around \"{}\" function argument",
            ),
        ]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let mut at_rules = Vec::new();
        for (_, node) in sheet.all(Kind::AtRule) {
            let scope = (node.start, node.end.unwrap_or(sheet.text.len()));
            self.check_source(
                sheet.params(node),
                sheet.params_start(node),
                scope,
                &mut at_rules,
            );
        }
        let mut declarations = Vec::new();
        for (index, node) in sheet.all(Kind::Declaration) {
            if !node.value.to_lowercase().contains("url(") || !is_standard_declaration(sheet, index)
            {
                continue;
            }
            let scope = (node.start, node.end.unwrap_or(sheet.text.len()));
            self.check_source(
                sheet.value(node),
                sheet.value_start(node),
                scope,
                &mut declarations,
            );
        }
        reports.extend(at_rules);
        reports.extend(declarations);
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionUrlQuotes, requires_quotes};
    use crate::scss::rules::tester::{Js, primary, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint() {
        let tally = run(
            "stylelint/function-url-quotes.txt",
            &|config| {
                let except = secondary(config)
                    .and_then(|each| each.texts("except"))
                    .unwrap_or_default();
                Ok(Box::new(FunctionUrlQuotes {
                    always: primary(config) == &Js::Text("always".to_owned()),
                    except_empty: except.iter().any(|each| each == "empty"),
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (101, 44, 4),
            "{:#?}",
            tally.apart
        );
        assert!(
            requires_quotes("'a b'") && !requires_quotes("'a.png'") && requires_quotes("\"a\\")
        );
    }
}
