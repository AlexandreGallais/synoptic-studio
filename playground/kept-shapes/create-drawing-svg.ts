import { createDrawingElements } from "./create-drawing-elements";
import { createViewSvg } from "./create-view-svg";
import { DRAWING_KEYS } from "./drawing-keys";

/**
 * Creates the drawing's `<svg>` at a fixed scale, 1 user unit to 1 CSS pixel (US-004), as large as
 * the canvas, its origin a margin from the top-left (`createViewSvg`), holding the shapes in the
 * order drawn: those below the one the panel edits, that one's elements, then those above it, so
 * that a shape picked from the middle keeps its place (F35, F36); the view stays put, so that a
 * turned shape turns about its center on screen and may leave the canvas (Product Owner, VAL-007).
 *
 * @kind procedure
 * @param document - page document
 * @param edited - the elements of the shape the panel edits; none while it is refused or none is
 * selected
 * @returns the `<svg>`, every shape of the drawing in it
 * @see RULE-interaction-2-selector
 */
export function createDrawingSvg(document: Document, edited: readonly SVGElement[]): SVGSVGElement {
  const svg = createViewSvg(document);

  svg.append(
    ...createDrawingElements(document, DRAWING_KEYS.below),
    ...edited,
    ...createDrawingElements(document, DRAWING_KEYS.above),
  );

  return svg;
}
