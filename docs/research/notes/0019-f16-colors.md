# 0019 — Sources of F16: colors in hexadecimal, HSL and HSB, and their picker

Spike SP-017, 2026-10-15, for F16 (colors and their pickers, E15). F05's refinement, round 2, gave the business rules: a hexadecimal value with an opacity from 0 to 100 %, both always visible, for a quick copy and paste; a picker on a click, with a hue slider and an opacity slider, in HSL or HSB, its hexadecimal translation always shown; no RGB fields, no CSS color names. Every page below was read in the local library before it was cited; the calculations are in `DERIV-color-spaces`, checked over every color by a script.

## The hexadecimal and the opacity (EN-037)

| Finding | Source |
| --- | --- |
| A hex color is `#` and 3, 4, 6 or 8 digits, case not mattering; 3 and 4 digits are doubled; the last pair of 8 digits is the alpha, `#0000ffcc` is 80 % blue | `REF-CSS-COLOR-4`, §5.2 |
| The HTML-compatible serialization of an 8-bit sRGB color is `#` and six lower-case digits, `#ff00ff`, "not #FF00FF" | `REF-CSS-COLOR-4`, §16.2.1 |
| An alpha percentage stands for the byte `p × 2.55`, rounded to the closest, up on a tie | `REF-CSS-COLOR-4`, §16.1.1 |
| Figma takes a 6-digit hex with a separate percentage field for the alpha, or an 8-digit hex whose last pair is the alpha | `REF-FIGMA-COLOR-MODELS` |

Computed in integers, every percent comes back from its byte, and `50 %` gives 128, where `50 × 2.55` in binary64 gives 127 (`DERIV-color-spaces`, step 2). An alpha byte pasted keeps its value to the nearest percent, since the opacity is an integer percent (F16.AC1).

## HSL and HSB (EN-038)

| Finding | Source |
| --- | --- |
| HSL to sRGB and back, as sample code; 0° is red; a saturation of 0 makes the hue "powerless" | `REF-CSS-COLOR-4`, §7, §7.1, §7.2 |
| HSV "is also often called HSB"; the common hue from the largest component, `S_V = C / V`; HSV to RGB by `f(n) = V − V·S·max(0, min(k, 4 − k, 1))` | `REF-WIKI-HSL-HSV` |
| A value rounded to "the closest integer (rounding up if two values are equally close)" | `REF-CSS-COLOR-4`, §16.1.1 |

The two directions undo each other on the sRGB cube (`DERIV-color-spaces`, step 7); in binary64 the error is below `4 × 10⁻¹³` of a byte, so the round trip from the hexadecimal is exact for all 16 777 216 colors (step 8). Shown as integers, as Figma shows them, HSL and HSB change about 89.4 % and 87.8 % of the colors when typed back; shown to one decimal, none (step 9, an exhaustive count). A grey's hue is undefined: the library gives 0, as Wikipedia does.

## The picker (US-035)

| Finding | Source |
| --- | --- |
| Pickers "show a two-dimensional slice through the model, along with a slider controlling which particular slice is shown" | `REF-WIKI-HSL-HSV` |
| CSS's HSL tables put the saturation across and the lightness down the page | `REF-CSS-COLOR-4`, §7.3 |
| Figma's picker opens on the swatch, has a hue slider and an opacity slider, and a dropdown of RGB, HEX, CSS, HSL and HSB; a model only describes the color | `REF-FIGMA-COLOR-PICKER`, `REF-FIGMA-COLOR-MODELS` |
| Inkscape 1.2 puts its picker's modes, HSL (its default) and HSV among them, in a drop-down; for the OKLch mode of 1.3, colors are still written as hex codes in the SVG | `REF-INKSCAPE-1-2-COLOR`, `REF-INKSCAPE-1-3-COLOR` |
| A slider: arrows by one step, Page Up and Down by more, Home and End to the ends; role `slider`, its value, minimum, maximum and label | `REF-WAI-APG-SLIDER` |

Figma says its models only describe a color (`REF-FIGMA-COLOR-MODELS`); Inkscape says so of its OKLch mode, whose colors stay hex codes in the SVG (`REF-INKSCAPE-1-3-COLOR`). Two independent editors, the second for one mode only: ADR-0056 rests on them and on ADR-0003, and on the exhaustive count of `DERIV-color-spaces`.

## Decisions of the spike (provisional, to confirm at VAL-016)

Taken from the closest reading of the refinement and the sources, written in `docs/domain/interaction.md` (Colors), Q38 and ADR-0056:

| Decision | Reading |
| --- | --- |
| The hexadecimal and the opacity are stored; HSL and HSB are views | integers in the model (ADR-0003); Figma; Inkscape's OKLch mode |
| 3, 4, 6 and 8 digits, `#` optional, any case, white space ignored; kept as six lower-case digits | CSS's forms; the refinement's aim of a quick copy and paste; CSS's serialization |
| An alpha typed sets the opacity to the nearest percent | Figma's 8-digit input; the opacity is an integer percent |
| Values shown to one decimal | the exhaustive count: integers lose nearly 9 colors in 10 |
| The plane in HSB first, the saturation across from 0 at the left, the other value up | HSB first: the agent's own choice, no source read; CSS's tables, the order made increasing |
| A grey shows the hue 0°, an open picker keeps its slider's hue | Wikipedia's 0; a picker that does not jump back to red when a grey is made lighter |
| The keyboard of the WAI-ARIA slider on each slider, and arrows on the plane | `REF-WAI-APG-SLIDER`; the APG has no two-dimensional pattern |
| The picker opens on a click on the swatch, closes on another click, Escape or a click outside | Figma opens it on the swatch; how it closes is the agent's choice, no source read |

## Retained

| Story | Uses |
| --- | --- |
| EN-037 | `DERIV-color-spaces` steps 1 and 2; `REF-CSS-COLOR-4` |
| EN-038 | `DERIV-color-spaces` steps 3 to 9; `REF-CSS-COLOR-4`, `REF-WIKI-HSL-HSV` |
| US-035 | `DERIV-color-spaces` step 10; ADR-0056; `REF-WAI-APG-SLIDER`, `REF-FIGMA-COLOR-PICKER`, `REF-FIGMA-COLOR-MODELS`, `REF-INKSCAPE-1-2-COLOR` |

## Open

- Saved colors and text styles: with the epic of libraries (F16, Left for later).
- Colors in other spaces than sRGB (Display P3, OKLCH, offered by Figma and Inkscape): not asked; the hexadecimal is sRGB.
- The eyedropper and the contrast check of Figma's picker: not asked.
- A picker on a touch screen (the views of E08): the sliders and the plane take pointer events, which cover touch; not tried on a device.
