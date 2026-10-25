import { fittedView, pickAreasBox } from "../../src";
import { readSelectedShapes } from "../kept-shapes";
import { readPickAreas } from "../picking";
import { readCanvasSize, readView } from "../view";

import { FIT_PADDING } from "./fit-padding";

import type { PickArea, View } from "../../src";

/**
 * Reads what the shapes drawn, or the shapes selected, offer the pointer (`readPickAreas`): their
 * contours as drawn.
 *
 * @kind procedure
 * @param document - page document
 * @param isSelection - whether the shapes selected are read, rather than every shape
 * @returns their areas, in the order drawn
 */
function readFittedAreas(document: Document, isSelection: boolean): readonly PickArea[] {
  const areas = readPickAreas(document);

  return isSelection ? readSelectedShapes(document).flatMap((index) => areas[index] ?? []) : areas;
}

/**
 * Reads the view fitting the straight box of the shapes drawn, or of the shapes selected, 40
 * pixels left on each side (`fittedView`, `pickAreasBox`, SP-030's answer 6): their contours as
 * drawn, a text never measured, which is not drawn, left out.
 *
 * @kind procedure
 * @param document - page document
 * @param isSelection - whether the shapes selected are fitted, rather than every shape
 * @returns the view; undefined when none of them is drawn
 */
export function readFittedView(document: Document, isSelection: boolean): View | undefined {
  const box = pickAreasBox(readFittedAreas(document, isSelection));

  // No shape with a corner: the box is empty, its minima +∞ and its maxima −∞.
  if (box.minX > box.maxX) {
    return undefined;
  }

  return fittedView(readView(document), box, {
    canvas: readCanvasSize(document),
    padding: FIT_PADDING,
  });
}
