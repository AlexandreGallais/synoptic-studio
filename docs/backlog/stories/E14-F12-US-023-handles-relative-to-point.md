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

- [ ] T1 — The method selector and dx, dy fields; tests
