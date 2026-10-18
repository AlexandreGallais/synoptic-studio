---
id: CHK-022
epic: E17
feature: F21
title: Checkpoint in the middle of F21
status: ready
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
| SP-023's question | research note 0025, For the Product Owner, "waits for the answer" | marked decided, in ADR-0078 |

The times: research note 0025, Each tool's time before and after, at the checkpoint, holds one table of each tool's time before and after its port (stories EN-102 to EN-107), and the uses recorded so far, the slowest and the failing tools each with an action or the reason to leave it.
