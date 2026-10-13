import { cornerTriples } from "./corner-triples";
import { turningAngles } from "./turning-angles";

import type { Point } from "../math";

/**
 * Whether every corner of a polygon, drawn clockwise on screen, turns at least half of the turn
 * of a regular polygon of as many corners, `π/n` (the corner guard of rule C, Product Owner,
 * `DERIV-stretched-polygon` step 3). A `NaN` coordinate gives a `NaN` turn, which fails.
 *
 * @kind geometry
 * @param polygon - vertices in drawing order, clockwise on screen
 * @returns `true` when no corner turns less than `π/n`
 * @see DERIV-stretched-polygon
 */
export function isSoundStretch(polygon: readonly Point[]): boolean {
  const guard = Math.PI / polygon.length;

  return turningAngles(cornerTriples(polygon)).every((turn) => turn >= guard);
}
