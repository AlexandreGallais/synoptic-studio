---
id: US-073
epic: E02
feature: F39
title: The arrow keys nudge the shapes selected
status: ready
points: 2
---

# E02 · F39 · US-073 — The arrow keys nudge the shapes selected

As a symbol designer, I want to move the shapes selected by the arrow keys, so that I place them exactly without the mouse.

The default keys (SP-027, Q50, `interaction.md` §1): an arrow nudges by 1, Shift + an arrow by 10, as Figma, Sketch and Penpot ([note 0029](../../research/notes/0029-f39-move-shapes-on-integers.md), §2). After US-072, whose model it reuses. SP-028's answer 3 (Product Owner, 2026-10-23): a nudge moves by exactly 1, or 10 with Shift, a fraction kept until a drag lands it, as Penpot never rounds a nudge.

## Acceptance criteria

The examples draw A, 100 × 50 at (10, 20), straight, A′, the same rectangle turned by 30° about its middle (60, 45), its straight box at (4.19873, −1.65064), and B, 100 × 50 at (200, 20) turned by 30° about its middle (250, 45), its straight box at (194.19873, −1.65064).

- Given A selected and the focus outside the panel's fields, when Right is pressed, then A is at (11, 20); Shift + Right, at (20, 20); Up, at (10, 19); Shift + Left, at (0, 20); the panel shows its new X and Y.
- Given A′ selected, when Right is pressed, then its X goes from 4.19873 to 5.19873; Left, to 3.19873; Shift + Left, to −5.80127; its Y, −1.65064, unchanged in each, the fraction kept until a drag lands it (SP-028's answer 3).
- Given A and B selected, when Right is pressed, then both move by the same shift, (1, 0), their distances kept, as SP-028's answer 2 moves several: A to (11, 20), B's straight box to (195.19873, −1.65064).
- Given an arrow held, then the shapes move one step at each key the browser sends, its repeats included: three keys sent move A to (13, 20) (agent's choice; whether a key held is one step of the history is for E02's next feature, Q7).
- Given the focus in a field of the panel, the anchor grid's radios included, or on a slider of the color picker, when an arrow is pressed, then the field or the slider takes it and no shape moves (`interaction.md` §1; the picker's sliders, agent's choice).
- Given no shape selected, the shape the panel edits refused, or Points mode, when an arrow is pressed, then nothing moves and the key is left to the browser, which may scroll the page; with shapes selected in Shape mode, the page does not scroll (agent's choice).
- Given an arrow pressed with Ctrl, Cmd or Alt held, then no shape moves and the key is left to the browser, which gives some of them to its own commands (agent's choice); given a press with the Selector moving shapes, an arrow moves nothing until the release (agent's choice).
- Given a text never measured among the shapes selected, then it stays where it is, having no straight box, as a value typed for several leaves it (F38).

## Product Owner test

`npm run dev`, `http://localhost:5173/`. Written at T2, from the answer.

## Tasks

- [x] T1 — The criteria in line with SP-028's answer 3; the shift of a nudge (`src/model/moving/`) and the arrows heard beside the selection's keys: interface and failing tests (1 h)
- [ ] T2 — The arrows and Shift + the arrows nudge the shapes selected outside the fields, Points mode and no selection leaving the key to the browser; the rules in `interaction.md` §1 and §2 and the Q table; the Product Owner test card and F39's guide page; the tests of T1 passing (2 h)

Files: `src/model/moving/` (the shift of a nudge, the drawing after it, US-072's), `playground/selection-drag/` (the drawing moved, US-072's), `playground/selection-keys/` (the arrows, beside the selection's keys and the field's check, moved there at T1 since `playground/selection/` holds 15 modules, its limit). `@see` `docs/domain/interaction.md#_1-tools`, `REF-FIGMA-ADJUST`, `REF-SKETCH-SHORTCUTS`, `REF-PENPOT-TRANSFORMS`.

One task = one commit, referenced as `US-073.Tn` in the commit footer.
