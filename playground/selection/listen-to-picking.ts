import { readMode, selectElement } from "../page";
import { readDrawingPoint } from "../placing";
import { readArmedTool } from "../toolbar";

import { PRESSED_TOOLS } from "./pressed-tools";
import { selectShapeAt } from "./select-shape-at";

/**
 * Reads the vertex's handle a click hit, shown in Points mode only.
 *
 * @kind procedure
 * @param event - the click on the drawing
 * @returns the handle; undefined when the click hit none
 * @see RULE-interaction-3-node-editing
 */
function readClickedHandle(event: MouseEvent): Element | undefined {
  return event.target instanceof Element
    ? (event.target.closest(".handle") ?? undefined)
    : undefined;
}

/**
 * Reads what a click on the drawing acts on (F37): in Points mode, a click on a vertex's handle
 * selects the vertex, and a click with Shift adds a vertex or takes it out, the mode editing one
 * shape, so that neither changes the shapes selected; any other click acts on the shapes.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the click on the drawing
 * @returns `points` for a click the points take, `shape` otherwise
 * @see RULE-interaction-3-node-editing
 */
function readClickTarget(document: Document, event: MouseEvent): "points" | "shape" {
  if (readClickedHandle(event) !== undefined) {
    return "points";
  }

  return event.shiftKey ? readMode(document) : "shape";
}

/**
 * Listens to clicks on the drawing with the selector (F36): each press keeps the tool armed then
 * (`PRESSED_TOOLS`); a click whose press was made with the selector picks the shape under it, or
 * none on the empty drawing, Shift adding it or taking it out (`selectShapeAt`, F37), then
 * redraws; the click ending a drag or placing a shape with a tool picks nothing, though the
 * selector is armed again by then, nor does a click the points take (`readClickTarget`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see RULE-interaction-2-selector
 */
export function listenToPicking(document: Document, update: () => void): void {
  const canvas = selectElement(document, "canvas");

  canvas.addEventListener("pointerdown", () => {
    PRESSED_TOOLS.set(document, readArmedTool(document));
  });

  canvas.addEventListener("click", (event) => {
    const pressed = PRESSED_TOOLS.get(document);

    PRESSED_TOOLS.delete(document);

    if (pressed !== "selector" || readClickTarget(document, event) === "points") {
      return;
    }

    selectShapeAt(document, readDrawingPoint(document, event), event.shiftKey);
    update();
  });
}
