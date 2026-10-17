//! `unicorn/no-incorrect-template-string-interpolation`: an untagged template writes no
//! `{name}` nor `$name}` where `${name}` was meant, for a name or a path of names
//! (`eslint-plugin-unicorn` 78.0.0). A named specifier or a destructuring opening an `import`,
//! `export`, `const`, `let` or `var` line, and a block comment, are left alone. Its fix is a
//! suggestion in `ESLint`: none here.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoIncorrectTemplateStringInterpolation;

/// Whether a byte starts a name: `$`, `_` or an ASCII letter.
const fn is_name_start(byte: u8) -> bool {
    byte == b'$' || byte == b'_' || byte.is_ascii_alphabetic()
}

/// Whether a byte continues a name: `$`, `_`, an ASCII letter or digit.
const fn is_name_part(byte: u8) -> bool {
    byte == b'$' || byte == b'_' || byte.is_ascii_alphanumeric()
}

/// The end of a path of names starting at an offset, `a.b.c`, when one starts there.
fn path_end(raw: &[u8], start: usize) -> Option<usize> {
    let mut at = start;
    loop {
        if !raw.get(at).copied().is_some_and(is_name_start) {
            return None;
        }
        at += 1;
        while raw.get(at).copied().is_some_and(is_name_part) {
            at += 1;
        }
        if raw.get(at) == Some(&b'.') && raw.get(at + 1).copied().is_some_and(is_name_start) {
            at += 1;
        } else {
            return Some(at);
        }
    }
}

/// The text before an offset on its line declares names: `import`, `export`, `const`, `let`
/// or `var`, then neither `;`, `=`, `{` nor `}`.
fn is_binding_declaration(raw: &str, index: usize) -> bool {
    let line_start = raw[..index].rfind('\n').map_or(0, |at| at + 1);
    let line = raw[line_start..index].trim_start();
    ["import", "export", "const", "let", "var"]
        .iter()
        .any(|keyword| {
            line.strip_prefix(keyword).is_some_and(|rest| {
                !rest
                    .chars()
                    .next()
                    .is_some_and(|next| next.is_alphanumeric() || next == '_' || next == '$')
                    && !rest.contains([';', '=', '{', '}'])
            })
        })
}

/// A mistaken interpolation: where it starts in the raw text, and what it should be.
struct Replacement {
    /// Where it starts.
    index: usize,
    /// The interpolation meant.
    correct: String,
}

/// The mistaken interpolations of a piece of raw template text, in order.
fn replacements(raw: &str) -> Vec<Replacement> {
    let bytes = raw.as_bytes();
    let before = |index: usize, back: usize| index.checked_sub(back).map(|at| bytes[at]);
    let mut found = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        let missing_dollar = bytes[at] == b'{'
            && !matches!(before(at, 1), Some(b'$' | b'\\' | b'{'))
            && !(before(at, 2) == Some(b'\\') && before(at, 1) == Some(b'u'));
        if missing_dollar
            && let Some(end) = path_end(bytes, at + 1)
            && bytes.get(end) == Some(&b'}')
            && bytes.get(end + 1) != Some(&b'}')
        {
            if !is_binding_declaration(raw, at) {
                found.push(Replacement {
                    index: at,
                    correct: format!("${}", &raw[at..=end]),
                });
            }
            at = end + 1;
            continue;
        }
        at += 1;
    }
    let mut at = 0;
    while at < bytes.len() {
        let missing_brace = bytes[at] == b'$' && !matches!(before(at, 1), Some(b'\\' | b'{'));
        if missing_brace
            && let Some(end) = path_end(bytes, at + 1)
            && bytes.get(end) == Some(&b'}')
            && bytes.get(end + 1) != Some(&b'}')
        {
            found.push(Replacement {
                index: at,
                correct: format!("${{{}}}", &raw[at + 1..end]),
            });
            at = end + 1;
            continue;
        }
        at += 1;
    }
    found.sort_by_key(|replacement| replacement.index);
    found
}

/// The ranges of the closed block comments of a text, `/* … */`.
fn block_comments(raw: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut from = 0;
    while let Some(open) = raw[from..].find("/*") {
        let start = from + open;
        let Some(close) = raw[start + 2..].find("*/") else {
            break;
        };
        let end = start + 2 + close + 2;
        ranges.push((start, end));
        from = end;
    }
    ranges
}

impl Rule for NoIncorrectTemplateStringInterpolation {
    fn name(&self) -> &'static str {
        "unicorn/no-incorrect-template-string-interpolation"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::TemplateLiteral(template) = node.kind() else {
            return;
        };
        if matches!(
            source.semantic.nodes().parent_kind(node.id()),
            AstKind::TaggedTemplateExpression(_)
        ) {
            return;
        }
        let start = template.span.start + 1;
        let end = template.span.end - 1;
        let mut masked: Vec<u8> = source.text.as_bytes()[start as usize..end as usize].to_vec();
        for pair in template.quasis.windows(2) {
            for byte in &mut masked
                [(pair[0].span.end - start) as usize..(pair[1].span.start - start) as usize]
            {
                *byte = b' ';
            }
        }
        let comments = block_comments(&String::from_utf8_lossy(&masked));
        for quasi in &template.quasis {
            let raw = crate::rules::unicorn::syntax::text(source, quasi.span);
            let offset = (quasi.span.start - start) as usize;
            for replacement in replacements(raw) {
                let position = offset + replacement.index;
                let inside = comments
                    .iter()
                    .any(|(from, to)| *from <= position && position < *to);
                if inside {
                    continue;
                }
                let at = quasi.span.start + u32::try_from(replacement.index).unwrap_or(0);
                findings.push(source.finding(
                    self.name(),
                    at,
                    format!(
                        "Use `{}` for template literal interpolation.",
                        replacement.correct
                    ),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoIncorrectTemplateStringInterpolation;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(
            &NoIncorrectTemplateStringInterpolation,
            "playground/a.ts",
            code,
        ))
    }

    #[test]
    fn reports_the_documented_mistakes() {
        assert_eq!(
            check("const greeting = `Hello {name}`;"),
            ["Use `${name}` for template literal interpolation."]
        );
        assert_eq!(
            check("const greeting = `Hello $user.name}`;"),
            ["Use `${user.name}` for template literal interpolation."]
        );
        assert_eq!(
            check("const a = `${x} {y} and $z}`;"),
            [
                "Use `${y}` for template literal interpolation.",
                "Use `${z}` for template literal interpolation."
            ]
        );
        assert_eq!(
            check("const code = `const x = {foo}`;"),
            ["Use `${foo}` for template literal interpolation."]
        );
    }

    #[test]
    fn accepts_the_documented_correct_and_ignored_forms() {
        for code in [
            "const greeting = `Hello ${name}`;",
            "const greeting = `Hello ${user.name}`;",
            "const query = gql`query { user { name } }`;",
            "const code = `import {foo} from \"bar\";`;",
            "const code = `const {foo} = bar;`;",
            "const code = `\n\t/**\n\t * @param {string} id\n\t */\n`;",
            "const a = `{name + suffix}`;",
            "const a = `{{name}}`;",
            "const a = `\\{name}`;",
            "const a = `\\u{41}`;",
            "const a = `{1x}`;",
            "const a = `${a}{b`;",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
