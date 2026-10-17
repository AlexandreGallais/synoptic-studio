//! The packages the repository's `package.json` names, by field, as `import-x`'s
//! `no-extraneous-dependencies` reads them: each field's names, an object's keys or an array's
//! items. Read with a scanner of JSON's strings and brackets, the workspace having no JSON crate.

/// The packages of each field of `package.json`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Dependencies {
    /// `dependencies`.
    pub dependencies: Vec<String>,
    /// `devDependencies`.
    pub dev: Vec<String>,
    /// `optionalDependencies`.
    pub optional: Vec<String>,
    /// `peerDependencies`.
    pub peer: Vec<String>,
    /// `bundleDependencies` or `bundledDependencies`.
    pub bundled: Vec<String>,
}

/// A JSON string starting at a byte, unescaped, and the byte after its closing quote.
fn string_at(text: &str, start: usize) -> Option<(String, usize)> {
    let mut value = String::new();
    let mut characters = text[start + 1..].char_indices();

    while let Some((at, character)) = characters.next() {
        match character {
            '"' => return Some((value, start + 1 + at + 1)),
            '\\' => {
                let (_, escaped) = characters.next()?;
                value.push(match escaped {
                    'n' => '\n',
                    't' => '\t',
                    other => other,
                });
            }
            other => value.push(other),
        }
    }
    None
}

/// The byte of the first character after blanks.
fn skip_blanks(text: &str, from: usize) -> usize {
    from + (text[from..].len() - text[from..].trim_start().len())
}

/// The names a member of the root object holds: an object's keys, or an array's strings.
fn members(text: &str, field: &str) -> Vec<String> {
    let mut depth = 0usize;
    let mut at = 0;
    let mut found = Vec::new();
    let mut reading = false;

    while at < text.len() {
        let character = text[at..].chars().next().unwrap_or(' ');
        match character {
            '"' => {
                let Some((value, after)) = string_at(text, at) else {
                    break;
                };
                let next = skip_blanks(text, after);
                let is_key = text[next..].starts_with(':');
                if depth == 1 && is_key {
                    reading = value == field;
                } else if reading && depth == 2 {
                    let in_array = !is_key && text[..at].trim_end().ends_with(['[', ',']);
                    if is_key || in_array {
                        found.push(value);
                    }
                }
                at = after;
                continue;
            }
            '{' | '[' => depth += 1,
            '}' | ']' => {
                depth = depth.saturating_sub(1);
                if depth == 1 {
                    reading = false;
                }
            }
            _ => {}
        }
        at += character.len_utf8();
    }
    found
}

/// The packages of a `package.json`'s text.
#[must_use]
pub fn parse(text: &str) -> Dependencies {
    let mut bundled = members(text, "bundleDependencies");
    bundled.extend(members(text, "bundledDependencies"));

    Dependencies {
        dependencies: members(text, "dependencies"),
        dev: members(text, "devDependencies"),
        optional: members(text, "optionalDependencies"),
        peer: members(text, "peerDependencies"),
        bundled,
    }
}

#[cfg(test)]
mod tests {
    use super::{Dependencies, parse};

    #[test]
    fn reads_each_field_s_names() {
        let text = r#"{
  "name": "editor",
  "scripts": { "dependencies": "x" },
  "dependencies": {},
  "devDependencies": { "@a/b": "1.0.0", "c": "^2", "d\"e": "3" },
  "peerDependencies": { "f": "1" },
  "bundledDependencies": ["g", "h"]
}"#;

        assert_eq!(
            parse(text),
            Dependencies {
                dependencies: Vec::new(),
                dev: vec!["@a/b".into(), "c".into(), "d\"e".into()],
                optional: Vec::new(),
                peer: vec!["f".into()],
                bundled: vec!["g".into(), "h".into()],
            }
        );
    }

    #[test]
    fn reads_nothing_from_a_text_that_is_no_package() {
        assert_eq!(parse(""), Dependencies::default());
        assert_eq!(parse("{\"dependencies\": "), Dependencies::default());
    }
}
