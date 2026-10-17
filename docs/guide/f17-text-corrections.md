# F17 — Text corrections after the E15 validation

## What you can do now

The text has three fewer options and five fewer defects, all found when E15 was validated on 2026-10-15. A word too long for its box always stays whole, the lines of a box always fill the same way, and no text is written with an HTML layer. Fonts held in a collection file are read, a weight such as 450 is drawn with the face it is measured with, values held down with an arrow key redraw without lag, a background band ends with its line, and a turned text shows its frames as a turned shape does.

Feature: [F17](../backlog/features/E15-F17-text-corrections-after-the-e15-validation.md); causes of the defects: [research note 0021](../research/notes/0021-f17-latin-script-and-defects.md); decision on the HTML layer and embedded fonts: [ADR-0060](../adr/0060-no-html-layer-and-fonts-embedded-at-export.md).

## What changed and why

| Change | Why | Story |
| --- | --- | --- |
| A word too long stays whole: past the box, or cut with "…" | fewer options; a word is never cut in the middle | US-042 |
| The lines of a box always fill greedily | one way to fill lines is easier to understand than two | US-043 |
| No HTML layer: no `switch` nor `foreignObject` | a symbol is plain SVG, read the same everywhere (ADR-0060) | US-044 |
| Fonts embedded only in the file exported | the drawing on screen stays light; the file carries its fonts | US-045 |
| Installed fonts read from `.ttc` collections | Helvetica, Helvetica Neue, Menlo were refused; a face Chrome cannot give is now said in one line | US-046 |
| A weight such as 450 drawn as it is measured | the frame hugged one face while another was drawn | US-047 |
| A held arrow key redraws once per frame | changing Size, Opacity or a box's Height fast lagged behind the key | US-048 |
| A background band ends with its line | the spaces at a line's end lengthened its band past the box | US-049 |
| A turned text: its frame blue, its box dashed | as a turned shape already showed | US-050 |
| One checklist page per epic | a feature is validated by ticking lines, in any order, over several sessions | US-051 |

| Term | In plain words |
| --- | --- |
| **Collection** | one font file (`.ttc`) holding several faces: Helvetica.ttc holds 6, from 300 to 700, upright and oblique |
| **Face** | one weight and style of a family: Arial 700 italic |
| **PostScript name** | a face's unique name inside its files, such as `Helvetica-Bold` |
| **Hung space** | a space at a line's end that is neither measured nor drawn |

## How to try it

`npm run dev`, then `http://localhost:5173/?checklist=E15`: E15's checklist, one line per option kept and per point to confirm, each what to try and what to see, a check box before it, the boxes ticked kept in this browser (US-051). F17's lines are at its end, marked F17; its fixes to F14 and F15 are marked with their story on those features' lines.

| Line | You should see |
| --- | --- |
| Hold the Up arrow on Size, Opacity, the box's Height | the drawing follows smoothly and stops when the key is released |
| A rectangle, then a text in a box, both turned by 30 | the turned frame thin and blue, the straight box dashed around it |
| Fit Box, the box's fields and the Drawing's options | no option for a word too long nor for how lines fill; Embed fonts alone |
| “Hello World ”, width 50, Background on | two bands, each as long as its word |
| Installed Helvetica; Import a font Helvetica.ttc | “Helvetica: 6 faces read.”; its faces imported, named in one line |
| IBM Plex Sans 400, 450, 500 | three weights, the frame hugging the letters at each |
| Embed fonts on, Export SVG | no `<style>` on screen; one `@font-face` per face in the file, no `switch` |
| The rectangle, the polygon, the round shape | drawn as before E15 |

## Readings to confirm

The agent's readings, written in the domain as provisional ([VAL-018](../backlog/stories/E15-F17-VAL-018-validate-f17.md)); each has its line on the checklist, marked "Reading to confirm".

