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

After EN-146, whose history it keeps for the page. The keys of SP-027 (`interaction.md` §1): Ctrl + Z undoes, Ctrl + Shift + Z and Ctrl + Y redo, Cmd for Ctrl on macOS. A step at each finished action on the drawing (Q7): a shape placed by a click, drawn by a drag at its release, shapes moved by a drag at its release, deleted, nudged by the arrows as SP-029's answer 3 groups them ([note 0030](../../research/notes/0030-f40-undo-and-redo.md), §3, §6). SP-029's answers (2026-10-23): the selection recorded with each state, at every change of the selection too, given back by undo and redo (answer 2, EN-146); a key held one step from its press to its release (answer 3); the Undo and Redo buttons in the toolbar, disabled while there is nothing to undo or redo, the history's size shown beside the size set (answer 8; the size set is US-076's, the list of steps a feature after F40); a field with nothing typed since its value was applied passes the keys to the drawing, a field being typed in keeps them (answer 9). The panel's changes are US-075's: until then, a change in the panel is recorded at the next step the page takes (agent's choice).

## Acceptance criteria

The examples draw A, a rectangle 100 × 50 at (10, 20) of radius 0, B the same at (200, 20) and C at (400, 20), drawn in that order, none selected; the focus outside the panel's fields. A key is pressed and released.

- Given A, B and C, when the Rectangle tool places a shape by a click at (250, 140), 100 × 100 at (200, 90), then Ctrl + Z removes it and Ctrl + Shift + Z puts it back, as Ctrl + Y does; Cmd + Z, Cmd + Shift + Z and Cmd + Y on macOS; a shape drawn by a drag from (10, 10) to (130, 90), 120 × 80, is undone and redone the same way.
- Given A clicked, then dragged by (30.6, 19.4), landing at (41, 39), when Ctrl + Z is pressed, then A is at (10, 20), the drawing as it was, A selected and loaded into the panel, and Ctrl + Shift + Z lands it at (41, 39) again, still selected; a drag the browser cancels makes no step, and a drag of a shape not selected gives back, undone, the selection before its press (SP-029's answer 2).
- Given B clicked, Shift + click on C, then Delete, when Ctrl + Z is pressed, then B and C are back in their places, A, B, C in the order drawn, B and C selected; Ctrl + Shift + Z removes them again, none selected (SP-029's answer 2).
- Given A moved, then B clicked, Shift + click on C, a selection rectangle, Ctrl + A or Escape, when Ctrl + Z is pressed, then the move is undone: selecting alone is never a step (Q7); and Ctrl + Shift + Z after an undo, the selection changed meanwhile, still redoes it.
- Given A clicked, Right held, 30 keys sent, A at (40, 20), released, then Right pressed once more, A at (41, 20), when Ctrl + Z is pressed twice, then A is at (40, 20), then at (10, 20): a key held is one step from its press to its release (SP-029's answer 3).
- Given A moved, Ctrl + Z, then B moved, when Ctrl + Shift + Z is pressed, then nothing changes: a step after an undo drops the steps undone.
- Given the page as it opens, then the toolbar's Undo and Redo buttons are disabled, their titles naming the keys, and the toolbar reads History 0 B of 100 MB; once its rectangle, 120 × 80 at (0, 0) of radius 10, 616 characters as the page keeps it, is nudged by Right, Undo is enabled and History 1.2 kB of 100 MB is shown (616 × 2 + 8 for its reference + 8 for its index = 1 248 bytes); Undo clicked undoes as Ctrl + Z, Redo then enabled, Undo disabled, Redo clicked redoes as Ctrl + Shift + Z (SP-029's answer 8).
- Given a press with the Selector moving shapes, or a tool's drag drawing one, when Ctrl + Z is pressed, then nothing changes until the release, as Delete (`readPressedMove`, agent's choice); a tool armed stays armed after an undo, as after Delete (agent's choice).
- Given 150 typed in A's W over 100 and Enter pressed, the focus staying in W, when Ctrl + Z is pressed, then A is 100 wide, the field showing 100, the key's default prevented; once a character is typed in W, Ctrl + Z is left to the field, the drawing unchanged (SP-029's answer 9).
- Given steps taken, when Reset is clicked, or a step of the guided test shown, then the history is emptied, the page as it opens: Undo and Redo disabled, History 0 B of 100 MB (agent's choice, `interaction.md` §1: the shapes placed are forgotten).

## Product Owner test

`npm run dev`, `http://localhost:5173/`. Written with T3, once SP-029's answers are given: place, draw, move, delete and nudge, each undone and redone by the keys and the buttons.

| Step | Do | You should see |
| --- | --- | --- |

## Tasks

- [ ] T1 — The criteria in line with SP-029's answers; the page's history, the step taken once the page is redrawn from an action, the keys beside the selection's (`playground/history/`): interface and failing tests (1 h)
- [ ] T2 — A step at a click placing, a drag's release, a deletion; Ctrl + Z, Ctrl + Shift + Z and Ctrl + Y restore a state, the drawing written back and the selection loaded as `writeShapeSelection` does; the tests of T1 on them passing (2 h)
- [ ] T3 — The arrows grouped as SP-029's answer 3; the Undo and Redo buttons; Reset; the rules in `interaction.md` §1 and §7, the Q table, the Product Owner test card and F40's guide page; every test of T1 passing (2 h)

Files: `playground/history/` (new, the page's history, its steps, its keys and buttons, heard beside the selection's keys and wired after the drawing's listeners and the toolbar's), `src/io/byte-size/` (new, the size as the toolbar writes it, a `format` function, which the playground's layer does not take), `playground/index.html` (the buttons), `playground/mount/` (the listeners, the history brought up to the page at each redraw), `playground/guided-test/` (`listenToGuide`, moved out of `mount-playground.ts`, held to 10 imports). `@see` ADR-0095, `docs/domain/interaction.md#_7-navigation-and-history`, `REF-APPLE-SHORTCUTS`, `REF-WINDOWS-SHORTCUTS`, `REF-PENPOT-UNDO`, `REF-INKSCAPE-DOCUMENT-UNDO`.

One task = one commit, referenced as `US-074.T1` in the commit footer.
