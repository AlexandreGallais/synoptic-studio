import { createSvgElement, textDrawing } from "../../src";
import { createTextDocument, writeLeftOutFaces } from "../document";
import { selectElement, writeText } from "../page";

import { createTextFrame } from "./create-text-frame";
import { writeTextStraightBox } from "./write-text-straight-box";

import type { FontMetrics, Text } from "../../src";

/** Distance from the canvas top-left corner to the origin of the SVG, as for the shapes. */
const MARGIN = 40;

/** Indentation of the JSON shown in the pipeline panel. */
const JSON_INDENT = 2;

/**
 * Draws a text at a fixed scale (1 user unit = 1 CSS pixel), turned by its angle, with its
 * measured block, or its box when it sits in one, behind it as a dashed frame turned with it — a
 * turned text also with its straight box, upright, and its x, y, width and height (US-039) — and
 * shows each stage in the pipeline column: the model, the block, the positioned lines and the SVG
 * written. The drawing is written with its options (`createTextDocument`, US-036): its embedded
 * fonts in `<defs>` before the text, then the text, and the faces left out listed; the option off,
 * exactly as before.
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
  const written = createTextDocument(document, { drawn, text });
  const serializer = new XMLSerializer();

  svg.append(...written.defs, ...createTextFrame(document, text, drawn), written.element);
  selectElement(document, "canvas").replaceChildren(svg);
  writeLeftOutFaces(document, written.leftOut);
  writeTextStraightBox(document, text, drawn.block);
  writeText(document, "model", JSON.stringify(text, undefined, JSON_INDENT));
  writeText(document, "contour", JSON.stringify(drawn.block, undefined, JSON_INDENT));
  writeText(document, "pieces", JSON.stringify(drawn, undefined, JSON_INDENT));

  writeText(
    document,
    "path-data",
    [...written.defs, written.element].map((node) => serializer.serializeToString(node)).join("\n"),
  );
}
