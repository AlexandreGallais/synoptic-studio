---
id: AUD-024
epic: E17
feature: F22
title: Audit F22
status: ready
points: 1
---

# E17 · F22 · AUD-024 — Audit F22

Audit F22 in depth and the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given F22's criteria, when the audit ends, then each is traced to a test titled `f22_acN_…` or `[F22.ACn] …`, broken once on purpose and seen failing.
- Given the release tool, when audited, then a release made from a copy of the repository's history gives the version and the notes ADR-0083 describes.

## Tasks

One task = one commit, referenced as `AUD-024.Tn`.

- [ ] T1 — A to G. Criteria traced, mutation spot-checks, consistency of the docs with ADR-0083; findings table (2 h)

## Findings
