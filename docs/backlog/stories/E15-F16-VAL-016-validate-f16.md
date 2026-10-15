---
id: VAL-016
epic: E15
feature: F16
title: Validate F16
status: ready
points: 1
---

# E15 · F16 · VAL-016 — Validate F16

The Product Owner types, pastes and picks colors in the playground, guided by short steps, and says what they see (ADR-0020).

## Acceptance criteria

- Given the playground, when the Product Owner follows the guided steps, then each way to type a color and each part of the picker is tried, and each rule marked "to confirm at VAL-016" in `docs/domain/interaction.md` (Colors), Q38 and ADR-0056 is confirmed or changed, with its example.

## Tasks

One task = one commit, referenced as `VAL-016.Tn`.

- [ ] T1 — The guided test, tested end to end (1 h)
- [ ] T2 — The demo page (30 min)
- [ ] T3 — The Product Owner's validation (1 h)

## Decisions to confirm

The rules of colors marked provisional in `docs/domain/interaction.md` (Colors), Q38 and ADR-0056, read by the agent from F05's refinement at SP-017 (2026-10-15).

| #   | Rule                                                                                                                                                         | Example                                                                               |
| --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------- |
| 1   | The hexadecimal and the opacity are stored; HSL and HSB are computed from them, never stored (ADR-0056)                                                      | a file holds `#ff8000` and 100, never 30.1°                                           |
| 2   | A color is typed with or without `#`, in any case, white space around it ignored, and kept as `#` and six lower-case digits                                  | `FF8000`, and `#ff8000` with spaces around it, give `#ff8000`                         |
| 3   | 3 digits are each doubled; 4 and 8 digits carry an alpha, which sets the opacity to the nearest percent                                                      | `f80` gives `#ff8800`; `#0000ffcc` gives `#0000ff` and 80 %; an alpha `ec` gives 93 % |
| 4   | Anything else is refused, the field showing its previous color; a color copied is six digits, the opacity copied from its own field                          | `red`, `#ff800` refused                                                               |
| 5   | The picker opens on a click on the swatch and closes on another click on it, on Escape or on a click outside                                                 | —                                                                                     |
| 6   | The plane is in HSB first, as Figma's square; HSL is one click away                                                                                          | —                                                                                     |
| 7   | The plane shows the saturation across, 0 at the left, and the lightness or brightness up, 100 % at the top                                                   | HSB at 210°, the middle of the right edge: `#004080`                                  |
| 8   | HSL and HSB values are shown to one decimal, so that typed back they give the same hexadecimal; integers, as Figma shows, change nearly 9 colors in 10       | `#336699` is HSB 210°, 66.7 %, 60 %; typed as 67 %, it would give `#326699`           |
| 9   | A value typed or picked becomes the hexadecimal by rounding each of red, green and blue to the nearest, a tie up                                             | HSL 30°, 100 %, 50 %: the green 127.5 gives 128, `#ff8000`                            |
| 10  | A grey shows the hue 0°; an open picker keeps the hue its slider shows                                                                                       | `#808080` with the slider at 200°, dragged to more saturation, keeps 200°             |
| 11  | Keys: arrows 1, Page Up and Page Down 10, Home and End to the ends, on each slider; on the plane, Left and Right the saturation, Up and Down the other value | hue 210°: Right 211°, Page Up 221°                                                    |
