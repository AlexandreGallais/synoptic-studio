//! The clean-up of the local clone after a feature (ADR-0072, AUD-020, EN-103): local branches
//! already merged into `develop` or whose branch on GitHub was deleted are removed with their
//! worktrees, and what is left reported. A branch just made for a story — never pushed, without a
//! commit beyond the branch it was made from — and a worktree a running process works in are kept.

/// Branches never deleted.
const KEPT: [&str; 2] = ["develop", "main"];

/// The branch a story's commits are compared with when it has no other base.
pub const DEVELOP: &str = "origin/develop";

/// A local branch's line of `git for-each-ref`: its name, `%(upstream:track)` (`[gone]`,
/// `[ahead 1]` or empty) and `%(upstream:short)` (`origin/<name>` once pushed, the branch it was
/// made from otherwise).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BranchFields {
    pub name: String,
    pub track: String,
    pub upstream: String,
}

impl BranchFields {
    /// The fields of a line `<name>|<track>|<upstream>`.
    #[must_use]
    pub fn of_line(line: &str) -> BranchFields {
        let mut fields = line.splitn(3, '|').map(str::to_owned);

        BranchFields {
            name: fields.next().unwrap_or_default(),
            track: fields.next().unwrap_or_default(),
            upstream: fields.next().unwrap_or_default(),
        }
    }

    /// Whether the branch was pushed under its own name.
    fn is_published(&self) -> bool {
        self.upstream == format!("origin/{}", self.name)
    }
}

/// A local branch as Git reports it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalBranch {
    /// Its own branch on GitHub was deleted (`[gone]`).
    pub is_gone: bool,
    /// Its commits are all in `origin/develop`.
    pub is_merged: bool,
    /// Never pushed under its own name and without a commit beyond its base: a story just started.
    pub is_fresh: bool,
    pub name: String,
}

/// The branch a local branch's own commits are counted from: the branch it was made from, which a
/// worktree of `worktree-new` tracks, or `origin/develop`.
#[must_use]
pub fn base_of(fields: &BranchFields) -> String {
    if fields.upstream.is_empty() || fields.is_published() || fields.track == "[gone]" {
        DEVELOP.to_owned()
    } else {
        fields.upstream.clone()
    }
}

/// A local branch read from its line: gone only when its own branch on GitHub was deleted, never
/// because the branch it was made from was.
#[must_use]
pub fn read_branch(fields: &BranchFields, is_merged: bool, own_commits: u64) -> LocalBranch {
    let is_pushed = fields.is_published();

    LocalBranch {
        is_fresh: !is_pushed && own_commits == 0,
        is_gone: is_pushed && fields.track == "[gone]",
        is_merged,
        name: fields.name.clone(),
    }
}

/// Local branches to delete: merged or gone, never `main`, `develop`, the one checked out nor a
/// story just started.
#[must_use]
pub fn stale_branches(branches: &[LocalBranch], current: &str) -> Vec<String> {
    branches
        .iter()
        .filter(|branch| !KEPT.contains(&branch.name.as_str()) && branch.name != current)
        .filter(|branch| branch.is_gone || (branch.is_merged && !branch.is_fresh))
        .map(|branch| branch.name.clone())
        .collect()
}

/// What a deleted branch's rows of the records become (AUD-050, EN-154): `merged` when its tip is
/// in a branch of `origin` other than its own — `develop`, a feature branch it was merged into —
/// else `dropped`, the drafts of a branch deleted without its merge, which no branch reads any
/// more but with `--all`. `holders`: the remote branches whose history holds its tip
/// (`git for-each-ref --contains`).
#[must_use]
pub fn rows_state(name: &str, holders: &[String]) -> &'static str {
    let own = format!("origin/{name}");

    if holders
        .iter()
        .any(|holder| *holder != own && holder != "origin/HEAD" && holder != "origin")
    {
        "merged"
    } else {
        "dropped"
    }
}

/// A worktree as `git worktree list --porcelain` reports it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Worktree {
    /// Branch checked out in it, empty when its head is detached.
    pub branch: String,
    pub path: String,
}

