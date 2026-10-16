---
id: VAL-018
epic: E15
feature: F17
title: Validate F17
status: ready
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
- [ ] T2 — The demo page (30 min)
- [ ] T3 — The Product Owner's validation (1 h)
