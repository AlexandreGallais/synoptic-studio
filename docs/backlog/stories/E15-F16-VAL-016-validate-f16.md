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

- [x] T1 — The guided test, tested end to end (1 h)
- [x] T2 — The demo page (30 min)
- [ ] T3 — The Product Owner's validation (1 h)

## Guided test (T1)

40 steps at `http://localhost:5173`, also after F14's at `?guide=epic` (steps 162 to 201): one line to do, one to look at, Next typing the text and its colors. `playground/guided-steps/epic-15/f16-steps.test.ts` checks each step end to end, each decision's example among them. The picker's model is not typed by Next: a step that needs one names it.

| Steps | What                                                                                                              |
| ----- | ----------------------------------------------------------------------------------------------------------------- |
| 1–7   | six digits, with `#` and spaces, three, four and eight digits, a color pasted, a value refused and the color back |
| 8–10  | the opacity, the swatch opening the picker, the picker closed by the swatch, Escape and a click outside           |
| 11–13 | the hue bar by mouse and by keys, the opacity bar                                                                 |
| 14–17 | the plane in HSB and in HSL, values typed, a grey keeping its hue                                                 |
| 18    | the background's color and its picker                                                                             |
| 19–38 | the 20 decisions below, one step each with its example                                                            |
| 39–40 | back to F14 and F05, the colors drawn as before, then back to the rectangle (F16.AC4)                             |

## Decisions to confirm

The rules of colors marked provisional in `docs/domain/interaction.md` (Colors), Q38 and ADR-0056, read by the agent from F05's refinement at SP-017, then while building US-035 and at AUD-017 (2026-10-15). Each example was checked against the playground at VAL-016.T1: rows 12 and 15 show the hue 0 as the field does, `0` rather than `0.0`; the others held.

