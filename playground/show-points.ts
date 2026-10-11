import { createHandleElements } from "./create-handle-elements";
import { readMode } from "./read-mode";
import { readSelection } from "./read-selection";
import { showPointsList } from "./show-points-list";
import { writeRadiusField } from "./write-radius-field";

import type { ShapeReading } from "./shape-reading";

/**
 * Shows what the mode says about the vertices: in Points mode their handles, the selected ones
 * ringed, their list, and a radius field for the selected vertices; in Shape mode no handle,
 * and a radius field for every vertex (F03.AC4, F03.AC5).
 *
 * @kind procedure
 * @param document - page document
 * @param reading - valid shape
 * @returns the handles to draw over the shape, none in Shape mode
 * @see docs/backlog/stories/E01-F03-US-013-points-list-selection.md
 */
export function showPoints(document: Document, reading: ShapeReading): readonly SVGPathElement[] {
  const contour = reading.corners.map((corner) => corner.point);
  const isPoints = readMode(document) === "points";
  const selection = readSelection(document);
  const shown = isPoints ? selection.selected : contour.map((point, vertex) => vertex);

  writeRadiusField(document, reading.radii, shown);
  showPointsList(document, reading, selection);

  return isPoints ? createHandleElements(document, contour, selection.selected) : [];
}
