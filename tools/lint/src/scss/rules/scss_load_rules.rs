//! `load-no-partial-leading-underscore` and `load-partial-extension` of stylelint-scss 7.3.0
//! (`src/rules/load-no-partial-leading-underscore/index.js`,
//! `src/rules/load-partial-extension/index.js`, REF-STYLELINT-SOURCE): the paths of `@use`,
//! `@forward`, `@import` and `meta.load-css()`: a partial named with its leading underscore,
//! reported over the path; an extension where the option refuses it (`"never"`, the project's,
//! stylelint-config-recommended-scss's) or none where it wants one, reported and, for `.scss`,
//! fixed by dropping it. CSS files, URLs and `url()` left alone.

use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;

/// Which of the two rules, with its option.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Load {
    /// `load-no-partial-leading-underscore`.
    NoLeadingUnderscore,
    /// `load-partial-extension`, whether an extension is wanted.
    Extension {
        /// `"always"`, else `"never"`.
        always: bool,
    },
}

/// The rule.
pub struct LoadRules {
    /// Which.
    pub load: Load,
}

impl LoadRules {
    /// The project's configurations.
    pub const NO_LEADING_UNDERSCORE: Self = Self {
        load: Load::NoLeadingUnderscore,
    };
    /// `load-partial-extension`'s, `"never"`.
    pub const EXTENSION: Self = Self {
        load: Load::Extension { always: false },
    };
}

/// Whether a path ends with `.css`, in lower case as the original compares it.
fn is_css(path: &str) -> bool {
    path.as_bytes().ends_with(b".css")
}

/// A path without its quotes and the spaces around them.
fn stripped(path: &str) -> &str {
    let trimmed = path.trim_start();
    let trimmed = trimmed
        .strip_prefix(['"', '\''])
        .map_or(trimmed, str::trim_start);
    let trimmed = trimmed.trim_end();
    trimmed
        .strip_suffix(['"', '\''])
        .map_or(trimmed, str::trim_end)
}

/// The arguments of `load-css(…)`: its first, parentheses dropped.
fn load_css_path(params: &str) -> Option<String> {
    let open = params.find('(')?;
    let close = params[open + 1..].find(')').map(|at| open + 1 + at)?;
    if close == open + 1 {
        return None;
    }
    let inside = &params[open..=close];
    Some(
        inside
            .split(',')
            .next()
            .unwrap_or_default()
            .replace(['(', ')'], ""),
    )
}

/// The paths of parameters, split at each quote followed by a comma (`split(/["']\s*,/)`).
fn split_paths(params: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut from = 0;
    for (at, character) in params.char_indices() {
        if at < from || !matches!(character, '"' | '\'') {
            continue;
        }
        let after = params[at + 1..].trim_start();
        if after.starts_with(',') {
            parts.push(params[from..at].to_owned());
            from = params.len() - after.len() + 1;
        }
    }
    parts.push(params[from..].to_owned());
    parts
}

/// A path's extension as Node.js's `path.extname` gives it, its dot dropped.
fn extension(path: &str) -> &str {
    let base = path.rsplit('/').next().unwrap_or(path);
    match base.rfind('.') {
        Some(at) if at > 0 && base[..at].chars().any(|c| c != '.') => &base[at + 1..],
        _ => "",
    }
}

/// Whether a path ends with a media type, as `@import`'s do (`(?:all|print|screen…)$`).
fn ends_with_media(path: &str) -> bool {
    let lower = path.trim().to_lowercase();
    [
        "all",
        "print",
        "screen",
        "speech",
        "tv",
        "tty",
        "projection",
        "handheld",
        "braille",
        "embossed",
        "aural",
    ]
    .iter()
    .any(|media| lower.ends_with(media))
}

impl LoadRules {
    /// The leading-underscore check of a path.
    fn underscore(sheet: &Context, index: usize, path: &str, reports: &mut Vec<Report>) {
        let path = stripped(path);
        let base_start = path.rfind(['/', '\\']).map_or(0, |at| at + 1);
        if !path[base_start..].starts_with('_')
            || path[base_start + 1..].is_empty()
            || path[base_start..].contains('/')
        {
            return;
        }
        let bytes = path.as_bytes();
        let trailing_word = path
            .rfind(|c: char| c.is_whitespace() || matches!(c, ',' | ')' | '"' | '\''))
            .is_some_and(|at| {
                at + 1 < bytes.len()
                    && path[at + 1..]
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
            });
        if path.starts_with("url(") || is_css(path) || path.contains("//") || trailing_word {
            return;
        }
        let node = sheet.node(index);
        let (start, end) = sheet.word_range(node, path);
        reports.push(Report::new(start, end, "expected"));
    }
}

