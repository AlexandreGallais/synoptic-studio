---
id: EN-148
epic: E02
feature: F42
title: The view of the drawing in the model
status: done
points: 2
---

# E02 · F42 · EN-148 — The view of the drawing in the model

The view as ADR-0098 sets it, accepted on 2026-10-23 (SP-030's answer 1): a scale, CSS pixels per user unit, and an origin, the point of the drawing at the canvas's top-left; never in the drawing, the history nor the export. Plain data and `domain` functions of `src/model/view/`: its `viewBox`, the point under the pointer, a zoom about a point within the bounds, a step, a pan, a fit to a box ([note 0031](../../research/notes/0031-f42-zoom-and-pan.md), §2, §4).

## Acceptance criteria

The examples use a canvas of 1 200 × 800 CSS pixels and the opening view, scale 1, origin (−40, −40).

- Given the opening view, then its `viewBox` is (−40, −40, 1 200, 800), today's; given scale 2 and origin (180, 130), it is (180, 130, 600, 400), the canvas's aspect (`REF-SVG2-COORDS`, §8.2).
- Given scale 2 and origin (180, 130), when the pointer is at (440, 340) on the canvas, then the point under it is (180 + 440 / 2, 130 + 340 / 2) = (400, 300); at the opening view, (40, 40) is (0, 0).
- Given the opening view and the pointer at (440, 340), over (400, 300), when the view is zoomed to scale 2 about the pointer, then its origin is (400 − 440 / 2, 300 − 340 / 2) = (180, 130), the point under the pointer still (400, 300) (`DERIV-view-zoom`); a scale beyond the bounds is taken at the bound, SP-030's answer 5: 300 asked with Inkscape's 256 gives 256, 0.001 gives 0.01.
- Given a scale, when it is stepped up or down, then it is multiplied or divided by a key's step, × √2 (SP-030's answer 4), within the bounds: from 1 it gives 1.41421, from 256 stays 256; the wheel and the pinch zoom by their deltas' sum δ, × (1 + 0.0057 · |δ|) for a δ toward the screen, its inverse for a positive δ (SP-030's answer 4): −100 pixels give × 1.57, 100 give × 0.63694.
- Given scale 2 and origin (180, 130), when the view is panned by a drag of (100, 50) screen pixels, then its origin is (130, 105); by a wheel's deltas of (0, 100), (180, 180).
- Given the straight box from (10, 20) to (410, 220) and the fits' padding of 40 pixels (SP-030's answer 6), when the view is fitted to it, then its scale is min(1 120 / 400, 720 / 200) = 2.8 and its origin (210 − 600 / 2.8, 120 − 400 / 2.8) = (−4.28571, −22.85714), within the bounds; a box 0 wide is fitted by its height alone, a box of 0 × 0 is centred at the view's scale (agent's choice).
- Given any view, then no function changes the view or the box given to it.

## Tasks

- [x] T1 — The view's type and its functions (`src/model/view/`: the `viewBox`, the point under the pointer, a zoom about a point, a step, a pan, a fit) with their TSDoc, `@kind domain`, and the tests of the criteria, hand-computed views with their computation in a comment, then a property test (a zoom keeps the point under the pointer); `DERIV-view-zoom` written for the `math-expert`'s check; failing (1 h)
- [x] T2 — The view computed, the tests of T1 passing; ADR-0098 named in `interaction.md` §7, the zoom's rules there and their Q row (2 h)

Files: `src/model/view/` (new), `src/render/view-box.ts` (the `ViewBox` type, reused, moved to `src/model/view/view-box.ts` since the model imports no module of `src/render/`, a layer above it), `docs/derivations/` (`DERIV-view-zoom`). `@see` ADR-0098, `REF-SVG2-COORDS`, `REF-PENPOT-ZOOM`, `REF-INKSCAPE-ZOOM-PREFS`, `DERIV-view-zoom`, `docs/domain/interaction.md#_7-navigation-and-history`.

One task = one commit, referenced as `EN-148.T1` in the commit footer.
