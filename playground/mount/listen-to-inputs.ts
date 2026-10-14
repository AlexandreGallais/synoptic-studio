import { NO_VERTEX_SELECTION } from "../../src";
import { selectElement, writeSelection } from "../page";
import { listenToPoints } from "../points";
import { listenToPosition, listenToTurn } from "../position";
import { listenToAnchor, listenToRadius, writeChosenFreeShape } from "../shape";
import { listenToModes } from "../toolbar";

/** Number inputs of the shapes, in display order. */
const INPUTS = ["corners", "width", "height", "radius"];

/**
 * Listens to the shape chosen in the selector: the selection no longer applies, and a free
 * shape kept comes back with its own size (F03.AC5, VAL-005); then calls back.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E01-F08-VAL-005-validate-node-editing.md
 */
function listenToShapeChoice(document: Document, update: () => void): void {
  selectElement(document, "shape").addEventListener("change", () => {
    writeSelection(document, NO_VERTEX_SELECTION);
    writeChosenFreeShape(document);
    update();
  });
}

/**
 * Calls back when a value of the shape changes: a number typed, another shape, anchor or mode
 * chosen, the window resized. A radius typed goes to every vertex: the different radii kept on the
 * radius input are dropped first (F03.AC2).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E01-F03-US-011-radius-field-mixed.md
 */
export function listenToInputs(document: Document, update: () => void): void {
  listenToRadius(document, update);

  // Another number of vertices: the selection no longer applies (F03.AC5).
  for (const id of ["corners", "shape"]) {
    selectElement(document, id).addEventListener("input", () => {
      writeSelection(document, NO_VERTEX_SELECTION);
    });
  }

  // First: a width typed places a turned shape before the page redraws (US-019).
  listenToPosition(document, update);

  for (const id of INPUTS) {
    selectElement(document, id).addEventListener("input", update);
  }

  listenToShapeChoice(document, update);

  listenToModes(document, update);
  listenToAnchor(document, update);
  listenToPoints(document, update);
  listenToTurn(document, update);
  document.defaultView?.addEventListener("resize", update);
}
