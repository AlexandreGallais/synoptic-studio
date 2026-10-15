---
id: CHK-015
epic: E15
feature: F14
title: Checkpoint in the middle of F14
status: ready
points: 1
---

# E15 · F14 · CHK-015 — Checkpoint in the middle of F14

Pause between the model stories and the playground stories of F14: bring documentation and methods up to date as the project grows, before going on (ADR-0028, `/run` skill).

## Acceptance criteria

- Given EN-033 to EN-036 merged, when the checkpoint runs, then `docs/domain/shapes.md` (Text), `DERIV-text-box`, the references, `AGENTS.md`, `CLAUDE.md` and `.claude/rules/` describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.
- Given the user stories merged so far, when the checkpoint ends, then their Product Owner test cards are listed for VAL-015.

## Tasks

One task = one commit, referenced as `CHK-015.Tn`.

- [x] T1 — Consistency of code, docs and agent configuration; fixes (1 h)
- [x] T2 — Light evolvability check; improvement log; test cards listed (30 min)

## Light evolvability check (2026-10-15)

| Question            | Answer                                                                                                                                                                                                                                                                                                         |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Size                | 442 modules in `src/` (model 202, geometry 117, math 66, io 50, render 6), barrels counted, against 379 at CHK-014; 276 test files; the longest module is `least-ragged-breaks.ts` (220 lines); the 15-module signal of ADR-0043 was hit four times in F14, each answered by a domain folder (improvement log) |
| Tools               | no trigger yet: `size-limit` still waits for the first npm publication, `api-extractor` for a stable API, Stryker for its issue 6210                                                                                                                                                                           |
| Fitness functions   | a first performance budget exists, as timing tests of the wrap and the balance bounded by Vitest's timeout; a budget per layer is not written yet                                                                                                                                                              |
| Patterns            | every paragraph step that built its results by spreading one item at a time became a scan computed by halves (`scan`, `math/sequence/`); the integer-range duplicates stay for NAM-001                                                                                                                         |
| Agent configuration | AGENTS.md and CLAUDE.md 147 lines together; three pitfalls added to `.claude/rules/library.md` (a commit-body line read as a footer, timing tests, branch names the pre-push hook refuses)                                                                                                                     |
| Process             | workers in worktrees, an auditor running brute-force and mutation checks on each story (ADR-0051): each of EN-033 to EN-036 had real defects found and fixed before its merge (improvement log)                                                                                                                |

Consistency — AGENTS.md's state line now names F14's model; `DERIV-text-box` lists `REF-WIKI-PREFIX-SUM` among its sources; Q37 states the balance's bounds (10 lines, 500 words), as `shapes.md` and `balancedParagraph` do. The folder map of `docs/conventions/files.md`, the Box rules of `shapes.md` and the code of `text-wrap/`, `text-box/`, `text-layout/`, `text-balance/`, `text-block/`, `line-characters/` and `math/sequence/` agree.

Product Owner test cards written so far in F14: none, its user stories (US-033, US-034) come after this checkpoint. F14's provisional rules are collected in VAL-015 ("Decisions to confirm").
