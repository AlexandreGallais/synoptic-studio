//! The globs of the scopes, read as `ESLint`'s flat configuration reads them: `**` for any number
//! of folders, `*` for any characters but `/`, `{a,b}` for either; a pattern without `/` matches
//! a file of the root only.

/// Whether a path from the root, `/`-separated, matches a glob.
#[must_use]
pub fn matches(pattern: &str, path: &str) -> bool {
    alternatives(pattern).iter().any(|each| {
        let pattern: Vec<&str> = each.split('/').collect();
        let path: Vec<&str> = path.split('/').collect();

        segments(&pattern, &path)
    })
}

/// Whether a path matches any of the globs.
#[must_use]
pub fn matches_any(patterns: &[&str], path: &str) -> bool {
    patterns.iter().any(|pattern| matches(pattern, path))
}

/// The patterns a glob stands for, its first `{a,b}` written out, recursively.
fn alternatives(pattern: &str) -> Vec<String> {
    let (Some(open), Some(close)) = (pattern.find('{'), pattern.find('}')) else {
        return vec![pattern.to_owned()];
    };
    if close < open {
        return vec![pattern.to_owned()];
    }

    pattern[open + 1..close]
        .split(',')
        .flat_map(|choice| {
            alternatives(&format!(
                "{}{choice}{}",
                &pattern[..open],
                &pattern[close + 1..]
            ))
        })
        .collect()
}

/// Whether the path's folders and name match the pattern's, `**` standing for any number of them.
fn segments(pattern: &[&str], path: &[&str]) -> bool {
    match (pattern.first(), path.first()) {
        (None, None) => true,
        (Some(&"**"), _) => {
            segments(&pattern[1..], path) || (!path.is_empty() && segments(pattern, &path[1..]))
        }
        (Some(each), Some(name)) => segment(each, name) && segments(&pattern[1..], &path[1..]),
        _ => false,
    }
}

/// Whether one name matches one segment of a pattern, `*` standing for any characters.
fn segment(pattern: &str, name: &str) -> bool {
    match pattern.split_once('*') {
        None => pattern == name,
        Some((head, rest)) => {
            let Some(tail) = name.strip_prefix(head) else {
                return false;
            };
            (0..=tail.len())
                .filter(|cut| tail.is_char_boundary(*cut))
                .any(|cut| segment(rest, &tail[cut..]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{matches, matches_any};

    #[test]
    fn matches_any_depth_with_two_stars() {
        assert!(matches("src/**/*.ts", "src/math/vector/norm.ts"));
        assert!(matches("src/**/*.ts", "src/index.ts"));
        assert!(!matches("src/**/*.ts", "playground/index.ts"));
        assert!(matches("**/*.test.ts", "a.test.ts"));
        assert!(matches("**/*.test.ts", "src/a/b.test.ts"));
        assert!(matches("dist/**", "dist/a/b.js"));
    }

    #[test]
    fn matches_the_root_only_without_a_slash() {
        assert!(matches("*.config.ts", "vite.config.ts"));
        assert!(!matches("*.config.ts", "docs/vite.config.ts"));
        assert!(matches("tests/*.ts", "tests/a.ts"));
        assert!(!matches("tests/*.ts", "tests/b/a.ts"));
    }

    #[test]
    fn needs_a_folder_for_a_star_between_two_stars() {
        assert!(matches(
            "playground/*/**/index.ts",
            "playground/points/index.ts"
        ));
        assert!(!matches("playground/*/**/index.ts", "playground/index.ts"));
    }

    #[test]
    fn reads_alternatives_and_stars_inside_a_name() {
        assert!(matches("**/*.{js,cjs,mjs}", "a/b.cjs"));
        assert!(!matches("**/*.{js,cjs,mjs}", "a/b.ts"));
        assert!(matches("a*b*c", "aXbYc"));
        assert!(!matches("a*b*c", "aXbY"));
        assert!(matches("{x", "{x"));
        assert!(matches("}x{", "}x{"));
        assert!(matches_any(&["a", "b"], "b"));
        assert!(!matches_any(&[], "b"));
    }
}
