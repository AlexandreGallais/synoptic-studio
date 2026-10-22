import { createSvgElement } from "../../src";
import { selectElement } from "../page";

import { DRAWING_MARGIN } from "./drawing-margin";

/**
 * Creates an empty `<svg>` of the drawing's view: at a fixed scale, 1 user unit to 1 CSS pixel
 * (US-004), as large as the canvas, its origin a margin from the top-left; the drawing and the
 * layer above it (F36) both, so that they match.
 *
 * @kind procedure
 * @param document - page document
 * @returns the `<svg>`, empty
 * @see docs/domain/interaction.md#_2-selector
 */
export function createViewSvg(document: Document): SVGSVGElement {
  const { height, width } = selectElement(document, "canvas").getBoundingClientRect();

  return createSvgElement(document, { height, width, x: -DRAWING_MARGIN, y: -DRAWING_MARGIN });
}