/// Worktrees listed by Git, the main checkout first: one block per worktree, blocks split at a
/// line holding only blanks.
#[must_use]
pub fn worktrees_of(porcelain: &str) -> Vec<Worktree> {
    let mut blocks: Vec<Vec<&str>> = vec![Vec::new()];

    for line in porcelain.split('\n') {
        if line.trim().is_empty() {
            blocks.push(Vec::new());
        } else if let Some(block) = blocks.last_mut() {
            block.push(line);
        }
    }

    blocks
        .iter()
        .map(|lines| {
            let field = |key: &str| {
                lines
                    .iter()
                    .find_map(|line| line.strip_prefix(key))
                    .unwrap_or_default()
                    .to_owned()
            };
            let branch = lines
                .iter()
                .find(|line| line.starts_with("branch "))
                .map(|line| line.get("branch refs/heads/".len()..).unwrap_or_default())
                .unwrap_or_default()
                .to_owned();

            Worktree {
                branch,
                path: field("worktree "),
            }
        })
        .filter(|worktree| !worktree.path.is_empty())
        .collect()
}

/// Worktrees to remove: those whose branch is stale or no longer exists, never the main checkout
/// nor a detached one.
#[must_use]
pub fn stale_worktrees(worktrees: &[Worktree], stale: &[String], local: &[String]) -> Vec<String> {
    worktrees
        .iter()
        .skip(1)
        .filter(|worktree| !worktree.branch.is_empty())
        .filter(|worktree| stale.contains(&worktree.branch) || !local.contains(&worktree.branch))
        .map(|worktree| worktree.path.clone())
        .collect()
}

/// The working folders of the running processes, from `lsof -d cwd -Fn`: a line `p<pid>` then
/// `n<folder>` per process.
#[must_use]
pub fn working_folders(lsof: &str) -> Vec<String> {
    lsof.split('\n')
        .filter_map(|line| line.strip_prefix('n'))
        .filter(|folder| folder.starts_with('/'))
        .map(str::to_owned)
        .collect()
}

/// Whether a running process works in a worktree or in one of its folders.
#[must_use]
pub fn is_in_use(path: &str, folders: &[String]) -> bool {
    let inside = format!("{path}/");

    folders
        .iter()
        .any(|folder| folder == path || folder.starts_with(&inside))
}

/// The report printed at the end: what was deleted, the worktrees kept and why, then what is left.
#[must_use]
pub fn report(deleted: &[String], kept: &[(String, String)], left: &Left) -> String {
    let or_none = |items: &[String]| {
        if items.is_empty() {
            "none".to_owned()
        } else {
            items.join(", ")
        }
    };
    let kept: Vec<String> = kept
        .iter()
        .map(|(path, reason)| format!("{path} ({reason})"))
        .collect();
    let remote: Vec<&str> = left
        .remote
        .iter()
        .map(String::as_str)
        .filter(|name| *name != "origin/HEAD" && *name != "origin")
        .collect();

    [
        format!("deleted: {}", or_none(deleted)),
        format!("worktrees kept: {}", or_none(&kept)),
        format!("local: {}", left.local.join(", ")),
        format!("on GitHub: {}", remote.join(", ")),
        format!("stashes: {}", left.stashes),
        format!("worktrees: {}", left.worktrees),
    ]
    .join("\n")
}

/// What is left after the clean-up.
#[derive(Clone, Debug, Default)]
pub struct Left {
    pub local: Vec<String>,
    pub remote: Vec<String>,
    pub stashes: usize,
    pub worktrees: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The feature branch a story's worktree is made from, as `worktree-new` tracks it.
    const FEATURE: &str = "origin/feature/f18-safe-and-cheap-parallel-run";

    fn branch(name: &str, is_gone: bool, is_merged: bool, is_fresh: bool) -> LocalBranch {
        LocalBranch {
            is_gone,
            is_merged,
            is_fresh,
            name: name.to_owned(),
        }
    }

    fn fields(name: &str, track: &str, upstream: &str) -> BranchFields {
        BranchFields {
            name: name.to_owned(),
            track: track.to_owned(),
            upstream: upstream.to_owned(),
        }
    }

    fn owned(items: &[&str]) -> Vec<String> {
        items.iter().map(|&item| item.to_owned()).collect()
    }

