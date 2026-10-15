import { createSvgElement, measuredTextDrawing } from "../../src";
import { createTextDocument, writeLeftOutFaces } from "../document";
import { selectElement, writeText } from "../page";

import { createTextFrame } from "./create-text-frame";
import { writeTextStraightBox } from "./write-text-straight-box";

import type { FontMetrics, MeasuredText, Text } from "../../src";

/** What a text is drawn from: the metrics of its face and what it measures. */
type TextMeasures = {
  /** The metrics of the face it is measured with. */
  readonly metrics: FontMetrics;
  /** Its lines shown and its block (`textMeasure`). */
  readonly measured: MeasuredText;
};

/** Distance from the canvas top-left corner to the origin of the SVG, as for the shapes. */
const MARGIN = 40;

/** Indentation of the JSON shown in the pipeline panel. */
const JSON_INDENT = 2;

/**
 * Draws a text at a fixed scale (1 user unit = 1 CSS pixel), turned by its angle, with its
 * measured block, or its box when it sits in one, behind it as a dashed frame turned with it — a
 * turned text also with its straight box, upright, and its x, y, width and height (US-039) — and
 * shows each stage in the pipeline column: the model, the block, the positioned lines and the SVG
 * written. The drawing is written without its embedded fonts, which only Export SVG writes
 * (`createTextDocument`, ADR-0060, US-045), the faces a file would leave out listed when Embed
 * fonts is on.
 *
 * @kind procedure
 * @param document - page document
 * @param text - a valid text
 * @param measures - the metrics of its face and its lines and block, drawn from once
 * (`measuredTextDrawing`, US-048)
 * @see docs/backlog/stories/E15-F05-US-032-turn-a-text.md
 */
export function showText(document: Document, text: Text, measures: TextMeasures): void {
  const { height, width } = selectElement(document, "canvas").getBoundingClientRect();
  const svg = createSvgElement(document, { height, width, x: -MARGIN, y: -MARGIN });
  const drawn = measuredTextDrawing(measures.metrics, text, measures.measured);
  const written = createTextDocument(document, { drawn, text });
  const serializer = new XMLSerializer();

  svg.append(...createTextFrame(document, text, drawn), written.element);
  selectElement(document, "canvas").replaceChildren(svg);
  writeLeftOutFaces(document, written.leftOut);
  writeTextStraightBox(document, text, drawn.block);
  writeText(document, "model", JSON.stringify(text, undefined, JSON_INDENT));
  writeText(document, "contour", JSON.stringify(drawn.block, undefined, JSON_INDENT));
  writeText(document, "pieces", JSON.stringify(drawn, undefined, JSON_INDENT));

  writeText(document, "path-data", serializer.serializeToString(written.element));
}
