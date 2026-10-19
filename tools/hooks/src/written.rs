//! Where a worker writes (EN-119, I12): a subagent's calls start in the main checkout, so a
//! `Write` or `Edit` given a path of the main checkout lands there rather than in the worktree its
//! prompt names. The `guard-bash` hook, run on those tools too, refuses such a write and names the
//! same file in the worktree.

use std::fs;
use std::path::{Path, PathBuf};

use git_repository::lexical_join;
use worktree::WORKTREES;

/// The Git folder every worktree shares, written by a worker on purpose (`.git/metrics/step`).
const GIT_FOLDER: &str = ".git";

/// Why a worker of the worktree `worktree` (its folder's name, `en-119`) may not write `file`, an
/// absolute path, in the checkout `project`: the path in its worktree to write instead. `None` for
/// a file of its worktree, outside the project (a scratch folder), in the Git folder every
/// worktree shares (`.git/metrics/step`), or a relative path, which the tools do not take. A file
/// of another worktree is named in its own, by its path inside that worktree.
#[must_use]
pub fn misplaced_write(file: &str, project: &str, worktree: &str) -> Option<String> {
    if !file.starts_with('/') {
        return None;
    }
    let file = lexical_join("/", file);
    let project = lexical_join("/", project);
    let inside = file.strip_prefix(&project)?.strip_prefix('/')?;
    let own = format!("{WORKTREES}/{worktree}");
    let under = |folder: &str| inside == folder || inside.starts_with(&format!("{folder}/"));

    if under(&own) || under(GIT_FOLDER) {
        return None;
    }
    let relative = inside
        .strip_prefix(&format!("{WORKTREES}/"))
        .map_or(inside, |rest| {
            rest.split_once('/').map_or("", |(_, path)| path)
        });
    let root = format!("{project}/{own}");
    let instead = if relative.is_empty() {
        root.clone()
    } else {
        format!("{root}/{relative}")
    };

    Some(format!(
        "This worker's worktree is `{root}`, its prompt's: write `{instead}`, not `{file}`, which lies outside it (EN-119)."
    ))
}

/// The file a path names, as the project's folder names it (AUD-033): the path's longest existing
/// folder resolved by the file system, its symbolic links followed and its letters in the case they
/// are stored in, the rest kept; a file inside the project so resolved is named from `project`
/// as given. Any other path is returned resolved, or as given when nothing of it can be read.
#[must_use]
pub fn as_seen_from(file: &str, project: &str) -> String {
    if !file.starts_with('/') {
        return file.to_owned();
    }
    let Some(real) = resolved(file) else {
        return file.to_owned();
    };
    let real_project = resolved(project).unwrap_or_else(|| PathBuf::from(project));

    match real.strip_prefix(&real_project) {
        Ok(rest) if rest.as_os_str().is_empty() => project.to_owned(),
        Ok(rest) => Path::new(project).join(rest).to_string_lossy().into_owned(),
        Err(_) => real.to_string_lossy().into_owned(),
    }
}

/// A path, its `..` removed by its letters, then its longest existing folder resolved by the file
/// system (`fs::canonicalize`) and the rest kept; `None` when no folder of it can be resolved.
fn resolved(path: &str) -> Option<PathBuf> {
    let lexical = PathBuf::from(lexical_join("/", path));

    lexical.ancestors().find_map(|ancestor| {
        let real = fs::canonicalize(ancestor).ok()?;

        Some(real.join(lexical.strip_prefix(ancestor).ok()?))
    })
}
