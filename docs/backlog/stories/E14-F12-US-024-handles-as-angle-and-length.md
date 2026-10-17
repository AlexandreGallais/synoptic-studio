---
id: US-024
epic: E14
feature: F12
title: "Type a handle as an angle and a length"
status: done
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

- [x] T1 — Derivation of a handle from an angle along its side
- [x] T2 — The two angle methods in the panel; tests

## Decisions during the story

| Decision | Reason |
| --- | --- |
| the four ways to type a handle are a model list, `HANDLE_WAYS`, with `fieldValuesOfHandle` and `typedHandle` | the conversions are business rules, tested on their own; the playground only reads and writes fields |
| an angle typed while the handle has no length stays in its field until the length is typed | a handle of length 0 has no angle; typing the angle first would otherwise be lost |
| the handle before's angle from the side is mirrored | the same values at both ends make a symmetric arch (Product Owner, 2026-10-13) |

## Product Owner test

Open the page, type **Width** = 100: points 1 (0, 0), 2 (100, 0), 3 (100, 80).

1. Click **Points**, point 1, then **Handles**; in **Typed as** choose **angle, length from the x axis**. The fields read angle and length.
2. Type **After angle** = 330, then **After length** = 30: the top side bends upwards, the handle's end at about (25.98, −15). Angles turn clockwise on screen, as a shape's angle. Point 1 is in Mirror angle and length, so its other handle bends the left side outwards too.
3. Click point 2 (top right), **Handles**, **Mode** = No mirroring, and **Typed as** = **angle, length from the side**. Type **Before angle** = 330, **Before length** = 30.
4. Click point 1 again: with **Typed as** still from the side, its After angle already reads 330, as the top side runs along the x axis.

What to look at: with the same angle and length at both ends, the top side is a symmetric arch above the shape. Type **After angle** = 370: it reads 10. A negative length is refused: the field shows it, the drawing keeps the last length.

## Review

Auditor and verifier, before the last commit. Fixed: a handle of length 0 showed a number left by another way or point as its angle — the angle typed is now kept for its point and way; tests that the axis way ignores the side and is never mirrored, that a length of 0 is accepted, that −90° reads 270° and 370° reads 10°, that the page way counts y from the point, a handle before at point 1 against the last side, the before fields read back; the domain rules of angles; the derivation's sources for the angle between two vectors; dead `+ 0` removed. Kept: a refused negative length stays in its field (the card says so).
