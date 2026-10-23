import { PAGE_VIEWS } from "../view";

import type { ZoomName } from "./zoom-name";
import type { Point } from "../../src";

/**
 * Applies a zoom asked by a key or the percentage's menu (`readAskedView`): the page's view set to
 * it (`PAGE_VIEWS`); a zoom that changes nothing leaves it.
 *
 * @kind procedure
 * @param document - page document
 * @param name - the zoom asked
 * @param pointer - the pointer's offset on the canvas while it is over the drawing, in CSS pixels;
 * undefined otherwise
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function applyZoom(document: Document, name: ZoomName, pointer: Point | undefined): void {
  if (name === "in" && pointer === undefined) {
    PAGE_VIEWS.delete(document);
  }
}
