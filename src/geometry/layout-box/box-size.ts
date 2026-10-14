import type { Size } from "./size";
import type { BoundingBox } from "../placement";

/**
 * Width and height of an upright box: the lengths of its intervals on each axis.
 *
 * Formula: (maxX − minX, maxY − minY).
 *
 * @kind geometry
 * @param box - the box, its bounds on each axis
 * @returns its width and height, ≥ 0 for a box of at least one point
 * @see REF-WIKI-BOUNDING-BOX
 */
export function boxSize(box: BoundingBox): Size {
  return { height: box.maxY - box.minY, width: box.maxX - box.minX };
}
