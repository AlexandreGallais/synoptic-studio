import { writeFontOptions, writeWeightChoice } from "../fonts";
import { selectElement, writeInputValue } from "../page";

import type { TextFields } from "./text-fields";

/**
 * Writes a value into a text field, a text area or a choice of the page, without firing any event.
 *
 * @kind procedure
 * @param document - page document
 * @param id - its `id` attribute, without `#`
 * @param value - the value written
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
 */
function writeCheckedValue(document: Document, id: string, isChecked: boolean): void {
  const box = selectElement(document, id);

  if (box instanceof HTMLInputElement) {
    box.checked = isChecked;
  }
}

/**
 * Types a text's box and background choices into the Text tool's fields, without firing any
 * event: Fit content or Box, long words, the ellipsis, the vertical alignment, the wrap, the trim
 * and the background with its color.
 *
 * @kind procedure
 * @param document - page document
 * @param text - the fields typed
 */
function writeTextBoxValues(document: Document, text: TextFields): void {
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
 * Types a text's underline and strike-through colors into the Text tool's fields, without firing
 * any event: each box "follows the text's color" ticked unless its color was chosen, then the
 * color and the opacity it keeps.
 *
 * @kind procedure
 * @param document - page document
 * @param text - the fields typed
 */
function writeTextStrokeColorValues(document: Document, text: TextFields): void {
  writeCheckedValue(document, "text-underline-follows", !text.hasUnderlineColor);
  writeCheckedValue(document, "text-strike-follows", !text.hasStrikeThroughColor);
  writeFieldValue(document, "text-underline-color", text.underlineColor);
  writeFieldValue(document, "text-strike-color", text.strikeThroughColor);
  writeInputValue(document, "text-underline-opacity", text.underlineOpacity);
  writeInputValue(document, "text-strike-opacity", text.strikeThroughOpacity);
}

/**
 * Types a text's style into the Text tool's fields, without firing any event: underline,
 * strike-through and italic, then the font chosen in its list and the weight among the family's
 * faces, the bold button holding no weight before.
 *
 * @kind procedure
 * @param document - page document
 * @param text - the fields typed
 */
function writeTextStyleValues(document: Document, text: TextFields): void {
  writeCheckedValue(document, "text-underline", text.isUnderlined);
  writeCheckedValue(document, "text-strike", text.isStruckThrough);
  selectElement(document, "text-italic").setAttribute("aria-pressed", String(text.isItalic));
  writeFontOptions(document, text.font);
  writeWeightChoice(document, text.weight);
  delete selectElement(document, "text-bold").dataset["heldWeight"];
}

/**
 * Types a text into the Text tool's fields, its font and weight chosen in their lists,
 * its box's and its background's among them, its strokes' colors, and its X, Y
 * and angle into the placement fields, without firing any event; the text last drawn goes
 * and the box typed, so that the text starts from its own values: the page's on
 * Reset, or a text picked on the drawing.
 *
 * @kind procedure
 * @param document - page document
 * @param text - every field of a text but its fallback
 */
export function writeTextFields(document: Document, text: TextFields): void {
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

  // The text last drawn: a text is never placed from the one before.
  delete selectElement(document, "text-options").dataset["drawnText"];
  // The box typed: its box measured again when Box is chosen.
  delete selectElement(document, "text-options").dataset["boxTyped"];
}
