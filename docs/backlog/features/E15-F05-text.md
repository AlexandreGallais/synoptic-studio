---
id: F05
epic: E15
title: Text and its characters
status: done
---

# E15 · F05 — Text and its characters

**Benefit hypothesis**: the designer labels a symbol with a text that stays where it is meant to be whatever its font, measured by the library so that its block is known without drawing it.

Moved from E01 to E15 on 2026-10-12 (Product Owner): text is a subject of its own.

## Refinement, round 1 (2026-10-14)

The Product Owner's answers, in neutral words. The second and last round follows the research on how text can be laid out and drawn portably.

- **Lines and box.** Text behaves as in Figma and in a word processor:
  - a text either fits its content (auto width) or sits in a box of a given width and height;
  - in a box, lines wrap by themselves, and a text that is too long can end with an ellipsis, with a maximum number of lines;
  - the text is aligned horizontally and vertically in its box, so that it can always be centred;
  - long texts are needed: in a synoptic view, a button may open a panel explaining a start-up procedure. Scrolling inside a box on a touch screen would be very welcome.
- **Rendering: the key question.** HTML in a `foreignObject` would give the browser's whole text layout (flex, alignment, ellipsis). It is not read by every SVG reader, though. The Product Owner prefers, when possible, to compute the layout with mathematics, without showing the text in a page, even when the font changes. The SVG then holds plain text, readable everywhere. To be answered by the research before round 2.
- **Options wanted** (the agent may add standard ones found while researching; the Product Owner then keeps or drops each):
  - the font, the size and the color, typed freely;
  - the weight: quick bold and italic buttons, as in Word, plus the full list of weights (thin to black);
  - line height, letter spacing and word spacing, needed to fit text in small frames;
  - underline and strike-through;
  - a background color behind the text, as a highlighter;
  - text transforms: uppercase, lowercase, capitalize the first word or every word, and the programmers' cases (camel, Pascal, kebab, snake, constant), so that a value typed once comes out formatted;
  - a vertical trim from the cap height to the baseline, so that texts of different sizes line up on where their letters start;
  - truncation with a maximum number of lines;
  - a wrap style ("pretty", "balance");
  - left-to-right and right-to-left writing.
- **Not wanted.** Lists (a dash typed by hand is enough), and highlight shapes beyond a background color.
- **Turning.** Like shapes: a text turns about its anchor point, clockwise, by an integer angle. When shapes get movable rotation anchors, text gets the same options.
- **Fonts.** Every font allowed. Showing the fonts installed on the computer is wanted in the editor later, if the browser allows it; the library takes a font name.
- **Text styles.** Named styles ("title", "body", "description") that predefine a text's options. In the free edition they are templates kept locally that forbid nothing. In the pro edition an administrator would set rules that everyone follows (`docs/domain/editions.md`). When they come is left to the agent.
- **Uses.** A symbol's identifier written beside it, by hand first, later a parameter of the symbol; long explanatory texts in views.

## Refinement, round 2 (2026-10-14)

The research is summarized in research note 0017 (written by SP-015). The Product Owner's answers, in neutral words:

- **Rendering.** The library computes the lines: it measures the text from the font's metrics, breaks the lines, and writes positioned lines readable by every SVG reader. An HTML layer for browsers comes on top (F15), so that a browser lays the text out live with its own fonts. It is plain JavaScript in the free edition.
- **Speed.** In a running synoptic view a text may change a hundred times a second, on a thousand texts. Measuring must therefore be as fast as possible: the font's metrics are read once and kept, and measuring a text is then a sum of numbers. Moving, turning, showing, hiding and recoloring stay cheap transforms.
- **Fonts.**
  - A symbol names its font with a fallback (`Inter`, else `sans-serif`).
  - The editor is installed on the computer, so it can read the font files: TrueType and OpenType files, imported or installed. It reads their metrics once and stores them in advance.
  - An option embeds a font in a symbol or in a synoptic view; a view would keep each font once, even when several of its symbols use it.
  - Text converted to paths is not wanted.
- **Split.** The epic is split into as many features as needed, and the agent builds the whole epic before the Product Owner validates it.
- **Units.** One unit of measure everywhere, as for shapes. A size is an integer from 1 to 100 000. No millimetres nor points: the editor is made for synoptic views, not for print.
- **Colors.**
  - A color is a hexadecimal value with an opacity from 0 to 100 %, both always visible, as in Figma, for a quick copy and paste.
  - A picker opens on a click, with a hue slider and an opacity slider, in HSL (with its levels of white) or HSB (numbers that move through the hexadecimal values). The hexadecimal translation is always shown.
  - RGB adds nothing to hexadecimal; CSS color names are not wanted.
  - Libraries of saved colors and of text styles: later, in the epic of libraries.
