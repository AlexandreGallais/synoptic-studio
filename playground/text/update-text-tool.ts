import { DEFAULT_FONT, isValidText, textFontMetrics } from "../../src";
import { selectElement, writeText } from "../page";

import { readTextModel } from "./read-text-model";
import { showText } from "./show-text";

/** Weight of the bold button (`REF-CSS-FONTS-4`, §2.2). */
const BOLD = 700;

/** Message shown when a text cannot enter the model. */
const REFUSED =
  "Size must be an integer from 1 to 100 000, opacity from 0 to 100, spacing from −100 000 to 100 000, X, Y and the angle integers, the color # and six hexadecimal digits, and the font named.";

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
 * Updates the page for the Text tool: the bold button pressed at weight 700 or heavier; the text drawn with
 * its block, measured with the default font's metrics (Inter) when its font has none (ADR-0052);
 * a refused text clears the drawing and says why, its fields outlined.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F05-US-031-place-a-text.md
 */
export function updateTextTool(document: Document): void {
  const text = readTextModel(document);
  const metrics = textFontMetrics(text, [DEFAULT_FONT]);
  const isValid = metrics !== undefined && isValidText(text);

  selectElement(document, "text-bold").setAttribute("aria-pressed", String(text.weight >= BOLD));
  writeTextValidity(document, isValid);

  if (isValid) {
    writeText(document, "status", "");
    showText(document, text, metrics);

    return;
  }

  writeText(document, "status", REFUSED);
  selectElement(document, "canvas").replaceChildren();
}
