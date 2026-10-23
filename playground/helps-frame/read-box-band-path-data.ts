import { boxSize, formatPathData } from "../../src";

import { readSidesPathData } from "./read-sides-path-data";

import type { BoxBandReach } from "./box-band-reach";
import type { BoundingBox } from "../../src";

/**
 * Path data of a band about a box's sides in the layer above the drawing (F37, F43): its sides
 * moved outward by the band's outer reach (`readSidesPathData`), then inward by its inner reach,
 * the other way round, a hole unless the box is no wider or no higher than twice the inner reach,
 * the band then the outer box alone; a fill, never a native stroke.
 *
 * @kind procedure
 * @param box - the box, in user units
 * @param reach - how far the band reaches outside and inside the sides, in user units, the inner
 * reach above 0
 * @returns the outer sub-path, then the inner one when the band has a hole
 * @see docs/domain/interaction.md#_2-selector
 */
export function readBoxBandPathData(box: BoundingBox, reach: BoxBandReach): string {
  const outer = readSidesPathData(box, -reach.outward);
  const size = boxSize(box);
  const isHollow = Math.min(size.width, size.height) > 2 * reach.inward;

  return isHollow ? formatPathData([outer, readSidesPathData(box, reach.inward)]) : outer;
}
