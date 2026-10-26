//! The files a run lints: the TypeScript files, the Markdown pages and the components' templates
//! of its targets, each a file or a folder walked down, the ignored paths left out, as oxlint and
//! `ESLint` take the same targets.

use std::fs;
use std::path::Path;

use crate::scopes::is_ignored;

/// The TypeScript files, the Markdown pages and the templates of the targets, from the root,
/// `/`-separated, sorted and once each.
#[must_use]
pub fn files(root: &Path, targets: &[String]) -> Vec<String> {
    let mut found = Vec::new();

    for target in targets {
        // lint-staged names each file by its absolute path.
        let relative = Path::new(target)
            .strip_prefix(root)
            .map_or(target.as_str(), |inside| {
                inside.to_str().unwrap_or_default()
            });
        if Path::new(relative).is_absolute() {
            continue;
        }
        let path = relative.trim_start_matches("./").trim_end_matches('/');
        collect(root, path, &mut found);
    }
    found.sort_unstable();
    found.dedup();
    found
}

/// Adds the TypeScript files, the Markdown pages and the templates under a path, unless it is ignored.
fn collect(root: &Path, path: &str, found: &mut Vec<String>) {
    let absolute = root.join(path);
    if is_ignored(path) || is_ignored(&format!("{path}/.")) {
        return;
    }
    if absolute.is_dir() {
        let Ok(entries) = fs::read_dir(&absolute) else {
            return;
        };
        for entry in entries.filter_map(Result::ok) {
            let name = entry.file_name().to_string_lossy().into_owned();
            let child = if path.is_empty() || path == "." {
                name
            } else {
                format!("{path}/{name}")
            };
            collect(root, &child, found);
        }
    } else if (absolute
        .extension()
        .is_some_and(|extension| extension == "ts")
        || crate::markdown::is_page(path)
        || crate::template::is_template(path))
        && absolute.is_file()
    {
        found.push(path.to_owned());
    }
}
