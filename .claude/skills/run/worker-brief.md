# Worker brief (ADR-0063)

The main thread gives this file's path to each worker, with the story id, its feature branch and its worktree. A worktree subagent does not load `.claude/rules/` by itself: it reads them from this list.

## Setup

1. The worktree is already the working directory: never `cd` into it. Its name is the story's (`npm run worktree:new -- <ID>` once EN-052 is built).
2. `git fetch origin`, then `git switch -c <type>/<id>-<topic> origin/<feature branch>` — one plain Git command per call, never chained with `&&`, `;` or a pipe: Claude Code refuses compound commands in a worktree (549 refusals in E15).
3. If `node_modules` is missing: `ln -s "<main checkout>/node_modules" node_modules`.
4. One scratch folder of its own: `<scratchpad>/<story id>-<role>`.

## Read first

`AGENTS.md`, `CLAUDE.md`, `.claude/rules/library.md` (`src/`, `playground/`), `.claude/rules/docs.md` (Markdown), `.claude/rules/tooling.md`, `docs/conventions/README.md` and the pages it lists for what the story touches, the story file, its feature file, and the ADRs, derivations, domain sections and `REF-*` rows it cites.

## Work

- One export per file, named after it in kebab-case; folders by business domain (ADR-0043); one `@kind` and one `@see` to a verified source per function; every calculation named (ADR-0049); reuse existing functions, never copy.
- Tests first: hand-computed examples with the computation in a comment, degenerate cases, a property test for each `math` and `geometry` function; a test proving a criterion is titled `[F18.ACn] …`; 100 % of `src/`.
- **Edge cases and mutations** (ADR-0064): write the story's edge cases into its tests; break each new test once on purpose and see it fail.
- One commit per task, footer `Refs: <ID>.Tn`, scope from `commitlint.config.ts`, the attribution line; a refused commit leaves its files staged.
- Before each commit: `npm run fix`, `npx tsc --noEmit`, the touched tests (`npx vitest run <paths>`). Edits across files: `npm run replace` or the Edit tool, never a Python script.
- Docs in the same task as the code: domain, derivations, references, playground.
- **Shared files** (ADR-0065): never edit the stories index, the logs (`journal.md`, `improvements.md`, `feedback.md`), the feature file nor the checklist; write the rows meant for them under `## For the main thread` in the story file.
- Never `npm run golden:write` in a worktree: the golden file is shared by every worktree.

## Heavy commands (ADR-0063)

One heavy command at a time across every worktree: `check:all`, a full Vitest run, `golden:check`, the docs build, and `git push` (its pre-push runs `check:all`). Until EN-047 puts the lock in the tools, take it by hand, in a single call, in the background if it may last over 2 minutes:

```sh
L="$(git rev-parse --git-common-dir)/heavy.lock"; until mkdir "$L" 2>/dev/null; do sleep 10; done; <heavy command>; rc=$?; rmdir "$L"; exit $rc
```

Between heavy commands, run single test files only.

## Limits

- Push with `git push -u origin HEAD` once the tasks are done; never `--no-verify`, never a force push.
- Stop before the pull request and before the `done` commit.
- A business rule missing, or code, domain and ADRs that contradict each other: stop and report the question.
- Final report, brief: branch, commits, files, decisions taken, evidence, open points.
