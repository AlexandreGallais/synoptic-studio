---
id: US-025
epic: E14
feature: F12
title: "Type a side's handles as shares of the side, or paste curve data"
status: ready
points: 3
---

# E14 · F12 · US-025 — Type a side's handles as shares of the side, or paste curve data

As a symbol designer, I type a side's two handles as whole percentages of the side, as CSS `cubic-bezier()` does, or paste a `C` segment or a `cubic-bezier()` value, so that a curve I know is reused on any side.

## Acceptance criteria

- Given the side from (100, 0) to (200, 0), when the user types 25 % along and 30 % across for the first handle, then its end is at (125, −30) (a derivation).
- Given the text `cubic-bezier(0.42, 0, 0.58, 1)` pasted on a side, then its two handles are set as those shares of the side.
- Given the text `C 130 -15 170 -15 200 0` pasted for the side from (100, 0), then its handles are set to those points; any other text is refused with a line saying why.

## Tasks

One task = one commit, referenced as `US-025.Tn`.

- [ ] T1 — Derivation of a handle from shares of its side
- [ ] T2 — The share fields; the parser of `C` and `cubic-bezier()`; tests
