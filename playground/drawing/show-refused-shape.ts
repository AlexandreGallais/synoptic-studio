import { DRAWING_KEYS, writeStoredDrawing } from "../kept-shapes";
import { selectElement, writeText } from "../page";

/**
 * Shows a refused shape: the effective radius, the list of points and the handles of the last
 * valid shape no longer match the inputs, so they go (Q8, F03.AC4); the radius field stays
 * usable, to correct the value refused.
 *
 * @kind procedure
 * @param document - page document
 */
export function showRefusedShape(document: Document): void {
  const list = selectElement(document, "points");
  const field = selectElement(document, "radius");

  writeText(document, "effective", "");
  list.hidden = true;
  list.replaceChildren();

  for (const mark of document.querySelectorAll("#canvas path.handle, #canvas path.halo")) {
    mark.remove();
  }

  // A refused shape is not kept when another is placed.
  writeStoredDrawing(document, DRAWING_KEYS.edited, []);

  // Nothing to convert nor to move on a refused shape.
  selectElement(document, "point-fields").hidden = true;

  if (field instanceof HTMLInputElement) {
    field.disabled = false;
  }
}
