import { shapeKind } from "../src";

import { selectElement } from "./select-element";
import { writeValidity } from "./write-validity";

import type { ShapeReading } from "./shape-reading";

/**
 * Shows the fields of the shape read: the number of corners and the anchor grid for a polygon
 * only (F02, F07); no radius field for a round shape, whose radius is always half its smaller
 * side (F10); then whether the shape is valid.
 *
 * @kind procedure
 * @param document - page document
 * @param reading - shape read from the inputs
 * @see docs/backlog/stories/E01-F10-US-020-round-shape-playground.md
 */
export function showShapeFields(document: Document, reading: ShapeReading): void {
  const kind = shapeKind(reading.model);

  selectElement(document, "corners-field").hidden = kind !== "polygon";
  selectElement(document, "anchor-field").hidden = kind !== "polygon";
  selectElement(document, "radius-field").hidden = kind === "round";
  selectElement(document, "point-radius-field").hidden = kind === "round";
  writeValidity(document, reading);
}
