import {
  commonRadius,
  contourPiecesToPathData,
  createPathElement,
  curveContour,
  drawnRadii,
  effectiveCornerRadius,
  formatSvgNumber,
  isReducedRadius,
  shapeStraightBox,
} from "../../src";
import { DRAWING_KEYS, createDrawingSvg, writeStoredDrawing } from "../kept-shapes";
import { readShapeKind, selectElement, writeText } from "../page";
import { showPoints } from "../points";

import { createBoxElements } from "./create-box-elements";
import { writeShapeBoxes } from "./write-shape-boxes";

import type { ShapeReading } from "../shape";

/** Indentation of the JSON shown in the pipeline panel. */
const JSON_INDENT = 2;

/** Text shown instead of one effective radius when the vertices have different radii. */
const MIXED_EFFECTIVE = "Effective radius: mixed, shown for each point in Points mode.";

/**
 * Shows the one effective radius of a shape, saying whether it was reduced to fit (Q8).
 *
 * @kind procedure
 * @param document - page document
 * @param radius - requested radius of every corner
 * @param effective - radius drawn at every corner
 * @see docs/backlog/stories/E01-F01-US-003-round-rectangle-corners.md
 */
function writeOneEffectiveRadius(document: Document, radius: number, effective: number): void {
  const text = isReducedRadius(radius, effective)
    ? `Effective radius: ${formatSvgNumber(effective)} (requested ${String(radius)}, reduced to fit)`
    : `Effective radius: ${formatSvgNumber(effective)} (as requested)`;

  writeText(document, "effective", text);
}

/**
 * Shows the effective corner radius next to the requested one, and says when it was reduced to
 * fit the shape (Q8: the requested value is kept, the effective one is derived); when the
 * vertices request or draw different radii, says where each one is shown (F03.AC2, F06.AC5).
 *
 * @kind procedure
 * @param document - page document
 * @param reading - valid shape
 * @see docs/backlog/stories/E01-F06-US-021-stretched-option.md
 */
function writeEffectiveRadius(document: Document, reading: ShapeReading): void {
  const radius = commonRadius(reading.radii);
  const effective = effectiveCornerRadius(drawnRadii(reading.corners));

  if (radius === undefined || effective === undefined) {
    writeText(document, "effective", MIXED_EFFECTIVE);

    return;
  }

  writeOneEffectiveRadius(document, radius, effective);
}

/**
 * Draws a shape with its rounded corners at a fixed scale (1 user unit = 1 CSS pixel, US-004),
 * between the shapes below and above it, and keeps it as the shape edited (F35), with the kind the
 * panel shows it as (F36); shows each pipeline stage: model, contour, evaluated contour, path data.
 *
 * @kind procedure
 * @param document - page document
 * @param reading - valid shape to draw
 * @see docs/backlog/stories/E01-F01-US-003-round-rectangle-corners.md
 */
export function showShape(document: Document, reading: ShapeReading): void {
  const contour = reading.corners.map((corner) => corner.point);
  const pieces = curveContour(reading.corners);
  const pathData = contourPiecesToPathData(pieces);
  const box = shapeStraightBox(reading.corners);
  const handles = showPoints(document, reading);
  const svg = createDrawingSvg(document, [
    ...createBoxElements(document, reading.model, box),
    createPathElement(document, pathData),
    ...handles,
  ]);

  selectElement(document, "canvas").replaceChildren(svg);

  writeStoredDrawing(document, DRAWING_KEYS.edited, [
    { kind: readShapeKind(document), shape: reading.model },
  ]);

  writeText(document, "model", JSON.stringify(reading.model, undefined, JSON_INDENT));
  writeText(document, "contour", JSON.stringify(contour, undefined, JSON_INDENT));
  writeText(document, "pieces", JSON.stringify(pieces, undefined, JSON_INDENT));
  writeEffectiveRadius(document, reading);
  writeText(document, "path-data", pathData);
  writeShapeBoxes(document, box, reading.model);
}