| #   | Rule                                                                                                                                                                                                                                                                                 | Example                                                                                          |
| --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------ |
| 1   | **Settled, 2026-10-15**: a hexadecimal typed or pasted is kept exactly as typed, never altered afterwards. The hexadecimal and the opacity are stored; HSL and HSB are computed from them, never stored (ADR-0056)                                                                   | a file holds `#ff8000` and 100, never 30.1°                                                      |
| 2   | A color is typed with or without `#`, in any case, white space around it ignored, and kept as `#` and six lower-case digits, the only form the model accepts; a color of F05 or F14 in upper case is turned to lower case when read                                                  | `FF8000`, and `#ff8000` with spaces around it, give `#ff8000`                                    |
| 3   | 3 digits are each doubled; 4 and 8 digits carry an alpha, which sets the opacity to the nearest percent                                                                                                                                                                              | `f80` gives `#ff8800`; `#0000ffcc` gives `#0000ff` and 80 %; an alpha `ec` gives 93 %            |
| 4   | Anything else is refused, the field showing its previous color; a color copied is six digits, the opacity copied from its own field                                                                                                                                                  | `red`, `#ff800` refused                                                                          |
| 5   | The picker opens on a click on the swatch and closes on another click on it, on Escape or on a click outside                                                                                                                                                                         | —                                                                                                |
| 6   | The plane is in HSB first, the agent's own choice; HSL is one click away                                                                                                                                                                                                             | —                                                                                                |
| 7   | The plane shows the saturation across, 0 at the left, and the lightness or brightness up, 100 % at the top                                                                                                                                                                           | HSB at 210°, the middle of the right edge: `#004080`                                             |
| 8   | **Changed, 2026-10-15 (ADR-0058, US-040)**: shown and typed as integers, one value edited rebuilt with the exact others. Was: HSL and HSB values are shown to one decimal, so that typed back they give the same hexadecimal; integers, as Figma shows, change nearly 9 colors in 10 | `#336699` is HSB 210°, 66.7 %, 60 %; typed as 67 %, it would give `#326699`                      |
| 9   | A value typed or picked becomes the hexadecimal by rounding each of red, green and blue to the nearest, a tie up                                                                                                                                                                     | HSL 30°, 100 %, 50 %: the green 127.5 gives 128, `#ff8000`                                       |
| 10  | A grey shows the hue 0°; an open picker keeps the hue its slider shows                                                                                                                                                                                                               | `#808080` with the slider at 200°, dragged to more saturation, keeps 200°                        |
| 11  | Keys: arrows 1, Page Up and Page Down 10, Home and End to the ends, on each slider; on the plane, Left and Right the saturation, Up and Down the other value; each slider and the plane give a screen reader their name and value                                                    | hue 210°: Right 211°, Page Up 221°                                                               |
| 12  | **Changed, 2026-10-15 (ADR-0058)**: whole degrees, End gives 359. Was: the hue slider holds the hue to one decimal, arrows add 1 to it, End gives 359.9°, a hue of 360 is shown 0                                                                                                    | `#ff8000`: 30.1°, Right 31.1°                                                                    |
| 13  | F16's fourth criterion names the text's color; the agent reads it as the background's color too                                                                                                                                                                                      | the background's color field has the same swatch and picker                                      |
| 14  | A color refused brings back the color the field had when it got the focus, a line under it saying why; while it is typed, the text follows the field                                                                                                                                 | `red` then Tab: `#ff8800` again, a line under it                                                 |
| 15  | A number typed or dragged beyond an end counts at that end; a value left empty shows its value again; a click on the opacity slider gives the nearest percent; a slider not laid out yet holds 0                                                                                     | hue 400 gives 0, hue −10 gives 0; 33.5 % clicked gives 34 %                                      |
| 16  | The picker opens under its color's fields, which stay usable; Escape gives the focus back to the swatch                                                                                                                                                                              | —                                                                                                |
| 17  | **Example changes, 2026-10-15 (ADR-0058)**: values shown as integers. Another model chosen keeps the hue the picker holds while the three values still give the color back, a grey always; else it shows the color's own hue                                                         | `#523a39` picked at HSB 0.7°, 30 %, 32.2 %: HSL 2.4°, 18 %, 27.3 %, as 0.7° would give `#523939` |
| 18  | Each slider's role, name and value sit on its track, which takes the focus; the thumb is only drawn (the WAI-ARIA pattern puts them on the thumb)                                                                                                                                    | a screen reader reads Hue, 30.1                                                                  |
| 19  | **Dropped, 2026-10-15 (ADR-0058)**: no tenth is shown. A value lying exactly on a half-tenth may be shown rounded down, either neighbour giving the hexadecimal back                                                                                                                 | `#002330`: hue 196.25°, shown 196.2; `#030330`: HSB saturation 93.75 %, shown 93.7               |
| 20  | On the plane, Page Up, Page Down, Home and End act on the saturation, as Left and Right                                                                                                                                                                                              | HSB 210°, 66.7 %, 60 %: Page Up gives 76.7 %, End 100 %                                          |

## Remarks before validation (2026-10-15)

The Product Owner tried the colors in the playground before the formal validation, which still runs at the end of the epic. Their remarks and decisions, in neutral words:

| #   | Remark                                                                            | Decision                                                                                                                                                                                                                                                                                                                                               | Work           |
| --- | --------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------- |
| 1   | The picker is much liked                                                          | kept; its look is polished later                                                                                                                                                                                                                                                                                                                       | none           |
| 2   | The values shown and typed in the picker have decimals                            | the picker's inputs show and accept integers only; a hexadecimal typed or pasted is kept exactly as typed, never altered afterwards; decimals may exist inside, never shown; one value edited is rebuilt with the exact values of the others; the picker reaching fewer colors than the hexadecimal is accepted (ADR-0058, decisions 8, 12, 17 and 19) | EN-043, US-040 |
| 3   | The underline's and the strike-through's colors cannot be chosen (F05's remark 6) | each gets its own color and opacity, through the same color field and picker                                                                                                                                                                                                                                                                           | US-041         |
