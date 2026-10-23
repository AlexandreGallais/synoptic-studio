---
id: AUD-043
epic: E02
feature: F41
title: Audit F41
status: done
points: 1
---

# E02 · F41 · AUD-043 — Audit F41

Audit F41 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F41's criteria, when the audit ends, then each is traced to a test titled `[F41.ACn] …`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — Criteria traced, findings fixed (1 h)

One task = one commit, referenced as `AUD-043.T1`.

## Findings

The `auditor` alone reviewed F41 read-only (ADR-0086), with benches of its own on the model: 25 mutants, 24 caught, one equivalent; F41 found correct, its findings each reproduced before they were fixed. F41's two criteria are traced to the `[F41.AC1]` tests of `src/model/step-names/`, `src/model/history-list/`, `sized-history-at.test.ts`, `history-list.test.ts` and `step-labels.test.ts`, and the `[F41.AC2]` tests of `src/model/held-shapes/`, `src/model/shape-memo/`, `src/model/sized-history/`, `undo-and-redo.test.ts` and `history-timings.test.ts`, each broken below.

| Id | Finding | Result |
| --- | --- | --- |
| 1 | A step costs every shape of the present, not what it moves: it adds the whole present to the past (`measureByState` copies the counts' path for each of its shapes), an undo or a redo moves a whole state each way, and `measureWithPresent` builds both drawings' sets; ADR-0097, decision 2, and both guide pages said a step costs what it changes | reproduced with the auditor's bench, 1 000 rectangles each step moving one, the median of the last 100 of 2 000 steps: a step 9.0 ms, an undo 10.0 ms, going back 14 steps 141 ms (the auditor's 8.6, 9.5 and 138 ms); 10, 100 and 1 000 shapes 0.07, 0.8 and 9 ms a step. Fixed in the docs: ADR-0097 a dated note, F40's and F41's guide pages, the measures in EN-147's and US-077's story files; the cheaper update is a proposal (For the main thread), not built |
| 2 | F41's guide said "at most 7 after the one marked", where the window gives a side holding fewer states its rows: back at the 7th state of 21, 6 before and 8 after (`history-list.test.ts`) | fixed: the guide says the window as the domain states it, 7 of the 14 beside the present after it at most, a side holding fewer giving its rows to the other, with that example; `interaction.md` §7 (The window of the list) written so too, its example added |
| 3 | A step was named by the model's field split into words, `Is italic`, `Is struck through`, `Radii`, `Stretched`, and a rectangle's radius `Corners`, where the panel says `Italic`, `Strike-through`, `Radius`, `Stretched to its box` | the agent's choice (2026-10-23): a step is named by the panel's label of the field changed, its unit or hint in brackets left out; `FIELD_LABELS` and `fieldLabel` (the label where the words differ, the words otherwise), the page reading a rectangle's corners as `radius` and a free shape's as `points` (`readKeptShapeFields`); tests: `fieldLabel`, `fieldsStepName`, and on the page a rectangle's radius `Radius`, a polygon's `Radius` and `Stretched to its box`, a text's `Italic` and `Strike-through`, and every label of `FIELD_LABELS` found among the panel's (`step-labels.test.ts`); `interaction.md` §7 (The list of steps, in the playground), Q7, F41's guide page |
| 4 | `nodeWithChild` left empty slots at a node's end where its TSDoc and `HeldCounts`'s say none past the last child: 33 set to 0 gave the root `[[…], undefined]` | fixed: a node ends at its last child (`findLastIndex`); `drops a node left without any count, then the root's` expects `[[undefined, undefined, 1]]`, and `ends a node at its last child` added |
| 5 | The trie's functions cited `interaction.md` §7, which does not describe the trie | fixed: `heldCount`, `withHeldCount`, `keyDepth`, `keyDigit`, `shapeKey`, `memorizedShape` and their private steps `@see ADR-0097` |
| 6 | Two test comments wrong: keys up to 40 000 are four levels, not three; `TWICE` undone twice holds A′ and A″ to come, not A and A′ | fixed: 40 000 = 1 × 32³ + 7 × 32² + 2 × 32, past 32³ = 32 768; "A′ and A″ held" |
| 7 | ADR-0049: three calculations computed by hand where a name fits: a shape's bytes, a state's slots, the most past states a bound drops | fixed: `shapeBytes`, `stateSlotBytes` and `mostDroppedStates`, used by `historySize`, `boundedHistory`, `sizedBoundedHistory`, `measureByState` and `measureWithPresent`, each with its examples |
| 8 | Duplicates: `sizedUndoneHistory` and `sizedRedoneHistory` the same around the move, `measureWithState` and `measureWithoutState` the same around ±1, two memos of the same shapes, `keptFields` written in two tests | fixed: `sizedMovedHistory` beside the two moves; `measureByState`, a `StateChange` of 1 or −1; one memo, `memorizedShape`, its key and its text's length written in one call, the next key kept in the same `WeakMap`, one exception row instead of three (ADR-0096 and ADR-0097, dated notes); `keptShapeFields` in `tests/history-fixtures.ts`. The two new modules took `src/model/history/` and `src/model/history-list/` past 15: split by domain, the memo in `src/model/shape-memo/`, the step names in `src/model/step-names/` |
| 9 | The bench behind F41's 16 ms was not kept, so the criterion could not be measured again | fixed: `tests/history-timings.test.ts`, opt-in by `--mode history-timings`, each limit Vitest's timeout and its measure alone (EN-129): 10 steps on 80 000 moves 24 ms within 160 ms, 16 ms a step; 10 steps on 1 000 rectangles 109 ms within 450 ms; 10 steps back 134 ms within 550 ms |
| 10 | Shapes only moved in the order were named by comparing index by index two different shapes: A and B swapped, `Move` | fixed: `isReordered`, a shape after the step the same object as one before it at another index, names the step `Change`; tests: A and B swapped, B sent to the top, B moved with A nudged; `interaction.md` §7 (The list of steps) |

