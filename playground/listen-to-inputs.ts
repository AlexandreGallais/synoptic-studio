import { listenToAnchor } from "./listen-to-anchor";
import { listenToModes } from "./listen-to-modes";
import { selectElement } from "./select-element";

/** Number inputs of the shapes, in display order. */
const INPUTS = ["corners", "width", "height", "radius"];

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
  const radius = selectElement(document, "radius");

  radius.addEventListener("input", () => {
    delete radius.dataset["radii"];
    radius.setAttribute("placeholder", "");
  });

  for (const id of INPUTS) {
    selectElement(document, id).addEventListener("input", update);
  }

  selectElement(document, "shape").addEventListener("change", update);
  listenToModes(document, update);
  listenToAnchor(document, update);
  document.defaultView?.addEventListener("resize", update);
}
