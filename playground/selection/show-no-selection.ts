import { createDrawingSvg } from "../kept-shapes";
import { selectElement, writeText } from "../page";
import { writeShapeKind } from "../toolbar";

/** What the panel says while no shape is selected. */
const NO_SELECTION = "Click a shape to edit it, or choose a tool and click the drawing to add one.";

/** The pipeline column's stages, empty while no shape is selected. */
const PIPELINE = ["model", "contour", "pieces", "path-data"];

/**
 * Shows the drawing with no shape selected: every shape drawn, in the order drawn, none
 * edited; the panel's fields hidden, its title saying none is selected and how to select one, the
 * pipeline column empty.
 *
 * @kind procedure
 * @param document - page document
 * @see RULE-interaction-2-selector
 */
export function showNoSelection(document: Document): void {
  selectElement(document, "canvas").replaceChildren(createDrawingSvg(document, []));
  selectElement(document, "shape-panel").hidden = true;
  writeShapeKind(document);
  writeText(document, "status", NO_SELECTION);

  for (const id of PIPELINE) {
    writeText(document, id, "");
  }
}
