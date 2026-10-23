---
id: US-070
epic: E02
feature: F38
title: The panel shows several shapes' position and size
status: draft
points: 3
---

# E02 · F38 · US-070 — The panel shows several shapes' position and size

As a symbol designer, I want to read the position and size of the shapes selected, so that I see at once whether they line up and match.

Written at SP-027 (note 0028, §4). Figma and Penpot show each shape's values, Mixed where they differ, and give a value typed to every shape selected (`REF-FIGMA-ADJUST`, `REF-PENPOT-MEASURES`); Inkscape and Affinity Designer show the box of the whole selection and transform it as one (`REF-INKSCAPE-SELECTOR`, `REF-AFFINITY-MOVE-TOOL`). Draft until SP-027's questions 4 (which way), 5 (what is typed in F37) and 6 (a text among several) are answered; if values are typed in F37, this story only shows them and the typing goes to a feature of its own (question 8).

## Acceptance criteria

The examples select two straight rectangles: A at (10, 20), 100 × 50, and B at (200, 20), 40 × 40.

- Given A and B selected, then the panel shows X, Y, W and H as SP-027's question 4 decides: X Mixed, Y 20, W Mixed, H Mixed, each shape's values (Figma); or X 10, Y 20, W 230, H 50, the box of the two (Inkscape).
- Given A turned by 30° among the shapes selected, then its X and Y are those of its straight box, as for one shape (`interaction.md` §2, ADR-0035), its W and H its own.
- Given a text among the shapes selected, then its values are those of its frame, its block or its box when it sits in one (question 6).
- Given A and B selected, when a value is typed in X, Y, W or H, then it does what question 5 decides: nothing, the fields shown but disabled until E02's next feature (x and y) and E18 (width and height); or each shape set to the value (Figma); or the selection moved or scaled as one (Inkscape).
- Given a value shown as Mixed, then its field is empty with the placeholder Mixed, as the radius of vertices of different radii (F03).
- Given one shape left selected, then the panel shows and edits it as before (F36).

## Product Owner test

`npm run dev`, `http://localhost:5173/`.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Open the page, then Reset; set the rectangle to X 10, Y 20, W 100, H 50; add a rectangle and set it to X 200, Y 20, W 40, H 40 | two rectangles on one line |
| 2 | Select both with Shift + click | X, Y, W and H as question 4 decides |
| 3 | Type 50 in X | what question 5 decides |
| 4 | Click one rectangle alone | its own values, editable as before |

## Tasks

- [ ] T1 — The values of several shapes, common or Mixed, or their box, by the answer to question 4: interface and failing tests (1 h)
- [ ] T2 — The panel shows them for several shapes, the fields as question 5 decides, the tests of T1 passing (2 h)
- [ ] T3 — The rules in `interaction.md` §2; the Product Owner test card (1 h)

Files: `src/model/drawing/` (the values of several shapes, from each straight box, `contourBoundingBox` and `textStraightBox`, the common value as `commonRadius` takes it), `playground/selection/` (the panel for several shapes), `playground/index.html` (the fields kept shown for several shapes), `playground/mount/update-page.ts`. `@see` `docs/domain/interaction.md#_2-selector`, `REF-FIGMA-ADJUST`, `REF-PENPOT-MEASURES`.

One task = one commit, referenced as `US-070.Tn` in the commit footer.
