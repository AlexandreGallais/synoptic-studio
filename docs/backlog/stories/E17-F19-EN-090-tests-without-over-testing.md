---
id: EN-090
epic: E17
feature: F19
title: Tests without over-testing, and the rules to keep them so
status: done
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

## Inventory

Every `it`, `it.each` and `test.prop` call of the 492 test files, read with Oxc's parser (2 424 calls, 3 514 tests once the tables and the RuleTester cases are expanded), on 2026-10-17 at `1ed4be9b`. Each call was classed by its kind and by the claim its assertions make; candidates were found by three searches, then read one by one: two tests of identical body, the assertions of a test all held by another test of the same file, and one assertion or one call with the same arguments written in two files.

| Folder | Examples | Tables | Properties | Of which name a criterion | Calls |
| --- | --- | --- | --- | --- | --- |
| `src/math` | 81 | 51 | 85 | 42 | 217 |
| `src/geometry` | 217 | 3 | 104 | 81 | 324 |
| `src/model` | 441 | 79 | 122 | 382 | 642 |
| `src/io` | 232 | 37 | 31 | 122 | 300 |
| `src/render` | 21 | 1 | 0 | 18 | 22 |
| `playground` | 617 | 30 | 0 | 463 | 647 |
| `tests` | 95 | 15 | 0 | 63 | 110 |
| `scripts` | 113 | 0 | 0 | 16 | 113 |
| `.claude/hooks`, `docs` | 37 | 4 | 0 | 5 | 41 |
| `eslint` (RuleTester cases not counted) | 5 | 3 | 0 | 2 | 8 |

### Claims proven twice, removed or merged

| Kind | Kept | Removed or merged into it |
| --- | --- | --- |
| identical tests, two criterion tags | `src/model/regular-polygon/regular-polygon-contour.test.ts:24`, `[F02.AC1] [F07.AC1]` | `:233`, `[F07.AC1] draws F02's triangle when centered on both axes` |
| an example checked against a property's oracle, no value computed by hand | the property `src/model/regular-polygon/regular-polygon-as-free-shape.test.ts:111`, its anchor now drawn and its angle checked | `:80`, the triangle turned by 90° |
| a narrower property inside a wider one | `src/model/regular-polygon/regular-polygon-corners.test.ts:399` `[F03.AC1] [F07.AC4]`, equal radii drawn too | `:305`, `[F07.AC4]` with one radius for every corner |
| two claims proven elsewhere, combined | `src/io/font-face/parse-font-face.test.ts:247`, the collection's faces read tagged `true` | `:267`, the same faces tagged `true` (the tag alone proven at `:233`) |
| a higher level repeating a lower one | `src/io/picked-color/exact-color-to-hex.test.ts`, the sample of 731 000 colors moved there, where the round trip is the function's own claim | `src/io/picked-color/whole-values-to-hex.test.ts:103`, the sample through values left as shown, which stand for the exact ones (a property keeps that claim); `exact-color-to-hex.test.ts:55`, the property drawing random colors, which the sample holds |
| a test card walked again | each step's own test: `playground/text/text-box.test.ts:119`, `:128`, `:137`, `:170`, `:180`, `:188`, `:195`, `:217`, `playground/text-box/fit-to-box.test.ts:177` (US-057), `playground/text/text-box-fields.test.ts:105`, and F14's guided steps | `playground/text/text-box.test.ts:227`, US-033's card from step 2 to step 12 |

### Read and kept

| Pair | Why both stay |
| --- | --- |
| a guided step's test and its panel's test (`playground/guided-steps/epic-15/f14-steps.test.ts` and `playground/text/text-box.test.ts`, 137 such tests whose assertions all appear elsewhere) | the step's own claim is the state it opens on and the result it states, as the Product Owner reads it at `VAL`; the same assertion text after other actions is another claim |
| `playground/picker-controls/color-test-card.test.ts:60` and `:121`, US-035's and US-040's cards | steps no other test proves: the hue keys with the saturation held exact (`#9c3436`), a click on a plane of 200 × 300 (31, 67) |
| `f05-steps.test.ts:504` and `f14-steps.test.ts:434`, identical bodies | two steps of two features, opened by their titles |
| the bounds 0 and 100 beside a property over 0 .. 100 (`is-valid-text-decorations.test.ts:42`) | the bounds of a range stay as examples (ADR-0070) |
| `regular-polygon-contour.test.ts:515`, the stretched hexagon of `stretchedPolygon` | it adds its own claim: placed at its position, its anchor without effect |
| `tests/backlog.test.ts:405` and `:429` | a folder's index and a parent's table, two places |
| `eslint/`, `scripts/` | no case written twice; the ESLint tests move to the Rust linter later in F19 |

