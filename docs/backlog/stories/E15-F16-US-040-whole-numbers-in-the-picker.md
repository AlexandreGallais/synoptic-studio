---
id: US-040
epic: E15
feature: F16
title: Whole numbers in the color picker
status: ready
points: 2
---

# E15 · F16 · US-040 — Whole numbers in the color picker

As a designer, I want the picker to show and take whole numbers only, while a color I type in hexadecimal keeps its exact value, so that I read and type simple values without the color drifting.

Product Owner decision of 2026-10-15 (ADR-0058), built on EN-043. The picker's look is polished later. Built on the latest branch of the chain, `feature/f15-fonts-html`, after US-036 and EN-043.

## Acceptance criteria

- Given `#ff8000` typed, when the picker opens, then it shows 30, 100, 100 in HSB and 30, 100, 50 in HSL, no decimal, and the hexadecimal `#ff8000` unchanged.
- Given `#336699`, when the designer types 61 in the brightness, then the color is `#34689c` and the saturation still shows 67.
- Given `#336699`, when the designer leaves the saturation at 67 and presses Enter, then the color stays `#336699`.
- Given the hue slider at 30, when the designer presses Right, then 31; End gives 359; Page Up from 30 gives 40.
- Given a click on the plane, then the saturation and the lightness or brightness are whole numbers.

## Product Owner test

Open `http://localhost:5173/`, click **Text**, click on the drawing, then the color swatch.

| Step | Do                                          | You should see                        |
| ---- | ------------------------------------------- | ------------------------------------- |
| 1    | Type `#ff8000` in the hexadecimal field     | HSB 30, 100, 100: no decimal anywhere |
| 2    | Type `#336699`, then 61 in the brightness   | `#34689c`; the saturation still 67    |
| 3    | Type `#336699`, Enter in the saturation, 67 | `#336699` unchanged                   |
| 4    | Press End on the hue slider                 | 359                                   |

## Tasks

One task = one commit, referenced as `US-040.Tn`.

- [ ] T1 — The picker's fields and sliders show and take integers, rebuilt through EN-043; the hue slider from 0 to 359; the playground's tests first (2 h)
- [ ] T2 — F16's guided steps and VAL-016's decisions 8, 12, 17 and 19 brought up to date; the test card (1 h)
