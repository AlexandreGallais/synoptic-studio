---
id: NAM-002
epic: E17
title: Naming pass over the code of E17
status: draft
points: 2
---

# E17 · NAM-002 — Naming pass over the code of E17

Before the review, go over the code E17 changed twice so that every calculation a field names is a named function with its source, with the full audit of the epic beside it (ADR-0049, ADR-0064).

## Acceptance criteria

- Given the code E17 changed, when the two passes end, then every calculation a field names is a function of that name with its `@see`, and the full audit's findings (duplicates, shareable code, the merge of parallel branches) are fixed or logged.
- Given every test and every tool of the repository, when the tooling and tests pass ends, then each claim proven twice is merged or removed, each slow preparation is done once, each wait or slow step found is fixed or logged, and `npm run test` and `check:all` are timed before and after (ADR-0069, ADR-0070).
