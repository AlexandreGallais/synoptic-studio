---
id: US-070
epic: E02
feature: F38
title: The panel shows several shapes' position and size
status: done
points: 3
---

# E02 · F38 · US-070 — The panel shows several shapes' position and size

As a symbol designer, I want to read the position and size of the shapes selected, so that I see at once whether they line up and match.

Written at SP-027 (note 0028, §4). Figma and Penpot show each shape's values, Mixed where they differ, and give a value typed to every shape selected (`REF-FIGMA-ADJUST`, `REF-PENPOT-MEASURES`); Inkscape and Affinity Designer show the box of the whole selection and transform it as one (`REF-INKSCAPE-SELECTOR`, `REF-AFFINITY-MOVE-TOOL`). Brought in line with SP-027's answers at T1 (2026-10-22, 2026-10-23): each shape's values, Mixed where they differ (question 4), as Figma; a value typed for several shapes, relative or fixed (questions 4 and 5), and a text fitting its content keeping its size (question 6) are US-071's, the next story, so that this one only shows the values, its fields read-only. F38 is not validated by a VAL (VAL-039 dropped): the agent's choices go to `interaction.md`, the Q table and F38's guide page, for the review's summary.

## Acceptance criteria

The examples select two straight rectangles, sharp: A at (10, 20), 100 × 50, and B at (200, 20), 40 × 40.

- Given A and B selected, then the panel shows X, Y, W and H, each shape's values, a value common to them as it is and Mixed where they differ: X Mixed, Y 20, W Mixed, H Mixed.
- Given a value shown as Mixed, then its field is empty with the placeholder Mixed, as the radius of vertices of different radii (F03); a common value shows as one shape's does, with the SVG's decimals.
- Given A turned by 30° among the shapes selected, then its X and Y are those of its straight box, as for one shape (`interaction.md` §2, ADR-0035), its W and H its own: X 4.19873, Y −1.65064, W 100, H 50, its half sizes about its middle (60, 45) being 50 cos 30° + 25 sin 30° = 55.80127 across and 50 sin 30° + 25 cos 30° = 46.65064 down.
- Given a text among the shapes selected, then its values are those of its frame, its block or its box when it sits in one, as under the pointer: its X and Y those of its straight box (`textStraightBox`), its W and H its frame's. B and a text in a box 40 × 40 at (200, 20) show X 200, Y 20, W 40, H 40; a text whose block is 80 × 30 at (300, 20), turned by 90° about its middle (340, 35), has X 325, Y −5, W 80, H 30.
- Given values that differ by no more than `EPSILON` (ADR-0003), then they are common, the first one shown: a side computed at 1.1 × 10⁻¹⁵ and one at 0 show 0 (agent's choice).
- Given a text never measured among the shapes selected, which is not drawn, then it is left out of the values, as out of the dashed box (agent's choice); with none left, each field is empty, without a placeholder.
- Given several shapes selected, then the four fields are read-only until US-071 brings the typing, and the panel's other fields stay hidden (agent's choice).
- Given one shape selected, or none, then the four fields are hidden and the panel shows and edits it as before (F36).

## Product Owner test

`npm run dev`, `http://localhost:5173/`.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Open the page, then Reset; set the rectangle to X 10, Y 20, W 100, H 50; add a rectangle and set it to X 200, Y 20, W 40, H 40 | two rectangles on one line |
| 2 | Select both with Shift + click | X empty reading Mixed, Y 20, Width and Height empty reading Mixed |
| 3 | Click in X and type 50 | nothing typed: the fields are read-only until the next story |
| 4 | Click the second rectangle alone, set its Y to 30, then Shift + click the first | Y now empty reading Mixed |
| 5 | Click one rectangle alone | its own values, editable as before, and the four fields of several shapes gone |

## Tasks

- [x] T1 — The criteria in line with SP-027's answers; the values of several shapes, common or Mixed (`src/model/drawing/`), and the panel's fields for several shapes: interface and failing tests (1 h)
- [x] T2 — The panel shows them for several shapes, read-only; the rules in `interaction.md` §2; the tests of T1 passing (2 h)
- [x] T3 — The Q table, F38's guide page; the Product Owner test card (1 h)

Files: `src/model/drawing/` (each shape's values, from its straight box, `contourBoundingBox` and `textStraightBox`, and their common values within `EPSILON`), `playground/selection/` (the fields for several shapes), `playground/hover/` (a text's block, read once for its area and its values), `playground/index.html` (the four fields of several shapes, beside the panel's), `playground/mount/update-page.ts`. `@see` `docs/domain/interaction.md#_2-selector`, `REF-FIGMA-ADJUST`, `REF-PENPOT-MEASURES`.

One task = one commit, referenced as `US-070.Tn` in the commit footer.
