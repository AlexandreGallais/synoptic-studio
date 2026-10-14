---
id: CHK-013
epic: E14
feature: F13
title: "Checkpoint between the two passes"
status: done
points: 1
---

# E14 · F13 · CHK-013 — Checkpoint between the two passes

Pause between the passes: conventions and docs describe the new functions; the light evolvability check.

## Acceptance criteria

- Given pass 1, when the checkpoint runs, then `docs/conventions/` and the folder table describe the code as it is.

## Tasks

One task = one commit, referenced as `CHK-013.Tn`.

- [x] T1 — Consistency pass and evolvability check (30 min)

## Light evolvability check (2026-10-14)

| Question            | Answer                                                                                                                                                                                         |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Size                | 224 modules in `src/` (geometry 104, model 68, math 42, io 6, render 4); 1 198 tests; `math` has grown from 4 folders to 8, each at 15 modules or fewer                                        |
| Tools               | a golden file of 400 results, written before pass 1, proves every result bit for bit after each story; it caught a reordered product on purpose                                                |
| Fitness functions   | a property per new math and geometry function; the golden file is a scratch tool, not a test of the repository: a candidate for the improvement log                                            |
| Patterns            | the duplicates for pass 2: the empty corners and handles, the shape kinds told apart three ways, a test helper; `cubicVelocity` and `derivativeCoefficients` stay two (their rounding differs) |
| Agent configuration | the math expert and the auditor each changed something in both stories of pass 1                                                                                                               |
| Process             | each pass reviewed before its last commit; the plan of note 0016 followed, its "left inline" table kept up to date                                                                             |

Consistency: `docs/conventions/files.md` lists the new folders and functions; research note 0016 lists what each pass named and what stays inline.