    #[test]
    fn deletes_merged_or_gone_branches_never_main_develop_nor_the_one_checked_out() {
        let branches = [
            branch("main", false, true, false),
            branch("develop", false, true, false),
            branch("feat/val-010-validate-f11", true, false, false),
            branch("feature/f11-code-base-ready-for-curves", false, true, false),
            branch("refactor/en-024-work-in-progress", false, false, false),
            branch("docs/sp-011-research-f12", true, true, false),
        ];

        assert_eq!(
            stale_branches(&branches, "docs/sp-011-research-f12"),
            [
                "feat/val-010-validate-f11",
                "feature/f11-code-base-ready-for-curves"
            ]
        );
    }

    #[test]
    fn f18_ac8_keeps_a_story_just_started_though_all_its_commits_are_in_develop() {
        let fresh = branch("fix/aud-020-topic", false, true, true);

        assert_eq!(
            stale_branches(std::slice::from_ref(&fresh), "develop"),
            Vec::<String>::new()
        );
        assert_eq!(
            stale_branches(
                &[LocalBranch {
                    is_fresh: false,
                    ..fresh
                }],
                "develop"
            ),
            ["fix/aud-020-topic"]
        );
    }

    #[test]
    fn f18_ac8_reads_a_story_just_made_from_its_feature_as_fresh_never_gone() {
        // `worktree-new` made it from the feature branch; no commit yet.
        let made = fields("fix/aud-020-topic", "", FEATURE);

        assert_eq!(
            read_branch(&made, true, 0),
            branch("fix/aud-020-topic", false, true, true)
        );
        // One commit of its own: no longer fresh.
        assert!(
            !read_branch(&fields("fix/aud-020-topic", "[ahead 1]", FEATURE), false, 1).is_fresh
        );
        // The feature branch deleted on GitHub: the story's own branch is not gone.
        assert!(!read_branch(&fields("fix/aud-020-topic", "[gone]", FEATURE), false, 3).is_gone);
    }

    #[test]
    fn reads_a_branch_pushed_under_its_own_name_as_gone_once_github_deleted_it() {
        let pushed = fields(
            "feat/us-061-font-list",
            "[gone]",
            "origin/feat/us-061-font-list",
        );

        assert_eq!(
            read_branch(&pushed, false, 0),
            branch("feat/us-061-font-list", true, false, false)
        );
        assert!(
            !read_branch(
                &BranchFields {
                    track: String::new(),
                    ..pushed
                },
                true,
                0
            )
            .is_gone
        );
        // A branch without upstream and without a commit of its own beyond develop.
        assert!(read_branch(&fields("spare", "", ""), true, 0).is_fresh);
    }

    #[test]
    fn counts_a_story_commits_from_the_branch_it_was_made_from_develop_otherwise() {
        assert_eq!(base_of(&fields("fix/aud-020-topic", "", FEATURE)), FEATURE);
        assert_eq!(
            base_of(&fields("fix/aud-020-topic", "[gone]", FEATURE)),
            "origin/develop"
        );
        assert_eq!(
            base_of(&fields("fix/aud-020-topic", "", "origin/fix/aud-020-topic")),
            "origin/develop"
        );
        assert_eq!(base_of(&fields("spare", "", "")), "origin/develop");
    }

    #[test]
    fn drops_the_rows_of_a_branch_no_other_branch_of_origin_holds() {
        let holders = |names: &[&str]| owned(names);

        assert_eq!(
            rows_state("chore/en-1-x", &holders(&["origin/develop"])),
            "merged"
        );
        assert_eq!(
            rows_state("chore/en-1-x", &holders(&["origin/feature/f99-x"])),
            "merged"
        );
        // Held by its own branch alone, or by none: never merged.
        assert_eq!(
            rows_state("chore/en-1-x", &holders(&["origin/chore/en-1-x"])),
            "dropped"
        );
        assert_eq!(
            rows_state("chore/en-1-x", &holders(&["origin/HEAD", "origin"])),
            "dropped"
        );
        assert_eq!(rows_state("chore/en-1-x", &[]), "dropped");
    }

