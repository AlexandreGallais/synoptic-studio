# ADR-0056 — A color stored as hexadecimal and an opacity; HSL and HSB as views

**Status**: Accepted (agent, SP-017, 2026-10-15; confirmed by the Product Owner at VAL-016, 2026-10-15). Decision 4 superseded by ADR-0058 (2026-10-15): HSL and HSB shown and typed as integers. Number 0053 stays reserved for F15's HTML layer.

## Context

- **The refinement.** A color is a hexadecimal value with an opacity from 0 to 100 %, both always visible, for a quick copy and paste; a picker in HSL or HSB shows its hexadecimal translation; RGB fields and CSS color names are not wanted (F05 refinement, round 2, 2026-10-14). F16's third criterion asks the conversions to be exact on the round trip from the hexadecimal.
- **Inputs are integers** (ADR-0003): the hexadecimal is three integers from 0 to 255, the opacity an integer percent. HSL and HSB are fractions: `#FF8000` has a hue of 30.11765° (`DERIV-color-spaces`).
- **Integers do not round-trip.** With HSL or HSB rounded to integers, about 89.4 % and 87.8 % of the 16 777 216 colors come back changed (`#336699` shown as HSB 210°, 67 %, 60 % gives `#326699`); rounded to one decimal, none does (`DERIV-color-spaces`, step 9).
- **What others do.** Figma describes a color in Hex, HSB, HSL, CSS or RGB, "Switching between color models only affects how Figma describes colors", Hex is the default, the opacity a separate percentage field (`REF-FIGMA-COLOR-MODELS`). Inkscape 1.2 puts its picker's modes, HSL and HSV among them, in a drop-down menu (`REF-INKSCAPE-1-2-COLOR`); for the OKLch mode added in 1.3, "color values will still be written as RGB hex codes in the SVG source" (`REF-INKSCAPE-1-3-COLOR`).

## Decision

1. **The stored truth** of a color is `#` and six lower-case hexadecimal digits, the only form the model's check accepts, with an opacity, an integer percent from 0 to 100, in separate fields. The SVG writes them as `fill` and `fill-opacity`, as F05 does.
2. **HSL and HSB are views**, computed from the hexadecimal by the formulas of `REF-CSS-COLOR-4` and `REF-WIKI-HSL-HSV`, never stored in the model or in a file. A grey's hue is 0.
3. **A value typed or picked** in HSL or HSB becomes the hexadecimal by rounding each byte, a tie rounded up. The picker keeps its own hue, saturation and lightness or brightness while it is open, so that a grey keeps the hue its slider shows; closing it keeps only the hexadecimal and the opacity.
4. **The values shown** in HSL and HSB have one decimal: typed back, every color gives its hexadecimal again.

## Reasons

| Option | For | Against |
| --- | --- | --- |
| Hexadecimal stored, HSL and HSB as views | integers in the model (ADR-0003); what the designer copies is what is stored; as Figma describes its models, and as Inkscape's OKLch mode keeps hex codes | the picker keeps a little state of its own for greys |
| HSL or HSB stored as fractions | the picker's state needs no rounding | floats in the model; two spellings of one color; a file no SVG reader reads as is |
| HSL or HSB stored as integers | integers | 88 % to 89 % of the colors cannot be stored |
| Views shown as integers, as Figma | shorter numbers | typed back, most colors change by one byte |
| Views shown to one decimal (chosen) | every color comes back exactly | one more digit to read |

## Consequences

- The model keeps its fields (`color`, `opacity`, `backgroundColor`, `backgroundOpacity`); its check accepts `#` and six lower-case digits only, and is shared by the text and every later color (E04's fills and strokes).
- Every reader of a color turns it to the stored form first: a color typed or pasted in another hexadecimal form (3, 4 or 8 digits, upper case, no `#`), a playground field, data loaded; a color of F05 or F14 written in upper case is turned to lower case when read (EN-037).
- `shapes.md` (Text) and `interaction.md` (Colors) state the rules; `DERIV-color-spaces` carries the formulas and the exhaustive check.

## References

`REF-CSS-COLOR-4`, `REF-WIKI-HSL-HSV`, `REF-FIGMA-COLOR-MODELS`, `REF-INKSCAPE-1-2-COLOR`, `REF-INKSCAPE-1-3-COLOR`, ADR-0003, `DERIV-color-spaces`
