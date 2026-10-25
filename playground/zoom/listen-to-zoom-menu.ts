import { fractionFromPercent, halfSize, zoomedView } from "../../src";
import { readNumber, selectElement } from "../page";
import { PAGE_VIEWS, readCanvasSize, readView } from "../view";

import { applyZoom } from "./apply-zoom";

import type { ZoomName } from "./zoom-name";

/** The items of the percentage's menu, by their button's id, and the zoom each asks. */
const ZOOM_ITEMS: ReadonlyMap<string, ZoomName> = new Map([
  ["zoom-actual", "actual"],
  ["zoom-fit-drawing", "drawing"],
  ["zoom-fit-selection", "selection"],
  ["zoom-in", "in"],
  ["zoom-out", "out"],
]);

/**
 * Applies the percentage typed in the menu's field about the canvas's centre, within the bounds
 * (`fractionFromPercent`, `zoomedView`), a fraction kept, the field having taken a value below
 * 1 % at 1 % (`OWN_FIELD_RANGES`); a field showing no number changes nothing, the redraw
 * then showing the zoom in force in the field (`showZoom`).
 *
 * @kind procedure
 * @param document - page document
 */
function applyTypedZoom(document: Document): void {
  const typed = readNumber(document, "zoom-field");

  // A text, or nothing, reads NaN, typed by a script the field never refused.
  if (Number.isNaN(typed)) {
    return;
  }

  const centre = halfSize(readCanvasSize(document));

  PAGE_VIEWS.set(document, zoomedView(readView(document), centre, fractionFromPercent(typed)));
}

/**
 * Listens to the percentage's menu (SP-030's answer 9): Zoom in, Zoom out, Zoom to 100 %, Fit the
 * drawing and Fit the selection each apply their zoom about the canvas's centre, the pointer being
 * on the menu (`applyZoom`), then close the menu; a percentage typed in its field and applied, by
 * Enter or by leaving the field, zooms to it about the canvas's centre, within the bounds, from
 * 1 % (agent's choice, `applyTypedZoom`); then a redraw asked.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 */
export function listenToZoomMenu(document: Document, update: () => void): void {
  for (const [id, name] of ZOOM_ITEMS) {
    selectElement(document, id).addEventListener("click", () => {
      applyZoom(document, name, undefined);
      selectElement(document, "zoom-box").removeAttribute("open");
      update();
    });
  }

  selectElement(document, "zoom-field").addEventListener("change", () => {
    applyTypedZoom(document);
    update();
  });
}
