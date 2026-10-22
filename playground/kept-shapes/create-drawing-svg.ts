import { createDrawingElements } from "./create-drawing-elements";
import { createViewSvg } from "./create-view-svg";

/**
 * Creates the drawing's `<svg>` at a fixed scale, 1 user unit to 1 CSS pixel (US-004), as large as
 * the canvas, its origin a margin from the top-left (`createViewSvg`), holding the shapes placed
 * below the one the panel edits (F35); the view stays put, so that a turned shape turns about its
 * center on screen and may leave the canvas (Product Owner, VAL-007).
 *
 * @kind procedure
 * @param document - page document
 * @returns the `<svg>`, the shape edited still to add
 * @see docs/backlog/stories/E02-F35-US-063-a-click-adds-a-shape.md
 */
export function createDrawingSvg(document: Document): SVGSVGElement {
  const svg = createViewSvg(document);

  svg.append(...createDrawingElements(document));

  return svg;
}
