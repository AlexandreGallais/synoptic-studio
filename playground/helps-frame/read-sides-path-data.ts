import { formatSvgNumber } from "../../src";

import type { BoundingBox } from "../../src";

/**
 * Path data of a box's sides moved inward by a distance, outward when it is negative or zero:
 * clockwise on the screen outward, the other way inward, so that the nonzero rule leaves an inner
 * one empty inside an outer one.
 *
 * @kind procedure
 * @param box - the box, in user units
 * @param inset - how far each side moves inward, in user units
 * @returns the closed sub-path
 * @see docs/domain/interaction.md#_2-selector
 */
export function readSidesPathData(box: BoundingBox, inset: number): string {
  const left = formatSvgNumber(box.minX + inset);
  const top = formatSvgNumber(box.minY + inset);
  const right = formatSvgNumber(box.maxX - inset);
  const bottom = formatSvgNumber(box.maxY - inset);

  return inset > 0
    ? `M${left} ${top}V${bottom}H${right}V${top}Z`
    : `M${left} ${top}H${right}V${bottom}H${left}Z`;
}
