//! `unicorn/prefer-single-replace`: a chain of global replacements of single characters by the
//! same string is one `replaceAll()` with a character class (`eslint-plugin-unicorn` 78.0.0).
//! Autofixed, as in `ESLint`, unless a comment sits in the chain.

use oxc_ast::AstKind;
use oxc_ast::ast::{Argument, CallExpression, Expression, RegExpFlags};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::escapes::numeric_escape;
use crate::rules::unicorn::syntax::{has_comment, method_call, parenthesized_text};
use crate::source::Source;

/// The rule.
pub struct PreferSingleReplace;

/// The characters a regular expression gives a meaning to outside a class.
const SYNTAX_CHARACTERS: &str = "^$\\.*+?()[]{}|/";

/// The character one escape writes in a regular expression without the `u` flag, and its length.
fn escaped_character(pattern: &str, in_class: bool) -> Option<(char, usize)> {
    let rest = pattern.strip_prefix('\\')?;
    let next = rest.chars().next()?;
    let simple = match next {
        'n' => Some('\n'),
        't' => Some('\t'),
        'r' => Some('\r'),
        'f' => Some('\u{C}'),
        'v' => Some('\u{B}'),
        'b' if in_class => Some('\u{8}'),
        '0' if !rest[1..].starts_with(|each: char| each.is_ascii_digit()) => Some('\0'),
        'x' | 'u' => None,
        'c' => {
            let letter = rest[1..].chars().next().filter(char::is_ascii_alphabetic)?;
            return char::from_u32(u32::from(letter) % 32).map(|each| (each, 3));
        }
        each if each.is_ascii_alphanumeric() => return None,
        each => Some(each),
    };
    if let Some(character) = simple {
        return Some((character, 1 + next.len_utf8()));
    }
    let escape = numeric_escape(pattern, 0)
        .filter(|escape| escape.form != crate::rules::unicorn::escapes::EscapeForm::CodePoint)?;
    char::from_u32(escape.value).map(|each| (each, escape.end))
}

/// The one character a regular expression pattern matches, written alone or in a class of one.
fn pattern_character(pattern: &str) -> Option<char> {
    let (inner, in_class) = match pattern.strip_prefix('[') {
        Some(rest) if !rest.starts_with('^') => (rest.strip_suffix(']')?, true),
        Some(_) => return None,
        None => (pattern, false),
    };
    let (character, length) = if inner.starts_with('\\') {
        escaped_character(inner, in_class)?
    } else {
        let character = inner.chars().next()?;
        let special = if in_class {
            character == ']'
        } else {
            SYNTAX_CHARACTERS.contains(character)
        };
        if special {
            return None;
        }
        (character, character.len_utf8())
    };
    (length == inner.len() && u32::from(character) <= 0xFFFF).then_some(character)
}

/// The character a `replace()` or `replaceAll()` call replaces everywhere, when it is one.
fn replaced_character(call: &CallExpression) -> Option<char> {
    let method = method_call(call)?;
    if !matches!(method.name, "replace" | "replaceAll")
        || !method.is_plain()
        || call.arguments.len() != 2
        || call
            .arguments
            .iter()
            .any(|argument| matches!(argument, Argument::SpreadElement(_)))
    {
        return None;
    }
    match &call.arguments[0] {
        Argument::StringLiteral(search) => {
            if method.name != "replaceAll" {
                return None;
            }
            let mut units = search.value.encode_utf16();
            let first = units.next()?;
            if units.next().is_some() {
                return None;
            }
            char::from_u32(u32::from(first))
        }
        Argument::RegExpLiteral(literal) => {
            let flags = literal.regex.flags;
            let safe = RegExpFlags::G | RegExpFlags::M | RegExpFlags::S;
            if !flags.contains(RegExpFlags::G) || !safe.contains(flags) {
                return None;
            }
            pattern_character(literal.regex.pattern.text.as_str())
        }
        _ => None,
    }
}

/// The string a call replaces with, when it is a literal.
fn replacement<'a>(call: &CallExpression<'a>) -> Option<&'a str> {
    match call.arguments.get(1)? {
        Argument::StringLiteral(literal) => literal.value.as_str(),
        _ => None,
    }
}

/// Whether a call replaces one character by a given string.
fn is_mergeable(call: &CallExpression, value: &str) -> bool {
    replaced_character(call).is_some() && replacement(call) == Some(value)
}

/// A character written inside a class, escaped where it would mean something.
fn class_character(character: char, first: bool, last: bool) -> String {
    match character {
        '\\' => "\\\\".to_owned(),
        ']' => "\\]".to_owned(),
        '\n' => "\\n".to_owned(),
        '\r' => "\\r".to_owned(),
        '\t' => "\\t".to_owned(),
        '\u{C}' => "\\f".to_owned(),
        '\u{B}' => "\\v".to_owned(),
        '\u{2028}' => "\\u2028".to_owned(),
        '\u{2029}' => "\\u2029".to_owned(),
        '^' if first => "\\^".to_owned(),
        '-' if !first && !last => "\\-".to_owned(),
        each if u32::from(each) < 0x20 || u32::from(each) == 0x7F => {
            format!("\\x{:02X}", u32::from(each))
        }
        each => each.to_string(),
    }
}

