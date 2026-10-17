---
id: US-054
epic: E15
feature: F17
title: X and Y of a turned text show its turned top-left corner
status: done
points: 3
---

# E15 · F17 · US-054 — X and Y of a turned text show its turned top-left corner

As a symbol designer, I want X and Y of a turned text to say where its top-left corner is drawn, so that turning a text changes what they show and typing them moves the text where I see it.

From the Product Owner's first pass of VAL-018 (2026-10-16), finding D4: turning a text left X and Y unchanged. They kept the top-left of the block, or of the box, before it turns (US-032, `shapes.md`, Text), while a turned shape shows the top-left of its straight box (ADR-0035). The same pass reported a crash, seen once while turning a text by +90°, which recovered.

**Decision** (provisional, taken while the Product Owner was away, to confirm at VAL-018's second pass; `docs/domain/shapes.md`, Text, Turning): X and Y of a turned text show its **turned top-left corner**, the corner a resize keeps in place (ADR-0057), as Figma, whose x and y stay at a layer's top-left when it turns (`REF-FIGMA-ROTATION`). They change when the angle changes, the text still turning about its middle (ADR-0055, decision 3). Typing X or Y moves that corner to the integer typed; the angle, the size and the other coordinate stay. At 0° the corner is the position, so nothing changes.

**What is stored** (no business choice: ADR-0035 and ADR-0057 settle it): the model keeps the top-left of the frame before it turns (ADR-0035). A corner typed is an integer **in the drawing**, as a coordinate typed on a turned shape (ADR-0035, last decision); the position stored is computed from it, `b = P − k + R(k)`, `P` the corner typed, `k` half the frame, `R` the turn (`positionKeepingPoint`, `DERIV-turned-shape` step 5), and may be fractional (ADR-0057, decision 5). At 0° it is the integer typed, bit for bit. A corner left as shown keeps its exact value behind its 5 decimals, as the position did (US-038).

**The crash at +90°**: not reproduced. 360 runs in the test page — Fit content and Box, a box 200 × 100, 0 × 0, 10 × 10, 50 × 0 and 0 × 40, 0, 1 and 2 lines at most, with and without the ellipsis, six contents among them an empty one, a word too long and five lines — each turned by 23°, six times by +90°, a letter spacing then −90°: no error, the text drawn each time. More detail is needed from the Product Owner's second pass: the text, the box, the angle before, and what was seen.

## Acceptance criteria (F17.AC12)

- Given a text 100 × 20 in a box at (47, 61), when it is turned by 23, then X and Y show 54.88207 and 42.2584, its turned top-left corner, and its stored position stays (47, 61).
- Given that text, when X is typed 60, then its turned top-left corner is drawn at x 60, y 42.25840, its angle and its size unchanged; its stored position is (52.11793, 61).
- Given a text in a box 120 × 40 at (47, 61) turned by 23, when +90° is pressed, then X and Y show its corner turned by 113 about the same middle, (148.85396, 33.58433).
- Given a turned text that fits its content, when its letter spacing grows, then X and Y keep showing the same corner (ADR-0057).
- Given a text at 0°, then X and Y show its position, as before.

## Product Owner test

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Text tool, Box, width 100, height 20, X 47, Y 61 | X 47, Y 61 |
| 2 | Type 23 in the angle | X 54.88207, Y 42.2584: the turned top-left corner; the text turned about its middle |
| 3 | Type 60 in X | the text slides right, its top-left corner at x 60, its angle and its size unchanged; Y kept |
| 4 | Press +90° twice | X and Y change at each press, the text turning about its middle |
| 5 | Fit content, letter spacing 10 | X and Y unchanged, the text grows along its direction from that corner |
| 6 | Angle 0 | X and Y show the top-left of the text, as before |
| 7 | Try again to turn a text by +90° in a box, with the ellipsis on | does the page still stop for a moment? If so, say the text, the box and the angle before |

## Tasks

One task = one commit, referenced as `US-054.Tn`.

- [x] T1 — X and Y show a turned text's top-left corner, a corner typed placing the text; `shapes.md` (2 h)