Mutations (A4), `npm run mutate` against each file's related tests: the 31 F41 modules with a body, 91 caught and 2 survivors, both equivalent: `memorizedShape`'s first key 1 rather than 0, keys being only distinct integers, and `ShapeChange`'s type `1 | -1` written `2 | -1`, a type erased at run time. By hand, each criterion once: `fieldsStepName` naming a move `Change` (`[F41.AC1]`, 14 fail in `src/model/step-names/`, `src/model/history-list/` and `history-list.test.ts`); `sizedHistoryAt` one undo short (`[F41.AC1]`, 6 fail); `measureByState` adding a shape's bytes only as it leaves its last state (`[F41.AC2]`, 21 fail, the properties among them); the root of the counts cut to 31 nodes (`[F41.AC2]`, 2 fail); `sizedHistoryWithStep` measuring the history again at each step (`[F41.AC2]`, the timings, 10 steps at 80 000 in 4 084 ms). And the new tests: `Italics` for `Italic` in `FIELD_LABELS` (4 fail, the page's labels read among them); the rectangle's corners not renamed (1); `isReordered` always false (6); a node not trimmed (2); the next key not moved on (2); `radii` left out of `FIELD_LABELS` (1); `stateSlotBytes` without the selection (19); `mostDroppedStates` without its floor at 0 (4); `sizedMovedHistory` always moving (2).

Properties (ADR-0022): F41's `domain` functions had theirs, the size kept against `historySize` through any steps, undos, redos and bounds; the new functions are named products and a test of their own, `isReordered` by examples.

Breadth (step 7): `check:all` passed with the push; the weekly link check is red since 2026-10-15, five links of `docs/references.md` refused by their sites with 403 (Adobe, npm), no link of F41's; `exceptions.md` keeps one row for the memo; `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `docs/conventions/` and `docs/tooling/` still describe what F41 did.

Next feature (G): Zoom and pan needs how the editors read zoom about the pointer and pan (Inkscape, Penpot, Figma: the wheel with Ctrl, Space held and dragged, the middle button, fit to the window), the wheel's units (`WheelEvent.deltaMode`, MDN), the zoom's range and steps, and where the helps of the layer above the drawing are drawn when only part of it is shown; none of those pages read yet, a research request for its spike. The cheaper update of the size (For the main thread) may come before it.

Evolvability (H):

- Size: `src/model/history/` 14 modules, `src/model/history-list/` 7, `src/model/step-names/` 10, `src/model/shape-memo/` 5, `src/model/held-shapes/` 10, `src/model/sized-history/` 13, after finding 8's split.
- Tools: no "considered for later" row is triggered by F41.
- Fitness functions: a cost claimed per step was measured on one shape moved alone (finding 1); the opt-in timings keep a wide drawing beside the long history.
- Patterns: two functions the same around a sign or a move (finding 8); the naming pass catches them.
- Agent configuration: nothing.
- Process: a cost claimed ("what it moves") measured on one case only; a story that claims a cost measures its widest case.

## For the main thread

- For the done commit's footer: the rule docs changed in this task, `interaction.md` §7 (The list of steps, The window of the list, The list of steps in the playground), the Q row Q7 of `docs/domain/README.md`, and F41's guide page (F40's too); F41 has no `VAL` (VAL-042 dropped, a feature ending with its audit).
- The step's names by the panel's labels are the agent's choice of 2026-10-23: `Stretched to its box` is the panel's whole label for the polygon's switch, where the request wrote `Stretched`; a text box's width and height take the panel's `Width` and `Height`, the guide's earlier `Box width` gone.
- The cheaper update of the size, proposed by the auditor, not built here (finding 1), for a story the main thread places:
  - **The cost now**: a step, an undo and a redo each count every shape of the states they move: with 1 000 rectangles, a step 8.6 ms, an undo 9.5 ms, a click on the list going back 14 steps 138 ms (the auditor's bench; reproduced 9.0, 10.0 and 141 ms); at 10 and 100 shapes 0.07 and 0.8 ms; 80 000 moves of one rectangle about 2 ms, the past's list copied.
  - **The idea**: a shape lives in one run of states, from the step that made it to the step that replaced it, since a step shares only the present's objects (`sharedDrawing`): the states holding it are consecutive. The counts then change only at a run's ends: a state added beside its neighbour adds the shapes it holds that the neighbour does not, and a state removed at an end removes those its neighbour does not hold; the shapes both hold keep their count, so that a step moving one shape of 1 000 changes one or two counts.
  - **What it needs**: the shapes two neighbouring states differ by without reading all their shapes: the step knows them (`sharedDrawing` keeps the present's object at each index whose text is the same), so that the history could keep, with each state, the indices it changed; an undo and a redo read the same indices back.
  - **Property**: the same as EN-147's, the size kept equal to `historySize` after any steps, undos, redos and bounds, and the run of each shape consecutive.
  - **Measure**: `tests/history-timings.test.ts`'s steps on 1 000 rectangles, now 109 ms for 10, aimed at the 80 000 moves' 2 ms a step.