impl Rule for PreferSingleReplace {
    fn name(&self) -> &'static str {
        "unicorn/prefer-single-replace"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        if replaced_character(call).is_none() {
            return;
        }
        let Some(value) = replacement(call) else {
            return;
        };
        if value.contains("$`") || value.contains("$'") {
            return;
        }
        let nodes = source.semantic.nodes();
        let parent = nodes.parent_node(node.id());
        if let AstKind::StaticMemberExpression(member) = parent.kind()
            && member.object.span() == call.span
            && let AstKind::CallExpression(outer) = nodes.parent_kind(parent.id())
            && outer.callee.span() == member.span
            && is_mergeable(outer, value)
        {
            return;
        }
        let mut characters = Vec::new();
        let mut walking = call;
        let base = loop {
            if !is_mergeable(walking, value) {
                break None;
            }
            characters.extend(replaced_character(walking));
            let Some(method) = method_call(walking) else {
                break None;
            };
            match method.object {
                Expression::CallExpression(inner) if is_mergeable(inner, value) => walking = inner,
                other => break Some(other),
            }
        };
        let Some(base) = base else {
            return;
        };
        if characters.len() < 2 {
            return;
        }
        characters.reverse();
        let mut unique: Vec<char> = Vec::new();
        for each in characters {
            if !unique.contains(&each) {
                unique.push(each);
            }
        }
        if unique.iter().any(|each| value.contains(*each)) {
            return;
        }
        let message = "Combine these single-character replacements into a single `replaceAll()` with a regular expression.".to_owned();
        if has_comment(source, call.span) {
            findings.push(source.finding(self.name(), call.span.start, message));
            return;
        }
        let class: String = unique
            .iter()
            .enumerate()
            .map(|(index, each)| class_character(*each, index == 0, index + 1 == unique.len()))
            .collect();
        let replacement_text = call.arguments[1]
            .as_expression()
            .map_or_else(String::new, |each| parenthesized_text(source, each));
        findings.push(source.fixed(
            self.name(),
            call.span.start,
            message,
            Fix {
                start: call.span.start,
                end: call.span.end,
                text: format!(
                    "{}.replaceAll(/[{class}]/g, {replacement_text})",
                    parenthesized_text(source, base)
                ),
            },
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferSingleReplace;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferSingleReplace, "playground/a.ts", code))
    }

    /// The code once fixed.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&PreferSingleReplace, "playground/a.ts", code))
    }

    const MESSAGE: &str = "Combine these single-character replacements into a single `replaceAll()` with a regular expression.";

    #[test]
    fn fixes_the_documented_chains() {
        let code =
            "const slug = string.replaceAll('a', '-').replaceAll('b', '-').replaceAll('c', '-');";
        assert_eq!(check(code), [MESSAGE]);
        assert_eq!(fix(code), "const slug = string.replaceAll(/[abc]/g, '-');");
        assert_eq!(
            fix("const cleaned = string.replace(/a/g, '_').replaceAll('b', '_');"),
            "const cleaned = string.replaceAll(/[ab]/g, '_');"
        );
        assert_eq!(
            fix(
                "const a = s.replace(/\\./g, '').replace(/[-]/gm, '').replaceAll('^', '').replaceAll(']', '');"
            ),
            "const a = s.replaceAll(/[.\\-^\\]]/g, '');"
        );
        assert_eq!(
            fix("const a = s.replace(/\\n/g, ' ').replace(/\\t/g, ' ');"),
            "const a = s.replaceAll(/[\\n\\t]/g, ' ');"
        );
    }

    #[test]
    fn accepts_first_only_reintroducing_and_flagged_replacements() {
        for code in [
            "const a = string.replace('a', '-').replace('b', '-');",
            "const a = string.replaceAll('a', '-');",
            "const a = string.replaceAll('a', '-').replaceAll('b', '_');",
            "const a = string.replaceAll('a', 'ab').replaceAll('b', 'ab');",
            "const a = string.replace(/a/gi, '-').replaceAll('b', '-');",
            "const a = string.replace(/a/gu, '-').replaceAll('b', '-');",
            "const a = string.replace(/ab/g, '-').replaceAll('b', '-');",
            "const a = string.replace(/\\d/g, '-').replaceAll('b', '-');",
            "const a = string.replaceAll('a', \"$'\").replaceAll('b', \"$'\");",
            "const a = string.replaceAll('ab', '-').replaceAll('b', '-');",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
