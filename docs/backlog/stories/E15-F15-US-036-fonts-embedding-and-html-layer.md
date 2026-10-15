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

Updated on 2026-10-15 from a try of the playground before VAL-014 (Product Owner decision): the font field typed by name let the designer choose a font without metrics, such as `monospace`, which the playground measured with Inter while the browser drew another font, so its block and its trim were wrong. Measuring only with Inter is the defect: the size of a text must be computed with the metrics of the font selected. The font field becomes a list of the families the editor can measure, and the weight list shows the chosen family's faces only.

## Acceptance criteria

- Given the Text tool, when the designer opens the font field, then it is a list of the families the editor can measure: Inter, the default, first, then the families imported, then the installed ones read with the permission; no name can be typed, so no font without metrics, such as `monospace`, can be chosen.
- Given the family Arial imported with its Regular (400) and Bold (700) faces, when the designer chooses Arial, then the weight list offers 400 and 700 only, and "Hello World" at 16 measures 82.6875 wide without kerning, with Arial's metrics, not Inter's; given Inter, then the weight list offers its nine weights, 100 to 900.
- Given a text in Arial at 700, when the designer chooses Inter, then the weight stays 700, a face Inter has; given a weight the new family lacks, then the nearest face by the rule of `shapes.md` (Font face) is chosen and shown.
- Given the Text tool, when the designer clicks **Import a font** and chooses a `.ttf` or `.otf` file (`accept=".ttf,.otf"`, `REF-MDN-INPUT-FILE`), then its family is added to the font list with its face, the text is measured with it, and a file that is not a font is refused with a line saying why (EN-039).
- Given Chrome or Edge, when the designer clicks **Installed fonts**, then the browser asks for the `local-fonts` permission, and once granted the list shows the installed families; choosing one reads its faces' files and measures the text with them (`REF-WICG-LOCAL-FONT-ACCESS`, `REF-CHROME-LOCAL-FONTS`); given Firefox or Safari, the button says the browser cannot list installed fonts and points to **Import a font**; given the permission refused, the page says so.
- Given a text in Arial imported and **Embed fonts** on, when the drawing is exported, then its SVG holds one `<style>` in `<defs>` with one rule per face used; given a text in Inter whose file is not imported, then the page lists Inter as not embedded, its file not held (EN-040).
- Given **HTML layer** on, when the drawing is shown and exported, then each text is a `switch` of its HTML layer and its positioned lines; given the exported SVG opened in a browser, the text is laid out by the browser; the same SVG drawn by resvg shows the positioned lines (EN-041).
- Given a text in Arabic (`مضخة`), the HTML layer on, when shown in a browser, then its letters are joined as the script writes them, which the positioned lines do not do (`REF-HARFBUZZ-SHAPING`).
- Given both options off, the page as it opens, then every shape and text of E01, E14, F05, F14 and F16 draws as before.

## Product Owner test

Open `http://localhost:5173/` in Chrome, click **Text** and click on the drawing. Type `Hello World` in **Content** and 16 in **Size**. The measured block is the `width` and `height` under **Contour** in the pipeline column. In the file dialog, Cmd+Shift+G opens a path such as `/System/Library/Fonts/Supplemental/Arial.ttf`.

| Step | Do                                                                                       | You should see                                                                                                             |
| ---- | ---------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| 1    | Open the **Font** list, then the **Weight** list                                         | Inter alone, no name to type; the nine weights 100 to 900; width 87.5546875                                                |
| 2    | Click **Import a font**, choose `/System/Library/Fonts/Supplemental/Arial.ttf`           | "Arial 400 imported."; the list Inter, Arial, Arial chosen; weights 400 only; width 82.3984375, height 18.3984375          |
| 3    | Click **Import a font**, choose `Arial Bold.ttf` in the same folder                      | "Arial 700 imported."; weights 400 and 700                                                                                 |
| 4    | Click **Bold**, then choose Inter in the font list                                       | Bold pressed, weight 700; Inter keeps 700                                                                                  |
| 5    | Choose weight 300, then Arial in the font list                                           | weight 400, Arial's face nearest to 300                                                                                    |
| 6    | Click **Import a font**, choose any file that is not a font, such as a `.txt` file       | "… is not a TrueType or OpenType font file (.ttf, .otf): refused."; the list unchanged                                     |
| 7    | Click **Installed fonts**, allow the browser                                             | "Installed families listed: N. Choose one in the font list."; the installed families after Arial                           |
| 8    | Choose Georgia in the font list                                                          | "Georgia: 4 faces read."; the text in Georgia, its block changed; weights 400 and 700                                      |
| 9    | Open the page in Firefox or Safari, click **Installed fonts**                            | a line saying the browser cannot list the installed fonts, pointing to **Import a font**                                   |
| 10   | Back in Chrome with Arial chosen, turn **Embed fonts** on                                | the SVG output starts with `<defs>` holding one `<style>` and one `@font-face` for `"Arial"` at weight 400; no face listed |
| 11   | Choose Inter                                                                             | "Inter 400: not embedded, its file not held." under the options; no `<style>`                                              |
| 12   | Choose Arial, click **Export SVG**, open `drawing.svg` in Chrome                         | the text in Arial, as on the page, without the dashed frame                                                                |
| 13   | Turn **Embed fonts** off and **HTML layer** on, type `مضخة` in **Content**               | the SVG output holds a `<switch>`; the letters joined in the drawing                                                       |
| 14   | Click **Export SVG**; open `drawing.svg` in Chrome, then draw it with resvg if installed | Chrome joins the letters; resvg draws the positioned lines, letters apart                                                  |
| 15   | Turn **HTML layer** off                                                                  | no `<switch>`, no `<defs>`: the drawing and the SVG output as before                                                       |

## Tasks

One task = one commit, referenced as `US-036.Tn`.

- [x] T1 — **Import a font**: a file picker for `.ttf` and `.otf`, the bytes read once (`REF-W3C-FILE-API`), the face kept and listed in the font field, a file refused said why; tests first (2 h)
- [x] T2 — **Installed fonts**: on a click, `queryLocalFonts()` where the browser has it, the permission refused or the API missing said, each face read from its `blob()` as a file; tests first, the API faked (2 h)
- [x] T3 — The font field as a list of the families with metrics, the weight list of the chosen family's faces, the text measured with the chosen family's metrics; the guided steps that typed a font name brought up to date; tests first (3 h)
- [x] T4 — **Embed fonts** and **HTML layer** toggles for the drawing, off as the page opens; the faces left out listed; the export holding them; the guided steps and the test card; tests first (3 h)
