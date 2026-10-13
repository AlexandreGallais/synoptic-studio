---
id: US-023
epic: E14
feature: F12
title: "Type a handle as dx and dy from its point"
status: ready
points: 2
---

# E14 · F12 · US-023 — Type a handle as dx and dy from its point

As a symbol designer, I type a handle as the step from its point, dx and dy, as Figma stores it, so that moving the point keeps the handle's shape.

## Acceptance criteria

- Given a point at (100, 0), when the user chooses dx, dy and types dx = 30, dy = −15, then the handle's end is at (130, −15).
- Given that handle, when the point is moved to (110, 0), then the handle still reads dx = 30, dy = −15.

## Tasks

One task = one commit, referenced as `US-023.Tn`.

- [x] T1 — The method selector and dx, dy fields; tests

## Decisions during the story

| Decision                                                                                      | Reason                                                        |
| --------------------------------------------------------------------------------------------- | ------------------------------------------------------------- |
| the way to type the handles is a list of the page (`#handle-method`), not stored in the model | it changes only how the same vectors are shown and typed      |
| the fields keep their ids; their labels read x, y or dx, dy                                   | one set of fields for every way, as the next stories add more |

## Product Owner test

Open the page (`npm run dev`, `http://localhost:5173`), type **Width** = 100: point 2 is at (100, 0).

1. Click **Points**, then point 2 (green, top right), then **Handles**.
2. In **Typed as**, choose **dx, dy from the point**: the fields read dx and dy, all 0.
3. Type **After dx** = 30, then **After dy** = −15: the handle's end is at (130, −15), its mirror at (−30, 15) from the point.
4. Choose **x, y on the page** again: the same handle reads 130, −15 and 70, 15.
5. Choose **dx, dy** again, then type the point's **X** = 110: the handle still reads 30, −15; it moved with its point.
