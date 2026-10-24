---
id: EN-147
epic: E02
feature: F41
title: The history's size kept from step to step
status: done
points: 3
---

# E02 · F41 · EN-147 — The history's size kept from step to step

AUD-042 measured a step at 63 ms with 80 000 moves after its cheap fixes (387 ms before), and the first step on a full history at about 460 ms, every shape's text written once; the Product Owner asked for a history that stays pleasant to use with up to about 1 000 symbols (SP-029's answer 6). The size is then kept from step to step instead of counted again, the auditor's proposal of 2026-10-23, written at AUD-042:

- **Invariant**: the history holds, beside its states, `size`, equal to `historySize(history)`, and `held`, for each shape a past or future state holds, how many of those states hold it, each state counting a shape once.
- **A state added** to the past or the states to come: each shape of it gets `held + 1`; one going from 0 to 1 and not in the present adds `2 × shapeTextLength(shape)`; the state adds `8 × (drawing.length + selection.length)`. **A state removed**, the oldest past dropped by the bound or the states to come dropped by a step, the reverse: from 1 to 0 and not in the present, `− 2 × length`.
- **The present changed** from P to P′: each shape of P not in P′ with `held > 0` adds `2 × length`; each of P′ not in P with `held > 0` takes it away. A selection replaced alone changes nothing.
- **The operations**: a step adds the present to the past, removes each state to come, then changes the present; an undo removes the last past state, adds the present to the states to come, then changes the present to it; a redo the other way; the bound removes the oldest past states one by one while `size` is over the size set and more than the steps kept are past, with no bisection.
- **Its cost**: each operation reads only the states and shapes it moves, a step's dropped future paid once for each state added; `held` copied at each step is a map of every shape held, unless a persistent map, or one kept by the shell, holds it: the story's question.
- **Property**: from an empty history, any sequence of steps, undos, redos and bounds drawn from `ANY_HISTORY_STATE` keeps `size` equal to `historySize` and `held` equal to the count of past and future states holding each shape, after each operation; and a bounded history equals `boundedHistory`'s.

## Acceptance criteria

- Given any sequence of steps, undos, redos and bounds, then the size kept equals `historySize` after each operation, and a bounded history equals `boundedHistory`'s (a property).
- Given a history of 80 000 moves, when a step is taken, then it costs under 16 ms, measured in the story.

## Tasks

- [ ] T1 — The size kept, its interface and failing tests, the property (1 h)
- [ ] T2 — Step, undo, redo and the bound keep it, the tests of T1 passing (2 h)
- [ ] T3 — The page keeps the size with its history; the timings (1 h)

One task = one commit, referenced as `EN-147.Tn` in the commit footer.

## The answer and the measures

The counts of the states holding each shape are kept in a persistent trie of 32 branches by an integer key of the shape, a change copying only the nodes on its key's path, so that each history keeps its own counts and the core stays pure (ADR-0097); a `Map` copied at each step was measured at 4.5 to 6.5 ms for 80 000 shapes, and the designer may set ten times the default size. Each shape gets its key once, in a module's `WeakMap`, registered in `exceptions.md`.

Measured on Node 24 with AUD-042's bench, its history of n past states each a new rectangle moved, the median of 6 steps after the first:

| n | Counted again (`boundedHistory`, `historySize`) | Kept (`sizedBoundedHistory`, `sizedHistoryWithStep`) |
| --- | --- | --- |
| 20 000 | first step 35 ms, then 4.2 ms | first step 0.36 ms, then 0.08 ms; a click taking no step 0.01 ms, an undo 0.04 ms, a redo 0.07 ms |
| 80 000 | first step 463 ms, then 63 ms | first step 2.8 ms, then 1.9 ms; a click 0.01 ms, an undo 0.09 ms, a redo 0.18 ms; a step dropping the oldest 1.9 ms |

The size kept equals `historySize` after each measure, and at 80 000 both drop the same 118 oldest states at the first step, the default 100 MB then full. A step at 80 000 costs about 2 ms, the copies of the past's list of 80 000 states it makes; measuring a whole history built at once (`measuredHistory`) writes every shape's text once, 441 ms at 80 000, which the page never does, its history starting empty.

AUD-043 measured a wide drawing on Node 24, 1 000 rectangles, each step moving one, the median of the last 100 of 2 000 steps: a step 8.6 ms, an undo 9.5 ms, going back 14 steps 138 ms. A step costs every shape of the present, not what it moves: it adds the whole present to the past, an undo or a redo moves a whole state each way, and the present changed is compared shape by shape (ADR-0097's note). With 10 shapes a step takes 0.07 ms, with 100 0.8 ms.

The measures are kept in `tests/history-timings.test.ts`, opt-in, run by `npx vitest run tests/history-timings.test.ts --mode history-timings`, each bounded by Vitest's timeout, the median of 5 runs of the file alone: 10 steps on 80 000 moves 24 ms, 2.4 ms a step, within 160 ms, 16 ms a step, F41's criterion 2; 10 steps on 1 000 rectangles 109 ms; 10 steps back on them 134 ms.
