import { createPathElement, pickAreasBox, screenToUser } from "../../src";
import { createStraightBoxElement } from "../drawing";
import { createOutlineElements, readBoxBandPathData } from "../helps-frame";
import { readKeptDrawing, readSelectedShapes } from "../kept-shapes";
import { readShapePickArea } from "../picking";
import { readView } from "../view";

import type { PickArea } from "../../src";

/** Thickness of the dashes and of the light frame under them, in screen pixels. */
const LINE = 1;

/**
 * Creates the dashed box around several shapes selected (F37, F43): the smallest upright box
 * holding each one's contour as drawn, or its corners for a shape without area, a shape 0 × 0 its
 * point (`pickAreasBox`); first a light frame 1 screen pixel thick inside its sides
 * (`readBoxBandPathData`), of class `selection-box-light`, then the dashes over it, as a turned
 * shape's straight box (`createStraightBoxElement`), of class `selection-box`, so that the gaps
 * between the dashes show light, as Inkscape's box alternates two colors (`REF-INKSCAPE-SELCUE`);
 * a text never measured, which is not drawn, holds no corner and is left out.
 *
 * @kind procedure
 * @param document - page document
 * @param areas - what each shape selected offers the pointer, its contour as drawn
 * @returns the light frame's `<path>`, then the dashes'
 * @see docs/domain/interaction.md#_2-selector
 */
function createSelectionBox(
  document: Document,
  areas: readonly PickArea[],
): readonly SVGPathElement[] {
  const box = pickAreasBox(areas);
  const inward = screenToUser(LINE, readView(document).scale);
  const light = createPathElement(document, readBoxBandPathData(box, { inward, outward: 0 }));
  const dashes = createStraightBoxElement(document, box);

  light.classList.add("selection-box-light");
  dashes.classList.add("selection-box");

  return [light, dashes];
}

/**
 * Creates the outlines of the shapes selected (F36, F37), in the layer above the drawing: each
 * the two bands about its contour as drawn, as the hover's (`createOutlineElements`), of classes
 * `selection-light` and `selection`, a fill, never a native stroke; with several selected, a
 * dashed box around them all (`createSelectionBox`); shown whichever tool is armed.
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
  const outlines = areas.flatMap((area) => createOutlineElements(document, area, "selection"));

  return areas.length > 1 ? [...outlines, ...createSelectionBox(document, areas)] : outlines;
}
