import type { Cubic } from "../contour";

/**
 * A cubic traced backwards: the same points, its control points in reverse order, so that
 * B_rev(t) = B(1 − t) (`DERIV-fillet-beside-curve` step 0, `REF-WIKI-BEZIER`).
 *
 * @kind geometry
 * @param cubic - start, control points and end
 * @returns the cubic from its end to its start
 * @see DERIV-fillet-beside-curve
 */
export function reversedCubic(cubic: Cubic): Cubic {
  return {
    end: cubic.start,
    first: cubic.second,
    kind: "cubic",
    second: cubic.first,
    start: cubic.end,
  };
}
