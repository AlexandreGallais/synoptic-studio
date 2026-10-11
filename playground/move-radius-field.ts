import { selectElement } from "./select-element";

/**
 * Moves the radius field where it speaks: with the shape's fields in Shape mode, where it rounds
 * every corner; with the point fields in Points mode, where it rounds the points selected
 * (Product Owner, VAL-005).
 *
 * @kind procedure
 * @param document - page document
 * @param isPoints - whether Points mode is on and the shape is valid
 * @see docs/backlog/stories/E01-F08-VAL-005-validate-node-editing.md
 */
export function moveRadiusField(document: Document, isPoints: boolean): void {
  const field = selectElement(document, "radius-field");

  if (isPoints) {
    selectElement(document, "point-x").closest("label")?.before(field);

    return;
  }

  selectElement(document, "height").closest("label")?.after(field);
}
