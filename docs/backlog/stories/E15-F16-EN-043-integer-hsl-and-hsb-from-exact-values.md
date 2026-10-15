---
id: EN-043
epic: E15
feature: F16
title: HSL and HSB as integers, one value edited with the exact others
status: ready
points: 2
---

# E15 · F16 · EN-043 — HSL and HSB as integers, one value edited with the exact others

The library shows HSL and HSB as integers and rebuilds a color from one value typed as an integer and the exact values of the two others, computed from the hexadecimal, so that editing one value never moves the others (ADR-0058, superseding ADR-0056's decision 4).

Product Owner decision of 2026-10-15, from a try of the playground before VAL-016. Built on the latest branch of the chain, `feature/f15-fonts-html`, after US-036.

## Acceptance criteria

- Given `#ff8000`, when shown, then HSL is 30°, 100 %, 50 % and HSB 30°, 100 %, 100 %; given a hue that rounds to 360, then it is 0 (`nearestInteger`, a tie up).
- Given `#336699`, exactly HSB 210°, 66.66667 %, 60 %, shown 210, 67, 60, when its brightness is set to 61, then the color is `#34689c`, from 210°, 66.66667 % and 61 % (red 51.85 → 52, green 103.7 → 104, blue 155.55 → 156), not `#33679c`, which the three shown integers would give.
- Given `#336699`, when its saturation is set to 67, the value shown, then the color stays `#336699` (agent's reading, to confirm at VAL-016).
- Given HSL 30°, 100 %, 50 % set from an empty color, then `#ff8000`, the green 127.5 rounded up, as before.
- Given any color and any one of its three values set to its shown integer plus 1, when rebuilt, then the two other values, recomputed from the new hexadecimal, stay within one byte's step of their exact values before (property test, `DERIV-color-spaces`).

## Tasks

One task = one commit, referenced as `EN-043.Tn`.

- [ ] T1 — The values shown as integers, a hue of 360 shown 0, `@see ADR-0058`, `DERIV-color-spaces` step 9; tests first (1 h)
- [ ] T2 — A color rebuilt from one value set as an integer and the exact values of the two others, a value equal to the one shown keeping the color; tests first, the cases above, property tests (2 h)
