---
id: US-024
epic: E14
feature: F12
title: "Type a handle as an angle and a length"
status: ready
points: 3
---

# E14 · F12 · US-024 — Type a handle as an angle and a length

As a symbol designer, I type a handle as an angle and a length, measured from the page's x axis or from its side, so that the same values bend any side the same way.

## Acceptance criteria

- Given a point at (100, 0) and its next point at (200, 0), when the user types angle 330° and length 30 from the x axis, then the handle's end is at about (125.98, −15).
- Given the side from (100, 0) to (100, 100), when the user types angle 330° and length 30 from the side, then the handle leaves the side at the same 30° as on a horizontal side (a derivation).
- Given an angle not between 0 and 359 or a negative length, when typed, then it is refused.

## Tasks

One task = one commit, referenced as `US-024.Tn`.

- [ ] T1 — Derivation of a handle from an angle along its side
- [ ] T2 — The two angle methods in the panel; tests