### Work done again, now done once

| Where | Before | After |
| --- | --- | --- |
| `tests/color-fixtures.ts` | the 731 000 indices of the color sample built at the import of `tests`, in every file importing it | built at the first round trip (`builtOnce`), frozen |
| `tests/backlog.test.ts` | every backlog file read again by each of 12 tests | read once per file, frozen |
| `tests/named-calculations.test.ts` | every source file of `src/` and `playground/` read again by each of 12 rows | read once per file, frozen |
| every test file outside the playground | Vitest's setup (the ESLint RuleTester), the `tests` barrel and the library loaded again in each of 435 files | loaded once per worker: Vitest project `shared`, `isolate: false`; the playground's 57 files stay `isolated` |

## Timings

`npm run test` on this Mac, alone under the heavy lock, 2026-10-17; Vitest's own `Duration`, the lock's wait left out.

| Run | Tests | Files | Duration | Shares |
| --- | --- | --- | --- | --- |
| before (`1ed4be9b`) | 3 514 (1 skipped) | 492 | 104.5 s | setup 35 %, import 28 %, tests 24 %, transform 9 % |
| after (`5e80683c`) | 3 509 (1 skipped) | 492 | 37.4 s | tests 51 %, import 15 %, setup 14 %, transform 12 % |

Five tests fewer: the six pairs above, the sample's two models now in `exactColorToHex` and one property left in `wholeValuesToHex`. Inside `npm run check:all`, alone under the lock, the same day:

| Run | Test lane | `check:all` | Its slowest lane |
| --- | --- | --- | --- |
| before (`1ed4be9b`) | 124.0 s | 124.0 s | the tests |
| after (`98c82b61`) | 49.3 s | 60.1 s | ESLint, 60.1 s (EN-091, then F19's Rust linter) |

Most of the 104.5 − 37.4 = 67 s saved alone come from sharing a worker's modules: a trial at the start, every file shared and nothing else changed, gave 33.4 s. The duplicates removed and the work done once give the rest, chiefly the sample of 731 000 colors run once per model instead of twice.

### Evidence

- `npm run mutate -- --all <file> -- npx vitest run '{test}'` on the six source files whose tests changed: 28 mutants, 28 caught (`whole-values-to-hex`, `exact-color-to-hex`, `regular-polygon-contour`, `regular-polygon-as-free-shape`, `regular-polygon-corners`, `parse-font-face`); on `playground/text/read-text-box-options.ts` and `playground/text-box/write-fitted-box.ts` against the text tests: 9 mutants, 9 caught.
- The repository tests made to read once, mutated by hand: an `atan2` planted in `src/math/vector/norm.ts` fails `named-calculations` (`[F13.AC1]`); a story set back to `draft` fails three tests of `backlog`.
- Coverage unchanged: `src/` at 100 %, the playground over its floor (statements 99.55 %, branches 96.62 % in all, as before).
- The `shared` project run in five shuffled orders (`--sequence.shuffle`, seeds 7919 to 39595): one failure, inside one file and unrelated to the sharing — the store test that marks a page read again changed the page the first test finds; that test now has a store of its own, and the five orders pass.

## For the main thread

- `docs/backlog/README.md` (a shared index, left alone): its `NAM` row (rule 4 and the closing paragraph) can name the tooling and tests pass now in the template, `docs/process/agile.md` and NAM-002 (ADR-0070 decision 7).
- `vite.config.ts` changed for EN-091 to know: the `include` list moved into two projects, `shared` (`isolate: false`) and `isolated` (the playground), kept in the constant `TEST_PROJECTS`; `vitest.setup.ts` untouched. Once EN-091 moves the RuleTester out of the shared setup, the playground's setup share falls too.
