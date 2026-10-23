---
id: EN-147
epic: E02
feature: F41
title: The history's size kept from step to step
status: ready
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
