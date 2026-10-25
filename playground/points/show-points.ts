import { isRoundShape } from "../../src";
import { createCurveHandleElements, showHandleFields } from "../handles";
import { readMode, readSelection, selectElement } from "../page";

import { createHandleElements } from "./create-handle-elements";
import { showPointFields } from "./show-point-fields";
import { showPointsList } from "./show-points-list";
import { writeRadiusField } from "./write-radius-field";

import type { VertexSelection } from "../../src";
import type { ShapeReading } from "../shape";

/**
 * Shows the radius fields: the shape's for every vertex, the point's for the selected ones. A
 * round shape has no radius: its fields are left as they are, the radii kept for the next
 * shape, and the point's field hidden.
 *
 * @kind procedure
 * @param document - page document
 * @param reading - valid shape
 * @param selection - vertices selected
 */
function writeRadiusFields(
  document: Document,
  reading: ShapeReading,
  selection: VertexSelection,
): void {
  if (isRoundShape(reading.model)) {
    selectElement(document, "point-radius-field").hidden = true;

    return;
  }

  writeRadiusField(document, "radius", {
    all: reading.radii,
    shown: Object.keys(reading.corners).map(Number),
  });

  writeRadiusField(document, "point-radius", { all: reading.radii, shown: selection.selected });
}

/**
 * Shows what the mode says about the vertices: in Points mode their handles, the selected ones
 * ringed, their list, and a radius field for the selected vertices; in both modes, the shape's
 * radius field for every vertex (F03.AC4, F03.AC5).
 *
 * @kind procedure
 * @param document - page document
 * @param reading - valid shape
 * @returns the handles to draw over the shape, none in Shape mode
 */
export function showPoints(document: Document, reading: ShapeReading): readonly SVGPathElement[] {
  const contour = reading.corners.map((corner) => corner.point);
  const isPoints = readMode(document) === "points";
  const selection = readSelection(document);

  showPointsList(document, reading, selection);
  showPointFields(document, reading, selection);
  showHandleFields(document, reading);
  writeRadiusFields(document, reading, selection);

  return isPoints
    ? [
        ...createCurveHandleElements(document, reading.corners),
        ...createHandleElements(document, contour, selection.selected),
      ]
    : [];
}
