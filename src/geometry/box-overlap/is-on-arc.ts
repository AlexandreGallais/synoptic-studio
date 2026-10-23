import { arcChordSide } from "../painted-area";

import type { Point } from "../../math";
import type { Arc } from "../contour";

/**
 * Whether a point of a fillet arc's circle lies on the arc: on the side of its chord away from
 * the centre, or on the chord's line (`arcChordSide`), which meets the circle at the arc's ends
 * only, the arc sweeping less than half a turn (`DERIV-box-overlap` step 7, `DERIV-painted-area`
 * step 3).
 *
 * Formula: σ ((E − S) × (Q − S)) ≤ 0, σ = +1 for a positive arc, −1 otherwise.
 *
 * @kind geometry
 * @param arc - its ends and direction
 * @param point - a point of the arc's circle
 * @returns true when the point is on the arc, its ends included
 * @see DERIV-box-overlap
 */
export function isOnArc(arc: Pick<Arc, "end" | "positive" | "start">, point: Point): boolean {
  return arcChordSide(arc, point) <= 0;
}
