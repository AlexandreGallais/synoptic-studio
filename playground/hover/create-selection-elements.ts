import { boundingBox, pickAreaBox } from "../../src";
import { createStraightBoxElement } from "../drawing";
import { readKeptDrawing, readSelectedShapes } from "../kept-shapes";

import { createOutlineElement } from "./create-outline-element";
import { readShapePickArea } from "./read-shape-pick-area";

import type { PickArea } from "../../src";

/**
 * Creates the dashed box around several shapes selected (F37): the smallest upright box holding
 * each one's contour as drawn, or its corners for a shape without area, a shape 0 × 0 its point
 * (`pickAreaBox`, `boundingBox`), dashed as a turned shape's straight box
 * (`createStraightBoxElement`), of class `selection-box`; a text never measured, which is not
 * drawn, holds no corner and is left out.
 *
 * @kind procedure
 * @param document - page document
 * @param areas - what each shape selected offers the pointer, its contour as drawn
 * @returns its `<path>`
 * @see docs/domain/interaction.md#_2-selector
 */
function createSelectionBox(document: Document, areas: readonly PickArea[]): SVGPathElement {
  const corners = areas
    .filter(({ corners: points }) => points.length > 0)
    .map((area) => pickAreaBox(area))
    .flatMap((box) => [
      { x: box.minX, y: box.minY },
      { x: box.maxX, y: box.maxY },
    ]);
  const element = createStraightBoxElement(document, boundingBox(corners));

  element.classList.add("selection-box");

  return element;
}

/**
 * Creates the outlines of the shapes selected (F36, F37), in the layer above the drawing: each the
 * band about its contour as drawn, as the hover's (`createOutlineElement`), of class `selection`, a
 * fill, never a native stroke; with several selected, a dashed box around them all
 * (`createSelectionBox`); shown whichever tool is armed.
 *
 * @kind procedure
 * @param document - page document
 * @returns their `<path>` elements; nothing while no shape is selected, or the one edited is
 * refused
 * @see docs/domain/interaction.md#_2-selector
 */
export function createSelectionElements(document: Document): readonly SVGPathElement[] {
  const drawing = readKeptDrawing(document);
  const areas = readSelectedShapes(document).flatMap((index) => {
    const kept = drawing[index];

    return kept === undefined ? [] : [readShapePickArea(document, kept.shape)];
  });
  const outlines = areas.map((area) => createOutlineElement(document, area, "selection"));

  return areas.length > 1 ? [...outlines, createSelectionBox(document, areas)] : outlines;
}