impl SheetRule for LoadRules {
    fn name(&self) -> &'static str {
        match self.load {
            Load::NoLeadingUnderscore => "scss/load-no-partial-leading-underscore",
            Load::Extension { .. } => "scss/load-partial-extension",
        }
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        match self.load {
            Load::NoLeadingUnderscore => &[(
                "expected",
                "Unexpected leading underscore in imported partial name",
            )],
            Load::Extension { .. } => &[
                ("expected", "Expected @{=import} to have an extension"),
                ("rejected", "Unexpected extension \".{}\" in @{=import}"),
            ],
        }
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (index, node) in sheet.all(Kind::AtRule) {
            let name = node.name.as_str();
            let params = &node.params;
            let Load::Extension { always } = self.load else {
                if matches!(name, "import" | "use" | "forward") {
                    for path in params.split(',') {
                        Self::underscore(sheet, index, path, reports);
                    }
                }
                if name == "include"
                    && params.contains("load-css")
                    && let Some(path) = load_css_path(params)
                {
                    Self::underscore(sheet, index, &path, reports);
                }
                continue;
            };
            if !matches!(name, "import" | "use" | "forward" | "include") {
                continue;
            }
            let load_css = params.contains("load-css");
            let (shown, paths): (&str, Vec<String>) = if load_css {
                ("meta.load-css", load_css_path(params).into_iter().collect())
            } else if name == "include" {
                continue;
            } else {
                (
                    name,
                    split_paths(params)
                        .into_iter()
                        .filter(|path| !ends_with_media(path))
                        .collect(),
                )
            };
            for path in paths {
                let clean = stripped(&path);
                if clean.starts_with("url(")
                    || is_css(clean)
                    || clean.contains("//")
                    || clean.contains(':')
                {
                    continue;
                }
                let found = extension(clean);
                if found.is_empty() && always {
                    let (start, end) = sheet.word_range(node, clean);
                    reports.push(Report::new(start, end, "expected").with(&[shown]));
                    continue;
                }
                if found == "scss" && !always {
                    let dot = format!(".{found}");
                    let Some(at) = params.rfind(&dot) else {
                        continue;
                    };
                    let start = sheet.params_start(node) + at;
                    let report =
                        Report::new(start, start + dot.len(), "rejected").with(&[found, shown]);
                    // `\.scss(['" ]*)$`: the extension at the parameters' end, or at the end of
                    // `load-css()`'s path.
                    let quotes = |tail: &str| tail.chars().all(|c| matches!(c, '\'' | '"' | ' '));
                    let removed = if quotes(&params[at + dot.len()..]) {
                        Some(start)
                    } else if load_css
                        && let (Some(item), Some(inner)) =
                            (params.find(path.as_str()), path.rfind(&dot))
                        && quotes(&path[inner + dot.len()..])
                    {
                        Some(sheet.params_start(node) + item + inner)
                    } else {
                        None
                    };
                    reports.push(match removed {
                        Some(from) => report.fixed(from, from + dot.len(), ""),
                        None => report,
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Load, LoadRules, extension};
    use crate::scss::rules::tester::{Js, primary, run};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss_leading_underscore() {
        let tally = run(
            "stylelint-scss/load-no-partial-leading-underscore.txt",
            &|_| Ok(Box::new(LoadRules::NO_LEADING_UNDERSCORE)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (20, 21, 0),
            "{:#?}",
            tally.apart
        );
    }

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss_extension() {
        let tally = run(
            "stylelint-scss/load-partial-extension.txt",
            &|config| {
                Ok(Box::new(LoadRules {
                    load: Load::Extension {
                        always: primary(config) == &Js::Text("always".to_owned()),
                    },
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (108, 41, 0),
            "{:#?}",
            tally.apart
        );
        assert_eq!(
            (extension("a/b.scss"), extension(".hidden"), extension("a")),
            ("scss", "", "")
        );
    }
}
