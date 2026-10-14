import { showRefusedShape, showShape } from "../drawing";
import { selectElement, writeValidity } from "../page";
import { readShape, writeCappedSizes } from "../shape";
import { writeShapeKind } from "../toolbar";

/**
 * Updates the page from the inputs: the selected shape is drawn when valid, refused otherwise;
 * the number of corners, the « Stretched » check box and the anchor are shown for the polygon
 * only, the anchor disabled while the polygon is stretched.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E01-F02-US-007-shape-selector-playground.md
 */
export function updatePlayground(document: Document): void {
  writeCappedSizes(document);

  const reading = readShape(document);
  const isPolygon = "anchor" in reading.model;
  const isStretched = "stretched" in reading.model && reading.model.stretched;

  writeShapeKind(document);
  selectElement(document, "corners-field").hidden = !isPolygon;
  selectElement(document, "stretched-field").hidden = !isPolygon;
  selectElement(document, "anchor-field").hidden = !isPolygon;
  // A round shape has no radius (F10).
  selectElement(document, "corners-section").hidden = "kind" in reading.model;
  // A stretched polygon fills its box: the anchor has no effect (ADR-0037).
  selectElement(document, "anchor-field").toggleAttribute("disabled", isStretched);
  writeValidity(document, reading);

  if (reading.isValid) {
    showShape(document, reading);
  } else {
    showRefusedShape(document);
  }
}
