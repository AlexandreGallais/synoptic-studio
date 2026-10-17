---
id: NAM-000
epic: E00
title: Naming pass over the code of E00, and the tooling and tests pass
status: draft
points: 3
---

# E00 · NAM-000 — Naming pass over the code of E00, and the tooling and tests pass

Before the review, go over the whole code base twice so that every calculation and every process that a field names is a function of that name, with its source (ADR-0049); then go over the whole repository's tooling and tests: duplicates, slow steps, waits (ADR-0069, ADR-0070).

## Acceptance criteria

- Given the whole code base, when the first pass ends, then every operation or run of steps that a field names — mathematics, geometry, physics, drafting, industry, the laboratory — is a call to a function of that name with its `@see`, and each one left inline is listed with its reason.
- Given the result of the first pass, when the second pass ends, then what the first missed and the duplicates found are named or merged, unless merging changes a floating-point result.
- Given a golden file of results written before the first pass, when each pass ends, then every result is identical bit for bit and no expected test value was edited.
- Given every test and every tool of the repository, when the tooling and tests pass ends, then each claim proven twice is merged or removed (`mutate` and the coverage gates showing nothing escapes), each slow preparation is done once, each wait or slow step found is fixed or logged in `docs/process/improvements.md`, and `npm run test` and `check:all` are timed before and after on this Mac, however small their share looks.

## Tasks

One task = one commit, referenced as `NAM-000.Tn`.

- [ ] T1 — Golden file of results; list, folder by folder, the unnamed calculations and their sources (1 h)
- [ ] T2 — First pass: name them, the `math-expert` checking each name (3 h)
- [ ] T3 — Second pass on the result, and the duplicates; golden file compared (2 h)
- [ ] T4 — Audit of both passes by the `auditor`; the list of what stays inline (1 h)
- [ ] T5 — Tooling and tests pass: claims proven twice, slow preparations, waits; timings before and after, the `verifier` checking the list (2 h)
