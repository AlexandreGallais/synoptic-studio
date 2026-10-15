import { isValidText, isValidTextBox, textFontMetrics } from "../../src";
import { writeColorSwatches } from "../color-fields";
import { writeLeftOutFaces } from "../document";
import { readFontSession, writeFontFields } from "../fonts";
import { selectElement, writeText } from "../page";

import { readTextModel } from "./read-text-model";
import { showText } from "./show-text";

import type { Text } from "../../src";

/** Weight of the bold button (`REF-CSS-FONTS-4`, §2.2). */
const BOLD = 700;

/** Message shown when a text cannot enter the model. */
const REFUSED =
  "Size must be an integer from 1 to 100 000, opacity and background opacity from 0 to 100, spacing from −100 000 to 100 000, X, Y and the angle integers, the color and background color # and six hexadecimal digits, the font named, and the box's width and height, its maximum lines and the line height integers from 0 to 100 000.";

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
 * Updates the page for the Text tool: the bold button pressed at weight 700 or while it holds the
 * weight before (US-037); the fields
 * read only in a box shown when Box is chosen, and while one of them is refused, so that the
 * value outlined stays in sight (US-033); the font and weight lists brought up to date with the
 * session's fonts (`writeFontFields`, US-036); the text drawn with its block, measured with the
 * metrics of the family chosen, Inter or a font imported or installed and read (US-036);
 * each color's swatch painted with it (US-035);
 * a refused text clears the drawing and says why, its fields outlined.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F05-US-031-place-a-text.md
 */
export function updateTextTool(document: Document): void {
  writeFontFields(document);

  const text = readTextModel(document);
  const metrics = textFontMetrics(text, readFontSession(document).kept);
  const isValid = metrics !== undefined && isValidText(text);

  writeBoldState(document, text.weight);
  writeTextBoxFields(document, text);
  writeColorSwatches(document);
  writeTextValidity(document, isValid);

  if (isValid) {
    writeText(document, "status", "");
    showText(document, text, metrics);

    return;
  }

  writeText(document, "status", REFUSED);
  selectElement(document, "canvas").replaceChildren();
  writeLeftOutFaces(document, []);
}
