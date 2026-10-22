import { centredPosition } from "../../src";
import { selectElement, writeInputValue } from "../page";
import { readTextModel, writeTextSections } from "../text";
import { readTextMeasure, readTextMetrics } from "../text-measure";

import type { Point } from "../../src";

/** The content of a new text, selected so that typing replaces it (SP-026). */
const NEW_TEXT_CONTENT = "Text";

/**
 * Makes the Text tool's fields those of a new text (SP-026): its content `Text`, fitting it,
 * every other option as the last text left it; the box typed for the last text forgotten, so that
 * Box chosen measures the new text's (US-057).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_1-tools
 */
function writeNewTextFields(document: Document): void {
  Object.assign(selectElement(document, "text-content"), { value: NEW_TEXT_CONTENT });
  Object.assign(selectElement(document, "text-fit"), { value: "content" });
  delete selectElement(document, "text-options").dataset["boxTyped"];
}

/**
 * Places a text by a click (F35): a new text (`writeNewTextFields`) whose measured block is
 * centred on the point on integers (`centredPosition`), a text whose font has no metrics yet
 * (US-056) taken as 0 × 0, its top-left on the point; its fields shown and its content selected in the Content field, so that
 * typing replaces it.
 *
 * @kind procedure
 * @param document - page document
 * @param point - the point clicked, in the coordinates of the SVG
 * @see docs/domain/interaction.md#_1-tools
 */
export function writePlacedText(document: Document, point: Point): void {
  writeNewTextFields(document);

  const text = readTextModel(document);
  const metrics = readTextMetrics(document, text);
  const block =
    metrics === undefined ? { height: 0, width: 0 } : readTextMeasure(metrics, text).block;
  const position = centredPosition(point, block);
  const content = selectElement(document, "text-content");

  writeInputValue(document, "shape-x", position.x);
  writeInputValue(document, "shape-y", position.y);

  // Shown before the redraw, so that the content can take the focus.
  writeTextSections(document, true);

  if (!(content instanceof HTMLTextAreaElement)) {
    return;
  }

  content.focus();
  content.select();
}
