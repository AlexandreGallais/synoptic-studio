import { selectElement } from "../page";

/** Panels of a shape's points; the shape's redraw shows them again in Points mode. */
const POINT_PANELS = ["points", "point-fields", "handle-fields"];

/**
 * Hides the panels of a shape's points and handles while the Text tool is chosen: a text has no
 * point to edit.
 *
 * @kind procedure
 * @param document - page document
 */
export function hidePointPanels(document: Document): void {
  for (const id of POINT_PANELS) {
    selectElement(document, id).hidden = true;
  }
}