    #[test]
    fn reads_a_line_of_for_each_ref() {
        assert_eq!(
            BranchFields::of_line("a|[gone]|origin/a"),
            fields("a", "[gone]", "origin/a")
        );
        assert_eq!(BranchFields::of_line("a||"), fields("a", "", ""));
        assert_eq!(BranchFields::of_line("a"), fields("a", "", ""));
    }

    #[test]
    fn reads_each_worktree_path_and_branch_a_detached_one_without_branch() {
        let porcelain = [
            "worktree /work/SVG editor",
            "HEAD 5f061278",
            "branch refs/heads/develop",
            "",
            "worktree /work/SVG editor/.claude/worktrees/us-061",
            "HEAD 9dea434e",
            "branch refs/heads/feat/us-061-font-list",
            "",
            "worktree /work/SVG editor/.claude/worktrees/review",
            "HEAD fee645b2",
            "detached",
            "",
        ]
        .join("\n");

        assert_eq!(
            worktrees_of(&porcelain),
            [
                Worktree {
                    branch: "develop".to_owned(),
                    path: "/work/SVG editor".to_owned()
                },
                Worktree {
                    branch: "feat/us-061-font-list".to_owned(),
                    path: "/work/SVG editor/.claude/worktrees/us-061".to_owned()
                },
                Worktree {
                    branch: String::new(),
                    path: "/work/SVG editor/.claude/worktrees/review".to_owned()
                },
            ]
        );
    }

    #[test]
    fn f18_ac8_removes_the_worktree_of_a_merged_or_deleted_branch_never_the_main_one_nor_a_detached_one()
     {
        let worktrees: Vec<Worktree> = [
            ("feat/us-061-font-list", "/main"),
            ("feat/us-061-font-list", "/main/.claude/worktrees/us-061"),
            ("chore/en-052-worktrees", "/main/.claude/worktrees/en-052"),
            (
                "chore/en-051-story-finish",
                "/main/.claude/worktrees/en-051",
            ),
            ("", "/main/.claude/worktrees/review"),
        ]
        .iter()
        .map(|(branch, path)| Worktree {
            branch: (*branch).to_owned(),
            path: (*path).to_owned(),
        })
        .collect();
        let local = owned(&["develop", "feat/us-061-font-list", "chore/en-052-worktrees"]);

        // US-061 merged, EN-051's branch deleted by hand, EN-052 still at work.
        assert_eq!(
            stale_worktrees(&worktrees, &owned(&["feat/us-061-font-list"]), &local),
            [
                "/main/.claude/worktrees/us-061",
                "/main/.claude/worktrees/en-051"
            ]
        );
    }

    #[test]
    fn f18_ac8_knows_a_worktree_a_running_process_works_in_or_in_one_of_its_folders() {
        let folders = working_folders(
            "p101\nn/main\np102\nn/main/.claude/worktrees/us-061/tools/hooks\np103\nn/main/.claude/worktrees/en-0520\nnot a folder\n",
        );

        assert_eq!(
            folders,
            [
                "/main",
                "/main/.claude/worktrees/us-061/tools/hooks",
                "/main/.claude/worktrees/en-0520"
            ]
        );
        assert!(is_in_use("/main/.claude/worktrees/us-061", &folders));
        assert!(is_in_use("/main/.claude/worktrees/en-0520", &folders));
        // A longer name beside it is not inside it.
        assert!(!is_in_use("/main/.claude/worktrees/en-052", &folders));
        assert!(!is_in_use("/main/.claude/worktrees/us-062", &[]));
    }

    #[test]
    fn reports_what_it_did_and_what_is_left() {
        let left = Left {
            local: owned(&["develop", "main"]),
            remote: owned(&["origin/HEAD", "origin", "origin/develop", "origin/main"]),
            stashes: 1,
            worktrees: 2,
        };

        assert_eq!(
            report(
                &owned(&["feat/a"]),
                &[(
                    "/w/b".to_owned(),
                    "a running process works in it".to_owned()
                )],
                &left
            ),
            "deleted: feat/a\nworktrees kept: /w/b (a running process works in it)\nlocal: develop, main\non GitHub: origin/develop, origin/main\nstashes: 1\nworktrees: 2"
        );
        assert!(
            report(&[], &[], &Left::default()).starts_with("deleted: none\nworktrees kept: none\n")
        );
    }
}
