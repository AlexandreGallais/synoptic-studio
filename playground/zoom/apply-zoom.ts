import { PAGE_VIEWS } from "../view";

import { readAskedView } from "./read-asked-view";

import type { ZoomName } from "./zoom-name";
import type { Point } from "../../src";

/**
 * Applies a zoom asked by a key or the percentage's menu (`readAskedView`): the page's view set to
 * it (`PAGE_VIEWS`); a fit with no shape to fit leaves it.
 *
 * @kind procedure
 * @param document - page document
 * @param name - the zoom asked
 * @param pointer - the pointer's offset on the canvas while it is over the drawing, in CSS pixels;
 * undefined otherwise
 */
export function applyZoom(document: Document, name: ZoomName, pointer: Point | undefined): void {
  const view = readAskedView(document, name, pointer);

  if (view !== undefined) {
    PAGE_VIEWS.set(document, view);
  }
}
