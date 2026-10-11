import {
  EPSILON,
  contourPiecesToPathData,
  createPathElement,
  createSvgElement,
  effectiveCornerRadius,
  formatSvgNumber,
  roundedContour,
} from "../src";

import { createBoxElements } from "./create-box-elements";
import { readNumber } from "./read-number";
import { selectElement } from "./select-element";
import { writeText } from "./write-text";

import type { ShapeReading } from "./shape-reading";

/** Distance from the canvas top-left corner to the shape origin, in user units (= CSS pixels). */
const MARGIN = 10;

/** Indentation of the JSON shown in the pipeline panel. */
const JSON_INDENT = 2;

/**
 * Reads the size of the drawing area in CSS pixels.
 *
 * @kind procedure
 * @param document - page document
 * @returns width and height of the `#canvas` element
 * @see docs/backlog/stories/E01-F01-US-004-fixed-scale-playground.md
 */
function readCanvasSize(document: Document): { readonly height: number; readonly width: number } {
  const { height, width } = selectElement(document, "canvas").getBoundingClientRect();

  return { height, width };
}

/**
 * Shows the effective corner radius next to the requested one, and says when it was reduced to
 * fit the shape (Q8: the requested value is kept, the effective one is derived).
 *
 * @kind procedure
 * @param document - page document
 * @param reading - valid shape
 * @see docs/backlog/stories/E01-F01-US-003-round-rectangle-corners.md
 */
function writeEffectiveRadius(document: Document, reading: ShapeReading): void {
  const effective = effectiveCornerRadius(reading.corners);
  const radius = readNumber(document, "radius");
  const isReduced = radius - effective >= EPSILON;
  const text = isReduced
    ? `Effective radius: ${formatSvgNumber(effective)} (requested ${String(radius)}, reduced to fit)`
    : `Effective radius: ${formatSvgNumber(effective)} (as requested)`;

  writeText(document, "effective", text);
}

/**
 * Draws a shape with its rounded corners at a fixed scale (1 user unit = 1 CSS pixel, US-004)
 * and shows each pipeline stage: model, contour, evaluated contour, path data.
 *
 * @kind procedure
 * @param document - page document
 * @param reading - valid shape to draw
 * @see docs/backlog/stories/E01-F01-US-003-round-rectangle-corners.md
 */
export function showShape(document: Document, reading: ShapeReading): void {
  const contour = reading.corners.map((corner) => corner.point);
  const pieces = roundedContour(reading.corners);
  const pathData = contourPiecesToPathData(pieces);
  const viewBox = { ...readCanvasSize(document), x: -MARGIN, y: -MARGIN };
  const svg = createSvgElement(document, viewBox);

  svg.append(...createBoxElements(document, reading.model), createPathElement(document, pathData));
  selectElement(document, "canvas").replaceChildren(svg);
  writeText(document, "model", JSON.stringify(reading.model, undefined, JSON_INDENT));
  writeText(document, "contour", JSON.stringify(contour, undefined, JSON_INDENT));
  writeText(document, "pieces", JSON.stringify(pieces, undefined, JSON_INDENT));
  writeEffectiveRadius(document, reading);
  writeText(document, "path-data", pathData);
}
