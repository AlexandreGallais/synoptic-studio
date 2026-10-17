---
id: AUD-021
epic: E17
feature: F19
title: Audit F19
status: ready
points: 1
---

# E17 · F19 · AUD-021 — Audit F19

Audit F19 in depth, the project in breadth ([audit.md](../../conventions/audit.md)).

## Acceptance criteria

- Given F19's stories merged, when the audit ends, then each criterion is traced to a test and the findings are fixed or logged.

## Tasks

One task = one commit, referenced as `AUD-021.Tn`.

- [x] T1 — Repair the pages a replacement string broke (E3-1, DE-01, DE-02)
- [x] T2 — Every rule reached by a scope, every scoped name a rule (V1)
- [x] T3 — Unused disable directives reported by oxlint (A-LOST-1)
- [x] T4 — Renames and the Rust toolchain in the docs build's inputs (AC8-1)
- [x] T5 — Criteria traced to tests (TRACE-1, V4, V17)
- [x] T6 — Tests the verifier found missing (V2, V3, V5 to V12, V18)
- [x] T7 — Licences of the ported code and copied cases (C3/C1-1)
- [x] T8 — Small code findings (A-API-1, E3-2, B1-1)
- [x] T9 — Documentation findings (DE-03 to DE-61, E1-1, E1-2)
- [ ] T10 — Findings table
