# ADR-0055 — A text measured with its fallback's metrics, and turned by its group

**Status**: Accepted (agent, US-032, 2026-10-15, settling two points of CHK-014 and the place of the turn; provisional, to confirm by the Product Owner at VAL-014). Amends ADR-0052, decisions 1 and 4; the rest of ADR-0052 and ADR-0054 stand. Number 0053 stays reserved for F15's HTML layer.

## Context

- **A font without metrics.** ADR-0052, decision 1, measures "a text whose font has no metrics" with the default font's. A text names its font with a fallback (`Inter`, else `sans-serif`, F05 refinement, round 2), and CSS draws a text with the first family of its list that the reader has (`REF-CSS-FONTS-4`, §5.1). Since EN-032, `textFontMetrics` looks for the font's face, then the fallback family's, then the default font's, and `shapes.md` says so; CHK-014 found the ADR saying otherwise.
- **The fallback's name.** The font's name is checked (`isFontName`: not empty, trimmed, no control character); the fallback's was not, although it is written in the same `font-family` list (ADR-0054, decision 1). CHK-014 noted the gap.
- **Where the turn is written.** ADR-0052, decision 4, puts the `transform` on the `<text>`; decision 5 groups the `<text>` and its underline and strike-through paths in one `g`. A `transform` on the `<text>` alone would leave the strokes straight under turned letters. `rotate(<rotate-angle> <cx> <cy>)` turns an element and its content about a point (`REF-SVG11-TRANSFORM`, §7.6), and a `g`'s `transform` applies to everything it holds.

## Decision

1. **A font without metrics** is measured with its fallback family's face, when that family's metrics were read, else with the default font's (Inter). Amends ADR-0052, decision 1.
2. **The fallback's name** follows the rule of the font's name (`isFontName`); a text whose fallback is empty, untrimmed or holds a control character is refused (`isValidText`).
3. **The turn** of a text, `rotate(α cx cy)` about the middle of its measured block (`DERIV-turned-shape`, step 4), is written on the `g` that holds its `<text>` and its decoration paths, not on the `<text>`; a text at 0° carries no `transform`. Amends ADR-0052, decision 4.

## Reasons

| Option                                       | For                                                                          | Against                                                                       |
| -------------------------------------------- | ---------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| Fallback's metrics, then the default font's  | measures what a reader lacking the font draws, as CSS's font list does       | one more look-up per text                                                     |
| Default font's metrics only (ADR-0052)       | one rule                                                                     | measures Inter where a reader draws the fallback named, Arial for instance    |
| Fallback name checked as the font's          | one rule for both names of the list; a line break cannot enter `font-family` | none: the playground writes `sans-serif`                                      |
| Turn on the `g` (chosen)                     | the underline and strike-through turn with the letters; one `transform`      | the criterion named the `<text>`: met through its group                       |
| Turn on the `<text>` and on each stroke path | the `<text>` carries it, as ADR-0052 wrote                                   | the same `transform` written once per path; a stroke forgotten stays straight |

## Consequences

- ADR-0052's status names this amendment; its text is unchanged. Q31 (`docs/domain/README.md`) names this ADR.
- `shapes.md` (Text) states the three rules; US-032's first criterion is met through the group (provisional, VAL-014).

## References

`REF-CSS-FONTS-4`, `REF-SVG11-TRANSFORM`, ADR-0052, ADR-0054, `DERIV-turned-shape`, `DERIV-text-width`
