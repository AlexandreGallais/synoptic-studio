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

After US-074, whose history and keys it uses. A value typed is one step when Enter is pressed or the field left, a value typed back unchanged none (Q7); the panel redraws the drawing at each character typed, so that the step is taken when the field is left or Enter pressed, at a choice (`change`, `REF-MDN-CHANGE-EVENT`) or at a button, for one shape, several, the points, the handles and a text ([note 0030](../../research/notes/0030-f40-undo-and-redo.md), §6). SP-029's answers (2026-10-23): each change finished in the color picker is a step, a key on a slider from its press to its release, a drag on the plane at its release, a hexadecimal when Enter is pressed or its field left (answer 4); a text placed and what is typed in it until its editing is left is one step, Ctrl + Z then removing it, nothing being there before (answer 5); and a click with the Text tool places a text with no content of its own, the designer clicking, then typing (Product Owner, 2026-10-23, superseding SP-026's `Text`). What an empty text shows, and what becomes of one left empty, are the agent's choices, for the review's summary, F40 having no validation of its own.

## Acceptance criteria

The examples draw A, a rectangle 100 × 50 at (10, 20) of radius 0, selected, and B the same at (200, 20); the focus starting outside the panel's fields. A text is in Inter Regular 32, whose line is (1984 + 494) × 32 / 2048 = 38.71875 tall.

- Given A, when 1, 5, 0 are typed in W over 100 and Enter pressed, then A is drawn 1, then 15, then 150 wide, and one Ctrl + Z, the focus outside the fields, gives it 100 again; the field left by Tab instead of Enter, the same.
- Given A, when 100 is typed again in W and the field left, then no step is taken: Ctrl + Z undoes the step before it.
- Given a regular polygon of 6 corners, when 13 is typed in Corners, refused (3 to 12, Q19), the field left, then 8 typed and the field left, then one step is taken, from 6 to 8: none while the shape is refused (agent's choice); a refused shape dropped by a click on the empty drawing is a step that Ctrl + Z undoes, the shape back with its last valid values, and Ctrl + Z while the shape is refused gives back its last valid values, the history unmoved (agent's choice).
- Given a choice of the panel — the anchor, Stretched, a handle's mode — or a button — a turn, Convert to a free shape, Add a point, Delete points, the handles on or off —, when it is made, then it is one step; a button that changes no shape, a color's swatch opening the picker, none.
- Given A and B selected, when `+50` is typed in W and Enter pressed, then A and B are 150 wide, and one Ctrl + Z gives both 100 again (F38).
- Given a text whose Content is `Pump`, when `Valve` is typed and the field left, then one Ctrl + Z gives `Pump` again.
- Given the Text tool, when the drawing is clicked at (250, 140), then a text with an empty content is placed, its Content field focused, and shows, while empty, a frame 1 wide and one line tall where its first character will go, centred on the point on integers: (250 − 0.5, 140 − 19.359375) = (249.5, 120.640625), at (250, 121), 1 × 38.71875 (agent's choice); once `Valve` is typed and the Content field left, one step is taken, the size typed meanwhile included, and one Ctrl + Z removes the text, Ctrl + Shift + Z bringing it back (SP-029's answer 5).
- Given a text placed and still empty, when the panel leaves it — a click on the empty drawing, another shape placed, Escape — then it is removed, no step taken: Ctrl + Z undoes the step before it; Ctrl + Z with its Content field not typed in removes it too, the history unmoved (agent's choice).
- Given a text's style changed — its size typed, bold, italic, its weight, its family, its box fitted or reset —, then each is one step; given the color picker open on the text's color `#ff8000`, when the hue slider's Right is pressed 5 times, the plane dragged once and released, then `#336699` typed in the color's field and the field left, then 7 steps are taken and the first Ctrl + Z gives back the color of the plane's drag (SP-029's answer 4).

## Product Owner test

`npm run dev`, `http://localhost:5173/`. Written with T3: a width typed, a corner count refused then corrected, a choice, a button, several shapes, a text placed and typed, its content and color, each undone and redone.

| Step | Do | You should see |
| --- | --- | --- |

## Tasks

- [ ] T1 — The criteria in line with SP-029's answers 4 and 5 and the text placed empty; the step taken when a field is left or Enter pressed, at a choice, at a button and in the color picker, none while the shape is refused, a new text one step until its editing is left, the frame of an empty text: interface and failing tests (1 h)
- [ ] T2 — A step for every field, choice and button of the panel, one shape, several, the points, the handles, a text's style, the color picker as SP-029's answer 4 says, none while the shape is refused, Ctrl + Z then giving back its last valid values; the rules in `interaction.md` §7 and the Q table; the tests of T1 on the panel passing (2 h)
- [ ] T3 — A click with the Text tool places a text with an empty content, a frame 1 wide and one line tall while empty, one step until its Content field is left, removed when the panel leaves it empty; the playground's tests that placed `Text` typing their content; the rules in `interaction.md` §1 and §7, `shapes.md`, the Q table; the Product Owner test card and F40's and F35's guide pages; the tests of T1 on the text passing (2 h)

Files: `playground/history/` (US-074's, the panel's steps beside the drawing's), `playground/kept-shapes/` (the new text being typed), `playground/placing/` and `playground/text/` (the text placed empty, its frame), `playground/index.html` (the panel's id). `@see` ADR-0095, `docs/domain/interaction.md#_7-navigation-and-history`, `REF-MDN-CHANGE-EVENT`, `REF-GIMP-UNDO`.

One task = one commit, referenced as `US-075.T1` in the commit footer.
