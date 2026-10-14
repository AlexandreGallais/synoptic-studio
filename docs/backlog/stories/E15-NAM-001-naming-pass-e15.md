---
id: NAM-001
epic: E15
title: Naming pass over the code of E15
status: draft
points: 3
---

# E15 · NAM-001 — Naming pass over the code of E15

Before the review, go over the whole code base twice so that every calculation and every process that a field names is a function of that name, with its source (ADR-0049).

## Acceptance criteria

- Given the whole code base, when the first pass ends, then every operation or run of steps that a field names is a call to a function of that name with its `@see`, and each one left inline is listed with its reason.
- Given the result of the first pass, when the second pass ends, then what the first missed and the duplicates found are named or merged, unless merging changes a floating-point result.
- Given a golden file written before the first pass (`npm run golden:write`), when each pass ends, then `npm run golden:check` passes and no expected test value was edited.

## Tasks

One task = one commit, referenced as `NAM-001.Tn`.

- [ ] T1 — Golden file; list the unnamed calculations and their sources (1 h)
- [ ] T2 — First pass (3 h)
- [ ] T3 — Second pass and duplicates (2 h)
- [ ] T4 — Audit of both passes (1 h)
