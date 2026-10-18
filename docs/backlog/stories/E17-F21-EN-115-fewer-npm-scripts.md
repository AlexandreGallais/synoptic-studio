---
id: EN-115
epic: E17
feature: F21
title: The npm scripts kept to what a person runs and what calls them
status: done
points: 2
---

# E17 · F21 · EN-115 — The npm scripts kept to what a person runs and what calls them

Decided on 2026-10-18: `package.json` held 52 scripts, of which the Product Owner runs a few; the agent already calls every Rust tool by its binary, `target/release/<binary>` (ADR-0078), so the scripts kept only for it are clutter. `package.json` keeps the scripts a person runs (`dev`, `build`, `preview`, `docs:dev`, `fix`, `check:all`, `test`) and those that something calls by their name — npm's lifecycle, a Git hook, the CI, a lane of `check:all`, a tool, or another script kept — each found by a search, never assumed. Every other script goes, and the pages and rules name the binary instead. It joins F21, done, since it changes how F21's tools are called after its validation.

## Acceptance criteria

- Given `package.json`, when the story ends, then each script left is one a person runs or one with a caller named in the inventory below, and every other script is gone.
- Given a page, a rule, a skill, a tool's message or doc comment, or a test that is not a historical record, when it names a removed script, then it names the binary or the command that replaces it instead; the journal, the retrospectives, the done stories and the ADRs' bodies stay as they were written.
- Given `docs/tooling/agent-tools.md`, when the story ends, then it lists the binaries the agent calls, without a column of scripts.
- Given an agent typing `npm run <removed script>`, when `guard-bash` refuses it, then its reason says that a tool is called by its binary, not only that a rebase may bring the script.
- Given `golden:write` removed, when a worktree runs its Vitest command (`--mode golden-write`), then `guard-bash` still refuses it.
- Given the change, when `check:all` runs, then it is green.

## Tasks

One task = one commit, referenced as `EN-115.Tn`.

- [x] T1 — The story file (15 min)
- [x] T2 — The inventory of the 52 scripts and their callers (30 min)
- [x] T3 — The scripts without a caller removed; the code, the tests, the pages and the rules on the binaries; ADR-0082 (2 h)

## Inventory

Searched on 2026-10-18 in `package.json`, `.githooks/`, `.github/`, `.mcp.json`, `.claude/`, the sources of `tools/` (`check-all`'s `LANES` and `BEFORE_LANES`, `scripts::run_script`, the hooks' messages) and `tests/`: `npm run <name>`, `npm run --silent <name>`, and the name quoted in code. A mention in a page or a doc comment is no caller: it names the command for a reader, and is rewritten when the script goes.

`check-all` runs its lanes' scripts by name, reading their commands from `package.json` and running them in a shell without npm, the heavy lock it holds taken off (`command_of`); `backlog-status` and `story-finish` run `backlog:index` the same way, and their tests give a temporary repository a `backlog:index` of its own. Those names are callers.

### Kept

| Script | Caller |
| --- | --- |
| `dev`, `build`, `preview`, `docs:dev`, `fix`, `test`, `check:all` | the Product Owner; `check:all` also the `pre-push` hook, `ci.yml`, `epic-check` and the pull request template; `build` also `ci.yml`; `test` also `check-all`'s `test` lane |
| `prepare` | npm's lifecycle, at `npm install` and `npm ci` |
| `rust:build` | `prepare`, `ci.yml`, `release.yml`, `check-all` before its lanes (`BEFORE_LANES`) |
| `golden:check` | the `pre-push` hook, when the golden file is written |
| `docs:build`, `playground:build` | `release.yml`; `check-all`'s docs lane |
| `deps:audit` | `build`; `check-all`'s lane of the repository's checks |
| `lint`, `format:check`, `lint:secrets`, `deps:unused`, `deps:outdated`, `check:rust` | `check-all`'s lanes |
| `rust:advisories`, `rust:coverage`, `rust:fresh` | `check:rust` |
| `docs:api` | `docs:dev`, `docs:build` |
| `backlog:index` | `backlog-status` and `story-finish` (`scripts::run_script`) |
| `story:finish` | the `check-handback` hook's message names it as the main thread's next command; it runs `story-finish` under the merge lock, and `.claude/settings.json` allows neither `target/release/story-finish` nor `heavy-lock`, which asks at each call (AUD-023) |
| `mutate` | the audit skill; it runs `mutate` under the heavy lock, and neither its binary nor `heavy-lock` is allowed without asking |

