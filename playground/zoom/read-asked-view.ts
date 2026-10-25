import { halfSize, steppedScale, zoomedView } from "../../src";
import { readCanvasSize, readView } from "../view";

import { readFittedView } from "./read-fitted-view";

import type { ZoomName } from "./zoom-name";
import type { Point, View } from "../../src";

/** How a zoom reads its view, from the page and the pointer's offset over the drawing. */
type AskedView = (document: Document, pointer: Point | undefined) => View | undefined;

/**
 * Reads the view a step asks, × √2 or ÷ √2 within the bounds (`steppedScale`), about the pointer
 * when it is over the drawing, else about the canvas's centre (`zoomedView`); at a bound, the view
 * as it is.
 *
 * @kind procedure
 * @param document - page document
 * @param direction - 1 to zoom in, −1 to zoom out
 * @param pointer - the pointer's offset on the canvas while it is over the drawing, in CSS pixels;
 * undefined otherwise
 * @returns the view stepped, its scale within the bounds
 * @see RULE-interaction-7-navigation-and-history
 */
function readSteppedView(document: Document, direction: -1 | 1, pointer: Point | undefined): View {
  const view = readView(document);
  const offset = pointer ?? halfSize(readCanvasSize(document));

  return zoomedView(view, offset, steppedScale(view.scale, direction));
}

/** The view each zoom asks (SP-030's answers 3 to 6). */
const ASKED_VIEWS: Readonly<Record<ZoomName, AskedView>> = {
  actual: (document) => zoomedView(readView(document), halfSize(readCanvasSize(document)), 1),
  drawing: (document) => readFittedView(document, false),
  in: (document, pointer) => readSteppedView(document, 1, pointer),
  out: (document, pointer) => readSteppedView(document, -1, pointer),
  selection: (document) => readFittedView(document, true),
};

/**
 * Reads the view a zoom asks (SP-030's answers 3 to 6): a step in or out, × √2 or ÷ √2 within the
 * bounds, about the pointer when it is over the drawing, else about the canvas's centre
 * (`readSteppedView`); 100 % about the canvas's centre (`zoomedView`); a fit of the drawing
 * or of the selection (`readFittedView`).
 *
 * @kind procedure
 * @param document - page document
 * @param name - the zoom asked
 * @param pointer - the pointer's offset on the canvas while it is over the drawing, in CSS pixels;
 * undefined otherwise
 * @returns the view; undefined for a fit with no shape to fit
 * @see RULE-interaction-7-navigation-and-history
 */
export function readAskedView(
  document: Document,
  name: ZoomName,
  pointer: Point | undefined,
): View | undefined {
  return ASKED_VIEWS[name](document, pointer);
}
