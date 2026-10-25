import { boxSize, formatSvgNumber } from "editor";

import type { BoundingBox } from "editor";

/**
 * Path data of a box's sides moved inward by a distance, clockwise on the screen from its top-left
 * corner: the line of a dashed box or of the selection rectangle laid half a pixel inside
 * the box, so that a line 1 pixel wide covers whole pixels; its area for a distance of 0. Each side
 * moves by at most half the box's size across it, so that a box 0 wide gives a line along its
 * middle.
 *
 * @kind procedure
 * @param box - the box, in user units
 * @param inset - how far each side moves inward, in user units, at least 0
 * @returns the closed sub-path
 */
export function readInsetBoxPathData(box: BoundingBox, inset: number): string {
  const size = boxSize(box);
  const across = Math.min(inset, size.width / 2);
  const down = Math.min(inset, size.height / 2);
  const left = formatSvgNumber(box.minX + across);
  const top = formatSvgNumber(box.minY + down);
  const right = formatSvgNumber(box.maxX - across);
  const bottom = formatSvgNumber(box.maxY - down);

  return `M${left} ${top}H${right}V${bottom}H${left}Z`;
}
