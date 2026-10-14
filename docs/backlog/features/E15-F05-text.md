---
id: F05
epic: E15
title: Text
status: in-progress
---

# E15 · F05 — Text

**Benefit hypothesis**: the designer labels a symbol.

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

## Acceptance criteria

1. A native text with content, font, integer size, anchor and color is placed (ADR-0002).

## Stories

To be written in refinement, after E15's first spike (text drawn natively or in a `foreignObject`, fonts, measured size: Q31).
