//! The HTML rules of the templates (F53, EN-166): the rules of `html-validate` 11.17.0 the
//! catalogue keeps (REF-HTML-VALIDATE-SOURCE, REF-HTML-VALIDATE-RULES), each one module reading
//! the template as html-validate's DOM holds a document (`view`), with HTML's element metadata
//! (`elements`) and its checks (`validator`); named `html-validate/<rule>`, their messages
//! html-validate's. Each rule is tested on every case of its test file html-validate's tests
//! can run on a template (`cases`), counted, the cases set apart counted by their reason.

pub mod a11y;
#[cfg(test)]
pub mod cases;
pub mod elements;
pub mod rules;
pub mod selector;
pub mod validator;
pub mod view;

use regex::Regex;

use super::lexer::{self, Span};
use super::rules::Report;
use crate::finding::Fix;

/// The one message of an HTML rule: html-validate writes each in full.
pub const MESSAGES: &[(&str, &str)] = &[("html", "{{message}}")];

/// A report of an HTML rule: where, and its message.
#[must_use]
pub fn report(span: Span, message: String) -> Report {
    Report {
        span,
        message_id: "html",
        data: vec![("message", message)],
        fix: None,
    }
}

/// A report with its fix.
#[must_use]
pub fn fixed(span: Span, message: String, fix: Fix) -> Report {
    Report {
        fix: Some(fix),
        ..report(span, message)
    }
}

/// The tokens of a value as `DOMTokenList` splits it (`src/dom/domtokenlist.ts`): at spaces,
/// tabs and line ends, each with its span, the value's first byte at a span's start.
#[must_use]
pub fn tokens(value: &str, span: Span) -> Vec<(&str, Span)> {
    let base = span.start as usize;
    let mut found = Vec::new();
    let mut begin = None;
    for (at, character) in value
        .char_indices()
        .chain(std::iter::once((value.len(), ' ')))
    {
        let blank = matches!(character, ' ' | '\t' | '\n' | '\r');
        match (begin, blank) {
            (None, false) => begin = Some(at),
            (Some(start), true) => {
                found.push((&value[start..at], Span::new(base + start, base + at)));
                begin = None;
            }
            _ => {}
        }
    }
    found
}

/// A fix removing a span, with the spaces and tabs before it and one line end before them, or
/// after it, when asked (`removeText` with `trimText` of `src/autofix/`).
#[must_use]
pub fn remove(text: &str, span: Span, trim_start: bool, trim_end: bool) -> Fix {
    let bytes = text.as_bytes();
    let (mut start, mut end) = (span.start as usize, span.end as usize);
    let mut allow_newline = true;
    if trim_start {
        while start > 0 && matches!(bytes[start - 1], b' ' | b'\t') {
            start -= 1;
        }
        if start > 0 && bytes[start - 1] == b'\n' {
            allow_newline = false;
            start -= 1;
            if start > 0 && bytes[start - 1] == b'\r' {
                start -= 1;
            }
        }
    }
    if trim_end {
        while end < bytes.len() && matches!(bytes[end], b' ' | b'\t') {
            end += 1;
        }
        if allow_newline {
            if bytes.get(end..end + 2) == Some(b"\r\n") {
                end += 2;
            } else if bytes.get(end) == Some(&b'\n') {
                end += 1;
            }
        }
    }
    Fix {
        start: lexer::offset(start),
        end: lexer::offset(end),
        text: String::new(),
    }
}

/// Whether a keyword matches one of a list of patterns (`keywordPatternMatcher`): `/…/` a
/// regular expression, any other text with `*` standing for one character or more, anchored.
#[must_use]
pub fn matches_pattern(list: &[String], keyword: &str) -> bool {
    list.iter().any(|pattern| {
        let source = match pattern
            .strip_prefix('/')
            .and_then(|rest| rest.strip_suffix('/'))
        {
            Some(inner) => format!("^{inner}$"),
            None => format!(
                "^{}$",
                Regex::new(r"\*+").map_or_else(
                    |_| pattern.clone(),
                    |stars| stars.replace_all(pattern, ".+").into_owned()
                )
            ),
        };
        Regex::new(&source).is_ok_and(|regex| regex.is_match(keyword))
    })
}

/// Whether a keyword is left out by `include` and `exclude` options read as patterns
/// (`isKeywordIgnored` with `keywordPatternMatcher`).
#[must_use]
pub fn is_pattern_ignored(
    include: Option<&[String]>,
    exclude: Option<&[String]>,
    keyword: &str,
) -> bool {
    include.is_some_and(|list| !matches_pattern(list, keyword))
        || exclude.is_some_and(|list| matches_pattern(list, keyword))
}

/// Whether a keyword is left out by a rule's `include` and `exclude` options
/// (`isKeywordIgnored` of `src/rules/helper/is-keyword-ignored.ts`), compared as written.
#[must_use]
pub fn is_keyword_ignored(
    include: Option<&[String]>,
    exclude: Option<&[String]>,
    keyword: &str,
) -> bool {
    include.is_some_and(|list| !list.iter().any(|each| each == keyword))
        || exclude.is_some_and(|list| list.iter().any(|each| each == keyword))
}

#[cfg(test)]
mod tests {
    use super::{is_keyword_ignored, remove, tokens};
    use crate::template::lexer::Span;

    #[test]
    fn splits_tokens_and_removes_text_as_html_validate_does() {
        // Our own cases, after `DOMTokenList` and `removeText` with `trimText`.
        let found: Vec<&str> = tokens(" a\tb\r\nc ", Span::new(0, 8))
            .into_iter()
            .map(|(token, _)| token)
            .collect();
        assert_eq!(found, ["a", "b", "c"]);
        let removed = remove("x  y \r\nz", Span::new(3, 4), true, true);
        assert_eq!((removed.start, removed.end), (1, 7));
        let after_newline = remove("x\n y \nz", Span::new(3, 4), true, true);
        assert_eq!((after_newline.start, after_newline.end), (1, 5));
    }

    #[test]
    fn ignores_a_keyword_as_include_and_exclude_say() {
        // Our own cases, after `isKeywordIgnored`.
        let list = ["a".to_owned()];
        assert!(!is_keyword_ignored(None, None, "a"));
        assert!(is_keyword_ignored(Some(&list), None, "b"));
        assert!(is_keyword_ignored(None, Some(&list), "a"));
    }
}
