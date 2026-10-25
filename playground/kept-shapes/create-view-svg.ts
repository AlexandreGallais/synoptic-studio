import { createSvgElement, shownViewBox } from "../../src";
import { readCanvasSize, readView } from "../view";

/**
 * Creates an empty `<svg>` of the drawing's view, as large as the canvas, its `viewBox` the
 * view's (`shownViewBox`, ADR-0098): the drawing and the layer above it (F36) both, so that they
 * match at any zoom; the shapes' paths never changed by it.
 *
 * @kind procedure
 * @param document - page document
 * @returns the `<svg>`, empty
 * @see RULE-interaction-7-navigation-and-history
 */
export function createViewSvg(document: Document): SVGSVGElement {
  return createSvgElement(document, shownViewBox(readView(document), readCanvasSize(document)));
}
