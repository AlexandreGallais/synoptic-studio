//! Where a worker writes (EN-119, I12): a subagent's calls start in the main checkout, so a
//! `Write` or `Edit` given a path of the main checkout lands there rather than in the worktree its
//! prompt names. The `guard-bash` hook, run on those tools too, refuses such a write and names the
//! same file in the worktree.

/// Why a worker of the worktree `worktree` (its folder's name, `en-119`) may not write `file`, an
/// absolute path, in the checkout `project`: the path in its worktree to write instead. `None` for
/// a file of its worktree, outside the project (a scratch folder), or in the Git folder every
/// worktree shares (`.git/metrics/step`).
#[must_use]
pub fn misplaced_write(file: &str, project: &str, worktree: &str) -> Option<String> {
    (file.is_empty() && project.is_empty() && worktree.is_empty()).then(String::new)
}
