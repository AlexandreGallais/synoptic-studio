---
id: US-022
epic: E14
feature: F12
title: "Switch a point to handles and type them as x and y"
status: ready
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

- [x] T1 — The switch and the mode in Points mode (with `freeShapeWithHandles`)
- [x] T2 — Handle fields as x and y, written into the model
- [x] T3 — Handles drawn on the drawing; tests end to end (with T2: one commit)

## Decisions during the story

| Decision                                                                                    | Reason                                                                                                                           |
| ------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| the switch is shown for one point selected of a rectangle or a free shape, not of a polygon | as the point's x and y (F08): a polygon's points are not typed; the rectangle becomes a free shape when its handles change (F09) |
| handles live in a new playground folder, `handles/`                                         | a panel of its own (ADR-0043); `points/` calls it                                                                                |
| the handles are drawn in Points mode only, a stem and a square in violet                    | as the points' marks; fills, never a native stroke (US-018)                                                                      |
| a handle typed as a decimal changes nothing, its field keeps the text                       | as the point's x and y                                                                                                           |

## Product Owner test

Run `npm run dev`, open `http://localhost:5173/?step=1`: the rectangle 120 × 80, radius 10.

1. Click **Points** in the toolbar, then point 1 (the red one, top left). Look at the panel: a **Handles** button above the radius.
2. Click **Handles**. The radius field disappears, replaced by **Mode** (Mirror angle and length) and four fields: the ends of the two handles, both at the point (0, 0).
3. Type **After x** = 30, Enter, then **After y** = −15, Enter. The top side bends upwards from point 1; the other handle moves to (−30, 15), its mirror; two violet marks show the handles.
4. Choose **Mode** = No mirroring, then type **After x** = 60: only that handle moves.
5. Click **Handles** again: the corner is round again, radius 10, as before.

What to look at: the shape's points never move; only the side next to point 1 bends.
