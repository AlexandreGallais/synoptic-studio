---
id: US-075
epic: E02
feature: F40
title: Undo and redo what is typed and chosen in the panel
status: ready
points: 2
---

# E02 · F40 · US-075 — Undo and redo what is typed and chosen in the panel

As a symbol designer, I want each value I type or choose in the panel undone in one step, so that undoing follows what I did, not each character I typed.

After US-074, whose history and keys it uses. A value typed is one step when Enter is pressed or the field left, a text when its editing is left (Q7); the panel redraws the drawing at each character typed, so that the step is taken when the field is left or Enter pressed, at a choice (`change`, `REF-MDN-CHANGE-EVENT`) or at a button, for one shape, several, the points, the handles and a text ([note 0030](../../research/notes/0030-f40-undo-and-redo.md), §6). Draft until SP-029's answers 4 and 5.

## Acceptance criteria

The examples draw A, a rectangle 100 × 50 at (10, 20) of radius 0, selected, and B the same at (200, 20); the focus starting outside the panel's fields.

- Given A, when 1, 5, 0 are typed in W over 100 and Enter pressed, then A is drawn 1, then 15, then 150 wide, and one Ctrl + Z, the focus outside the fields, gives it 100 again; the field left by Tab instead of Enter, the same.
- Given A, when 100 is typed again in W and the field left, then no step is taken: Ctrl + Z undoes the step before it.
- Given a regular polygon of 6 corners, when 13 is typed in Corners, refused (3 to 12, Q19), then 8, and the field left, then one step is taken, from 6 to 8: none while the shape is refused (agent's choice); a refused shape dropped by a click elsewhere is a step that Ctrl + Z undoes, the shape back with its last valid values (agent's choice).
- Given a choice of the panel — the shape's kind, the anchor, Stretched, a handle's mode — or a button — a turn, Convert to a free shape, Add a point, Delete points, the handles on or off —, when it is made, then it is one step.
- Given A and B selected, when `+50` is typed in W and Enter pressed, then A and B are 150 wide, and one Ctrl + Z gives both 100 again (F38).
- Given a text whose Content is `Text`, when `Valve` is typed and the field left, then one Ctrl + Z gives `Text` again; a text placed and typed at once as SP-029's answer 5 says.
- Given a text's style changed — its size typed, bold, italic, its weight, its family, its box fitted or reset —, then each is one step; its colors as SP-029's answer 4 groups them.

## Product Owner test

`npm run dev`, `http://localhost:5173/`. Written with T2, once SP-029's answers are given: a width typed, a corner count refused then corrected, a choice, a button, several shapes, a text's content and color, each undone and redone.

| Step | Do | You should see |
| --- | --- | --- |

## Tasks

- [ ] T1 — The criteria in line with SP-029's answers 4 and 5; the step taken when a field is left or Enter pressed, at a choice and at a button of the panel: interface and failing tests (1 h)
- [ ] T2 — A step for every field, choice and button of the panel, one shape, several, the points, the handles, a text, its colors and fonts, none while the shape is refused; the rules in `interaction.md` §7 and the Q table; the Product Owner test card and F40's guide page; the tests of T1 passing (2 h)

Files: `playground/history/` (US-074's, the panel's steps beside the drawing's), `playground/mount/` (the listeners). `@see` ADR-0095, `docs/domain/interaction.md#_7-navigation-and-history`, `REF-MDN-CHANGE-EVENT`, `REF-GIMP-UNDO`.

One task = one commit, referenced as `US-075.T1` in the commit footer.