- **Long words.** An option: break a word too long for its box at a character, or move the whole word to the next line. No hyphenation.
- **Empty text.** A text created by a click and left empty disappears when the designer changes tool or clicks elsewhere: an editor behavior (E02). The library draws an empty text as nothing.
- **Scripts.** Start simple: the library measures the scripts that need no shaping (Latin, and with them Greek and Cyrillic), plus Hebrew written right to left. Arabic and Indian scripts are left to the browser's HTML layer. A later server-side layout would need a shaping engine.
- **Turning.** A text turns about the middle of its block: the middle of the measured text when it fits its content, the middle of its box otherwise. When shapes get rotation anchors, text gets them too.

## Acceptance criteria

1. A font file, TrueType or OpenType, is read once into its metrics: advances, kerning pairs, units per em, vertical metrics; the default font's metrics ship with the library (`DERIV-text-width`).
2. A text holds its content (lines broken by the designer), font and fallback, an integer size from 1 to 100 000, a weight from 100 to 900 and italic, a color in hexadecimal with an opacity from 0 to 100 %, letter and word spacing, underline and strike-through, a case transform, a direction (left to right or right to left), an alignment of its lines, a position and an angle.
3. The library measures each line as `size × Σ advance / unitsPerEm` plus kerning and spacing, and the text's block from its lines; "Hello World" in Arial 16 measures 82.6875 without kerning.
4. The text is drawn as one `<text>` with one positioned `<tspan>` per line, readable by any SVG reader (ADR-0052); its case is applied to the string by the library.
5. A text turns about the middle of its block, by an integer angle kept from 0 to 359; its block is shown when it is selected; turned, a change of its block's size keeps its turned top-left corner in place (added 2026-10-15, ADR-0057, EN-042).
6. Every shape of E01 and E14 draws as before.

## Stories

In delivery order. Set ready by the run (ADR-0045).

| ID | Type | Title | Status |
| --- | --- | --- | --- |
| [SP-015](../stories/E15-F05-SP-015-research-e15-text.md) | Spike | Research for E15: text measured from font metrics and drawn as positioned lines | done |
| [EN-031](../stories/E15-F05-EN-031-font-metrics-from-a-font-file.md) | Enabler | Font metrics read from a TrueType or OpenType file | done |
| [EN-032](../stories/E15-F05-EN-032-text-in-the-model.md) | Enabler | A text in the model: its characters, its lines and its measured block | done |
| [US-031](../stories/E15-F05-US-031-place-a-text.md) | User story | Place a text with the Text tool | done |
| [CHK-014](../stories/E15-F05-CHK-014-checkpoint-f05.md) | Checkpoint | Checkpoint in the middle of F05 | done |
| [US-032](../stories/E15-F05-US-032-turn-a-text.md) | User story | Turn a text and see its measured block | done |
| [AUD-015](../stories/E15-F05-AUD-015-audit-f05.md) | Audit | Audit F05 | done |
| [US-037](../stories/E15-F05-US-037-bold-gives-back-the-weight-and-no-sentence-case.md) | User story | The bold button gives back the weight before, and sentence case removed | done |
| [EN-042](../stories/E15-F05-EN-042-a-turned-text-keeps-its-corner-when-it-grows.md) | Enabler | A turned text keeps its turned top-left corner when its block changes size | done |
| [US-038](../stories/E15-F05-US-038-a-turned-text-grows-in-place.md) | User story | A turned text grows along its direction and stays in place | done |
| [VAL-014](../stories/E15-F05-VAL-014-validate-f05.md) | Validation | Validate F05 | done |

## Feature plan

| Acceptance criterion | Realized by | Verified by |
| --- | --- | --- |
| F05.AC1 | SP-015, EN-031 | AUD-015 |
| F05.AC2 | EN-032, US-031, US-037 | AUD-015 |
| F05.AC3 | SP-015, EN-032 | AUD-015 |
| F05.AC4 | EN-032, US-031 | AUD-015 |
| F05.AC5 | US-032, EN-042, US-038 | AUD-015, VAL-014 |
| F05.AC6 | every story | `npm run check:all` |
