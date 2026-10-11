import { createHandleElements } from "./create-handle-elements";
import { readMode } from "./read-mode";
import { readSelection } from "./read-selection";
import { showPointFields } from "./show-point-fields";
import { showPointsList } from "./show-points-list";
import { writeRadiusField } from "./write-radius-field";

import type { ShapeReading } from "./shape-reading";

/**
 * Shows what the mode says about the vertices: in Points mode their handles, the selected ones
 * ringed, their list, and a radius field for the selected vertices; in both modes, the shape's
 * radius field for every vertex (F03.AC4, F03.AC5, VAL-005).
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

  writeRadiusField(document, "radius", {
    all: reading.radii,
    shown: Object.keys(contour).map(Number),
  });

  writeRadiusField(document, "point-radius", { all: reading.radii, shown: selection.selected });
  showPointsList(document, reading, selection);
  showPointFields(document, reading, selection);

  return isPoints ? createHandleElements(document, contour, selection.selected) : [];
}
