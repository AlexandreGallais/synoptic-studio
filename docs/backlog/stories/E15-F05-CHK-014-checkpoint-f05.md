---
id: CHK-014
epic: E15
feature: F05
title: Checkpoint in the middle of F05
status: ready
points: 1
---

# E15 · F05 · CHK-014 — Checkpoint in the middle of F05

Docs and methods brought up to date, consistency checked (ADR-0028).

## Acceptance criteria

- Given F05's first stories, when the checkpoint ends, then the domain, the references, AGENTS.md and the rules say the same as the code, and what deserves the retrospective is logged.

## Tasks

One task = one commit, referenced as `CHK-014.Tn`.

- [ ] T1 — Consistency and evolvability check (1 h)

## Light evolvability check (2026-10-15)

| Question            | Answer                                                                                                                                                                                      |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Size                | 379 modules in `src/` (model 147, geometry 117, math 62, io 47, render 6), 239 test files; F05 added the `io` font reader and the `model` text folders; `inter-metrics.json` weighs 1.60 MB |
| Tools               | `size-limit` waits for the first npm publication, which the Inter data now concerns (improvement log); the other rows of "considered for later" have no trigger yet                         |
| Fitness functions   | the allowed SVG elements (ADR-0052) are written in AGENTS.md and checked by the render tests only: a candidate check over every element created                                             |
| Patterns            | the integer-range check exists twice or more (left for NAM-001); a text's underline and strike-through go through the shapes' path writer since US-031's review                             |
| Agent configuration | AGENTS.md and CLAUDE.md 147 lines together; three pitfalls added to `.claude/rules/library.md` (lint of nested readonly types alone, happy-dom `<select>`, `[hidden]` against rules by id)  |
| Process             | workers in worktrees with three parallel reviews per story (ADR-0051): every story had findings fixed before its merge; incidents logged in the improvement log                             |

Consistency: AGENTS.md's state line now says E15 is in progress; `shapes.md` says a line 0 wide or less has no underline nor strike-through, as `lineDecorations` does. Left for a decision: ADR-0052 (decision 1) and Q31 measure a font without metrics with the default font, while the code and `shapes.md` try the fallback family first; the fallback family is not checked as the font is.

Product Owner test cards written so far in F05: US-031's ("Product Owner test" in its story file).
