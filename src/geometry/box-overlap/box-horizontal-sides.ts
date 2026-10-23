import type { Segment } from "../contour";
import type { BoundingBox } from "../placement";

/**
 * The top and bottom sides of an upright box, each from its left end to its right end
 * (`DERIV-box-overlap` step 4).
 *
 * Formula: (minX, minY) to (maxX, minY); (minX, maxY) to (maxX, maxY).
 *
 * @kind geometry
 * @param box - its extreme coordinates
 * @returns its top side, then its bottom side
 * @see DERIV-box-overlap
 */
export function boxHorizontalSides(box: BoundingBox): readonly [Segment, Segment] {
  return [
    { end: { x: box.maxX, y: box.minY }, kind: "segment", start: { x: box.minX, y: box.minY } },
    { end: { x: box.maxX, y: box.maxY }, kind: "segment", start: { x: box.minX, y: box.maxY } },
  ];
}
