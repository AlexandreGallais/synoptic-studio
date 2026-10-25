import { NO_VERTEX_SELECTION } from "../../src";
import { listenToEditedFields } from "../edited-fields";
import { listenToNumberFields } from "../number-fields";
import { selectElement, writeSelection } from "../page";
import { listenToPoints } from "../points";
import { listenToTurn } from "../position";
import { listenToSeveralFields } from "../selection";
import { listenToAnchor, listenToRadius, writeChosenFreeShape } from "../shape";
import { listenToTextFields } from "../text";
import { listenToModes } from "../toolbar";

/** The fields of the shape's numbers the page reads when it redraws, in display order. */
const INPUTS = ["corners", "radius"];

/**
 * Listens to the shape chosen in the selector: the selection no longer applies, and a free
 * shape kept comes back with its own size (F03.AC5, VAL-005); then calls back.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see VAL-005
 */
function listenToShapeChoice(document: Document, update: () => void): void {
  selectElement(document, "shape").addEventListener("change", () => {
    writeSelection(document, NO_VERTEX_SELECTION);
    writeChosenFreeShape(document);
    update();
  });
}

/**
 * Calls back when a control of the panel other than a number of the shape changes: a mode, the
 * anchor, the points, the angle, or a value typed for several shapes, which sets each of them
 * (`listenToSeveralFields`, F38).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/domain/interaction.md#_2-selector
 */
function listenToPanelControls(document: Document, update: () => void): void {
  listenToModes(document, update);
  listenToAnchor(document, update);
  listenToPoints(document, update);
  listenToTurn(document, update);
  listenToSeveralFields(document, update);
}

/**
 * Calls back when a value of the shape changes: a number typed, another shape, anchor or mode
 * chosen, a field of the text typed (`listenToTextFields`); the window resized changes the
 * canvas's size, which `listenToCanvasSize` hears (US-084). A radius typed
 * goes to every vertex: the different radii kept on the radius input are dropped first
 * (F03.AC2). X, Y, Width and Height act as number fields, a value applied on Enter or when the
 * field is left and the arrows stepping it (`listenToEditedFields`, US-085), as the fields of
 * several shapes, whose value sets each of them (`listenToSeveralFields`, F38); every other number
 * field likewise, its value its own, read as the page redraws, one mechanism for them all
 * (`listenToNumberFields`, US-087, AUD-046).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see US-011
 */
export function listenToInputs(document: Document, update: () => void): void {
  listenToNumberFields(document);
  listenToRadius(document, update);

  // Another number of vertices: the selection no longer applies (F03.AC5).
  for (const id of ["corners", "shape"]) {
    selectElement(document, id).addEventListener("change", () => {
      writeSelection(document, NO_VERTEX_SELECTION);
    });
  }

  // X, Y, Width and Height act as number fields, as several shapes' (US-085).
  listenToEditedFields(document, update);

  for (const id of INPUTS) {
    selectElement(document, id).addEventListener("change", update);
  }

  listenToShapeChoice(document, update);

  listenToPanelControls(document, update);

  listenToTextFields(document, update);
}