| Story | Reading | Example |
| --- | --- | --- |
| US-046 | a face the browser cannot give is said in the family's line; from a whole collection, only the face of its PostScript name is kept; an imported collection says its faces in one line | `<full name>: <Chrome's message> Import its file with Import a font`; `Helvetica 400, Helvetica 700, … imported.` |
| US-049 | the spaces at the end of a paragraph's last line are hung, as those of every line: they neither place the line nor lengthen its band | “World ” right-aligned in a box 50, Arial 16, ends at x 8.219, not 3.773 |
| US-047 | a face the browser cannot load is said in one line, the text measured with it and drawn with another face | `Broken 450 italic could not be loaded by the browser (…): measured with it, drawn with another face.` |

## Questions from the audit

Found by [AUD-019](../backlog/stories/E15-F17-AUD-019-audit-f17.md); nothing is built for them yet.

| Point | What was seen | Options |
| --- | --- | --- |
| Condensed faces | Helvetica Neue holds Bold and Condensed Bold, both 700: 700 draws the condensed one, “Pump P-101” at 32 155.08 wide against 180.27 at 400 | keep the faces of normal width when there is one; match widths as CSS does; or list condensed faces as a family |
| Trailing spaces | the last line's spaces are hung (the US-049 reading); CSS `pre-wrap` would keep them before a line break | hang them, as built; or keep them for the last line |
| A family name with digits | the browser is given the name unquoted; Firefox loads “Source Sans 3”, “3D Font”, “Font-2”; Safari not tried | import Source Sans 3 in Safari: if refused, quote the names that are not CSS identifiers |

## Limits

- **Apple Color Emoji must not be chosen yet**, installed or imported: its collection of 192 MB exhausts the page's memory while its faces are copied out. The fix writes bytes in place, which the functional lint forbids; it waits for a decision (AUD-019, improvement log). Every other system collection is read, 127 files and 543 faces, Songti.ttc in 8.5 s.
- No search field over the installed fonts, no reset button on the stroke colors: a later epic on the editor's ease of use (E02).

## First validation pass

First tried on 2026-10-16; the validation is not finished.

- **Works**: empty lines, every weight of IBM Plex Sans, italic, colors, spacing, letter cases, right-to-left text, fit content, trim, background, ellipsis, a turned text's frames, and the SVG export (opened in Inkscape).
- **Accepted as built**: spaces typed at a line's end do not widen the background.
- **To correct**: Inter's weights and box padding (D1), the box not following italic (D2), a right-aligned line not ending exactly at the edge (D3), X and Y not following a turn (D4), and unreadable installed fonts still listed (D5).
- **Change asked**: switching from fit content to a box keeps the size it had (C1).

Each point is investigated for its cause first, then becomes a story.

## Second validation pass

Tried again on 2026-10-16; the validation is still not finished.

- **Works**: installed fonts read ahead, Apple Color Emoji no longer crashing the tab, a turned text's X and Y following its top-left corner, fit content then box keeping its size, letter spacing with right alignment, and the rest of the checklist.
- **To correct**: a font that is not available (Inter included) is no longer listed, and "Installed fonts" comes first (F1); a font that forbids embedding or lacks the basic Latin letters is not listed (F2); the box size typed is kept when coming back to Box (F3); an ellipsis lands on the last shown line that holds text (F4).
- **Later**: handling text like other shapes when turned, a single-line option, and a curated set of fonts with an Install button.

## Third validation pass

Tried on 2026-10-16, after US-059 and US-060: the font chosen automatically is embedded with **Embed fonts**. Everything validated at the earlier passes stays validated.

## Validation

Validated by the Product Owner on 2026-10-16 (`VAL-018`), at the third pass.

| Remark | What was done or where it went |
| --- | --- |
| The font list showed fonts not read yet, then fonts not exportable | US-056, US-059: only fonts read and approved listed |
| The font chosen automatically was not embedded | US-060: its files read again, as a font chosen by hand |
| The right-to-left direction is of no use for the Latin target | kept for now; removed at the later UI/UX epic (E02, ideas) |
