import { measuredTextDrawing } from "editor";

import { createTextDocument, writeLeftOutFaces } from "../document";
import { DRAWING_KEYS, createDrawingSvg, writeStoredDrawing } from "../kept-shapes";
import { NEW_TEXTS } from "../new-text";
import { selectElement, writeText } from "../page";

import { createTextFrame } from "./create-text-frame";
import { readShownTextBlock } from "./read-shown-text-block";
import { writeTextStraightBox } from "./write-text-straight-box";

import type { FontMetrics, MeasuredText, Text } from "editor";

/** What a text is drawn from: the metrics of its face and what it measures. */
type TextMeasures = {
  /** The metrics of the face it is measured with. */
  readonly metrics: FontMetrics;
  /** Its lines shown and its block (`textMeasure`). */
  readonly measured: MeasuredText;
};

/** Indentation of the JSON shown in the pipeline panel. */
const JSON_INDENT = 2;

/**
 * Draws a text at a fixed scale (1 user unit = 1 CSS pixel), between the shapes below and above
 * it, kept as the shape edited, turned by its angle, with its
 * measured block, or its box when it sits in one, behind it as a dashed frame turned with it, a
 * new text being typed showing, while empty, a frame 1 wide and one line tall
 * (`readShownTextBlock`, `NEW_TEXTS`) — a
 * turned text also with its straight box, upright, and its x, y, width and height — and
 * shows each stage in the pipeline column: the model, the block, the positioned lines and the SVG
 * written. The drawing is written without its embedded fonts, which only Export SVG writes
 * (`createTextDocument`, ADR-0060), the faces a file would leave out listed when Embed
 * fonts is on.
 *
 * @kind procedure
 * @param document - page document
 * @param text - a valid text
 * @param measures - the metrics of its face and its lines and block, drawn from once
 * (`measuredTextDrawing`)
 */
export function showText(document: Document, text: Text, measures: TextMeasures): void {
  const drawn = measuredTextDrawing(measures.metrics, text, measures.measured);
  // Only a new text being typed shows a frame while empty.
  const block = NEW_TEXTS.has(document)
    ? readShownTextBlock(measures.metrics, text, drawn.block)
    : drawn.block;
  const written = createTextDocument(document, { drawn, text });
  const serializer = new XMLSerializer();
  const svg = createDrawingSvg(document, [
    ...createTextFrame(document, text, { ...drawn, block }),
    written.element,
  ]);

  selectElement(document, "canvas").replaceChildren(svg);
  writeStoredDrawing(document, DRAWING_KEYS.edited, [{ kind: "text", shape: text }]);
  writeLeftOutFaces(document, written.leftOut);
  writeTextStraightBox(document, text, drawn.block);
  writeText(document, "model", JSON.stringify(text, undefined, JSON_INDENT));
  writeText(document, "contour", JSON.stringify(drawn.block, undefined, JSON_INDENT));
  writeText(document, "pieces", JSON.stringify(drawn, undefined, JSON_INDENT));

  writeText(document, "path-data", serializer.serializeToString(written.element));
}
