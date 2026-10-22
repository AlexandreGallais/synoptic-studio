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

import { DRAWING_KEYS } from "./drawing-keys";
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
 * @see docs/backlog/stories/E02-F35-US-063-a-click-adds-a-shape.md
 */
function createKeptShapeElement(document: Document, shape: Shape): SVGPathElement {
  const pieces = curveContour(shapeCorners(shape));

  return createPathElement(document, contourPiecesToPathData(pieces));
}

/**
 * Creates the elements of the shapes placed below the one the panel edits, in the order drawn, so
 * that each is drawn above those before it and below the one edited (F35).
 *
 * @kind procedure
 * @param document - page document
 * @returns their elements, the first drawn first
 * @see docs/backlog/stories/E02-F35-US-063-a-click-adds-a-shape.md
 */
export function createDrawingElements(document: Document): readonly SVGElement[] {
  return readStoredDrawing(document, DRAWING_KEYS.below).flatMap((shape) =>
    isDrawnText(shape)
      ? createKeptTextElements(document, shape)
      : [createKeptShapeElement(document, shape)],
  );
}
