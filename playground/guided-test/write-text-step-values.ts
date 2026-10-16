import { DEFAULT_DOCUMENT_OPTIONS } from "../../src";
import { writeFontOptions, writeWeightChoice } from "../fonts";
import { selectElement, writeInputValue } from "../page";

import type { GuidedText } from "../guided-steps";

/**
 * Writes a value into a text field, a text area or a choice of the page, without firing any event.
 *
 * @kind procedure
 * @param document - page document
 * @param id - its `id` attribute, without `#`
 * @param value - the value written
 * @see docs/backlog/stories/E15-F05-VAL-014-validate-f05.md
 */
function writeFieldValue(document: Document, id: string, value: string): void {
  // An input, a text area and a select each hold their value in `value`.
  Object.assign(selectElement(document, id), { value });
}

/**
 * Ticks or unticks a check box of the page, without firing any event.
 *
 * @kind procedure
 * @param document - page document
 * @param id - its `id` attribute, without `#`
 * @param isChecked - whether it is ticked
 * @see docs/backlog/stories/E15-F05-VAL-014-validate-f05.md
 */
function writeCheckedValue(document: Document, id: string, isChecked: boolean): void {
  const box = selectElement(document, id);

  if (box instanceof HTMLInputElement) {
    box.checked = isChecked;
  }
}

/**
 * Types a step's box and background choices into the Text tool's fields, without firing any
 * event: Fit content or Box, long words, the ellipsis, the vertical alignment, the wrap, the trim
 * and the background with its color (VAL-015).
 *
 * @kind procedure
 * @param document - page document
 * @param text - the fields typed
 * @see docs/backlog/stories/E15-F14-VAL-015-validate-f14.md
 */
function writeTextBoxValues(document: Document, text: GuidedText): void {
  for (const [id, value] of [
    ["text-fit", text.isInBox ? "box" : "content"],
    ["text-vertical-alignment", text.verticalAlignment],
    ["text-background-color", text.backgroundColor],
  ] as const) {
    writeFieldValue(document, id, value);
  }

  writeCheckedValue(document, "text-ellipsis", text.hasEllipsis);
  writeCheckedValue(document, "text-trim", text.isTrimmed);
  writeCheckedValue(document, "text-background", text.hasBackground);
}

/**
 * Types a step's underline and strike-through colors into the Text tool's fields, without firing
 * any event: each box "follows the text's color" ticked unless its color was chosen, then the
 * color and the opacity it keeps (US-041).
 *
 * @kind procedure
 * @param document - page document
 * @param text - the fields typed
 * @see docs/backlog/stories/E15-F16-US-041-underline-and-strike-through-colors.md
 */
function writeTextStrokeColorValues(document: Document, text: GuidedText): void {
  writeCheckedValue(document, "text-underline-follows", !text.hasUnderlineColor);
  writeCheckedValue(document, "text-strike-follows", !text.hasStrikeThroughColor);
  writeFieldValue(document, "text-underline-color", text.underlineColor);
  writeFieldValue(document, "text-strike-color", text.strikeThroughColor);
  writeInputValue(document, "text-underline-opacity", text.underlineOpacity);
  writeInputValue(document, "text-strike-opacity", text.strikeThroughOpacity);
}

/**
 * Types a step's style into the Text tool's fields, without firing any event: underline,
 * strike-through and italic, then the font chosen in its list and the weight among the family's
 * faces (US-036), the bold button holding no weight before (US-037); the drawing's options those
 * of a new document (`DEFAULT_DOCUMENT_OPTIONS`), Embed fonts off, as the page opens (US-036).
 *
 * @kind procedure
 * @param document - page document
 * @param text - the fields typed
 * @see docs/backlog/stories/E15-F15-US-036-import-choose-and-embed-a-font.md
 */
function writeTextStyleValues(document: Document, text: GuidedText): void {
  writeCheckedValue(document, "text-underline", text.isUnderlined);
  writeCheckedValue(document, "text-strike", text.isStruckThrough);
  selectElement(document, "text-italic").setAttribute("aria-pressed", String(text.isItalic));
  writeFontOptions(document, text.font);
  writeWeightChoice(document, text.weight);
  delete selectElement(document, "text-bold").dataset["heldWeight"];
  writeCheckedValue(document, "embed-fonts", DEFAULT_DOCUMENT_OPTIONS.hasEmbeddedFonts);
}

/**
 * Types a step's text into the Text tool's fields, its font and weight chosen in their lists
 * (US-036), its box's and its background's among them (VAL-015), its strokes' colors (US-041), the drawing's options off (US-036), and its X, Y and angle into the placement fields, without firing any event; the
 * placements each tool kept go, and the text last drawn (US-038), so that the step starts from
 * its own values (VAL-014).
 *
 * @kind procedure
 * @param document - page document
 * @param text - the fields typed
 * @see docs/backlog/stories/E15-F05-VAL-014-validate-f05.md
 */
export function writeTextStepValues(document: Document, text: GuidedText): void {
  for (const [id, value] of [
    ["text-content", text.content],
    ["text-color", text.color],
    ["text-case", text.textCase],
    ["text-direction", text.direction],
    ["text-alignment", text.alignment],
  ] as const) {
    writeFieldValue(document, id, value);
  }

  writeTextStyleValues(document, text);

  for (const [id, value] of [
    ["text-size", text.size],
    ["text-opacity", text.opacity],
    ["text-letter-spacing", text.letterSpacing],
    ["text-word-spacing", text.wordSpacing],
    ["text-box-width", text.boxWidth],
    ["text-box-height", text.boxHeight],
    ["text-maximum-lines", text.maximumLines],
    ["text-line-height", text.lineHeight],
    ["text-background-opacity", text.backgroundOpacity],
    ["shape-x", text.x],
    ["shape-y", text.y],
    ["angle", text.angle],
  ] as const) {
    writeInputValue(document, id, value);
  }

  writeTextBoxValues(document, text);
  writeTextStrokeColorValues(document, text);

  // The keys of `TOOL_PLACEMENT`, the placements each tool kept.
  delete selectElement(document, "placement").dataset["textPlacement"];
  delete selectElement(document, "placement").dataset["shapePlacement"];
  // The text last drawn (US-038): a step is never placed from the step before.
  delete selectElement(document, "text-options").dataset["drawnText"];
}
