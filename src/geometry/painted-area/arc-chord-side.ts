import { perpDot, subtract } from "../../math";

import type { Point } from "../../math";
import type { Arc } from "../contour";

/**
 * Which side of a fillet arc's chord a point lies on, signed by the arc's direction: negative on
 * the side away from the centre, where the arc bulges, positive on the centre's, 0 on the chord's
 * line (`DERIV-painted-area` step 3, `DERIV-box-overlap` step 7).
 *
 * Formula: σ ((E − S) × (P − S)), σ = +1 for a positive arc, −1 otherwise.
 *
 * @kind geometry
 * @param arc - its ends and direction
 * @param point - the point asked
 * @returns the signed perp dot product, negative on the arc's side of its chord
 * @see REF-MATHWORLD-PERP-DOT
 */
export function arcChordSide(arc: Pick<Arc, "end" | "positive" | "start">, point: Point): number {
  const side = perpDot(subtract(arc.end, arc.start), subtract(point, arc.start));

  return arc.positive ? side : -side;
}
