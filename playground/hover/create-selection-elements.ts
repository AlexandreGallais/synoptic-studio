import { pickAreasBox, screenToUser } from "../../src";
import { createDashedLineElements, createOutlineElements, readInsetBoxPathData } from "../helps";
import { readKeptDrawing, readSelectedShapes } from "../kept-shapes";
import { readShapePickArea } from "../picking";
import { readView } from "../view";

import type { PickArea } from "../../src";

/** Width of the dashed box's line, in screen pixels, laid half of it inside the box. */
const LINE = 1;

/**
 * Creates the dashed box around several shapes selected: the smallest upright box
 * holding each one's contour as drawn, or its corners for a shape without area, a shape 0 × 0 its
 * point (`pickAreasBox`); the dashed line of the helps 1 screen pixel wide on its sides moved half
 * a pixel inward (`readInsetBoxPathData`, `createDashedLineElements`), of classes
 * `selection-box-light` and `selection-box`, so that it covers whole pixels, as Inkscape's box
 * alternates two colors (`REF-INKSCAPE-SELCUE`); a text never measured, which is not drawn, holds
 * no corner and is left out.
 *
 * @kind procedure
 * @param document - page document
 * @param areas - what each shape selected offers the pointer, its contour as drawn
 * @returns the white line's `<path>`, then the dashes'
 * @see RULE-interaction-2-selector
 */
function createSelectionBox(
  document: Document,
  areas: readonly PickArea[],
): readonly SVGPathElement[] {
  const inset = screenToUser(LINE / 2, readView(document).scale);

  return createDashedLineElements(document, {
    data: readInsetBoxPathData(pickAreasBox(areas), inset),
    name: "selection-box",
    width: LINE,
  });
}

/**
 * Creates the outlines of the shapes selected, in the layer above the drawing: each
 * the dashed line on its contour as drawn, as the hover's (`createOutlineElements`), of classes
 * `selection-light` and `selection`; with several selected, a dashed box around them all
 * (`createSelectionBox`); shown whichever tool is armed.
 *
 * @kind procedure
 * @param document - page document
 * @returns their `<path>` elements; nothing while no shape is selected, or the one edited is
 * refused
 * @see RULE-interaction-2-selector
 */
export function createSelectionElements(document: Document): readonly SVGPathElement[] {
  const drawing = readKeptDrawing(document);
  const areas = readSelectedShapes(document).flatMap((index) => {
    const kept = drawing[index];

    return kept === undefined ? [] : [readShapePickArea(document, kept.shape)];
  });
  const outlines = areas.flatMap((area) => createOutlineElements(document, area, "selection"));

  return areas.length > 1 ? [...outlines, ...createSelectionBox(document, areas)] : outlines;
}
