import { selectElement } from "../page";

import type { Size } from "../../src";

/**
 * Reads the size of the canvas, the area the drawing is shown in, as the browser lays it out.
 *
 * @kind procedure
 * @param document - page document
 * @returns its width and height, in CSS pixels
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function readCanvasSize(document: Document): Size {
  const { height, width } = selectElement(document, "canvas").getBoundingClientRect();

  return { height, width };
}
