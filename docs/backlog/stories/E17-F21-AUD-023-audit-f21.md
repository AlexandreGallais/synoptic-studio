---
id: AUD-023
epic: E17
feature: F21
title: Audit F21
status: ready
points: 2
---

# E17 · F21 · AUD-023 — Audit F21

Audit F21 in depth and the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given F21's criteria, when the audit ends, then each is traced to a Rust test titled `f21_acN_…` or a TypeScript test titled `[F21.ACn] …`, and each test is broken once on purpose and seen failing.
- Given each port, when audited, then its output was compared with the TypeScript tool's on recorded cases before the tool was removed, and the comparison is in its story.
- Given each crate F21 added, when audited, then its row in `docs/tooling/dependencies.md` holds its exact version, its age at the time it was added, its licence and its release notes; `cargo deny` finds no advisory; every package whose code or cases were ported is in `NOTICE` and `THIRD-PARTY-LICENSES.md` (ADR-0076).
- Given each tooling story, when audited, then it states its times before and after (ADR-0069), and the record of each tool's use (EN-102) shows no tool slower than its story stated.
- Given the project, when audited, then no duplicate, no copied code and no inconsistency between the tools, the docs, the ADRs and the agent's rules remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-023.Tn`.

- [ ] T1 — A. Independent review by the `auditor` subagent; criteria traced, mutation spot-checks (3 h)
- [ ] T2 — B to E. Sources, crates and licences, duplicates, consistency (2 h)
- [ ] T3 — F, G. Quality gates and times; findings table (1 h)

## Findings

| Check | Result | Evidence | Action |
| --- | --- | --- | --- |
