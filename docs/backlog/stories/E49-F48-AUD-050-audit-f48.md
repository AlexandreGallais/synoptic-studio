---
id: AUD-050
epic: E49
feature: F48
title: Audit F48
status: ready
points: 3
---

# E49 · F48 · AUD-050 — Audit F48

Full check of F48 in depth and of the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill. A tooling feature: no mathematics; the depth is in the concurrency, the snapshot's safety and the privacy of the synced folder's path.

## Acceptance criteria

- Given the crate `tools/knowledge`, when audited, then each function survives a mutation spot-check, and two writers, a snapshot under a writer and a newer schema are each tried for real, not only in the tests.
- Given the sources, when audited, then every `REF-*` F48 cites is re-verified and dated.
- Given the repository, when audited, then no file, test, commit or log holds the synced folder's path or any personal information, and no duplicate with `tools/library`'s store is left unrecorded.

## Tasks

One task = one commit, referenced as `AUD-050.Tn`.

- [ ] T1 — A. Independent review by the `auditor` subagent; mutation spot-checks; the real tries (1 h 30)
- [ ] T2 — B to E. Sources, provenance, duplicates, consistency; the fixes (1 h)
- [ ] T3 — F to H. Quality gates, what the next feature needs, the findings table (30 min)

## Findings
