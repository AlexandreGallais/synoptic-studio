---
id: AUD-010
epic: E14
feature: F11
title: Audit F11
status: ready
points: 2
---

# E14 · F11 · AUD-010 — Audit F11

Check that F11 changed the structure and nothing else, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the code before and after F11, when audited, then no test value changed and no behavior moved.
- Given the project, when audited, then no duplicate helper, writer or fixture remains.

## Tasks

One task = one commit, referenced as `AUD-010.Tn`.

- [ ] T1 — Independent review by the `auditor` and `verifier` agents; findings table (2 h)

## Findings

| Check | Result | Evidence | Action |
| ----- | ------ | -------- | ------ |
