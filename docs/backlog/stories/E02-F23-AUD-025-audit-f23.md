---
id: AUD-025
epic: E02
feature: F23
title: Audit F23
status: draft
points: 2
---

# E02 · F23 · AUD-025 — Audit F23

Audit F23 in depth and the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given F23's criteria, when the audit ends, then each is traced to a test titled `f23_acN_…` or `[F23.ACn] …`, broken once on purpose and seen failing.
- Given the cards C1 to C9, when audited, then each is replayed and the tool now does what was needed, or the card says why not.

## Tasks

One task = one commit, referenced as `AUD-025.Tn`.

- [ ] T1 — A to G. Criteria traced, cards replayed, mutation spot-checks; findings table (2 h)

## Findings
