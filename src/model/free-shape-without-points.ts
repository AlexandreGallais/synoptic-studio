import { freeShapeAroundCorners } from "./free-shape-around-corners";
import { freeShapeCorners } from "./free-shape-corners";

import type { FreeShape } from "./free-shape";

/**
 * Free shape without some of its points: each goes with its radius, and its two neighbours become
 * consecutive, joined by a straight edge, the contour staying closed; the others keep their place
 * and their order, numbered from the first one left (F08.AC4, note 0007). The caller keeps at
 * least 3 points.
 *
 * @kind domain
 * @param shape - free shape, as drawn at its size
 * @param removed - numbers of the points to remove, from 0, in any order
 * @returns the free shape of the points left, around them as drawn
 * @see REF-INKSCAPE-NODE-OPS
 */
export function freeShapeWithoutPoints(shape: FreeShape, removed: readonly number[]): FreeShape {
  return freeShapeAroundCorners(
    freeShapeCorners(shape).filter((corner, index) => !removed.includes(index)),
  );
}
