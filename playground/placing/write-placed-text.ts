import { centredPosition } from "../../src";
import { writeInputValue } from "../page";
import { readTextModel } from "../text";
import { readTextMeasure, readTextMetrics } from "../text-measure";

import { writeNewTextFields } from "./write-new-text-fields";

import type { Point } from "../../src";

/**
 * Places a text by a click (F35): a new text (`writeNewTextFields`) whose measured block is
 * centred on the point on integers (`centredPosition`), a text whose font has no metrics yet
 * (US-056) taken as 0 × 0, its top-left on the point.
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

  writeInputValue(document, "shape-x", position.x);
  writeInputValue(document, "shape-y", position.y);
}
