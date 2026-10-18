---
id: CHK-022
epic: E17
feature: F21
title: Checkpoint in the middle of F21
status: done
points: 1
---

# E17 · F21 · CHK-022 — Checkpoint in the middle of F21

Pause once the agent's own tools run in Rust (EN-102 to EN-107), before the packages of `package.json` are rewritten: bring the docs and methods up to date, with a light evolvability check (ADR-0028).

## Acceptance criteria

- Given the stories merged since SP-023, when the checkpoint ends, then `docs/tooling/`, `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, the skills and the worker brief name the Rust binaries, and no page names a TypeScript script or hook that is gone.
- Given the record of each tool's use (EN-102), when the checkpoint reads it over the run so far, then the slowest tools and those that failed most are listed, each with an action or a reason to leave it.
- Given the times stated by the stories so far, when the checkpoint ends, then one table in the feature's research note holds each tool's time before and after.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.

## Tasks

One task = one commit, referenced as `CHK-022.Tn`.

- [ ] T1 — Docs, rules and skills in step with the binaries; the tools' uses read; the times gathered (1.5 h)
- [ ] T2 — Light evolvability check; improvement log (0.5 h)

## Consistency after the ports

Checked on 2026-10-18 over `AGENTS.md`, `CLAUDE.md`, `.claude/` (rules, skills, agents), `docs/conventions/`, `docs/tooling/`, `docs/guide/` and the configuration files, after SP-023 and EN-102 to EN-107 were merged into F21.

| Check | How | Result |
| --- | --- | --- |
| Scripts and hooks removed | every `scripts/<name>.ts` and `.claude/hooks` outside the history (ADRs, research notes, the backlog, the logs, the retrospectives) | `tsconfig.json`, `vite.config.ts` and `docs/conventions/typescript.md` still named `.claude/hooks`: dropped. F18's guide page gave `.claude/hooks/status-line.test.ts` as AC9's evidence: now `tools/hooks/tests/status.rs`, which carries its cases. The Rust modules' "rewritten from" lines, `docs/tooling/metrics.md` on `session-report.ts` and `docs/conventions/rules.md`'s measure of those folders' history name them as the past: kept |
| Lint globs of `.claude/hooks` | `.oxlintrc.json` (`.claude/hooks/**/*.ts`), `tools/lint/src/scopes.rs` (`TOOLING_FILES`, the `node-run` scope), `docs/conventions/lint.md` | left to EN-113: each shares its override or scope with `scripts/`, which still holds two tools, and EN-113's criteria name these settings, removed with what they cover; a glob without a file matches nothing meanwhile |
| Agent tools called through `npm run` | every `npm run` and `cargo run` of the run, story, audit and retro skills, the worker brief, `project-rules`, the clerk agent, `CLAUDE.md` and `.claude/rules/`, against ADR-0078 and the table of `docs/tooling/agent-tools.md` | fixed, each now `target/release/<binary>`: `worktree:new`, `backlog:status`, `git:commit`, `run:start`, `run:end`, `run:should-stop`, `epic:check`, `val:prepare`, `activity`, `branches:clean`, `records:save`, `metrics`, `metrics:session`, `replace`. Kept, as ADR-0078 says: `check:all`, `story:finish`, `mutate`, `fix`, `test`, `golden:*`, and `dev`, `docs:dev` for the Product Owner. `project-rules` and the audit skill already fit |
| The binaries' table | `docs/tooling/agent-tools.md`'s table against the binaries of `target/release` | `metrics`, `session-report` and `activity` had no row: added. The page still called how the binaries are called "EN-104's question": now ADR-0078 |
| Pages for people | `docs/tooling/*.md`, the Commands table of `AGENTS.md` | left: they name `npm run` scripts, the form for people (ADR-0078, decision 1); the last row of `AGENTS.md`'s table already sends the agent to the binaries |
| EN-108, merged during the checkpoint | `husky`, `.husky`, `lint-staged`, `commitlint`, `validate-branch-name` outside the history, the references, `NOTICE`, `THIRD-PARTY-LICENSES.md` and `tools/git-hooks` | the pages name the packages only as what `tools/git-hooks` ports. Left to EN-113: ADR-0078, accepted, names `.husky/pre-push` and `.husky/commit-msg` (a new ADR for the hooks' install in `.githooks/` is EN-113's); `guard-bash` still refuses `HUSKY=0`, now without effect, and its two messages name lint-staged and commitlint (`tools/hooks/src/guard.rs`), which `CLAUDE.md`, `project-rules` and `docs/tooling/agent.md` describe as it is |
| SP-023's question | research note 0025, For the Product Owner, "waits for the answer" | marked decided, in ADR-0078 |

The times: research note 0025, Each tool's time before and after, at the checkpoint, holds one table of each tool's time before and after its port (stories EN-102 to EN-107), and the uses recorded so far, the slowest and the failing tools each with an action or the reason to leave it.

## Evolvability, light

| Question | Answer |
| --- | --- |
| Size | F21 changed no file of `src/` nor `playground/`; `tools/` holds 20 crates, about 113,000 lines of Rust, 73,000 of them `lint`'s; `scripts/` is down to `default-font-metrics` and `tool-versions` (EN-109's). `check:all` 57.0 s on this branch (load 19.6; a first run 106.8 s at load 22), against a median of 60 s at F20's end (14 passes on 2026-10-17, 38.5 to 100.6 s, from the transcripts) and of 60.3 s over F21's 15 passes so far (52.8 to 112.9 s, from `tool-uses.jsonl`): unchanged. `check:rust` is now the longest lane, 56.7 s, and runs on every change: an improvement row |
| Tools | the three rows of Considered for later (`dependencies.md`): no npm publication, the public API not stable, Stryker's issue not read again. Nothing yet |
| Fitness functions | ADR-0078's rule that the agent's pages call the binaries is only written: T1 fixed 34 lines by hand. A test could hold it: an improvement row |
| Patterns | six stories timed their tools with a timer script of their own in the scratch folder (SP-023, EN-102, EN-103, EN-104, EN-106, EN-107), the same method each time: a tool of the workspace, an improvement row |
| Agent configuration | `AGENTS.md` and `CLAUDE.md` 154 lines together, under 200; rules, skills and agents name the binaries (T1). Guards: `guard-bash` cuts a command at a quoted `\|`, and `check:all`'s `.git/config` guard fails when another worktree pushes with `-u`: two improvement rows |
| Process | the checkpoint ran after the six ports, as planned; the uses record (EN-102) made the tools' review one table rather than a search of the transcripts. Nothing yet |

## For the main thread

- Commits of this story are docs and two configuration files; `Rule-docs:` none affected, no rule of `docs/domain/` changed.
- **Test cards**: none so far. F21 has no user story; VAL-022 validates it on its demo page and checklist.
- **Decisions to tell the Product Owner at the end** (taken by the agent under ADR-0069; for VAL-022's demo page):
  - ADR-0078 (EN-104): the hooks, the status line, the Git hooks and the agent call `target/release/<binary>` instead of `npm run` and `cargo run`, about 150 ms saved a call; a binary is rebuilt at `prepare`, after `git:sync` and after `story:finish`, and `rust:fresh` names a stale one. `.claude/settings.json` allows `Bash(target/release/*)` (713c6328), so any release binary of the checkout runs without a prompt.
  - ADR-0077 (EN-106): the library's file is named by `AGENT_LIBRARY_FILE`, no longer `LIBRARY_PATH`, which Apple's `python3` sets to `/usr/local/lib`; a `LIBRARY_PATH` set for the library on their side is renamed, and an older checkout still reads the old name.
  - EN-104: after `cd "<worktree>" || exit <n>;`, `guard-bash` lets one Git command through piped into read-only filters (`head`, `tail`, `grep`, `wc`, `cut`, `sort`, `uniq`, `sed -n`, `awk` without `system`); a second command chained is still refused.
  - EN-102: `run-totals status`, the status line's poll once a second, is not recorded (decided at merge).
- **For EN-113**: a new ADR for the Git hooks' install in `.githooks/`, superseding ADR-0078's naming of `.husky/`; `HUSKY=0` and the names lint-staged and commitlint in `guard-bash`; the `.claude/hooks` globs of `.oxlintrc.json`, `tools/lint/src/scopes.rs` and `docs/conventions/lint.md`.
- **check:all, this checkpoint's second run** failed only on the `.git/config` guard: `branch.chore/en-108-git-hooks-in-rust.merge` changed from the feature branch to the story's own while EN-108 pushed beside it; every lane passed.

### Improvements

| 2026-10-18 | CHK-022 | The run's most frequent finding, 92 since 2026-10-17 (`target/release/metrics --since 2026-10-17`, Findings): `A worktree refuses a chained Git command`, five of them met by this checkpoint. Two causes would go: `guard-bash` cuts a command at a `\|` inside quotes, so `git grep -E "a\|b"` is refused as a chain; and the Git commands chained most are reads (`status`, `diff`, `show`, `log`, about half the refusals in the transcripts), which one call printing a worktree's state would answer: `git-sequences state`, the branch, its base, `status --short`, `diff --stat` and the commits ahead | tool |
| 2026-10-18 | CHK-022 | The run's second finding, `@typescript-eslint/naming-convention` 77 times since 2026-10-17, with the TypeScript errors after it (`TS2345` 35, `TS2322` 25): all in the TypeScript tools edited during F20, and one lint finding left since 2026-10-18 (UTC); they go with the ports and EN-113, so RET-004 checks that the TypeScript findings of E17's last features are none | process |
| 2026-10-18 | CHK-022 | The run's third finding, the pedantic Clippy lints of the ports (`doc_markdown` 37, `assert_is_empty` 34, `case_sensitive_file_extension_comparisons` 11, `format_push_string` 9 since 2026-10-17), each met after the code was written: `npm run fix` runs no Clippy nor `cargo fmt`; a step `cargo clippy --fix` in it would write the fixes Clippy can apply and report the rest before the commit rather than at `check:rust` | tool |
| 2026-10-18 | CHK-022 | `check:all`'s `.git/config` guard (EN-103) failed this checkpoint's run of 57 s: EN-108's `git push -u` beside it rewrote its branch's `merge` line, a change a parallel worktree makes on every first push. Comparing the file without the `[branch "…"]` sections' `remote` and `merge` lines would keep the guard on what a test may break | tool |
| 2026-10-18 | CHK-022 | `check:rust` is now `check:all`'s longest lane (56.7 s at load 19.6) and runs on every change, this checkpoint's docs included; `docs:build` already runs only when the docs change: a lane run only when `tools/`, `Cargo.toml`, `Cargo.lock` or the toolchain changed would save it on the stories without Rust | tool |
| 2026-10-18 | CHK-022 | `worktree-new` takes 33.6 s (median of 5 recorded uses), most of it building every release binary in the new worktree from nothing; a copy of the main checkout's `target/release` (APFS `cp -c`) before an incremental build may take seconds: to measure | tool |
| 2026-10-18 | CHK-022 | ADR-0078's rule that the agent's pages call the binaries is only written, and this checkpoint fixed 34 lines by hand: a test failing when a skill, an agent, `CLAUDE.md` or a rule names `npm run <script>` for a script of `docs/tooling/agent-tools.md`'s table of binaries would hold it | tool |
| 2026-10-18 | CHK-022 | Six stories of F21 timed their tools with a timer script written again in their scratch folder, the same method (N calls after a warm-up, median and mean, the load average): a `bench` binary of the workspace, its output a Markdown row, would make each story's Measures one call and the same method by construction | tool |
