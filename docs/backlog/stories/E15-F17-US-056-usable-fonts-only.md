---
id: US-056
epic: E15
feature: F17
title: Only the fonts the editor can use are listed
status: done
points: 2
---

# E15 · F17 · US-056 — Only the fonts the editor can use are listed

As a symbol designer, I want the font list to offer only fonts that the editor can draw, measure and embed, and to know that reading the installed fonts comes first, so that every font I choose gives a frame that fits its letters.

From the Product Owner's second pass of VAL-018 (2026-10-16), items F1 and F2:

- **F1**: Inter stayed first in the font list though it is not installed and cannot be drawn. **Decision**: a font that is not available is not listed, Inter included; the playground chooses a default font once the installed fonts are read; **Installed fonts** comes first and shows clearly that it must be done first.
- **F2**: Apple Color Emoji gives a wrong box, and Al Nile and Apple Braille are of no use for Latin text. **Decision**: a font whose `fsType` forbids embedding is not listed. **Agent's addition** (ADR-0060, Latin script the target): a font lacking a letter of the ISO basic Latin alphabet is not listed either. **Confirmed** by the Product Owner on 2026-10-16, asked by questionnaire (ADR-0061): such an installed font is left out whatever its script, a Greek-, Cyrillic- or Hebrew-only one included, and can still be imported by hand.

**Cause of the emoji box, checked first** (probe on the Product Owner's Mac, 2026-10-16): Apple Color Emoji maps 28 characters — the space, `#`, `*` and the digits of its keycaps — and none of the 52 letters A to Z, a to z; its `fsType` is 4, embeddable. The browser draws the letters with a fallback font while the editor measures them with `.notdef`, 800 units wide on 800 per em, so the frame is a square per letter. Al Nile maps 36 characters and Apple Braille 3, no letter either; Helvetica maps every letter.

**Decisions** (provisional, taken by the agent on 2026-10-16, to confirm at VAL-018; `docs/domain/shapes.md`, Text, Default font available and Only usable fonts listed): the letters required are the 52 of the ISO basic Latin alphabet (`REF-WIKI-ISO-BASIC-LATIN`), settled by ADR-0061; a face both not embeddable and without the letters is said as not embeddable; a family is left out when none of its faces is listed, its reason counted in the reading line; a family chosen keeps its faces that are listed and says the others; a file imported with **Import a font** is not judged, the designer having chosen it; once a family is read while the text's font has no metrics, the first family of the list is chosen; **Installed fonts** sits first, above the font list, in the accent color, under a line saying to start with it.

## Acceptance criteria (F17.AC14)

- Given a browser that draws none of Inter's faces, when the Text tool opens, then the font list is empty, no text is drawn, and the page says to click Installed fonts first.
- Given that browser and two installed families, Test Sans then Test Serif, when Installed fonts is clicked and the families read, then Test Sans is chosen, the text drawn in it, and the page says it was chosen because Inter is not installed.
- Given a browser that draws Inter, when the installed fonts are read, then Inter stays first and chosen.
- Given installed families Test Emoji (no Latin letter), Test Locked (`fsType` 2), Test Sans and Test Bad (not a font), when they are read, then only Inter and Test Sans are listed, and the line says `Installed fonts read: 1 families; 3 left out: 1 not embeddable (Test Locked); 1 no Latin letters (Test Emoji); 1 unreadable (Test Bad).`
- Given Test Sans with a regular face and a bold face whose `fsType` is 2, when Test Sans is chosen, then only 400 is offered and the page says the bold face was refused because its `fsType` forbids embedding it.
- Given the 52 letters mapped but one, z or A, then the font is refused; with all 52, whatever else it maps, it is kept.

## Product Owner test

| Step | Do                                                                   | You should see                                                                                  |
| ---- | -------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| 1    | Chrome, Inter not installed: open the page, Text tool                | Installed fonts first, in blue, under "Start with Installed fonts"; an empty font list; no text |
| 2    | Click Installed fonts, allow                                         | the first family read chosen at once and the text drawn in it; the line counting the families   |
| 3    | Wait for the line `Installed fonts read: …`, then open the font list | no Apple Color Emoji, Al Nile nor Apple Braille; the line counts them under "no Latin letters"  |
| 4    | Choose Helvetica, then Size 32                                       | the dashed frame hugging the letters                                                            |

## Tasks

One task = one commit, referenced as `US-056.Tn`.

- [x] T1 — The basic Latin letters a font must map, and why a face cannot be listed: not embeddable, or a letter missing (`hasBasicLatinLetters`, `unlistedFaceReason`); `shapes.md`, `references.md`, the story registered (1 h)
- [x] T2 — The playground lists only usable fonts: installed faces left out and counted by reason, Inter left out when not drawn, the first family read chosen, Installed fonts first; `shapes.md`, `files.md`, the checklist lines (2 h)
- [x] T3 — The verifier's findings, first: a family the designer chose while the others are read stays chosen, its message kept (the guard of `writeDefaultChoice`) (0.5 h)
- [x] T4 — A face imported by hand that lacks the Latin letters is kept and listed, never judged (0.25 h)
- [x] T5 — One order of the reasons a face or a family is left out, for a face, a family and the reading line (`LEFT_OUT_REASONS`), written once in `shapes.md`; a family whose faces have different reasons (0.5 h)
- [x] T6 — The layout tested: Installed fonts before the font list, under the line saying to start with it (0.25 h)
- [x] T7 — ADR-0061: an installed font lacking a basic Latin letter is not listed, whatever its script (the Product Owner's decision, refining ADR-0060, decision 4); `shapes.md` settles it (0.25 h)
