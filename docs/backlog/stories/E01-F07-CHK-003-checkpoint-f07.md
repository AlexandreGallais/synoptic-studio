---
id: CHK-003
epic: E01
feature: F07
title: Checkpoint in the middle of F07
status: done
points: 1
---

# E01 · F07 · CHK-003 — Checkpoint in the middle of F07

Pause between the anchored polygon and its playground control: bring documentation and methods up to date as the project grows, before going on (ADR-0028, `/run` skill).

## Acceptance criteria

- Given the stories merged since the spike, when the checkpoint runs, then `docs/domain/`, derivations, references, conventions, `CLAUDE.md` and `.claude/rules/` describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.
- Given the user stories merged so far, when the checkpoint ends, then their Product Owner test cards are listed for the next stop.

## Tasks

One task = one commit, referenced as `CHK-003.Tn`.

- [x] T1 — Consistency of code, docs and agent configuration; fixes (0.5 h)
- [x] T2 — Light evolvability check; improvement log; test cards listed (0.5 h)

## Light evolvability check (2026-10-11)

| Question            | Answer                                                                                                                                                                                                                                                                                                         |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Size                | 60 modules in `src/` (geometry 26, model 13, math 11, io 6, render 4); 333 tests in about 5 s; `check:all` about 28 s; longest procedure `mountPlayground` (24 lines, limit 40), longest geometry `roundedContour` (21 lines with its TSDoc-free body within 20 code lines); no layer close to a package split |
| Tools               | StrykerJS still blocked (stryker-js issue 6210 open, rechecked today); mutations by hand, every review caught all but equivalent mutants; `knip`: trigger met (five layers), adoption left to the E01 retrospective; size-limit and api-extractor: triggers not met (no npm publication, API not stable)       |
| Fitness functions   | nothing new; the "every `geometry` function has a property" rule holds for `fitInBox` (four properties, three new) — `alignmentFactor` is a three-value table, tested by example                                                                                                                               |
| Patterns            | three lists of alignments became one (`ALIGNMENTS` → `Alignment`, review of US-009); `expectPoints` now in three test files (improvement log row of 2026-10-09 still open)                                                                                                                                     |
| Agent configuration | `CLAUDE.md` + rules 188 lines; three pitfalls added (cast for an out-of-union value, mutation restore by content, closed commit scopes); `/run` order guard used for the first time and gave F07                                                                                                               |
| Process             | 3 stories merged by the run (#64, #65, #66); every review changed its story, and the spike's review caught a false "exact in floating point" claim before any code was written on it                                                                                                                           |

## Product Owner test cards so far

None yet in F07: US-009 has no playground of its own; US-010 will carry the card that also shows EN-009 and US-009.
