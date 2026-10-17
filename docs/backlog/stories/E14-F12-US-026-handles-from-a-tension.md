---
id: US-026
epic: E14
feature: F12
title: "Smooth a point by a tension, its handles computed"
status: done
points: 3
---

# E14 · F12 · US-026 — Smooth a point by a tension, its handles computed

As a symbol designer, I smooth a point by typing one tension, its handles computed from the points before and after it, as Inkscape's auto-smooth nodes and the cardinal spline do (`REF-INKSCAPE-NODE-TYPES`, `REF-WIKI-HERMITE`), so that the same tension gives the same kind of curve at every point.

## Acceptance criteria

- Given a point A (100, 0) between (0, 0) and (200, 100), when the user types tension 0 %, then A's handles are ±(next − previous) / 6 = ±(33.33333, 16.66667) (Catmull–Rom, a derivation from `m = (next − previous) / 2` and handles `m / 3`).
- Given tension 100 %, then both handles are (0, 0): the sides are straight at A.
- Given a tension outside 0 to 100, or not an integer, then it is refused.
- Given the neighbours move, when the point is in mode `auto` (Smooth by a tension), then its handles follow them, as an auto-smooth node does.

## Tasks

One task = one commit, referenced as `US-026.Tn`.

- [x] T1 — Derivation and function of handles from a tension
- [x] T2 — The tension field in the panel; handles kept up to date; tests

## Decisions during the story

| Decision | Reason |
| --- | --- |
| a fourth handle mode, `auto`, with a `tension` stored in the handles | the handles are computed, so the point must say it is smoothed; the tension is the only value typed |
| the automatic handles are computed from the points as drawn and stored as last computed (`smoothedCorners`), in the drawing and in the box of the curve | they follow the neighbours; a stretch commutes with them (both linear); the stored values read as they are drawn |
| a handle typed in mode `auto` turns the point to mode angle | the agent's choice: the handles stay on one line, the tension no longer applies; no source read states what a tool does here |
| a point in mode `auto` next to a point deleted is smoothed again from its new neighbours | that is what mode `auto` means: its handles follow its neighbours; the outer side moves (domain) |

## Product Owner test

Open the page, type **Width** = 100: points 1 (0, 0), 2 (100, 0), 3 (100, 80).

1. Click **Points**, point 2 (top right), **Handles**, then **Mode** = **Smooth by a tension**. A **Tension** field appears at 0: the corner is rounded smoothly from point 1 to point 3, its handles parallel to the line from point 1 to point 3.
2. Type **Tension** = 50: the curve tightens; 100: the sides are straight again.
3. Type **Tension** = 0, click point 3 and type its **X** = 150: point 2's curve follows, its handles turning towards the new place of point 3.
4. Click point 2 and type a handle's **After x**: **Mode** reads Mirror angle and the **Tension** field disappears; the handles no longer follow the neighbours.

## Review

Auditor, also in the verifier's role, before the last commit. Fixed: Inkscape's page was cited for what it does not say (a dragged auto-smooth handle, who moves) — the derivation cites the Hermite page, the switch to mode angle is the agent's choice; the automatic handles are now stored as computed; a point in mode auto next to a deleted point is smoothed again (domain); tests of mode auto when typing (before and after, a zero handle), a split next to an auto point, a tension out of range, the factors with an automatic bulge, the tension field's value.
