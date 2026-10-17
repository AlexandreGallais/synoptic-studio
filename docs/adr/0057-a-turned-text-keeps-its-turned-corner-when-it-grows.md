# ADR-0057 — A turned text keeps its turned top-left corner when it grows

**Status**: Accepted (Product Owner, 2026-10-15, from a try of the playground before VAL-014). Adds to ADR-0055, decision 3, which stands: a text turns about the middle of its block, or of its box. Settles VAL-015's decision 21: a text in a box turns about the middle of its box.

## Context

- **ADR-0055, decision 3**, turns a text about the middle of its measured block, as F05's refinement, round 2, said (2026-10-14); a text in a box turns about the middle of its box (F14). `textDrawing` computes the middle with `boxCenter` (`DERIV-turned-shape`, step 4).
- **What the Product Owner saw.** A turned text fitting its content whose letter or word spacing grows: its block grows about its stored position, so its middle moves, and the whole text moves on the drawing. The expected behavior is a text that grows along its own direction, its width with the text and its height with its lines, its turned top-left corner staying where it is.
- **What shapes already do.** A width typed on a turned shape keeps its turned top-left corner in place, as Figma does (F04, `DERIV-turned-shape`, step 5): the stored position is computed again, `b' = P − k' + R(k')`, where `P` is the corner, `k'` half the new box and `R` the turn. The position of a turned shape may then be fractional (ADR-0035).

## Decision

1. **The point of the turn is unchanged**: the middle of the block when the text fits its content, the middle of its box otherwise (ADR-0055, decision 3).
2. **A change of the block's size keeps the turned top-left corner in place.** Whatever changes the size of a turned text's block — its content, its letter or word spacing, its size, its number of lines, its font — the stored position is computed again by `DERIV-turned-shape`, step 5, so that the turned top-left corner keeps its place on the drawing. A block 100 × 20 at (47, 61) turned by 23 has its turned top-left corner at (54.88207, 42.25840); grown to 120 wide, its position becomes (46.20505, 64.90731) and the corner stays, where turning about the new middle alone would move it to (55.67702, 38.35108).
3. **A box resized** while turned keeps its turned top-left corner in place too, as a shape does; a spacing does not change a box.
4. **At 0°** nothing is computed: the position does not change, the block grows to the right and down, as before.
5. **The position stored** may then be fractional, as a turned shape's (ADR-0035); typed by the designer, it is an integer.

## Reasons

| Option | For | Against |
| --- | --- | --- |
| Middle kept, the turned corner fixed (chosen) | the text grows along its direction and stays in place; as shapes and Figma; the middle kept | the position is computed again at each change and may become fractional |
| Middle kept, the position fixed (ADR-0055 as built) | the position stays the integers typed | the text moves on the drawing whenever its block changes size |
| Turn about the position | the position fixed, no computing | a text turns about its corner, unlike shapes; the refinement turns it about its middle |

## Consequences

- ADR-0055's status line names this ADR; its text is unchanged.
- `shapes.md` (Text, Turning) states the rule; EN-042 computes the position again, US-038 shows it in the playground; the straight box of a turned text (US-039) follows.

## References

ADR-0035, ADR-0055, `DERIV-turned-shape`, `REF-SVG11-TRANSFORM`
