---
id: US-047
epic: E15
feature: F17
title: A weight such as 450 drawn with the face it is measured with
status: done
points: 2
---

# E15 · F17 · US-047 — A weight such as 450 drawn with the face it is measured with

As a symbol designer, I want a text drawn with the face it is measured with so that its frame hugs its letters at every weight.

From the Product Owner's validation of E15 (2026-10-15): weight 450 of a family such as IBM Plex was measured but not visibly drawn, the browser seeming to draw another face. SP-019 ([research note 0021](../../research/notes/0021-f17-latin-script-and-defects.md)) found the code path right once the 450 `FontFace` has loaded, and two causes that only a browser tells apart: the 450 face's load failed unseen (`registerFontFace` gives `update` as the error handler, and CSS then matches 500, `REF-CSS-FONTS-4` §5.2), or the drawing came before the load. The library measures "Pump P-101" at 16 as 90.688 at 400, 91.264 at 450 and 91.92 at 500 (IBM Plex Sans Regular, Text, Medium): a drawn width tells the face within 0.5.

## Acceptance criteria

- Given IBM Plex Sans read from the installed fonts (its 450 face, IBM Plex Sans Text, listed in the family on macOS), when "Pump P-101" at 16 is set to 450, then the browser draws that face: the drawn width (`getComputedTextLength()`), read in a test page, is 91.264 within 0.5.
- Given the same text at 400 and 500, when drawn, then the drawn widths are 90.688 and 91.92 within 0.5.
- Given a face whose `FontFace` load fails, when it fails, then one line names the face and the browser's reason, and no error is left unseen.
- Given a family without a 450 face, when 450 is asked, then the face the Font face rule matches is measured and drawn (`shapes.md`, Font face).

## Cause seen

Neither of SP-019's two causes: a third one, seen on 2026-10-16 in Chromium's headless shell with a test page that gives the browser IBM Plex Sans Regular, Text and Medium as `FontFace`s at 400, 450 and 500 and reads `getComputedTextLength()` of "Pump P-101" at 16 (`REF-CHROMIUM-FONT-FACE-CODE`):

| Family given to `FontFace` | 400 | 450 | 500 | Drawn with |
| --- | --- | --- | --- | --- |
| `"IBM Plex Sans"`, a CSS string, as before | 90.703 | 91.922 | 91.922 | the installed family, its 500 face for 450 |
| `IBM Plex Sans`, the name alone, the fix | 90.703 | 91.266 | 91.922 | the faces given, 450 with IBM Plex Sans Text |
| measured by the library | 90.688 | 91.264 | 91.92 |  |

Chrome keeps `FontFace`'s `family` argument as it is, never parsed as CSS: the quotes the page wrote (`formatFontFamilyName`) became part of the family's name, so no text's `font-family` matched the faces given, and every face loaded was ignored. The text was drawn with the installed family of that name, whose 450 the browser matches with its 500 face, 0.658 wider than measured; at 400 and 500 the installed faces are the same files, so the defect showed at 450 only, and an imported font that is not installed was drawn with the fallback. All three loads succeeded. The fix gives `FontFace` the family's name alone; a failed load is now said in one line.

## Product Owner test

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Choose IBM Plex Sans, weight 450, "Pump P-101" | letters a little heavier than 400, the frame hugging them |
| 2 | Switch between 400, 450 and 500 | three different weights, the frame following |

## Tasks

One task = one commit, referenced as `US-047.Tn`.

- [x] T1 — A failed face load said in one line; the test page comparing drawn and measured widths at 400, 450 and 500, the cause seen there written here, and its fix (2 h)
- [x] T2 — Test card, guided step and `shapes.md` when a rule changes (1 h)
