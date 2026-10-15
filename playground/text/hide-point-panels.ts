import { selectElement } from "../page";

/** Panels of a shape's points; the shape's redraw shows them again in Points mode. */
const POINT_PANELS = ["points", "point-fields", "handle-fields"];

/**
 * Hides the panels of a shape's points and handles while the Text tool is chosen: a text has no
 * point to edit.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F05-US-031-place-a-text.md
 */
export function hidePointPanels(document: Document): void {
  for (const id of POINT_PANELS) {
    selectElement(document, id).hidden = true;
  }
}
