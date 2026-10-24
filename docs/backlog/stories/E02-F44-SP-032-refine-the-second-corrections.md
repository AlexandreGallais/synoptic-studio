---
id: SP-032
epic: E02
feature: F44
title: Refine the corrections from the second try
status: ready
points: 1
---

# E02 · F44 · SP-032 — Refine the corrections from the second try

A short spike that opens F44: the code each correction touches read, the bug reproduced, F44's stories written, the questions asked only where the code and the Product Owner's words leave a choice open.

## Acceptance criteria

- Given the three corrections below, when the spike ends, then each is traced to the playground's code it touches, the bug of correction 3 is reproduced with its steps and its cause, and F44's stories are written, 2 to 4 of at most 3 tasks each, criteria with numeric examples.
- Given what the code and the Product Owner's words leave open, when the spike ends, then the questions left are listed at once, each with options and a numeric example; none when the Product Owner's words settle it.

## The corrections asked (Product Owner, 2026-10-24)

1. **The helps as one dashed line, white and black**: the outline of the shape under the pointer, of each shape selected, the dashed box of several shapes and the selection rectangle are each one line of white and black dashes alternating, seen on every color, white and black included; US-080's accent over a white band is unseen on a white shape. In the layer above the drawing the line may be drawn by a native dashed stroke (`stroke-dasharray`), since the helps are not shapes: the shapes and texts never take a stroke, their edges drawn as paths (ADR-0002). That exception is a structural decision: an ADR, and the band computed about a contour (`contourBand`, EN-149) replaced for the helps by the shape's own path stroked, where the code allows (the two layers' row of the improvement log).
2. **The panel's fields the same for one shape as for several**: Shift with an arrow steps by 10 in one shape's X, Y, W and H as in several shapes' fields (US-081), today 1; the fields of one shape behave as the fields of several (a text field acting as a number field, its arrows, its bursts, its relative values), so that one rule holds everywhere.
3. **The view after a resize**: after the window is resized, or the pipeline panel shown or hidden, the hover is drawn offset from its shape, the drawing seems to shrink, and a tool's click sometimes places nothing (seen twice; once also a click on the work area after a text doing nothing): the view's canvas size, its `viewBox` and the pointer's point should follow the work area's size (`playground/view/read-canvas-size.ts`, `listenToView`, ADR-0098, decision on a resize).

Placed elsewhere, not in F44: the box of several shapes' position shown (E18); a group resized proportionally (E13, once groups exist), several shapes keeping each its own size as now.

## Tasks

- [ ] T1 — The code read, the bug reproduced, F44's stories drafted, the questions left (1 h)

One task = one commit, referenced as `SP-032.T1`.
