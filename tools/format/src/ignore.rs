//! The `ignorePatterns` of `.oxfmtrc.json`, so that the table pass skips the files oxfmt skips
//! (`CHANGELOG.md`, generated pages). Only plain paths are read: a name without `/` matches that
//! name at any depth, a path with `/` from the root; a glob is refused, never guessed.

/// The strings of the `ignorePatterns` array of a configuration's text; none without the key.
///
/// # Errors
///
/// A pattern holding a glob character (`*`, `?`, `[`, `!`), or an array left open.
pub fn patterns(config: &str) -> Result<Vec<String>, String> {
    let Some(start) = config.find("\"ignorePatterns\"") else {
        return Ok(Vec::new());
    };
    let after = &config[start..];
    let open = after.find('[').ok_or("ignorePatterns: no array")?;
    let close = after.find(']').ok_or("ignorePatterns: array left open")?;
    let found: Vec<String> = after[open + 1..close]
        .split(',')
        .map(|item| item.trim().trim_matches('"').to_owned())
        .filter(|item| !item.is_empty())
        .collect();

    match found
        .iter()
        .find(|pattern| pattern.contains(['*', '?', '[', '!']))
    {
        Some(glob) => Err(format!(
            "ignorePatterns: the glob {glob:?} is not read by tools/format; write a plain path"
        )),
        None => Ok(found),
    }
}

/// Whether a path from the root is ignored by one of the patterns.
#[must_use]
pub fn is_ignored(path: &str, patterns: &[String]) -> bool {
    let path = path.strip_prefix("./").unwrap_or(path);

    patterns.iter().any(|pattern| {
        let anchored = pattern.trim_start_matches('/').trim_end_matches('/');

        if pattern.trim_end_matches('/').contains('/') {
            path == anchored || path.starts_with(&format!("{anchored}/"))
        } else {
            path.split('/').any(|component| component == anchored)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{is_ignored, patterns};

    #[test]
    fn reads_the_patterns_of_the_configuration() {
        let config = "{\n  \"printWidth\": 100,\n  \"ignorePatterns\": [\n    \"dist\",\n    \"docs/api\",\n    \"CHANGELOG.md\"\n  ]\n}\n";

        assert_eq!(
            patterns(config),
            Ok(vec![
                "dist".into(),
                "docs/api".into(),
                "CHANGELOG.md".into()
            ])
        );
    }

    #[test]
    fn reads_no_pattern_without_the_key() {
        assert_eq!(patterns("{ \"printWidth\": 100 }"), Ok(Vec::new()));
    }

    #[test]
    fn refuses_a_glob() {
        assert!(patterns("{ \"ignorePatterns\": [\"**/*.md\"] }").is_err());
        assert!(patterns("{ \"ignorePatterns\": [\"dist\"").is_err());
    }

    #[test]
    fn matches_a_name_at_any_depth_and_a_path_from_the_root() {
        let list = vec![
            "dist".to_owned(),
            "docs/api".to_owned(),
            "CHANGELOG.md".to_owned(),
        ];

        assert!(is_ignored("CHANGELOG.md", &list));
        assert!(is_ignored("./CHANGELOG.md", &list));
        assert!(is_ignored("playground/dist/a.md", &list));
        assert!(is_ignored("docs/api/index.md", &list));
        assert!(!is_ignored("docs/apis.md", &list));
        assert!(!is_ignored("src/docs/api/a.md", &list));
        assert!(!is_ignored("README.md", &list));
    }
}
