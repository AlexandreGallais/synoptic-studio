---
id: EN-146
epic: E02
feature: F40
title: The history of the drawing in the model
status: ready
points: 2
---

# E02 · F40 · EN-146 — The history of the drawing in the model

The history as ADR-0095 proposes it, once the Product Owner decides (SP-029's question 1): the drawing's past, present and future states, a new state sharing by reference the shapes it did not change, its size counted from its data, its oldest states dropped beyond the size set. Plain data and `domain` functions of `src/model/history/`, for any shape type, as `drawingWithoutShapes` is ([note 0030](../../research/notes/0030-f40-undo-and-redo.md), §2, §4, §5). Draft until SP-029's answers 1, 2 and 7.

## Acceptance criteria

The examples draw A, a rectangle 100 × 50 at (10, 20) of radius 0, kept as the playground keeps it, 614 characters of JSON; A′ and A″ the same at (11, 20) and (12, 20), 614 each; B the same at (200, 20), 615.

- Given a history whose present is A and B, none past nor to come, when the drawing A′ and B is recorded, then its past holds one state, A and B, its present A′ and B, nothing to come, and the present's B is the past state's object, shared (`sharedDrawing`, ADR-0095, decision 2).
- Given a drawing whose every shape has the JSON text of the present's, in the same order, when it is recorded, then the history is the same: no step, so that selecting alone, or a value typed back as it was, never makes one (Q7).
- Given the history above, when undone, then its present is A and B, nothing past, the state A′ and B to come; redone, it is as before; undone with nothing past, or redone with nothing to come, it is the same history.
- Given that history undone, when the drawing A and B″ is recorded, B″ being B moved, then nothing is to come: the state undone is dropped, as in every source read (§3).
- Given the present A′ and B and the past state A and B, then the history's size is 1 244 bytes: A, held by the past alone, 614 × 2 = 1 228, and the past state's two references, 16 (ADR-0095, decision 4); 1 252 with A's index kept as its selection, if SP-029's answer 2 keeps one; a history with nothing past nor to come has a size of 0.
- Given A nudged twice, the past holding A and B, then A′ and B, the present A″ and B, then its size is 2 488 bytes: A and A′, 1 228 each, B counted in none, held by the present, and four references, 32.
- Given that history and a size of 2 000 bytes, when it is bounded, then the oldest state, A and B, is dropped, the history holding A′ and B alone, 1 244 bytes; the minimum kept, and a state larger than the whole size, as SP-029's answer 7 says.
- Given any history, then no function changes the history, the drawing or the shapes given to it.

## Tasks

- [ ] T1 — The history's type and its functions (`src/model/history/`: a step recorded, undo, redo, `sharedDrawing`, the size, the bound) with their TSDoc, `@kind domain`, and the tests of the criteria, hand-computed sizes with their computation in a comment, failing (1 h)
- [ ] T2 — The history recorded, undone, redone, shared, measured and bounded; ADR-0095 named in `interaction.md` §7 and the Q7 row; the tests of T1 passing (2 h)

Files: `src/model/history/` (new). `@see` ADR-0095, `REF-REDUX-UNDO`, `REF-IMMER`, `REF-ECMA-262-STRING-TYPE`, `REF-V8-POINTER-COMPRESSION`, `REF-GIMP-PREFS-RESOURCES`, `docs/domain/interaction.md#_7-navigation-and-history`.

One task = one commit, referenced as `EN-146.T1` in the commit footer.
