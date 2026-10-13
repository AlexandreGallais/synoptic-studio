import { cyclicItem } from "../contour";
import { edgeFactor } from "../corner-radius";

import type { CornerSide } from "../fillet-side";

/**
 * The factor of each side after a corner: on a straight side, the share of its length both
 * corners' setbacks may take (ADR-0007, `DERIV-local-radius-clamp`); 1 on a curve, whose other end
 * is sharp (ADR-0047).
 *
 * @kind geometry
 * @param sides - each corner's two sides (`cornerSides`)
 * @param setbacks - each corner's two setbacks (`requestedSetbacks`)
 * @returns one factor per side, the side after each corner, from 0 to 1
 * @see DERIV-local-radius-clamp
 */
export function sideFactors(
  sides: readonly (readonly [CornerSide, CornerSide])[],
  setbacks: readonly (readonly [number, number])[],
): readonly number[] {
  return sides.map(([, after], index) => {
    const demand =
      cyclicItem(setbacks, index, [0, 0])[1] + cyclicItem(setbacks, index + 1, [0, 0])[0];

    return after.isStraight ? edgeFactor(after.length, demand) : 1;
  });
}