`story:finish` and `mutate` have no caller of the list the request names: they stay because calling their binary instead takes no lock, and calling it under `heavy-lock` asks the Product Owner each time; changing the permissions is not the agent's to do (For the main thread).

### Removed

| Script | Callers found | Called instead |
| --- | --- | --- |
| `check` | none: `check:all` runs the same checks in lanes | `npm run check:all` |
| `test:watch` | none | `npx vitest` |
| `typecheck` | none: `fix` and `lint` report the type errors (EN-091) | `npx tsc --noEmit` |
| `format` | none: `fix` formats, then lints | `npm run fix` |
| `docs:preview` | none | `npx vitepress preview docs` |
| `metrics`, `metrics:session`, `activity` | none | `target/release/metrics`, `target/release/session-report`, `target/release/activity` |
| `context:size` | none | `target/release/context-size` |
| `page` | none | `target/release/page` |
| `library` | none: `.mcp.json` starts `library-mcp` by Cargo | `target/release/library` |
| `worktree:new` | none | `target/release/worktree-new` |
| `git:sync`, `git:commit` | none | `target/release/git-sequences sync`, `commit` |
| `run:start`, `run:end`, `run:should-stop` | none | `target/release/run-totals start`, `end`, `should-stop` |
| `branches:clean` | none | `target/release/branches-clean` |
| `records:save` | none | `target/release/records-save` |
| `replace` | none | `target/release/replace` |
| `golden:write` | none: `guard-bash` refuses it in a worktree, by its name | `npx vitest run tests/golden-results.test.ts --mode golden-write`, which `guard-bash` refuses in a worktree by its mode |
| `backlog:status` | none | `target/release/backlog-status` |
| `val:prepare` | none | `target/release/val-prepare .` |
| `release:prepare` | none | `target/release/release prepare` |
| `epic:check` | none | `target/release/epic-check` |
| `font:metrics` | none | `node scripts/default-font-metrics.ts` |

26 kept, 26 removed.

## Measures (ADR-0069)

None: no command runs faster or slower. The agent already called each removed script's binary (ADR-0078); `check:all` runs the same lanes with the same commands; `guard-bash` reads a `package.json` of 26 scripts instead of 52 and compares the words of a command instead of one prefix, both well under the hook's 3 ms.

## For the main thread

- ADR-0082 is the agent's decision, to be told to the Product Owner: `story:finish` and `mutate` stay as scripts, though only the agent runs them, since calling their binary takes no lock and calling `heavy-lock` asks each time; dropping them too needs a permission rule for their binaries in `.claude/settings.json`, which is the Product Owner's.
- Option B of ADR-0082, 9 more scripts removed by moving the lanes' commands into `check-all`, is left: it would write the formatter's and linters' targets twice and rewrite F18's and F19's criterion tests.
- Left as written, as records: the validated checklists `playground/checklists/epic-17-f18-lines.ts` to `epic-17-f21-lines.ts` and their test `playground/checklist/checklist-page.test.ts`, which name scripts EN-115 removed; the journal, the retrospectives, the done stories, the ADRs' bodies and the research notes.
- The story auto-merge into `develop` of a story outside a feature (the coordinator's extra task): not done. The edit of `.github/workflows/story-automerge.yml` widening its condition was refused by the permission check as a CI bypass, so neither the workflow, nor ADR-0072's flow, nor `AGENTS.md`'s Merge row changed; the Product Owner makes or allows that change.

### Feedback

| 2026-10-18 | The Product Owner was asked to merge by hand into `develop` EN-114 (#358), the agent's own tooling story, which `story-automerge.yml` does not merge: it accepts into `develop` only `feat/val-`, `nam`, `rev` and `ret` branches. | Handle a tooling story outside a feature alone, merged by GitHub like the others. | Not yet: EN-115's change to the workflow was refused by the permission check as a CI bypass; the Product Owner makes or allows it. |
