import { createSvgElement, createTextElement, textDrawing } from "../../src";
import { selectElement, writeText } from "../page";

import { createTextFrame } from "./create-text-frame";

import type { FontMetrics, Text } from "../../src";

/** Distance from the canvas top-left corner to the origin of the SVG, as for the shapes. */
const MARGIN = 40;

/** Indentation of the JSON shown in the pipeline panel. */
const JSON_INDENT = 2;

/**
 * Draws a text at a fixed scale (1 user unit = 1 CSS pixel), turned by its angle, with its
 * measured block, or its box when it sits in one, behind it as a dashed frame turned with it, and
 * shows each stage in the pipeline column: the model, the block, the positioned lines and the SVG
 * written.
 *
 * @kind procedure
 * @param document - page document
 * @param text - a valid text
 * @param metrics - the metrics of the face it is measured with
 * @see docs/backlog/stories/E15-F05-US-032-turn-a-text.md
 */
export function showText(document: Document, text: Text, metrics: FontMetrics): void {
  const { height, width } = selectElement(document, "canvas").getBoundingClientRect();
  const svg = createSvgElement(document, { height, width, x: -MARGIN, y: -MARGIN });
  const drawn = textDrawing(metrics, text);
  const group = createTextElement(document, text, drawn);

  const { block } = drawn;

  svg.append(...createTextFrame(document, text, drawn), group);
  selectElement(document, "canvas").replaceChildren(svg);
  writeText(document, "model", JSON.stringify(text, undefined, JSON_INDENT));
  writeText(document, "contour", JSON.stringify(block, undefined, JSON_INDENT));
  writeText(document, "pieces", JSON.stringify(drawn, undefined, JSON_INDENT));

  const serializer = new XMLSerializer();

  writeText(document, "path-data", serializer.serializeToString(group));
}
