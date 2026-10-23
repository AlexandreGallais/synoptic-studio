---
id: AUD-042
epic: E02
feature: F40
title: Audit F40
status: done
points: 1
---

# E02 · F40 · AUD-042 — Audit F40

Audit F40 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F40's criteria, when the audit ends, then each is traced to a test titled `[F40.ACn] …`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — Criteria traced, findings fixed (1 h)

One task = one commit, referenced as `AUD-042.T1`.

## Findings

The `auditor` alone reviewed F40 read-only (ADR-0086), with probes of its own on the page and on the model; its findings were each reproduced before they were fixed. F40's four criteria are traced to the `[F40.AC1]` tests of `history-with-step.test.ts`, `undone-history.test.ts`, `redone-history.test.ts` and `undo-and-redo.test.ts`, the `[F40.AC2]` tests of `undo-and-redo.test.ts` and `read-shown-text-block.test.ts`, the `[F40.AC3]` tests of `are-same-items.test.ts`, `shared-drawing.test.ts`, `history-with-step.test.ts` and `undo-and-redo.test.ts`, and the `[F40.AC4]` tests of `history-size.test.ts`, `bounded-history.test.ts`, `moves-within-size.test.ts`, `default-history-bound.test.ts`, `shape-text-length.test.ts` and `history-bound.test.ts`, each broken below.

| Id | Finding | Result |
| --- | --- | --- |
| 1 | `sharedDrawing` mapped equal texts to the last of the present's objects: a present `[p1, p2]` of equal texts, recorded again, gave `[p2, p2]`, another drawing, so a step was taken though nothing changed | reproduced on the page: the opening rectangle and a second one set to the same values by the panel's several fields, then a click on the empty drawing took a 6th step, its past state's text the present's; fixed: the present's shape at the same index kept when its text is the same, before the texts' map; `[F40.AC3] keeps the history itself for two shapes of equal texts read again` (`toBe`), `[F40.AC3] keeps each of two present shapes of equal texts at its index, read again`, and the property `[F40.AC3] takes no step for a copy of the present, each shape a new object of the same text`, the three seen failing with the map alone (counterexample `[["b"],["b"]]`); the probe then takes no step; `interaction.md` §7 (A step), Q7 |
| 2 | Speed: each measure of the history wrote again the JSON text of every shape it holds, and a click that took no step was measured again | measured with a history of 20 000 and 80 000 moves of A, about the 80 380 of 100 MB, the median of 6 steps: before, 59 ms and 387 ms a step (bounded and shown), 30 ms and 129 ms a click; fixed: each shape's text length kept once measured in a `WeakMap` (`shapeTextLength`, ADR-0096, its one write registered in `exceptions.md`), `historySize` and `boundedHistory` taking objects (`T extends WeakKey`); the page bounds no record that took no step, past and states to come the same lists, while the history was bounded by the same bound (`BOUNDED_HISTORIES` in `save-history-step.ts`), so that an undo or a bound applied is bounded as before. After: 4 ms and 63 ms a step, a click 2 ms and 21 ms in the model and 0.04 ms on the page, the bounding skipped; the first step after building 80 000 states, every text written once, 460 ms. The size kept from step to step is F41's (For the main thread) |
| 3 | `REF-V8-POINTER-COMPRESSION` says a tagged value is 32 bits with compression, 64 without, where the TSDoc said "a 64-bit slot" as a fact | fixed: 8 bytes the slot without pointer compression, an upper bound, 4 with it, in `history-size.ts`'s TSDoc and `SLOT_BYTES`, `interaction.md` §7 (Its size) and the guide page; ADR-0095, accepted, gets a dated note, not a rewrite |
| 4 | `formatByteSize` rounded with `Math.round` and took the quotient and the remainder by hand, where `nearestInteger`, `integerQuotient` and `nonNegativeRemainder` name them (ADR-0049) | fixed: the three calls; its tests unchanged and passing |
| 5 | Coverage: `arc-crossing.ts:28`'s branch for an arc travelled negatively inside its segment was reached only by the properties, so that fast-check's draws decided it | reproduced with 1 run per property: 75 % of the branches, line 28; fixed: `counts 0 for (8, 8) with the arc travelled the other way`, its chord +1, σ = −1 inside the segment, 0; 100 % with 1 run, and seen failing with `chord + 1` on both sides |
| 6 | The toolbar's `History 1.2 kB` of a nudge of the page's opening rectangle did not pin its bytes, and the fields' titles gave 1,244 bytes, A's, a rectangle 100 × 50, as a move of one shape where the page's 120 × 80 adds 1,248 | fixed: `historySize` asserted 1 248 after one nudge of the opening rectangle (`undo-and-redo.test.ts`); the titles say 1,244 is a rectangle 100 × 50 alone's and 1,248 the page's (`index.html`, `history-bound.test.ts`), `interaction.md` §7 (The bound's fields), Q7, the guide page; `MOVE_BYTES` keeps A's 1 244 |
| 7 | The domain's claim that a history within its size is measured once and one over it by a step's state twice had no test | fixed: `[F40.AC4] measures the history once within its size` and `twice over it by its oldest state`, and `measures 10 moves of A 7 times to drop 7 for 3 732 bytes`, counting the reads of the newest past state's selection through a `Proxy`; seen failing with the doubling's range starting at 0 (3 measures for 2) and at `⌊reach / 3⌋ + 1` (8 for 7), the mutation of finding 8 |
| 8 | Mutation: `bounded-history.ts:64`, `integerQuotient(reach.reach, 2)` → `3`, survived: a range starting lower gives the same count dropped, one measure more | fixed by finding 7's count of 7 measures |

