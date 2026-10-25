import { shapesEnclosedByBox, shapesTouchedByBox } from "../../src";

import type { BoundingBox, PickArea } from "../../src";

/**
 * Reads the shapes a selection rectangle takes: those whose painted area it touches
 * (`shapesTouchedByBox`), only those it encloses with Alt held (`shapesEnclosedByBox`).
 *
 * @kind procedure
 * @param areas - what each shape offers the pointer, its contour as drawn, in the order drawn
 * @param box - the rectangle, in the coordinates of the drawing's SVG
 * @param isAlt - whether Alt is held
 * @returns their indices in the order drawn
 */
export function readShapesTaken(
  areas: readonly PickArea[],
  box: BoundingBox,
  isAlt: boolean,
): readonly number[] {
  return isAlt ? shapesEnclosedByBox(areas, box) : shapesTouchedByBox(areas, box);
}
