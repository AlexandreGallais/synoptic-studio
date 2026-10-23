import { startHelpsFrame } from "../helps-frame";
import { createViewSvg } from "../kept-shapes";
import { selectElement } from "../page";

import { createHoverElements } from "./create-hover-elements";
import { createSelectionElements } from "./create-selection-elements";
import { createSelectionRectangleElements } from "./create-selection-rectangle-elements";
import { readHoverPoint } from "./read-hover-point";
import { readStartedMove } from "./read-started-move";

/**
 * Draws the layer above the drawing (F36): an `<svg>` of the drawing's size and origin
 * (`createViewSvg`), which the pointer passes through, holding the outline of the shape selected
 * (`createSelectionElements`), then, with the selector armed, the outline of the shape under the
 * pointer (`createHoverElements`): each a band 2 screen pixels wide about its contour as drawn, a
 * fill, never a native stroke; no hover with a tool armed, nor once the pointer has left the
 * drawing, nor while shapes are moved (`readStartedMove`, F39), nor while a selection rectangle
 * is drawn, which comes last (`createSelectionRectangleElements`, F37). The outlines are computed
 * within the box of the frame it starts (`startHelpsFrame`, EN-149), the area shown widened by half
 * the canvas on each side, kept while a pan stays within it. The drawing's `<svg>` is never
 * touched, so that the export holds no outline.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_2-selector
 */
export function showHover(document: Document): void {
  startHelpsFrame(document);

  const svg = createViewSvg(document);
  const rectangle = createSelectionRectangleElements(document);
  const isShown = rectangle.length === 0 && readStartedMove(document) === undefined;

  svg.append(
    ...createSelectionElements(document),
    ...createHoverElements(document, readHoverPoint(document, isShown)),
    ...rectangle,
  );

  selectElement(document, "overlay").replaceChildren(svg);
}
