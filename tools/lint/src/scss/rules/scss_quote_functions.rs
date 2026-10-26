//! `function-quote-no-quoted-strings-inside` and `function-unquote-no-unquoted-strings-inside` of
//! stylelint-scss 7.3.0 (`src/rules/function-quote-no-quoted-strings-inside/index.js`,
//! `src/rules/function-unquote-no-unquoted-strings-inside/index.js`, REF-STYLELINT-SOURCE):
//! `quote()` of a string already quoted (or of a variable holding one), `unquote()` of a word
//! already unquoted (or of a variable holding one), reported over the function's name and fixed
//! by unwrapping its argument.

use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;
use crate::scss::value::{self, Kind as Part};

/// Which of the two rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quoting {
    /// `function-quote-no-quoted-strings-inside`.
    Quote,
    /// `function-unquote-no-unquoted-strings-inside`.
    Unquote,
}

/// The rule.
pub struct QuoteFunctions {
    /// Which.
    pub quoting: Quoting,
}

/// A value with its first `name(…)` unwrapped to the last `)` of its line
/// (`replace(/name\((.*)\)/, "$1")`).
fn unwrap(value: &str, name: &str) -> String {
    let opener = format!("{name}(");
    let Some(open) = value.find(&opener) else {
        return value.to_owned();
    };
    let start = open + opener.len();
    let line_end = value[start..]
        .find(['\n', '\r'])
        .map_or(value.len(), |at| start + at);
    match value[start..line_end].rfind(')') {
        Some(close) => format!(
            "{}{}{}",
            &value[..open],
            &value[start..start + close],
            &value[start + close + 1..]
        ),
        None => value.to_owned(),
    }
}

impl SheetRule for QuoteFunctions {
    fn name(&self) -> &'static str {
        match self.quoting {
            Quoting::Quote => "scss/function-quote-no-quoted-strings-inside",
            Quoting::Unquote => "scss/function-unquote-no-unquoted-strings-inside",
        }
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        match self.quoting {
            Quoting::Quote => &[(
                "rejected",
                "Quote function used with an already-quoted string",
            )],
            Quoting::Unquote => &[(
                "rejected",
                "Unquote function used with an already-unquoted string",
            )],
        }
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        // Each variable with the kind of its value's last part.
        let mut variables: Vec<(String, Part)> = Vec::new();
        for (_, node) in sheet.all(Kind::Declaration) {
            if !node.prop.starts_with('$') {
                continue;
            }
            let mut last = None;
            value::parse(&node.value).walk(&mut |_, part| {
                last = Some(part.kind);
                true
            });
            if let Some(kind) = last {
                variables.retain(|(name, _)| *name != node.prop);
                variables.push((node.prop.clone(), kind));
            }
        }
        let kind_of = |name: &str| {
            variables
                .iter()
                .find(|(each, _)| each == name)
                .map(|(_, kind)| *kind)
        };
        let wanted = match self.quoting {
            Quoting::Quote => "quote",
            Quoting::Unquote => "unquote",
        };
        for (_, node) in sheet.all(Kind::Declaration) {
            let parsed = value::parse(&node.value);
            let start = sheet.value_start(node);
            parsed.walk(&mut |_, part| {
                if part.kind != Part::Function || part.value != wanted {
                    return true;
                }
                let Some(&first) = part.nodes.first() else {
                    return true;
                };
                let argument = parsed.part(first);
                let redundant = match self.quoting {
                    Quoting::Quote => {
                        argument.quote.is_some() || kind_of(&argument.value) == Some(Part::String)
                    }
                    Quoting::Unquote => {
                        (argument.quote.is_none() && !argument.value.starts_with('$'))
                            || kind_of(&argument.value) == Some(Part::Word)
                    }
                };
                if redundant {
                    let at = start + part.start;
                    reports.push(Report::new(at, at + part.value.len(), "rejected").fixed(
                        start,
                        start + node.value.len(),
                        unwrap(&node.value, wanted),
                    ));
                }
                true
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{QuoteFunctions, Quoting, unwrap};
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss_quote() {
        let tally = run(
            "stylelint-scss/function-quote-no-quoted-strings-inside.txt",
            &|_| {
                Ok(Box::new(QuoteFunctions {
                    quoting: Quoting::Quote,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (2, 3, 0),
            "{:#?}",
            tally.apart
        );
        assert_eq!(unwrap("a quote(\"b\") c", "quote"), "a \"b\" c");
    }

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss_unquote() {
        let tally = run(
            "stylelint-scss/function-unquote-no-unquoted-strings-inside.txt",
            &|_| {
                Ok(Box::new(QuoteFunctions {
                    quoting: Quoting::Unquote,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (2, 3, 0),
            "{:#?}",
            tally.apart
        );
    }
}