Mutations (A4), `npm run mutate` against each file's related tests: the 9 modules of `src/model/history/` with a body and `formatByteSize`, 30 caught and 1 survivor (finding 8). By hand, each criterion once: `undoneHistory` leaving the present out of the states to come (`[F40.AC1]`, 37 fail in `src/model/history/` and `undo-and-redo.test.ts`); the panel's step asked at each `input` rather than each `change` (`[F40.AC2]`, 12 fail); `historyWithStep` stepping for every drawing (`[F40.AC3]`, 57 fail); `SLOT_BYTES` 4 rather than 8 (`[F40.AC4]`, 18 fail across the model and the playground). And the new code: `sharedDrawing` by the texts' map alone (3 fail, finding 1); `shapeTextLength` without its write (1 fails); the page's bounding skipped whatever the bound (`drops the oldest steps at once when the bound is set`, 1 fails); `arcCrossing` with `chord + 1` for both directions, 1 run per property (1 fails, finding 5).

Properties (ADR-0022): `historyWithStep` gets one, a copy of the present taking no step (finding 1); F40's other `domain` functions had theirs; `shapeTextLength`, a memo of one length, is tested by examples.

Breadth (step 7): `check:all` passed with the push; the weekly link check was not read again for this audit; `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `docs/conventions/` and `docs/tooling/` still describe what F40 did, `exceptions.md` given its row for ADR-0096.

Next feature (G): F41, the list of the history's steps, needs how the editors read name each step by its action and show their lists (GIMP's Undo History, Penpot's history panel, none of their pages read yet: a research request for its spike), what a click on an entry does to the steps after it (kept to redo, as its criterion says), and the size kept from step to step (For the main thread), whose update rules are a derivation to validate.

Evolvability (H):

- Size: `playground/history/` holds 15 modules, at the signal of `files.md`; `src/model/history/` 13, `playground/history-bound/` 11.
- Tools: no "considered for later" row is triggered by F40.
- Fitness functions: finding 5 is a branch covered only by a property's draws; a coverage run with 1 run per property, as the auditor's, would hold it at each review.
- Patterns: a calculation by hand where a named function exists (finding 4); the naming pass at each review catches it.
- Agent configuration: nothing.
- Process: a size claimed fast without a measure (finding 2) and a count of measures claimed without a test (finding 7); a story that claims a cost measures it.

## For the main thread

- For the done commit's footer: the rule docs changed in this task, `interaction.md` §7 (A step, Its size, The bound's fields), the Q row Q7 of `docs/domain/README.md`, and F40's guide page; F40 has no `VAL` (VAL-041 dropped, a feature ending with its audit).
- ADR-0096 is written Accepted, as the audit's fix asked for; if the Product Owner should decide it, its status becomes Proposed and the memo waits.
- The feature file F40 is the main thread's (ADR-0065): its criterion 4's "a move of A alone adds 1 244 bytes" holds, A being a rectangle 100 × 50.
- The size kept from step to step, for its story in F41 (the auditor's proposal was not handed to this worker; it is written here from its outline):
  - **Invariant**: the history holds, beside its states, `size`, equal to `historySize(history)`, and `held`, for each shape a past or future state holds, how many of those states hold it, each state counting a shape once.
  - **A state added** to the past or the states to come: each shape of it gets `held + 1`; one going from 0 to 1 and not in the present adds `2 × shapeTextLength(shape)`; the state adds `8 × (drawing.length + selection.length)`. **A state removed**, the oldest past dropped by the bound or the states to come dropped by a step, the reverse: from 1 to 0 and not in the present, `− 2 × length`.
  - **The present changed** from P to P′: each shape of P not in P′ with `held > 0` adds `2 × length`; each of P′ not in P with `held > 0` takes it away. A selection replaced alone changes nothing.
  - **The operations**: a step adds the present to the past, removes each state to come, then changes the present; an undo removes the last past state, adds the present to the states to come, then changes the present to it; a redo the other way; the bound removes the oldest past states one by one while `size` is over the size set and more than the steps kept are past, with no bisection.
  - **Its cost**: each operation reads only the states and shapes it moves, a step's dropped future paid once for each state added; `held` copied at each step is a map of every shape held, unless a persistent map, or one kept by the shell, holds it: the story's question.
  - **Property**: from an empty history, any sequence of steps, undos, redos and bounds drawn from `ANY_HISTORY_STATE` keeps `size` equal to `historySize` and `held` equal to the count of past and future states holding each shape, after each operation; and a bounded history equals `boundedHistory`'s.
