import type { BoundingBox } from "../../src";

/**
 * Path data of a box's sides moved inward by a distance, clockwise on the screen from its top-left
 * corner (US-086): the line of a dashed box or of the selection rectangle laid half a pixel inside
 * the box, so that a line 1 pixel wide covers whole pixels; its area for a distance of 0.
 *
 * @kind procedure
 * @param box - the box, in user units
 * @param inset - how far each side moves inward, in user units, at least 0
 * @returns the closed sub-path
 * @see docs/domain/interaction.md#_2-selector
 */
export function readInsetBoxPathData(box: BoundingBox, inset: number): string {
  return `M${box.minX + inset} ${box.minY + inset}Z`;
}
