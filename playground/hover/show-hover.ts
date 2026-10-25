import { createViewSvg } from "../kept-shapes";
import { selectElement } from "../page";

import { createHoverElements } from "./create-hover-elements";
import { createSelectionElements } from "./create-selection-elements";
import { createSelectionRectangleElements } from "./create-selection-rectangle-elements";
import { createTakenElements } from "./create-taken-elements";
import { readHoverPoint } from "./read-hover-point";
import { readStartedMove } from "./read-started-move";

/**
 * Draws the layer above the drawing (F36): an `<svg>` of the drawing's size and origin
 * (`createViewSvg`), which the pointer passes through, holding the outline of the shape selected
 * (`createSelectionElements`), then, with the selector armed, the outline of the shape under the
 * pointer (`createHoverElements`): each one line of white and black dashes 2 screen pixels wide on
 * its contour as drawn, a native stroke allowed in this layer alone (US-086, ADR-0099); no hover
 * with a tool armed, nor
 * once the pointer has left the drawing, nor while shapes are moved (`readStartedMove`, F39), nor
 * while a selection rectangle is drawn, which outlines instead the shapes it would take
 * (`createTakenElements`, F43) and comes last (`createSelectionRectangleElements`, F37). The
 * drawing's `<svg>` is never touched, so that the export holds no outline.
 *
 * @kind procedure
 * @param document - page document
 * @see RULE-interaction-2-selector
 */
export function showHover(document: Document): void {
  const svg = createViewSvg(document);
  const rectangle = createSelectionRectangleElements(document);
  const isShown = rectangle.length === 0 && readStartedMove(document) === undefined;

  svg.append(
    ...createSelectionElements(document),
    ...createHoverElements(document, readHoverPoint(document, isShown)),
    ...createTakenElements(document),
    ...rectangle,
  );

  selectElement(document, "overlay").replaceChildren(svg);
}
