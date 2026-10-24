---
id: EN-146
epic: E02
feature: F40
title: The history of the drawing in the model
status: done
points: 2
---

# E02 · F40 · EN-146 — The history of the drawing in the model

The history as ADR-0095 decides it (SP-029's answer 1): the drawing's past, present and future states, a new state sharing by reference the shapes it did not change, each state with the selection as it was when its step was taken (answer 2), its size counted from its data, its oldest states dropped beyond the memory size set, never below the number of steps kept whatever their size (answers 6 and 7). Plain data and `domain` functions of `src/model/history/`, for any shape type, as `drawingWithoutShapes` is ([note 0030](../../research/notes/0030-f40-undo-and-redo.md), §2, §4, §5).

## Acceptance criteria

The examples draw A, a rectangle 100 × 50 at (10, 20) of radius 0, kept as the playground keeps it, 614 characters of JSON; A′ and A″ the same at (11, 20) and (12, 20), 614 each; B the same at (200, 20), 615. A state is a drawing and its selection, the indices of the shapes selected.

- Given a history whose present is A and B, A selected, none past nor to come, when the drawing A′ and B is recorded, A selected, then its past holds one state, A and B with A selected, its present A′ and B with A selected, nothing to come, and the present's B is the past state's object, shared (`sharedDrawing`, ADR-0095, decision 2).
- Given a drawing whose every shape has the JSON text of the present's, in the same order, when it is recorded with the present's selection, then the history is the same; recorded with another selection, then no step is taken either, the present's selection replaced, its past and its future kept: selecting alone, or a value typed back as it was, never makes a step (Q7).
- Given the history above, B then selected alone, when B is deleted, the drawing A′ recorded with none selected, then the past's last state is A′ and B with B selected, the selection as it was when the step was taken; undone, the present is A′ and B, B selected, showing what the step changed; redone, A′, none selected (SP-029's answer 2).
- Given the history of the first criterion, when undone, then its present is A and B, A selected, nothing past, the state A′ and B to come; redone, it is as before; undone with nothing past, or redone with nothing to come, it is the same history.
- Given that history undone, when the drawing A and B″ is recorded, B″ being B moved, then nothing is to come: the state undone is dropped, as in every source read (§3).
- Given the present A′ and B and the past state A and B, A selected in both, then the history's size is 1 252 bytes: A, held by the past alone, 614 × 2 = 1 228, the past state's two references, 16, and its index, 8 (ADR-0095, decision 4); a history with nothing past nor to come has a size of 0.
- Given A nudged twice, A selected, the past holding A and B, then A′ and B, the present A″ and B, then its size is 2 504 bytes: A and A′, 1 228 each, B counted in none, held by the present, four references, 32, and two indices, 16.
- Given that history, a memory size of 2 000 bytes and 1 step kept whatever its size, when it is bounded, then the oldest state, A and B, is dropped, the history holding A′ and B alone, 1 252 bytes; with 5 steps kept, nothing is dropped, 2 504 bytes kept over the size; with 0 steps, a size of 1 000 bytes drops both states, a step larger than the whole size kept only within the steps kept (SP-029's answer 7).
- Given that history undone once, then its size is 2 504 bytes, the state to come counted; bounded by 2 000 bytes and 0 steps, its past state alone is dropped, 1 252 bytes left: the bound never drops a state to come.
- Given the bound by default, then it is 100 MB, 100 000 000 bytes, and 5 steps kept whatever their size (SP-029's answers 6 and 7).
- Given any history, then no function changes the history, the drawing or the shapes given to it.

## Tasks

- [ ] T1 — The criteria in line with SP-029's answers; the rules in `interaction.md` §7 and the Q7 row, ADR-0095 named; F40's guide page and its row; the history's types and functions (`src/model/history/`: a step recorded, undo, redo, `sharedDrawing`, the size, the bound and its default) with their TSDoc, `@kind domain`, and the tests of the criteria, hand-computed sizes with their computation in a comment, failing (1 h)
- [ ] T2 — The history recorded, undone, redone, shared, measured and bounded; the tests of T1 passing (2 h)

Files: `src/model/history/` (new). `@see` ADR-0095, `REF-REDUX-UNDO`, `REF-IMMER`, `REF-ECMA-262-STRING-TYPE`, `REF-V8-POINTER-COMPRESSION`, `REF-GIMP-PREFS-RESOURCES`, `docs/domain/interaction.md#_7-navigation-and-history`.

One task = one commit, referenced as `EN-146.T1` in the commit footer.
