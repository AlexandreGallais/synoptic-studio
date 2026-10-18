---
id: AUD-025
epic: E17
feature: F23
title: Audit F23
status: ready
points: 1
---

# E17 · F23 · AUD-025 — Audit F23

Audit F23 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F23's criteria, when the audit ends, then each is traced to a test titled `[F23.ACn] …` or `f23_acn_…`, broken once on purpose and seen failing.
- Given the cards C1, C3, C4 and C6 and those of `worktree-new` from the main checkout and of an epic's `SP`, when audited, then each is replayed and the tool now does what was needed, or the card says why not.

## Tasks

- [ ] T1 — Criteria traced, cards replayed, findings fixed (1 h)

One task = one commit, referenced as `AUD-025.T1`.
