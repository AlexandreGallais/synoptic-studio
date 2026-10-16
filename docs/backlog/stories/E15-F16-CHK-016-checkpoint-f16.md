---
id: CHK-016
epic: E15
feature: F16
title: Checkpoint in the middle of F16
status: done
points: 1
---

# E15 · F16 · CHK-016 — Checkpoint in the middle of F16

Pause between the model stories and the playground story of F16: bring documentation and methods up to date as the project grows, before going on (ADR-0028, `/run` skill).

## Acceptance criteria

- Given EN-037 and EN-038 merged, when the checkpoint runs, then `docs/domain/interaction.md` (Colors), `shapes.md` (Text), ADR-0056, `DERIV-color-spaces`, the references, `AGENTS.md`, `CLAUDE.md` and `.claude/rules/` describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.
- Given the user stories merged so far, when the checkpoint ends, then their Product Owner test cards are listed for VAL-016.

## Tasks

One task = one commit, referenced as `CHK-016.Tn`.

- [x] T1 — Consistency of code, docs and agent configuration; fixes (1 h)
- [x] T2 — Light evolvability check; improvement log; test cards listed (30 min)

## Light evolvability check (2026-10-15)

| Question            | Answer                                                                                                                                                                                                                                                                                                                                                                  |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Size                | 481 modules in `src/` (model 207, geometry 117, math 86, io 64, render 6), barrels counted, against 442 at CHK-015; 311 test files; the longest module is still `parse-kern-table.ts` (228 lines); `math/color/` holds 15 modules and `math/arithmetic/` 14, at the 15-module signal of ADR-0043 and one below: the next color step asks for a domain folder (HSL, HSB) |
| Tools               | no trigger since CHK-015: `size-limit` waits for the first npm publication, `api-extractor` for a stable API, Stryker for its issue 6210                                                                                                                                                                                                                                |
| Fitness functions   | the golden check (`npm run golden:check`) acted as one: it caught a 10⁻¹⁴ drift of E01 and E14 results from a change to the shared `modulo` (improvement log); it runs only by hand, not in `check:all`                                                                                                                                                                 |
| Patterns            | `modulo` and `nonNegativeRemainder` now both place a number on a cycle: the first for any sign, the second exact for a non-negative number; `hslComponent` and `hsbComponent` stay on `modulo` (a hue typed may be negative). Every rounding goes through `nearestInteger` (four callers in `io/color/`)                                                                |
| Agent configuration | AGENTS.md and CLAUDE.md 147 lines together; two pitfalls added to `.claude/rules/library.md`: a shared math function keeps its float behavior, and a worker never runs `golden:write`                                                                                                                                                                                   |
| Process             | workers in worktrees share `.git/metrics/`: one wrote the shared golden file by mistake (restored, improvement log); the auditor's review applied before each done commit, as in F14                                                                                                                                                                                    |

Consistency — AGENTS.md's state line now names F16's model and F14 as audited; `DERIV-color-spaces` says its `round` is computed by `Math.round` (`nearestInteger`) and its shown hue taken by `nonNegativeRemainder`; the Colors section of `interaction.md` writes its example in the stored, lower-case form. `src/math/color/`, `src/io/color/`, `src/model/color/` and `src/math/arithmetic/` agree with `interaction.md` §5, `shapes.md` (Text, colors), Q38, ADR-0056, `DERIV-color-spaces`, the folder map of `docs/conventions/files.md` and the references they cite (`REF-MDN-MATH-ROUND`, `REF-CSS-VALUES-4`, `REF-ECMA-262-NUMBER-ADD`, `REF-ECMA-262-NUMBER-REMAINDER`, each with its row).

Product Owner test cards written so far in F16: none, its user story (US-035) comes after this checkpoint. F16's provisional rules are collected in VAL-016 ("Decisions to confirm"), 13 rows, each with its example; row 11 now also names the screen reader's names and values.
