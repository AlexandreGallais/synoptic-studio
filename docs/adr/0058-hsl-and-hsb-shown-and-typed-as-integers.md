# ADR-0058 — HSL and HSB shown and typed as integers

**Status**: Accepted (Product Owner, 2026-10-15, from a try of the playground before VAL-016); decision 2 amended by ADR-0059. Supersedes ADR-0056, decision 4 (values shown to one decimal); its decisions 1 to 3 stand: the hexadecimal and the opacity are the stored truth, HSL and HSB are views.

## Context

- **ADR-0056, decision 4**, shows HSL and HSB to one decimal, so that every value typed back gives the same hexadecimal; with integers, about 89.4 % (HSL) and 87.8 % (HSB) of the colors would change by one byte when typed back (`DERIV-color-spaces`, step 9).
- **What the Product Owner asked.** The values shown and typed in the picker are whole numbers, as in Figma (`REF-FIGMA-COLOR-MODELS`); a color typed or pasted in hexadecimal is kept exactly as typed and never altered afterwards, the hexadecimal staying the truth; decimals may exist inside, never shown. The picker then reaches fewer colors than the hexadecimal, which is accepted.
- **Why one value edited must not move the others.** Rebuilding a color from the three integers shown would change it even when only one value is edited: `#336699` is HSB 210°, 66.66667 %, 60 %, shown 210, 67, 60; its brightness set to 61 with the saturation read as 67 gives `#33679c`, while with the exact 66.66667 it gives `#34689c`.

## Decision

1. **HSL and HSB are shown as integers**, each value rounded to the nearest, a tie up (`nearestInteger`): `#ff8000` shows HSL 30°, 100 %, 50 % and HSB 30°, 100 %, 100 %; a hue rounding to 360 shows 0.
2. **A value typed, keyed or dragged is an integer.** The color is rebuilt from that integer and the exact, unrounded values of the two others, computed from the hexadecimal (or the hue the open picker holds, ADR-0056, decision 3), then rounded to bytes as ADR-0056 says. `#336699`, its HSB brightness typed 61: HSB 210°, 66.66667 %, 61 % gives `#34689c`.
3. **A value typed equal to the one shown changes nothing**: `#336699`, its saturation shown 67 and typed 67 again, stays `#336699` (agent's reading, so that a field left as it is never changes the color; confirmed at VAL-016, 2026-10-15).
4. **The hue slider** holds integers from 0 to 359: End gives 359, an arrow adds 1 to the hue shown (`#ff8000`, shown 30: Right gives 31).

## Reasons

| Option                                                 | For                                                                                         | Against                                                                                                                                  |
| ------------------------------------------------------ | ------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| Integers shown, the others kept exact (chosen)         | whole numbers, as Figma shows; the hexadecimal stays exact; one value edited moves no other | the picker reaches fewer colors than the hexadecimal (accepted); a value typed by hand may give a color one byte away from the one meant |
| One decimal shown (ADR-0056)                           | every value typed back gives its hexadecimal                                                | a decimal to read and type in every field                                                                                                |
| Integers shown, the color rebuilt from the three shown | one rule                                                                                    | editing one value changes the others' bytes: `#336699` drifts to `#33679c`                                                               |

## Consequences

- ADR-0056's status line names this ADR; its text is unchanged.
- `interaction.md` §5 (Colors) and Q38 state the rule; VAL-016's decisions 8, 12, 17 and 19 change with it.
- EN-043 adds the integer views and the rebuild from one edited value; US-040 shows them in the picker.

## References

ADR-0056, `DERIV-color-spaces`, `REF-FIGMA-COLOR-MODELS`, `REF-CSS-COLOR-4`
