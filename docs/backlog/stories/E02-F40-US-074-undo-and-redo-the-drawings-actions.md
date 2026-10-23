---
id: US-074
epic: E02
feature: F40
title: Undo and redo what is done on the drawing
status: ready
points: 3
---

# E02 · F40 · US-074 — Undo and redo what is done on the drawing

As a symbol designer, I want to undo and redo what I did on the drawing, by the keys and by buttons, so that a mistake costs a key, never the work done.

After EN-146, whose history it keeps for the page. The keys of SP-027 (`interaction.md` §1): Ctrl + Z undoes, Ctrl + Shift + Z and Ctrl + Y redo, Cmd for Ctrl on macOS. A step at each finished action on the drawing (Q7): a shape placed by a click, drawn by a drag at its release, shapes moved by a drag at its release, deleted, nudged by the arrows as SP-029's answer 3 groups them ([note 0030](../../research/notes/0030-f40-undo-and-redo.md), §3, §6). Draft until SP-029's answers 1, 2, 3, 8 and 9.

## Acceptance criteria

The examples draw A, a rectangle 100 × 50 at (10, 20) of radius 0, B the same at (200, 20) and C at (400, 20), drawn in that order; the focus outside the panel's fields.

- Given A, B and C, when the Rectangle tool places a shape by a click at (250, 140), 100 × 100 at (200, 90), then Ctrl + Z removes it and Ctrl + Shift + Z puts it back, as Ctrl + Y does; Cmd + Z, Cmd + Shift + Z and Cmd + Y on macOS; a shape drawn by a drag from (10, 10) to (130, 90), 120 × 80, is undone and redone the same way.
- Given A dragged by (30.6, 19.4), landing at (41, 39), when Ctrl + Z is pressed, then A is at (10, 20), its fractions as they were, and Ctrl + Shift + Z lands it at (41, 39) again; a drag the browser cancels makes no step.
- Given B and C selected and deleted, when Ctrl + Z is pressed, then B and C are back in their places, A, B, C in the order drawn.
- Given A moved, then B clicked, Shift + click on C, a selection rectangle, Ctrl + A or Escape, when Ctrl + Z is pressed, then the move is undone: selecting alone is never a step (Q7); what is selected after an undo or a redo, as SP-029's answer 2 says.
- Given A nudged by Right held, its repeats sent, then released, when Ctrl + Z is pressed, then A goes back as SP-029's answer 3 groups the nudges.
- Given A moved, Ctrl + Z, then B moved, when Ctrl + Shift + Z is pressed, then nothing changes: a step after an undo drops the steps undone.
- Given the toolbar, then an Undo and a Redo button undo and redo as the keys do, their titles naming the keys, each disabled while there is nothing to undo or redo, as SP-029's answer 8 shows them.
- Given a press with the Selector moving shapes, or a tool's drag drawing one, when Ctrl + Z is pressed, then nothing changes until the release, as Delete (`readPressedMove`, agent's choice); a tool armed stays armed after an undo, as after Delete (agent's choice).
- Given the focus in a field of the panel, when Ctrl + Z is pressed, then as SP-029's answer 9 says.
- Given Reset, or a step of the guided test, then the history is emptied, the page as it opens (agent's choice, `interaction.md` §1: the shapes placed are forgotten).

## Product Owner test

`npm run dev`, `http://localhost:5173/`. Written with T3, once SP-029's answers are given: place, draw, move, delete and nudge, each undone and redone by the keys and the buttons.

| Step | Do | You should see |
| --- | --- | --- |

## Tasks

- [ ] T1 — The criteria in line with SP-029's answers; the page's history, the step taken once the page is redrawn from an action, the keys beside the selection's (`playground/history/`): interface and failing tests (1 h)
- [ ] T2 — A step at a click placing, a drag's release, a deletion; Ctrl + Z, Ctrl + Shift + Z and Ctrl + Y restore a state, the drawing written back and the selection loaded as `writeShapeSelection` does; the tests of T1 on them passing (2 h)
- [ ] T3 — The arrows grouped as SP-029's answer 3; the Undo and Redo buttons; Reset; the rules in `interaction.md` §1 and §7, the Q table, the Product Owner test card and F40's guide page; every test of T1 passing (2 h)

Files: `playground/history/` (new, the page's history, its steps, its keys and buttons), `playground/selection-keys/` (the keys heard beside Ctrl + A), `playground/toolbar/` and `playground/index.html` (the buttons), `playground/mount/` (the listeners). `@see` ADR-0095, `docs/domain/interaction.md#_7-navigation-and-history`, `REF-APPLE-SHORTCUTS`, `REF-WINDOWS-SHORTCUTS`, `REF-PENPOT-UNDO`, `REF-INKSCAPE-DOCUMENT-UNDO`.

One task = one commit, referenced as `US-074.T1` in the commit footer.
