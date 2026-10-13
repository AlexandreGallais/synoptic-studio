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

- [ ] T1 — The switch and the mode in Points mode
- [ ] T2 — Handle fields as x and y, written into the model
- [ ] T3 — Handles drawn on the drawing; tests end to end
