import { isValidText, isValidTextBox } from "../../src";
import { writeColorSwatches, writeFollowedColors } from "../color-fields";
import { DEFAULT_FONT_MISSING } from "../default-font";
import { writeLeftOutFaces } from "../document";
import { writeFontFields } from "../fonts";
import { selectElement, writeText } from "../page";
import { readTextMeasure, readTextMetrics } from "../text-measure";
import { writeTextPosition } from "../text-placement";

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
 * @see docs/backlog/stories/E15-F05-US-031-place-a-text.md
 */
function writeTextValidity(document: Document, isValid: boolean): void {
  for (const field of document.querySelectorAll("#text-options input, #placement input")) {
    field.setAttribute("aria-invalid", String(!isValid));
  }
}

/**
 * Shows the fields read only in a box when Box is chosen, and while one of them is refused, so
 * that the value outlined stays in sight after Fit content is chosen (US-033).
 *
 * @kind procedure
 * @param document - page document
 * @param text - the text read from the fields, valid or not
 * @see docs/backlog/stories/E15-F14-US-033-put-a-text-in-a-box.md
 */
function writeTextBoxFields(document: Document, text: Text): void {
  selectElement(document, "text-box-fields").hidden = !text.isInBox && isValidTextBox(text);
}

/**
 * Shows the bold button pressed at weight 700, or while it holds the weight before (US-037).
 *
 * @kind procedure
 * @param document - page document
 * @param weight - the text's weight
 * @see docs/backlog/stories/E15-F05-US-037-bold-gives-back-the-weight-and-no-sentence-case.md
 */
function writeBoldState(document: Document, weight: number): void {
  const bold = selectElement(document, "text-bold");

  bold.setAttribute("aria-pressed", String(weight === BOLD || "heldWeight" in bold.dataset));
}

/**
 * Clears what a refused text drew: the drawing, its straight box's line (US-039) and the faces
 * left out of its document.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F14-US-039-straight-box-of-a-turned-text.md
 */
function clearTextDrawing(document: Document): void {
  selectElement(document, "canvas").replaceChildren();
  selectElement(document, "straight-box").hidden = true;
  writeLeftOutFaces(document, []);
}

/**
 * Draws a valid text from what it measures, its lines and block kept while their inputs stay the
 * same (`readTextMeasure`, US-048), placed so that a turned text keeps its corner
 * (`writeTextPosition`, US-038), then drawn once (`showText`).
 *
 * @kind procedure
 * @param document - page document
 * @param text - a valid text
 * @param metrics - the metrics of its face
 * @see docs/backlog/stories/E15-F17-US-048-fluid-changes-held-keys.md
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
 * @see docs/backlog/stories/E15-F05-US-031-place-a-text.md
 */
function writeTextRefusal(document: Document, isFontMissing: boolean): void {
  writeText(document, "status", isFontMissing ? DEFAULT_FONT_MISSING : REFUSED);
  clearTextDrawing(document);
}

/**
 * Updates the page for the Text tool: the underline's and the strike-through's fields showing the
 * text's color while they follow it (`writeFollowedColors`, US-041); the bold button pressed at weight 700 or while it holds the
 * weight before (US-037); the fields
 * read only in a box shown when Box is chosen, and while one of them is refused, so that the
 * value outlined stays in sight (US-033); the font and weight lists brought up to date with the
 * session's fonts (`writeFontFields`, US-036); the text drawn with its block, measured with the
 * metrics of the family chosen — the metrics, lines and block kept while their inputs stay the
 * same (`readTextMetrics`, `readTextMeasure`, US-048) — Inter or a font imported or installed and read (US-036), a
 * turned text whose block or box changed size placed so that its turned top-left corner stays
 * (`writeTextPosition`, US-038);
 * each color's swatch painted with it (US-035);
 * a refused text clears the drawing and its straight box's line and says why, its fields outlined;
 * a text whose font has no face, Inter when the browser draws none of its faces (US-053), is not
 * drawn either, the page saying Inter is missing, no field outlined.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F05-US-031-place-a-text.md
 */
export function updateTextTool(document: Document): void {
  writeFontFields(document);
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
