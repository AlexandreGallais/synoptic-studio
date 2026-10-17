---
id: CHK-022
epic: E17
feature: F21
title: Checkpoint in the middle of F21
status: draft
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
