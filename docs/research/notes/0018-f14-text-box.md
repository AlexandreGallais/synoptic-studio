# 0018 — Sources of F14: a box of text wrapped, cut with an ellipsis and aligned

Spike SP-016, 2026-10-15, for F14 (a box of text, E15). F05's refinement, rounds 1 and 2, gave the business rules: a text fits its content or sits in a box, lines wrap by themselves, a word too long either breaks at a character or moves whole (no hyphenation), an ellipsis with a maximum number of lines, alignment in both directions, a line height, a wrap style, a background as a highlighter, a trim from the cap height to the baseline, turning about the middle of the box, scrolling later. Every page below was read in the local library before it was cited; the calculations are in `DERIV-text-box`, checked by hand on Arial's metrics.

## Where a line wraps (EN-033)

| Finding                                                                                                                                                                 | Source                                     |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------ |
| Wrapping happens only at a soft wrap opportunity; the UA "must minimize the amount of content overflowing a line"                                                       | `REF-CSS-TEXT-3`, §5                       |
| A break after the last space of a run (LB18), never after a no-break space (LB12, class GL); line breaks typed are hard breaks (LB5)                                    | `REF-UAX14`                                |
| Spaces at the end of a line "are ordinarily not measured for fit" and stay "invisibly on the previous line"; CSS makes them hang, outside alignment                     | `REF-UAX14` (SP), `REF-CSS-TEXT-3`, §4.1.2 |
| The greedy wrap uses the fewest lines; LibreOffice Writer and Word wrap that way                                                                                        | `REF-WIKI-LINE-WRAP`                       |
| `overflow-wrap: anywhere` breaks an unbreakable word "if there are no otherwise-acceptable break points in the line", no hyphen; `normal` breaks only at allowed points | `REF-CSS-TEXT-3`, §5.4                     |

F14 keeps spaces as the only break opportunity, as its first criterion says: a hyphen, a slash or an em dash, which UAX #14 also breaks at, do not break a line (open below).

## The lines shown and the ellipsis (EN-034)

| Finding                                                                                                                                                                                                           | Source                                          |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------- |
| `line-clamp` keeps a number of lines, the rest "neither rendered nor measured"; `max-lines` takes positive integers                                                                                               | `REF-CSS-OVERFLOW-4`, §5                        |
| `block-ellipsis: auto` puts U+2026 at the end of the last line, "after the last soft wrap opportunity" that lets it fit, measured with the line for alignment                                                     | `REF-CSS-OVERFLOW-4`, §4.2                      |
| `text-overflow: ellipsis` hides characters at the end of an overflowing line "as necessary to fit the ellipsis"; the first character stays; "three dots" when U+2026 is missing; a single word too long overflows | `REF-CSS-OVERFLOW-3`, §5.1                      |
| Arial's ellipsis advances 2048 units, a full em: 16 at size 16; three full stops 13.33594                                                                                                                         | Arial 5.01, read by a script (`DERIV-text-box`) |

## Line height, trim, vertical alignment (EN-035)

| Finding                                                                                                                                                                               | Source                        |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------- |
| A chosen line height spreads its leading `L = line-height − (A + D)` half above the ascent, half below the descent; `L` may be negative                                               | `REF-CSS-INLINE-3`, §5.3      |
| Latin text's visual bounds are the cap height and the baseline; `text-box-trim` trims the first line to the cap height and the last to the baseline, so centering centers the letters | `REF-CSS-INLINE-3`, §6, §6.2  |
| The cap height is "Calculated using `sCapHeight` in OpenType"; fallback `.66em`                                                                                                       | `REF-CSS-INLINE-3`, §3.2, A.2 |
| `sCapHeight` exists from `OS/2` version 2, "set to 0" when the font has no `H`; Arial 5.01 has version 3, `sCapHeight` 1467                                                           | `REF-OT-OS2`                  |

The vertical alignment places the block of lines in the box as F05 places a line across its block: none, half or all of the room before it (`DERIV-text-width`, step 5; `DERIV-text-box`, step 8).

## The balanced wrap (EN-036)

| Finding                                                                                                                                                                               | Source                      |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------- |
| `balance` evens the space left in each line, last line included, keeps the number of lines (for 5 or fewer it must), works per paragraph, after clamping; the algorithm is UA-defined | `REF-CSS-TEXT-4`, §5.4      |
| Minimum raggedness minimizes the sum of the squares of the spaces left                                                                                                                | `REF-WIKI-LINE-WRAP`        |
| Knuth–Plass computes it by dynamic programming in O(n²); worked example "AAA BB CC DDDDD": greedy 17, optimum 11                                                                      | `REF-WIKI-KNUTH-PLASS`      |
| Figma's Balance "distributes text as evenly as possible across every line"                                                                                                            | `REF-FIGMA-TEXT-PROPERTIES` |

