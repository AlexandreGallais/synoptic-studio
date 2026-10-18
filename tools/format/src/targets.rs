//! The targets named in `package.json` (EN-045: folders and files by name, never `.`): `*` stands
//! for the root's files, which this tool lists itself, since oxfmt reads `*` at any depth.

/// The targets with `*` replaced by the root's files, each named once, in order.
///
/// # Errors
///
/// A glob other than `*`: oxfmt would read it at any depth.
pub fn expand(targets: &[String], root_files: &[String]) -> Result<Vec<String>, String> {
    let mut expanded: Vec<String> = Vec::new();

    for target in targets {
        let named: Vec<String> = if target == "*" {
            root_files.to_vec()
        } else if target.contains(['*', '?', '[', '!']) {
            return Err(format!(
                "{target:?}: only `*` (the root's files) is read as a glob"
            ));
        } else {
            vec![target.clone()]
        };

        for name in named {
            if !expanded.contains(&name) {
                expanded.push(name);
            }
        }
    }

    Ok(expanded)
}

#[cfg(test)]
mod tests {
    use super::expand;

    fn strings(items: &[&str]) -> Vec<String> {
        items.iter().map(|item| (*item).to_owned()).collect()
    }

    #[test]
    fn replaces_the_star_by_the_root_files() {
        assert_eq!(
            expand(
                &strings(&["*", "docs", "README.md"]),
                &strings(&["README.md", "package.json"])
            ),
            Ok(strings(&["README.md", "package.json", "docs"]))
        );
    }

    #[test]
    fn refuses_another_glob() {
        assert!(expand(&strings(&["*.md"]), &[]).is_err());
        assert!(expand(&strings(&["docs/**"]), &[]).is_err());
    }
}
