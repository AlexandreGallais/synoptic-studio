import { isValidText, isValidTextBox } from "../../src";
import { writeColorSwatches, writeFollowedColors } from "../color-fields";
import { DEFAULT_FONT_MISSING, writeFontAvailability } from "../default-font";
import { writeFontFields } from "../fonts";
import { selectElement, writeText } from "../page";
import { readTextMeasure, readTextMetrics } from "../text-measure";
import { writeTextPosition } from "../text-placement";

import { clearTextDrawing } from "./clear-text-drawing";
import { readTextModel } from "./read-text-model";
import { showText } from "./show-text";

import type { FontMetrics, Text } from "../../src";

/** Weight of the bold button (`REF-CSS-FONTS-4`, §2.2). */
const BOLD = 700;

/** Message shown when a text cannot enter the model. */
const REFUSED =
  "Size must be an integer from 1 to 100 000, opacity, underline opacity, strike-through opacity and background opacity from 0 to 100, spacing from −100 000 to 100 000, X, Y and the angle integers, the color, underline, strike-through and background colors # and six hexadecimal digits, the font named, and the box's width and height, its maximum lines and the line height integers from 0 to 100 000.";

/**
 * Outlines the Text tool's fields and the position fields when the text is refused.
 *
 * @kind procedure
 * @param document - page document
 * @param isValid - whether the text can enter the model
 */
function writeTextValidity(document: Document, isValid: boolean): void {
  for (const field of document.querySelectorAll("#text-options input, #placement input")) {
    field.setAttribute("aria-invalid", String(!isValid));
  }
}

/**
 * Shows the fields read only in a box when Box is chosen, and while one of them is refused, so
 * that the value outlined stays in sight after Fit content is chosen.
 *
 * @kind procedure
 * @param document - page document
 * @param text - the text read from the fields, valid or not
 */
function writeTextBoxFields(document: Document, text: Text): void {
  selectElement(document, "text-box-fields").hidden = !text.isInBox && isValidTextBox(text);
}

/**
 * Shows the bold button pressed at weight 700, or while it holds the weight before.
 *
 * @kind procedure
 * @param document - page document
 * @param weight - the text's weight
 */
function writeBoldState(document: Document, weight: number): void {
  const bold = selectElement(document, "text-bold");

  bold.setAttribute("aria-pressed", String(weight === BOLD || "heldWeight" in bold.dataset));
}

/**
 * Draws a valid text from what it measures, its lines and block kept while their inputs stay the
 * same (`readTextMeasure`), placed so that a turned text keeps its corner
 * (`writeTextPosition`), then drawn once (`showText`).
 *
 * @kind procedure
 * @param document - page document
 * @param text - a valid text
 * @param metrics - the metrics of its face
 */
function showMeasuredText(document: Document, text: Text, metrics: FontMetrics): void {
  const measured = readTextMeasure(metrics, text);

  showText(document, writeTextPosition(document, { block: measured.block, text }), {
    measured,
    metrics,
  });
}

/**
 * Says why a text is not drawn, then clears the drawing: its font without a face, Inter missing
 * (US-053), or a value refused.
 *
 * @kind procedure
 * @param document - page document
 * @param isFontMissing - whether the text's font has no face the browser draws
 */
function writeTextRefusal(document: Document, isFontMissing: boolean): void {
  writeText(document, "status", isFontMissing ? DEFAULT_FONT_MISSING : REFUSED);
  clearTextDrawing(document);
}

/**
 * Updates the page for the Text tool: the underline's and the strike-through's fields showing the
 * text's color while they follow it (`writeFollowedColors`); the bold button pressed at weight 700 or while it holds the
 * weight before; the fields
 * read only in a box shown when Box is chosen, and while one of them is refused, so that the
 * value outlined stays in sight; the font and weight lists brought up to date with the
 * session's fonts (`writeFontFields`), the font list, Installed fonts and the Content
 * field enabled only when they can be used (`writeFontAvailability`); the text drawn with its block, measured with the
 * metrics of the family chosen — the metrics, lines and block kept while their inputs stay the
 * same (`readTextMetrics`, `readTextMeasure`) — Inter or a font imported or installed and read, a
 * turned text placed from the start of its straight box X and Y show, a change of size keeping
 * its turned top-left corner (`writeTextPosition`);
 * each color's swatch painted with it;
 * a refused text clears the drawing and its straight box's line and says why, its fields outlined;
 * a text whose font has no face, Inter when the browser draws none of its faces, is not
 * drawn either, the page saying Inter is missing, no field outlined.
 *
 * @kind procedure
 * @param document - page document
 */
export function updateTextTool(document: Document): void {
  writeFontFields(document);
  writeFontAvailability(document);
  writeFollowedColors(document);

  const text = readTextModel(document);
  const metrics = readTextMetrics(document, text);
  const isValid = metrics !== undefined && isValidText(text);

  writeBoldState(document, text.weight);
  writeTextBoxFields(document, text);
  writeColorSwatches(document);
  writeTextValidity(document, isValidText(text));

  if (isValid) {
    writeText(document, "status", "");

    showMeasuredText(document, text, metrics);

    return;
  }

  writeTextRefusal(document, metrics === undefined);
}
