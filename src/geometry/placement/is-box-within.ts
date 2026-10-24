import { EPSILON } from "../../math";

import { widenedBox } from "./widened-box";

import type { BoundingBox } from "./bounding-box";

/**
 * Whether an upright box lies within another, sides included: its interval on each axis within
 * the other's widened by `EPSILON` (`widenedBox`), a side passing beyond by less than `EPSILON`
 * taken as on it, a box computed from a contour's arcs carrying such rounding errors (ADR-0003,
 * `DERIV-box-overlap` step 2).
 *
 * Formula: outer.minX − ε ≤ inner.minX, inner.maxX ≤ outer.maxX + ε, outer.minY − ε ≤ inner.minY
 * and inner.maxY ≤ outer.maxY + ε.
 *
 * @kind geometry
 * @param inner - the box asked
 * @param outer - the box that may hold it
 * @returns true when every point of the inner box is in the outer one, within `EPSILON`
 * @see REF-WIKI-BOUNDING-BOX
 */
export function isBoxWithin(inner: BoundingBox, outer: BoundingBox): boolean {
  const reach = widenedBox(outer, EPSILON);

  return (
    reach.minX <= inner.minX &&
    inner.maxX <= reach.maxX &&
    reach.minY <= inner.minY &&
    inner.maxY <= reach.maxY
  );
}
