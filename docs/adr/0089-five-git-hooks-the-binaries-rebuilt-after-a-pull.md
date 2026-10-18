# ADR-0089 — Five Git hooks, the release binaries rebuilt after a pull too

**Status**: Accepted (the agent, 2026-10-18, AUD-031, from F29's audit; to be told to the Product Owner). Supersedes ADR-0080, decision 1, for its list of hooks: `.githooks/` holds five. Supersedes ADR-0078, decision 2, for its list of the rebuilds, and its third consequence, for when the main checkout's binaries are rebuilt: a pull that changed a Rust input rebuilds them too. The rest of both ADRs stands.

## Context

- ADR-0080, decision 1, names the Git hooks `.githooks/` holds: `pre-commit`, `commit-msg` and `pre-push`.
- ADR-0078, decision 2, rebuilds a checkout's release binaries at `prepare`, after `git:sync` and after `story:finish`; its third consequence says the hooks run the main checkout's binaries, "rebuilt when `story:finish` brings its base up to date".
- A pull request merged on GitHub and pulled into the main checkout left its binaries older than their sources: at REV-004, `target/release/guard-bash` predated EN-115 (EN-131).
- EN-131 added `post-merge` and `post-rewrite` to `.githooks/`: after a merge, a fast-forward or a rebase whose files changed since `ORIG_HEAD` include a Rust input (`RUST_INPUTS`, EN-130), `git-hooks` rebuilds the release binaries and names those it wrote again. `post-merge` runs after `git merge`, a `git pull` among them, and cannot change its outcome; `post-rewrite` runs after `git rebase` and `git commit --amend` (`REF-GITHOOKS`).
- The CI into `main` holds six lanes since EN-130, the Rust coverage a job of its own beside the Rust lane; ADR-0087's consequence counts five.

## Decision

1. **The Git hooks are the five tracked files of `.githooks/`**: `pre-commit`, `commit-msg`, `pre-push`, `post-merge` and `post-rewrite`, each as ADR-0080, decision 1, describes the first three.
2. **A stale binary is met by five rebuilds** of the checkout's release binaries, one Cargo build (`RELEASE_BUILD` of `tools/stale-binaries`, as `npm run rust:build`): at `prepare`, at `worktree:new`, after `git-sequences sync`, after `story:finish` in the checkout of the base, and after a merge, a fast-forward or a rebase that changed a Rust input (`post-merge`, `post-rewrite rebase`). A failed build in a post hook is printed with the command to run again, and the hook exits 0: the pull is done.

## Reasons

- The list of the hooks and the list of the rebuilds are each read as complete; extended without an ADR, ADR-0080 and ADR-0078 would each name one less than the repository holds.

## Consequences

- The main checkout's binaries, which the hooks run, are rebuilt after `story:finish` and after each pull that changed a Rust input. Two pulls build nothing: the one that brings the post hooks, run with a `git-hooks` built before them, and a merge stopped on a conflict and finished by `git commit`, which runs no `post-merge`; after either `npm run rust:fresh` and the `session-state` hook name a stale binary, and `npm run rust:build` rebuilds it (`docs/tooling/git-workflow.md`).
- ADR-0087's consequence reads six lanes: `main`'s ruleset requires the six lanes' jobs, `Rust coverage` among them, beside the history job, `Dependency review` and `Analyze` (`docs/tooling/ci.md`).
- `tools/git-hooks/tests/rebuild.rs` tests the post hooks; `[F24.AC1]` reads the lanes against the CI's matrix and the checks `main` requires.

## References

ADR-0078, ADR-0080, ADR-0087, `REF-GITHOOKS`, [EN-131](../backlog/stories/E17-F29-EN-131-binaries-rebuilt-after-a-pull.md), [AUD-031](../backlog/stories/E17-F29-AUD-031-audit-f29.md), `docs/tooling/git-workflow.md`
