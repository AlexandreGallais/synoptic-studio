---
id: US-076
epic: E02
feature: F40
title: The history kept within a memory size the designer sets
status: draft
points: 2
---

# E02 · F40 · US-076 — The history kept within a memory size the designer sets

As a symbol designer, I want to set how much memory the history keeps, so that a long session never slows the editor while the last steps stay undoable.

After US-075. The history is bounded by a memory size the user sets, never by a count of steps (Q7); its size is counted from its data (ADR-0095, decision 4), the oldest steps dropped first, as GIMP, Inkscape and Penpot drop them ([note 0030](../../research/notes/0030-f40-undo-and-redo.md), §3, §5). Draft until SP-029's answers 6, 7 and 8.

## Acceptance criteria

The examples draw A, a rectangle 100 × 50 at (10, 20) of radius 0, 1 228 bytes as the history counts it, the only shape of the drawing.

- Given the size set as SP-029's answer 6 says, where it says, when the page opens, then the history is bounded by it; the size is kept for the next session, or not, as that answer says.
- Given a size of 10 MB (10 000 000 bytes), when A is moved again and again, then each move adds 1 236 bytes to the history and about 8 090 moves are kept, the oldest dropped first; in a drawing of 100 rectangles, 2 028 bytes a move, about 4 930.
- Given a history over a smaller size just set, then its oldest steps are dropped at once until it fits, the steps to redo counted too; what stays below it, and a step larger than the whole size, as SP-029's answer 7 says.
- Given the history, then what is used and the size set are shown as SP-029's answer 8 says.

## Product Owner test

`npm run dev`, `http://localhost:5173/`. Written with T2, once SP-029's answers are given: the size set small, steps made until the oldest go, the size used shown.

| Step | Do | You should see |
| --- | --- | --- |

## Tasks

- [ ] T1 — The criteria in line with SP-029's answers 6, 7 and 8; the size's field and the size shown: interface and failing tests (1 h)
- [ ] T2 — The size set, kept as answer 6 says, the history bounded by it at each step and when it changes, the size used shown; the rules in `interaction.md` §7 and the Q table; the Product Owner test card and F40's guide page; the tests of T1 passing (2 h)

Files: `playground/history/` (US-074's), `playground/index.html` (the field). `@see` ADR-0095, `docs/domain/interaction.md#_7-navigation-and-history`, `REF-GIMP-PREFS-RESOURCES`, `REF-GIMP-UNDO-CODE`.

One task = one commit, referenced as `US-076.T1` in the commit footer.
