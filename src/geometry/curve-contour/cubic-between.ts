import { splitCubic } from "../curve";

import type { Cubic } from "../contour";

/**
 * The piece of a cubic between two parameters, exactly on it: split at the end, then the first
 * piece split at the start's share of it (`DERIV-fillet-beside-curve` step 9,
 * `REF-WIKI-DE-CASTELJAU`); the cubic itself from 0 to 1.
 *
 * @kind geometry
 * @param cubic - start, control points and end
 * @param from - where the piece starts, from 0
 * @param to - where it ends, up to 1, after `from`
 * @returns the piece
 * @see DERIV-fillet-beside-curve
 */
export function cubicBetween(cubic: Cubic, from: number, to: number): Cubic {
  const [head] = to === 1 ? [cubic] : splitCubic(cubic, to);

  return from === 0 ? head : splitCubic(head, from / to)[1];
}
