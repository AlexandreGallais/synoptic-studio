---
name: worker-brief
description: How a run's worker sets up its worktree, works a story, commits, takes the heavy lock and stops before the done commit (ADR-0072); preloaded by the story-worker and researcher agent types (EN-065).
user-invocable: false
---

# Worker brief (ADR-0072)

The `story-worker` and `researcher` agent types start with this brief and the `project-rules` digest preloaded, without `CLAUDE.md` nor `AGENTS.md` (EN-065); a `general-purpose` worker is given this file's path, `.claude/skills/worker-brief/SKILL.md` inside its worktree. The main thread's prompt names the story id, its feature branch and its worktree. A worktree subagent does not load `.claude/rules/` by itself: it reads them from this list.

## Setup

1. The main thread makes the worktree with `npm run worktree:new -- <ID>` (EN-052): `.claude/worktrees/<id>`, already on the story's branch from the feature branch on GitHub, `node_modules` linked, its own Husky hooks. Read this brief from inside it, never from the main checkout: the main checkout stays on `develop` during a run, and its copy may be older.
2. **Every Bash call starts with `cd "<worktree>" || exit 9`**: a subagent's calls start in the main checkout, whatever its prompt says (EN-049 wrote files there). Write and Edit take absolute paths inside the worktree; never `git -C`; never edit a file outside the worktree.
3. After that `cd`, one Git command per call, never chained with another Git command by `&&`, `;` or a pipe: Claude Code refuses compound commands in a worktree (549 refusals in E15). The frequent sequences are one call each (EN-100, [agent tools](../../../docs/tooling/agent-tools.md)): `npm run git:sync` fetches `origin` and rebases the story on its feature branch; `npm run git:commit -- -F <message file|-> [--push] <paths>` stages the files named, commits and pushes; each stops at the first failing step with Git's own message. First `npm run git:sync`.
4. One scratch folder of its own: `<scratchpad>/<story id>-<role>`.

## Read first

The rules of the area the story touches, from the worktree: `.claude/rules/library.md` (`src/`, `playground/`), `.claude/rules/docs.md` (Markdown), `.claude/rules/tooling.md` (tooling, tests, `tools/`); `docs/conventions/README.md` and the pages it lists for what the story touches; the story file, its feature file, and the ADRs, derivations, domain sections and `REF-*` rows it cites. A `general-purpose` worker also reads `AGENTS.md` and `CLAUDE.md`; a typed worker has their digest preloaded (`project-rules`) and opens `AGENTS.md` only for a section the digest does not cover.

## Work

- One export per file, named after it in kebab-case; folders by business domain (ADR-0043); one `@kind` and one `@see` to a verified source per function; every calculation named (ADR-0049); reuse existing functions, never copy.
- **Every new script, hook or agent tool is written in Rust** (ADR-0073): a crate of `tools/`, without a crates.io dependency other than the reviewed ones of `[workspace.dependencies]` (`docs/tooling/dependencies.md`, Crates: Oxc's, EN-072), `[lints] workspace = true` in its `Cargo.toml` (the root workspace checks it in `check:rust`), run by `cargo run --release -q -p <crate>`; the existing TypeScript tools may still be edited until F21 rewrites them.
- Tests first: hand-computed examples with the computation in a comment, degenerate cases, a property test for each `math` and `geometry` function; a test proving a criterion is titled `[F18.ACn] …`; 100 % of `src/`.
- **Edge cases and mutations** (ADR-0064): write the story's edge cases into its tests; break each new test once on purpose and see it fail.
- One commit per task, footer `Refs: <ID>.Tn`, scope from `commitlint.config.ts`, the attribution line, the message in a file of the scratch folder (or a here-document on stdin, `-F -`), made by `npm run git:commit -- -F <file> <paths>`; a refused commit leaves its files staged.
- Write the current step, one short line in French, to `.git/metrics/step` of the main checkout (`git rev-parse --git-common-dir` gives its folder): the status line shows it.
- Before each commit: `npm run fix` (the type errors come with the lint, EN-091), the touched tests (`npx vitest run <paths>`). Edits across files: `npm run replace` or the Edit tool, never a Python script.
- Docs in the same task as the code: domain, derivations, references, playground.
- **Shared files** (ADR-0065): never edit the indexes of `docs/backlog/`, the logs (`journal.md`, `improvements.md`, `feedback.md`), the feature file nor the checklists (`playground/checklists/epic-*`): the pre-push hook refuses the branch and names the file (`tools/shared-files`, EN-050). Write the rows meant for a log in the story file, under `## For the main thread`, beneath `### Journal`, `### Improvements` or `### Feedback`, each row as the log writes it: `story:finish` moves them into the log at merge. Rows only, one per line: under `### Journal` a bullet opening with its tag (``- `#run` EN-050: …``); under `### Improvements` and `### Feedback` a table row of the log's columns, its first cell the date (`| 2026-10-17 | EN-050 | … | tool |`), with no header nor separator. Nothing to write: leave the heading out, never `- None.`. Any other line stays in the story file and is named by `move-rows`. Anything else for the main thread goes in that section, outside those headings.
- Never `npm run golden:write` in a worktree: the golden file is shared by every worktree.

## Heavy commands (ADR-0072)

One heavy command at a time across every worktree: `check:all`, `npm run test`, `golden:check`, `docs:build`, and `git push` (its pre-push runs `check:all`). They take the lock `.git/heavy.lock` themselves (EN-047, `docs/tooling/commands.md`): run them as they are, in the background when they may last over a minute (`CLAUDE.md`, While working); one waiting prints which process holds the lock. Never take the lock by hand with `mkdir`, nor wrap a command in such a lock: the command waits for a lock it cannot see the owner of, and a `git push` inside it waits forever for its own pre-push `check:all`, and every other worktree with it.

Between heavy commands, run single test files only (`npx vitest run <paths>`); a full run is `npm run test`, never `npx vitest run` without a path.

## Limits

- Push once the tasks are done: `--push` on the last task's `npm run git:commit`, or `git push -u origin HEAD`; never `--no-verify`, never a force push (after a `git:sync` that moved a pushed branch, `--push-with-lease`, a story branch only).
- Stop before the pull request and before the `done` commit: the main thread makes that commit in the worktree (`backlog:status`, `Closes:`, the `Rule-docs:` line when needed), then runs `story:finish`: the `done` commit writes the shared files — the stories' index and the feature's table — which only the main thread writes, at merge (ADR-0065, decision 4; the pre-push check `refuse-shared-files` refuses them on a story branch, EN-050).
- A story that changes a rule in `docs/domain/` brings in its tasks the Q row of `docs/domain/README.md`, the decisions to confirm of its feature's VAL and its guide page; the `done` commit's `commit-msg` hook refuses the story otherwise (`tools/rule-docs`, EN-059). One the change does not affect: say so in the final report, with its reason, for the footer line `Rule-docs: none affected, <why>` of the `done` commit.
- A business rule missing, or code, domain and ADRs that contradict each other: stop and report the question.
- Final report, brief: branch, commits, files, decisions taken, evidence, open points.