Chosen: the minimum raggedness with the greedy wrap's number of lines. With that number fixed, the sum of the squares falls with their spread, which is what CSS measures; it is exact, small (a box holds few words a line) and testable by hand. `pretty` is left out: CSS leaves its improvements to each user agent, and F14's criterion names only the balanced style.

## The background (US-034)

The content area of an inline box fits the text of its font, for example from its ascender to its descender (`REF-CSS-INLINE-3`, §6.4): one rectangle per line, from the ascender to the descender, as long as the line, drawn as one `<path>` before the `<text>` (ADR-0002, ADR-0052).

## Decisions of the spike (provisional, to confirm at VAL-015)

Taken from the closest reading of the refinement and the sources, written in `docs/domain/shapes.md` (Text, Box):

| Decision                                                                                                                                          | Reading                                                                                                                  |
| ------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| A word too long, broken at a character, first starts its own line, then breaks                                                                    | `overflow-wrap: anywhere`; the option concerns only "a word too long"                                                    |
| A word too long, moved whole, stays on its own line, cut with "…" when the ellipsis is on, drawn whole when it is off                             | `text-overflow: ellipsis` on an overflowing word                                                                         |
| The ellipsis is an option, on by default; it removes characters, not whole words                                                                  | F05 round 1 "can end with an ellipsis"; `text-overflow`'s rule, which keeps more text in a small box                     |
| At least one line is shown, even in a box shorter than a line, never more than the lines wrapped                                                  | a text never vanishes                                                                                                    |
| When the first character and the ellipsis do not fit, both are drawn past the box                                                                 | CSS clips the ellipsis itself, a primitive the project forbids (ADR-0052); alternatives: draw only what fits, or nothing |
| "pretty" is not built in F14, balance only                                                                                                        | CSS leaves it to each user agent; keep it for later, or drop it                                                          |
| Wrapping, maximum lines, ellipsis, vertical alignment and balance apply in a box only; line height, trim and background in both modes             | Figma truncates only fixed-size text; a text fitting its content does not wrap                                           |
| An automatic line height keeps F05's placement (gap below the line); a chosen one spreads its leading half above, half below                      | F14.AC6 (draw as before); CSS half-leading                                                                               |
| A paragraph with a long word, and a text cut by the ellipsis, keep their greedy lines under balance                                               | a simpler rule than CSS's balance after clamping                                                                         |
| The background covers each line from the ascender to the descender                                                                                | CSS's content area                                                                                                       |
| A box's width and height are integers from 0 to 100 000, a maximum of lines from 0 (none) to 100 000, a line height from 0 (automatic) to 100 000 | the bounds of a size, and Q15 (0 allowed)                                                                                |

## Retained

| Story  | Uses                                                                                    |
| ------ | --------------------------------------------------------------------------------------- |
| EN-033 | `DERIV-text-box` steps 1 to 4; `REF-UAX14`, `REF-CSS-TEXT-3`, `REF-WIKI-LINE-WRAP`      |
| EN-034 | `DERIV-text-box` steps 5 and 6; `REF-CSS-OVERFLOW-3`, `REF-CSS-OVERFLOW-4`              |
| EN-035 | `DERIV-text-box` steps 7 and 8; `REF-CSS-INLINE-3`, `REF-OT-OS2`; `DERIV-turned-shape`  |
| EN-036 | `DERIV-text-box` step 9; `REF-CSS-TEXT-4`, `REF-WIKI-LINE-WRAP`, `REF-WIKI-KNUTH-PLASS` |
| US-033 | ADR-0052 (positioned lines), `DERIV-text-box`                                           |
| US-034 | `DERIV-text-box` step 10; `REF-CSS-INLINE-3`                                            |

## Open

- Break opportunities other than spaces (after a hyphen, a slash, an em dash, UAX #14): not in F14; a later feature, if the Product Owner asks for it.
- Which line height and vertical metrics a browser's HTML layer uses, against the positioned lines: F15 compares them (note 0017).
- `pretty` wrap style: not built; CSS leaves it to each user agent.
- Scrolling a long text in its box on a touch screen: with the synoptic views (E08), through the HTML layer (F15).
