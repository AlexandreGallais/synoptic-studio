---
id: US-087
epic: E02
feature: F44
title: Every number field of the panel the same
status: ready
points: 3
---

# E02 · F44 · US-087 — Every number field of the panel the same

As a symbol designer, I want every number field of the panel to answer the keyboard the same way, so that I never wonder which field steps by 10 with Shift or takes `+10`.

Asked by the Product Owner on 2026-10-24, after SP-032: the rule of US-085 is for every number field, not for X, Y, Width and Height alone, overriding SP-032's choice 2, which kept the other fields on the browser's step of 1. After US-085, which makes the rule for the fields one shape and several share; this story carries it to the panel's other number fields: the corners and the radius, the polygon's sides, the angle, a text's size, line height, letter and word spacing and its lines, a point's X and Y and its handles, the history's memory size and steps kept, and any other `type="number"` field of `playground/index.html` (the list read in T1 from the page).

## Acceptance criteria

The examples use the page as it opens, the rectangle 120 × 80 at (0, 0), selected alone, its corner radius 10.

- Given the radius field at 10, when Up is pressed, then it is 11; Shift + Up from 10, 20; Shift + Down from 20, 10; with Ctrl, Cmd or Alt held, the field keeps the key, as US-081 and US-085.
- Given `+5` typed in the radius and Enter, then the radius is 15; a value a field refuses (a radius of `-5`, an angle of `abc`) leaves it unchanged and shows its value again, the field's own bounds kept (a polygon's sides at least 3).
- Given Up held in the radius for a second, 30 keys, then released, then the history has one step for the burst, one Ctrl + Z giving 10 (`listenToArrowBursts`, 500 ms).
- Given a field whose step is not 1 (a font's size, the memory size in MB), then Up and Shift + Up step by its own step and ten times it; the list of fields and their steps written in `interaction.md` §2.
- Given every `type="number"` field of the page, then a test reads them all and finds each one under the rule, so that a field added later without it fails.

## Product Owner test

`npm run dev`, `http://localhost:5173/`. The page opens with a rectangle 120 × 80 at (0, 0), selected.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Click in the corner radius and press Shift + Up | the radius 10 more, the corners rounder |
| 2 | Type `+5` in the radius, then Enter | the radius 5 more, only once Enter is pressed |
| 3 | Pick the polygon, click in its sides and press Shift + Up | 10 more sides |
| 4 | Hold Up in the radius for a second, release, then Ctrl + Z (Cmd + Z on a Mac) | one undo brings the radius back |

## Tasks

- [ ] T1 — The page's number fields listed with their steps and bounds, the rule of US-085 given to them all as text fields acting as number fields; `interaction.md` §2 and its Q row; tests of the criteria first, failing, the test over every field included (1 h)
- [ ] T2 — The code that passes them, each field's `input` listener replaced by `change` and `keydown` as US-085's, its burst and its step; the existing tests of those fields brought to the new behaviour (2 h)
- [ ] T3 — The Product Owner test card, and the guide pages that name a field's step (1 h)

One task = one commit, referenced as `US-087.Tn` in the commit footer.
