---
id: US-073
epic: E02
feature: F39
title: The arrow keys nudge the shapes selected
status: draft
points: 2
---

# E02 · F39 · US-073 — The arrow keys nudge the shapes selected

As a symbol designer, I want to move the shapes selected by the arrow keys, so that I place them exactly without the mouse.

The default keys (SP-027, Q50, `interaction.md` §1): an arrow nudges by 1, Shift + an arrow by 10, as Figma, Sketch and Penpot ([note 0029](../../research/notes/0029-f39-move-shapes-on-integers.md), §2). After US-072, whose model it reuses. Draft until SP-028's question 3, a nudge from an anchor holding a fraction, is answered; T1 brings the criteria in line with it.

## Acceptance criteria

The examples draw A, 100 × 50 at (10, 20), straight, and A′, the same rectangle turned by 30° about its middle, its straight box at (4.19873, −1.65064).

- Given A selected and the focus outside the panel's fields, when Right is pressed, then A is at (11, 20); Shift + Right, at (20, 20); Up, at (10, 19); Shift + Left, at (0, 20); the panel shows its new X and Y.
- Given A′ selected, when Right is pressed, then its X goes from 4.19873 to 5.19873 with SP-028's answer 3 (a), to 5 with 3 (b) or 3 (c); when Left is pressed instead, to 3.19873 with (a), 4 with (b), 3 with (c); its Y, −1.65064, unchanged in each.
- Given several shapes selected, when an arrow is pressed, then each moves as one shape does, as SP-028's answer 2 lands several.
- Given an arrow held, then the shapes move one step at each key the browser sends, its repeats included (agent's choice; whether a key held is one step of the history is for E02's next feature, Q7).
- Given the focus in a field of the panel, the anchor grid's radios included, when an arrow is pressed, then the field takes it and no shape moves (`interaction.md` §1).
- Given no shape selected, or Points mode, when an arrow is pressed, then nothing moves and the key is left to the browser, which may scroll the page; with shapes selected in Shape mode, the page does not scroll (agent's choice).
- Given a text never measured among the shapes selected, then it stays where it is, having no straight box, as a value typed for several leaves it (F38).

## Product Owner test

`npm run dev`, `http://localhost:5173/`. Written at T2, from the answer.

## Tasks

- [ ] T1 — The criteria in line with SP-028's answer 3; the shift of a nudge (`src/model/moving/`) and the arrows heard beside the selection's keys: interface and failing tests (1 h)
- [ ] T2 — The arrows and Shift + the arrows nudge the shapes selected outside the fields, Points mode and no selection leaving the key to the browser; the rules in `interaction.md` §1 and §2 and the Q table; the Product Owner test card and F39's guide page; the tests of T1 passing (2 h)

Files: `src/model/moving/` (the shift of a nudge, the drawing after it, US-072's), `playground/selection-drag/` (the arrows, `playground/selection/` holding 15 modules), `playground/selection/listen-to-selection-keys.ts` (the field's check reused). `@see` `docs/domain/interaction.md#_1-tools`, `REF-FIGMA-ADJUST`, `REF-SKETCH-SHORTCUTS`, `REF-PENPOT-TRANSFORMS`.

One task = one commit, referenced as `US-073.Tn` in the commit footer.
