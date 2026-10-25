import {
  contourPiecesToPathData,
  createPathElement,
  createTextElement,
  curveContour,
  isDrawnText,
  measuredTextDrawing,
  shapeCorners,
} from "../../src";
import { readTextMeasure, readTextMetrics } from "../text-measure";

import { KEPT_TEXT_METRICS } from "./kept-text-metrics";
import { readStoredDrawing } from "./read-stored-drawing";

import type { Shape, Text } from "../../src";

/**
 * Creates the elements of a text kept in the drawing, measured with its font's metrics; when they
 * are no longer read, a family read ahead whose files could not be read again leaving the session
 * (US-061) while Inter is not drawn (US-056), with the metrics it was last drawn with
 * (`KEPT_TEXT_METRICS`), so that it stays in the drawing and the export as it was (F35).
 *
 * @kind procedure
 * @param document - page document
 * @param text - a text kept, valid when it was drawn
 * @returns its group; nothing for a text never drawn with metrics
 * @see docs/domain/interaction.md#_1-tools
 */
function createKeptTextElements(document: Document, text: Text): readonly SVGGElement[] {
  const key = JSON.stringify(text);
  const metrics = readTextMetrics(document, text) ?? KEPT_TEXT_METRICS.get(key);

  if (metrics === undefined) {
    return [];
  }

  KEPT_TEXT_METRICS.set(key, metrics);

  const drawn = measuredTextDrawing(metrics, text, readTextMeasure(metrics, text));

  return [createTextElement(document, text, drawn)];
}

/**
 * Creates the path of a shape kept in the drawing, from its corners as drawn (`shapeCorners`).
 *
 * @kind procedure
 * @param document - page document
 * @param shape - a shape kept, valid when it was drawn
 * @returns its path
 * @see US-063
 */
function createKeptShapeElement(document: Document, shape: Shape): SVGPathElement {
  const pieces = curveContour(shapeCorners(shape));

  return createPathElement(document, contourPiecesToPathData(pieces));
}

/**
 * Creates the elements of the shapes kept below or above the one the panel edits, in the order
 * drawn, so that each is drawn above those before it (F35, F36).
 *
 * @kind procedure
 * @param document - page document
 * @param key - `DRAWING_KEYS.below` or `DRAWING_KEYS.above`
 * @returns their elements, the first drawn first
 * @see docs/domain/interaction.md#_2-selector
 */
export function createDrawingElements(document: Document, key: string): readonly SVGElement[] {
  return readStoredDrawing(document, key).flatMap(({ shape }) =>
    isDrawnText(shape)
      ? createKeptTextElements(document, shape)
      : [createKeptShapeElement(document, shape)],
  );
}
