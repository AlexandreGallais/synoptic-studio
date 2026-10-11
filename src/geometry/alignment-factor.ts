import type { Alignment } from "./alignment";

/** Share of the room placed before the shape, per alignment (`REF-SVG2-COORDS` §8.2). */
const FACTORS = { max: 1, mid: 0.5, min: 0 } as const;

/**
 * Share of the room left on an axis that goes before the shape: 0 for `min`, ½ for `mid`, 1 for
 * `max` (`DERIV-regular-polygon-fit` step 5; SVG's viewport transform, steps 11–14).
 *
 * @kind geometry
 * @param alignment - alignment on the axis
 * @returns 0, 0.5 or 1
 * @see DERIV-regular-polygon-fit
 */
export function alignmentFactor(alignment: Alignment): number {
  return FACTORS[alignment];
}
