# ADR-0080 — The Git hooks in `.githooks/`, installed by `git-hooks install`

**Status**: Accepted (the agent, 2026-10-18, EN-113, from F21's table of dev packages, whose `husky` row asks the install to be named in an ADR; to be told to the Product Owner). Supersedes ADR-0078 where it names `.husky/` (decision 1 and its first consequence): the Git hooks are `.githooks/`.

## Context

- F21's table of dev packages (2026-10-17) rewrites husky: "a Rust `hooks install` sets the hooks folder once; the guard on `core.hooksPath` stays for the agent's commands, the install named in the ADR" ([F21](../backlog/features/E17-F21-agent-tools-in-rust.md)).
- EN-108 removed husky, lint-staged, commitlint and validate-branch-name: the hooks became tracked shell files of `.githooks/` that call the release binaries by their path through `.githooks/tool.sh`, and `git-hooks install` (`tools/git-hooks/src/install.rs`) sets `core.hooksPath` ([EN-108](../backlog/stories/E17-F21-EN-108-git-hooks-in-rust.md)). It left the ADR to the main thread.
- ADR-0078, written by EN-104 before EN-108, names `.husky/` as the Git hooks that call the binaries, and `.husky/pre-push` and `.husky/commit-msg` among its consequences.
- husky 9.1.7 set `core.hooksPath` to `.husky/_` and wrote a stub for each of 14 hook names there, in an ignored folder, each stub running `.husky/<name>` (`REF-HUSKY-INSTALL`).
- Git runs the hooks of the folder `core.hooksPath` names, and "a relative path is taken as relative to the directory where the hooks are run": the root of the working tree in a non-bare repository (`REF-GIT-HOOKS-PATH`). The setting lives in the common `.git/config`, so the main checkout and every linked worktree read the same value (`REF-GIT-WORKTREE`).
- A worktree whose hooks path was absolute ran the main checkout's `pre-push`, on another branch's files (EN-055).

## Decision

1. **The Git hooks are the tracked files of `.githooks/`**: `pre-commit`, `commit-msg` and `pre-push`, each a few lines of shell that source `.githooks/tool.sh` and call `target/release/<binary>`, through `cargo run --release -q -p <crate>` only when the binary is missing. No stub, no generated folder, no package.
2. **`git-hooks install` sets `core.hooksPath` to `.githooks`, relative**, once: at `npm install` (`prepare`, after the release build) and at `worktree:new`, and writes nothing when the value is already `.githooks`, nor outside a Git work tree. Relative, the path resolves in each checkout's own root, so each worktree runs its own branch's hooks.
3. **The setting is changed by nothing else**: `guard-bash` refuses the agent's commands that set or unset `core.hooksPath` (`git config`, `git -c`), and lets a read of it through (EN-113); `check:all` fails when a lane changed a key of `[core]` (F21.AC9).

## Reasons

| Option | For | Against |
| --- | --- | --- |
| A — keep husky | an install the npm ecosystem knows | a dev package and its stubs for one `git config`; husky's `HUSKY=0` a second way to switch the hooks off |
| B — `.githooks/` set by a Rust command, relative (chosen) | no package; the hooks are the tracked files themselves; each worktree runs its own | a branch without `.githooks/` (`develop` before F21's merge) runs no hook once the setting is switched (EN-108, For the main thread) |
| C — hooks copied into `.git/hooks/` | no setting at all | the common `.git/hooks/` holds one copy for every worktree: the absolute-path problem of EN-055 again, and a copy to refresh after each change |

## Consequences

- ADR-0078's decisions hold for `.githooks/`: its hooks call `target/release/<binary>`; `tests/check-targets.test.ts` fails on a hook run otherwise.
- `HUSKY=0` switches nothing off: `guard-bash` no longer refuses it and `git-sequences` no longer removes it (EN-113); `--no-verify`, `commit -n` and a change of `core.hooksPath` stay refused.
- `docs/tooling/git-workflow.md` (The hooks) describes the install; `tools/git-hooks/tests/install.rs` tests it.

## References

ADR-0021, ADR-0073, ADR-0078, `REF-GIT-HOOKS-PATH`, `REF-GIT-WORKTREE`, `REF-HUSKY-INSTALL`, [EN-108](../backlog/stories/E17-F21-EN-108-git-hooks-in-rust.md), [EN-113](../backlog/stories/E17-F21-EN-113-clean-up-of-e17.md), `docs/tooling/git-workflow.md`
