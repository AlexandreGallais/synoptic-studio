import { curveFillets } from "./curve-fillets";

import type { CurveCorner } from "../handles";

/**
 * The radius each corner is drawn with: 0 at a point in handles mode, reduced by ADR-0007 between
 * straight sides and by ADR-0047 beside a curve — what the drawing shows, for the list of points.
 *
 * @kind geometry
 * @param corners - corners of the shape, in contour order
 * @returns one radius per corner, from 0 to the one asked
 * @see ADR-0047
 */
export function drawnRadii(corners: readonly CurveCorner[]): readonly number[] {
  return curveFillets(corners).map((fillet) => fillet.arc.radius);
}
