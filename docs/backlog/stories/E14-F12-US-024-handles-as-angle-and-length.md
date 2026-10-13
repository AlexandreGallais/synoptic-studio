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
- Given an angle outside 0 to 359, when typed, then it is brought back into that range as a shape's angle is (370 becomes 10, −90 becomes 270); a negative length or a side of length 0 is refused.
- Given the same angle and length typed at both ends of a side, then the two handles leave towards the outside, a symmetric arch (Product Owner, 2026-10-13).

## Tasks

One task = one commit, referenced as `US-024.Tn`.

- [ ] T1 — Derivation of a handle from an angle along its side
- [ ] T2 — The two angle methods in the panel; tests
