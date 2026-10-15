---
id: US-036
epic: E15
feature: F15
title: Import or choose a font, embed it, and turn the HTML layer on
status: ready
points: 5
---

# E15 · F15 · US-036 — Import or choose a font, embed it, and turn the HTML layer on

As a designer, I want to use any font of my computer, imported from its file or chosen among the installed ones, to embed the fonts my drawing uses, and to let browsers lay its texts out with their own fonts, so that a symbol shows the same text wherever it is opened.

The playground's drawing stands for a document: its options apply to all its texts. Rules: `shapes.md` (Fonts from files, Embedded fonts, HTML layer), ADR-0053, `DERIV-font-embedding`, `DERIV-html-layer`.

## Acceptance criteria

- Given the Text tool, when the designer clicks **Import a font** and chooses a `.ttf` or `.otf` file (`accept=".ttf,.otf"`, `REF-MDN-INPUT-FILE`), then its family is added to the font list with its face, the text is measured with it, and a file that is not a font is refused with a line saying why (EN-039).
- Given Chrome or Edge, when the designer clicks **Installed fonts**, then the browser asks for the `local-fonts` permission, and once granted the list shows the installed families; choosing one reads its faces' files and measures the text with them (`REF-WICG-LOCAL-FONT-ACCESS`, `REF-CHROME-LOCAL-FONTS`); given Firefox or Safari, the button says the browser cannot list installed fonts and points to **Import a font**; given the permission refused, the page says so.
- Given a text in Arial imported and **Embed fonts** on, when the drawing is exported, then its SVG holds one `<style>` in `<defs>` with one rule per face used; given a text in Inter whose file is not imported, then the page lists Inter as not embedded, its file not held (EN-040).
- Given **HTML layer** on, when the drawing is shown and exported, then each text is a `switch` of its HTML layer and its positioned lines; given the exported SVG opened in a browser, the text is laid out by the browser; the same SVG drawn by resvg shows the positioned lines (EN-041).
- Given a text in Arabic (`مضخة`), the HTML layer on, when shown in a browser, then its letters are joined as the script writes them, which the positioned lines do not do (`REF-HARFBUZZ-SHAPING`).
- Given both options off, the page as it opens, then every shape and text of E01, E14, F05, F14 and F16 draws as before.

## Product Owner test

Open `http://localhost:5173/` in Chrome, click **Text** and click on the drawing.

| Step | Do                                                             | You should see                                                 |
| ---- | -------------------------------------------------------------- | -------------------------------------------------------------- |
| 1    | Click **Import a font**, choose a `.ttf` file of your computer | its family in the font list, the text drawn and measured in it |
| 2    | Click **Installed fonts**, allow the browser                   | the installed families listed; choose one, the text in it      |
| 3    | Turn **Embed fonts** on, export the drawing                    | the SVG holds a `<style>` with one `@font-face` per face used  |
| 4    | Choose Inter, which has no file imported                       | a line saying Inter is not embedded, its file not held         |
| 5    | Turn **HTML layer** on, type `مضخة` in the text                | the letters joined in the drawing                              |
| 6    | Turn both options off                                          | the drawing as before                                          |

## Tasks

One task = one commit, referenced as `US-036.Tn`.

- [ ] T1 — **Import a font**: a file picker for `.ttf` and `.otf`, the bytes read once (`REF-W3C-FILE-API`), the face kept and listed in the font field, a file refused said why; tests first (2 h)
- [ ] T2 — **Installed fonts**: on a click, `queryLocalFonts()` where the browser has it, the permission refused or the API missing said, each face read from its `blob()` as a file; tests first, the API faked (2 h)
- [ ] T3 — **Embed fonts** and **HTML layer** toggles for the drawing, off as the page opens; the faces left out listed; the export holding them; the guided steps and the test card; tests first (3 h)
