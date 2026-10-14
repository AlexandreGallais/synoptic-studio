import { createSvgElement, createTextElement, textBlock, textDrawing } from "../../src";
import { selectElement, writeText } from "../page";

import { createBlockFrameElements } from "./create-block-frame-elements";

import type { FontMetrics, Text } from "../../src";

/** Distance from the canvas top-left corner to the origin of the SVG, as for the shapes. */
const MARGIN = 40;

/** Indentation of the JSON shown in the pipeline panel. */
const JSON_INDENT = 2;

/**
 * Draws a text at a fixed scale (1 user unit = 1 CSS pixel) with its measured block as a frame
 * behind it, and shows each stage in the pipeline column: the model, the block, the positioned
 * lines and the SVG written.
 *
 * @kind procedure
 * @param document - page document
 * @param text - a valid text
 * @param metrics - the metrics of the face it is measured with
 * @see docs/backlog/stories/E15-F05-US-031-place-a-text.md
 */
export function showText(document: Document, text: Text, metrics: FontMetrics): void {
  const { height, width } = selectElement(document, "canvas").getBoundingClientRect();
  const svg = createSvgElement(document, { height, width, x: -MARGIN, y: -MARGIN });
  const block = textBlock(metrics, text);
  const drawn = textDrawing(metrics, text);
  const group = createTextElement(document, text, drawn);

  svg.append(...createBlockFrameElements(document, text, block), group);
  selectElement(document, "canvas").replaceChildren(svg);
  writeText(document, "model", JSON.stringify(text, undefined, JSON_INDENT));
  writeText(document, "contour", JSON.stringify(block, undefined, JSON_INDENT));
  writeText(document, "pieces", JSON.stringify(drawn, undefined, JSON_INDENT));

  const serializer = new XMLSerializer();

  writeText(document, "path-data", serializer.serializeToString(group));
}
