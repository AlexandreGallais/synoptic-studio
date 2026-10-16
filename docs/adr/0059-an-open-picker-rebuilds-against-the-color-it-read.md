# ADR-0059 — An open picker rebuilds each edit against the color it read

**Status**: Accepted by the agent (2026-10-15, US-040); confirmed by the Product Owner at VAL-016, 2026-10-15. Amends ADR-0058, decision 2 (the exact values "computed from the hexadecimal"); its decisions 1, 3 and 4 stand.

## Context

- **ADR-0058, decision 2**, rebuilds a color from the integer edited and the exact values of the two others, computed from the hexadecimal. Read after every edit, the hexadecimal of the previous edit is already rounded to bytes, so its exact values drift from the integers typed.
- **The drift, measured.** On `#1e293b`, HSL 30, then 100, then 50, typed one after the other, give `#ff8400`, not the `#ff8000` of HSL 30°, 100 %, 50 %: the hue read back after H 30 is no longer 30°, and S and L keep that hue (in HSB, H 30 typed gives `#3b2d1e`, whose own hue is 60 · 15 / 29 = 31.03°). In HSB, H 210, S 67, B 60 typed on the same color end on `#316899`, shown 208, 68, 60.

## Decision

1. **While the picker is open, an edit is rebuilt against the color its values were last read from**: when it opened, when a hexadecimal was typed, when another model was chosen. A value equal to the one read stands for its exact value; any other value is the integer given, held as given until the color is read again.
2. ADR-0058's decisions 1, 3 and 4 stand: integers shown, a value typed equal to the one shown changes nothing, the hue slider from 0 to 359.

## Reasons

| Option                                                                           | For                                                                                      | Against                                                                                           |
| -------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| Rebuilt against the color last read (chosen)                                     | no drift: HSL 30, 100, 50 typed on `#1e293b` give `#ff8000`; the values typed stay shown | the values shown can differ from the hexadecimal's own; two paths to one color may part by a byte |
| Rebuilt against the hexadecimal after each edit (ADR-0058, decision 2 as worded) | the values shown are always the hexadecimal's own                                        | each edit rounds the next one's base: `#ff8400` for HSL 30, 100, 50 typed on `#1e293b`            |

## Consequences

- **The values shown can differ from the hexadecimal's own**: H 30 typed on `#1e293b` gives `#3b2d1e`, whose own hue is 31; the picker shows 30 until the color is read again.
- **One hexadecimal and the same values, reached by two paths, can give results a byte apart after the same edit**: `#326699` typed as a hexadecimal (saturation exactly 103 / 153), its brightness set to 61, gives `#33689c`; `#326699` reached from `#004080` by S 67 and B 60 typed (saturation held at 67 %), its brightness set to 61, gives `#33679c`.
- **A value left as read keeps the color**: `#336699` open in HSB (210, 67, 60), S 50 gives `#4d7399`, then S 67 gives `#336699` back; with ADR-0058's wording it would give `#326699`.
- `interaction.md` §5 states the rule; VAL-016's decision 8 carries it with these numbers; the playground keeps the color read on the picker (`writePickedColor`, US-040).

## References

ADR-0056, ADR-0058, `DERIV-color-spaces` (step 12)
