---
id: US-022
epic: E14
feature: F12
title: "Switch a point to handles and type them as x and y"
status: done
points: 5
---

# E14 · F12 · US-022 — Switch a point to handles and type them as x and y

As a symbol designer, I switch a selected point from its corner radius to its handles and type each handle's end as x and y on the page, in one of Figma's three modes, so that a side bends.

## Acceptance criteria

- Given Points mode and one point selected, when the user switches it to handles, then two handle fields appear (before, after), each as x and y, with the mode, angle and length by default; the radius field is replaced, its value kept.
- Given a point at (100, 0) in mode angle and length, when the user types the handle after at x = 130, y = −15, then the side bends and the handle before shows x = 70, y = 15.
- Given handles on, when the user switches back to radius, then the radius typed before is drawn again.
- Given the drawing, when a point is in handles mode, then its two handles are drawn as small marks joined to the point.

## Tasks

One task = one commit, referenced as `US-022.Tn`.

- [x] T1 — `freeShapeWithHandles`, the model side of the switch and the fields
- [x] T2 — The switch, the mode and the handle fields as x and y in Points mode
- [x] T3 — Handles drawn on the drawing (with T2); the test card and the review

## Decisions during the story

| Decision | Reason |
| --- | --- |
| the switch is shown for one point selected of a rectangle or a free shape, not of a polygon | as the point's x and y (F08): a polygon's points are not typed; the rectangle becomes a free shape when its handles change (F09) |
| handles live in a new playground folder, `handles/` | a panel of its own (ADR-0043); `points/` calls it |
| the handles are drawn in Points mode only, a stem and a square in violet | as the points' marks; fills, never a native stroke (US-018) |
| a handle typed as a decimal changes nothing, its field keeps the text | as the point's x and y |

## Product Owner test

Run `npm run dev` and open the page (`http://localhost:5173`): the rectangle 120 × 80, radius 10, point 1 at the top left (0, 0).

1. Click **Points** in the toolbar, then point 1 (the red one). In the panel, a **Handles** button sits above the radius.
2. Click **Handles**: the button looks pressed, the radius field gives way to **Mode** (Mirror angle and length) and four fields, the ends of the two handles, both at the point (0, 0). Point 1 is now sharp: in handles mode a point has no rounding.
3. Type **After x** = 30, Enter, then **After y** = −15, Enter. The top side bends upwards from point 1, and the left side bulges to the left: the other handle is the mirror, at (−30, 15). Points 2 and 4, next to a curved side, lose their rounding. Two violet marks show the handles; Width and Height read 133.33333 × 86.66667, the box of the curve.
4. Choose **Mode** = No mirroring, then type **After x** = 60: only that handle moves, the left side stays as it is.
5. Click **Handles** again: point 1 is round again, radius 10, and the shape is the rectangle 120 × 80 once more.

What to look at: the points never move, only the sides next to point 1 bend; the radius comes back as it was.

## Review

Auditor and verifier, before the last commit. Fixed: one coordinate of a handle typed alone was refused on a turned shape (the other read as a decimal); tests on a point away from the origin (absolute x and y told from a vector), the marks' place, the handle before, mode angle, the mode shown when the selection changes, no mark for zero or off handles, no switch for two points or a polygon; the other points' handles untouched; the test card now says what the page shows (the mirrored handle bends the previous side, the neighbours lose their rounding, the box grows); the pressed switch styled; the tags. Kept: `readOnlySelected` is written in `points/` and in `handles/`, a three-line twin.
