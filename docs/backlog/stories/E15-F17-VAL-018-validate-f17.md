---
id: VAL-018
epic: E15
feature: F17
title: Validate F17
status: done
points: 1
---

# E15 · F17 · VAL-018 — Validate F17

The Product Owner tries the corrections of F17 in the playground, on E15's checklist page (US-051), and says what they see (ADR-0020).

## Acceptance criteria

- Given E15's checklist page, when the Product Owner ticks its lines, then each correction of F17 is tried and each remark of the validation of 2026-10-15 is seen closed.

## Readings to confirm

The agent's readings, written in `docs/domain/shapes.md` as provisional, for the Product Owner to confirm here:

| Story  | Reading                                                                                                                                                                                                                                                                      |
| ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| US-046 | an installed face the browser cannot give is said in the family's line as `<full name>: <Chrome's message> Import its file with Import a font`; a whole collection given for a face keeps the face of its PostScript name; an imported collection says its faces in one line |
| US-049 | the trailing spaces of a paragraph's last line in a box are ignored for its placement and its background band, as every line's: "World " right-aligned in a box 50 (Arial 16) ends at the box's edge, x 8.219, its band ending with "World"                                  |
| US-047 | a face the browser cannot load is said in one line, `<face> could not be loaded by the browser (<reason>): measured with it, drawn with another face.`                                                                                                                       |

## Questions from the audit (AUD-019)

Open points the audit found, for the Product Owner to decide here; nothing is built for them yet:

| Point                     | What was seen                                                                                                                                                                                                                                                    | Options                                                                                                                                                                            |
| ------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Condensed faces (US-046)  | macOS's Helvetica Neue holds Bold and Condensed Bold, both 700 upright; the face read last replaces the other, so Bold 700 draws Condensed Bold: "Pump P-101" at 32 is 155.08 wide, narrower than at 400 (180.27), seen in Chromium on 2026-10-16                | keep only faces of normal width (OS/2 `usWidthClass` 5) when one exists; or match widths as CSS does (`font-stretch`, `REF-CSS-FONTS-4` §5.2); or list condensed faces as a family |
| Trailing spaces (US-049)  | the last line of a paragraph in a box now hangs its spaces, as `white-space: normal` would; CSS `pre-wrap` keeps them before a forced break (`REF-CSS-TEXT-3`, §4.1.2): "World " right-aligned in a box 50 ends at x 8.219 now, at 3.773 with `pre-wrap`         | hang them (as built); or keep them for the last line                                                                                                                               |
| A family name with digits | the browser is now given the family's name unquoted (US-047); the CSS Font Loading spec parses it as CSS (`REF-CSS-FONT-LOADING` §2.1); Firefox loads "Source Sans 3", "3D Font" and "Font-2" given so (headless Firefox, AUD-019, 2026-10-16); Safari not tried | import "Source Sans 3" in Safari: if refused, quote the names that are not CSS identifiers                                                                                         |

## Tasks

One task = one commit, referenced as `VAL-018.Tn`.

- [x] T1 — The checklist's lines of F17, tested (1 h)
- [x] T2 — The demo page (30 min)
- [x] T3 — The Product Owner's validation (1 h)

## Result (first pass, 2026-10-16)

The Product Owner walked E15's checklist page on branch `feat/val-018-text-corrections`. The validation is not finished: VAL-018 stays open until the defects below are corrected and tried again.

**What works**

- Empty lines show their frames and draw nothing; sizes 48, 16, 48, 16 no longer lag.
- IBM Plex Sans: every weight, 450 drawn as measured; italic on and off; color, opacity, both pickers, HSL and HSB, letter and word spacing, the underline's and strike-through's colors, and every letter case.
- Right-to-left text with right alignment; fit content, trim, line height, background, ellipsis with every vertical alignment, and a maximum of 0 lines.
- A turned text has a blue box and a dashed bounding box, as a shape; typing in fit content while turned works.
- The Export SVG button serves as a test tool (the real export comes later); the file opens correctly in Inkscape.

**Decision**: trailing spaces (answers question 1 of AUD-019) are accepted as built: the background does not grow with spaces typed at a line's end.

**Defects, to investigate cause first; stories to come**

| Id  | Seen                                                                                                                                                                                                                                       | To check                                                                                     |
| --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------- |
| D1  | Inter without an imported file: 800 looks like 900, 700 like 600, 500 like 400; the weight list offers every weight anyway; the box padding varies with the weight (Thin overflows by the "1", Medium too wide, ExtraBold and Black wrong) | whether the installed Inter is read at all or default metrics stand in for missing faces     |
| D2  | Italic on: the box does not change                                                                                                                                                                                                         | whether the text is measured upright                                                         |
| D3  | Left-to-right text with right alignment: the second line does not end exactly at x 100, the "g" looks off                                                                                                                                  | advance against ink at a line's end                                                          |
| D4  | After turning a text by 90, the X and Y fields keep their old values while the box moves; the readout shows the new ones; one crash seen, which recovered                                                                                  | X and Y (the top-left corner) should follow the turn                                         |
| D5  | An installed font that cannot be read ("ABC Oracle Book could not be accessed") is still listed                                                                                                                                            | read every installed font ahead; list only the fonts and weights really available, Inter too |

**Change C1**: switching from fit content to a box keeps the fit-content size as the box's size, then editable (to investigate; story to come).

**Points not answered**: Helvetica Neue 700, an unquoted family name with a digit in Safari, and Apple Color Emoji.ttc. The Product Owner left the choice to the agent: Apple Color Emoji is handled by the recommended option (b) unless a business choice remains.

## Result (second pass, 2026-10-16)

The Product Owner walked E15's checklist page again on branch `feature/f17-text-corrections`, in Chrome. Four points are routed to stories; VAL-018 stays open until they are corrected and tried again.

**Accepted**

- Installed fonts read ahead: slow but fine; the reading is visible, the list is no longer guesswork, fonts that cannot be read are left out, and the weights offered are the real ones.
- Apple Color Emoji no longer crashes the tab.
- Rotation: X and Y follow the turned top-left corner, accepted for now; the box grows from that corner. Handling text like rectangles, polygons and round shapes is left to the later UI/UX epic.
- Fit content then Box keeps the fitted width and height.
- Letter spacing 10 with right alignment, left-to-right and right-to-left: the last letter touches the edge.
- Colors, every letter case, trim, line height, background and the rest of the checklist; the critiques of the first pass are gone.

**Routed to stories**

| Id  | Seen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Story  |
| --- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ |
| F1  | Inter stays first in the list though it is not installed and cannot be drawn. Decision: a font that is not available is not listed, Inter included; the playground selects a default font once the installed fonts are read; "Installed fonts" comes first and shows clearly that it must be done first                                                                                                                                                                                                           | US-056 |
| F2  | Fonts in the list must be usable: Apple Color Emoji gives a wrong box, and some fonts measure badly (an Arabic-script font such as Al Nile; Apple Braille is expected to be useless for Latin text). Decision: a font whose `fsType` forbids embedding is not listed. Agent's addition (ADR-0060, Latin the target): a font lacking the basic Latin letters is not listed either; the cause of the emoji box is checked first (Latin letters missing, the browser falling back while the measure uses `.notdef`?) | US-056 |
| F3  | Fit content, Box, reduce the height, Fit content, Box: the width and height typed for the box are lost (measured again). Expected: the box size the designer set is kept when coming back to Box (the maximum of lines is already kept)                                                                                                                                                                                                                                                                           | US-057 |
| F4  | Ellipsis on an empty line: 4 lines, the third empty, a maximum of 3 lines: the ellipsis lands on the empty third line. The rule asked for is the most logical one; the agent's proposal: the ellipsis goes on the last shown line that holds text, and the empty shown lines after it are dropped                                                                                                                                                                                                                 | US-058 |

**Not now**: the single-line option (I3) and the curated set of fonts with an Install button (I4) are logged as ideas in E02.

## Result (third pass, 2026-10-16)

The Product Owner tried F17 again after US-059 and US-060, in Chrome. **F17 is validated.**

- **US-059 accepted**: the font list holds only the fonts read and approved, imported ones too.
- **US-060 accepted**: the font chosen automatically is embedded with **Embed fonts**, as a font chosen by hand.
- Everything accepted at the first and second passes stays accepted.
- A tour of the whole Text tool followed, taken as E15's review (REV-003): every option works.

**Open points left by US-060**, logged in the [improvement log](../../process/improvements.md) for RET-003: the font list enabled for a moment while the default font's files are read again; faces refused at the second read keeping the metrics read ahead; the 2026-10-16 rows of the feedback log reported as duplicated, of which none is byte-identical, so none was removed.
