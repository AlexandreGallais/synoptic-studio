---
id: NAM-000
epic: E00
title: Naming pass over the code of E00
status: draft
points: 3
---

# E00 · NAM-000 — Naming pass over the code of E00

Before the review, go over the whole code base twice so that every calculation and every process that a field names is a function of that name, with its source (ADR-0049).

## Acceptance criteria

- Given the whole code base, when the first pass ends, then every operation or run of steps that a field names — mathematics, geometry, physics, drafting, industry, the laboratory — is a call to a function of that name with its `@see`, and each one left inline is listed with its reason.
- Given the result of the first pass, when the second pass ends, then what the first missed and the duplicates found are named or merged, unless merging changes a floating-point result.
- Given a golden file of results written before the first pass, when each pass ends, then every result is identical bit for bit and no expected test value was edited.

## Tasks

One task = one commit, referenced as `NAM-000.Tn`.

- [ ] T1 — Golden file of results; list, folder by folder, the unnamed calculations and their sources (1 h)
- [ ] T2 — First pass: name them, the `math-expert` checking each name (3 h)
- [ ] T3 — Second pass on the result, and the duplicates; golden file compared (2 h)
- [ ] T4 — Audit of both passes by the `auditor`; the list of what stays inline (1 h)
