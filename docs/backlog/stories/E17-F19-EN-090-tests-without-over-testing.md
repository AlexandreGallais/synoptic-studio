---
id: EN-090
epic: E17
feature: F19
title: Tests without over-testing, and the rules to keep them so
status: ready
points: 5
---

# E17 · F19 · EN-090 — Tests without over-testing, and the rules to keep them so

Go over every test of the repository (about 3 500 in 490 files on 2026-10-17) and remove what tests the same thing twice, share what is slow to prepare, and write the rules that keep the suite lean. Follows EN-089, which does it for the font files.

Asked by the Product Owner on 2026-10-17: test, never over-test. A behaviour proven once in one place is not proven again elsewhere; what is slow to load or build is prepared once, its preparation tested once, and the tests after it use it; tools that make tests short to write are welcome. A fast check matters beyond its own seconds: a command that ends at once spares the agent a loop of checking whether it has ended, whose cost does not show in the tool times but multiplies the time spent thinking.

## Acceptance criteria

- Given every test file, when the inventory is made, then each test is classed by the claim it proves; two tests proving the same claim on the same function, or a higher-level test repeating a lower-level one's claim without adding its own, are listed, and one of each pair is removed or merged, with the list in the story.
- Given a test removed or merged, when the mutation runner (`mutate`) is run on the code it covered, then every mutant caught before is still caught, and the coverage of `src/` stays 100 % and the playground's floor holds.
- Given a fixture slow to prepare (a file read, a document built, a font parsed for many tests), when the suite runs, then it is prepared once per worker, its preparation proven by one test, and given to the others without letting them change it.
- Given `docs/conventions/testing.md`, when the story ends, then it holds the rules: one claim, one place; the level each kind of claim is tested at; shared fixtures and how to freeze them; what a test helper may do; the limit of time per test kept as a guard against hangs only; and a new ADR records them, citing ADR-0022 and ADR-0027.
- Given the test lane timed before and after on this Mac (alone and inside `check:all`), when the story ends, then both counts of tests and both durations are written in the story.
- Given the epic routine (`docs/process/`), when the story ends, then each epic's end holds a pass over the whole repository's tooling and tests — duplicates, slow steps, waits — beside the naming pass (ADR-0049).

## Tasks

One task = one commit per group of folders, referenced as `EN-090.Tn`.

- [ ] T1 — Inventory of the claims, duplicates listed; timings before (2 h)
- [ ] T2 — `src/` tests: duplicates removed or merged, shared fixtures; mutation and coverage checked (3 h)
- [ ] T3 — `playground/`, `tests/`, `scripts/`, `eslint/` tests: the same (3 h)
- [ ] T4 — The testing rules, the ADR, the epic routine; timings after (1 h)
