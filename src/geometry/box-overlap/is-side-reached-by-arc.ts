import { isBetween } from "../../math";

import { circleLevelAbscissas } from "./circle-level-abscissas";
import { isOnArc } from "./is-on-arc";

import type { Arc, Segment } from "../contour";

/**
 * Whether a fillet arc reaches a horizontal side: one of its circle's points on the side's line
 * (`circleLevelAbscissas`) lies between the side's ends and on the arc (`isOnArc`)
 * (`DERIV-box-overlap` step 7).
 *
 * Formula: some x = Cₓ ± √(r² − (c − C_y)²) between a and b with (x, c) on the arc.
 *
 * @kind geometry
 * @param arc - its ends, centre, radius and direction
 * @param side - a horizontal side, its ends at the same ordinate
 * @returns true when a point of the arc lies on the side
 * @see DERIV-box-overlap
 */
export function isSideReachedByArc(arc: Arc, side: Pick<Segment, "end" | "start">): boolean {
  const level = side.start.y;

  return circleLevelAbscissas(arc, level).some(
    (x) => isBetween(x, side.start.x, side.end.x) && isOnArc(arc, { x, y: level }),
  );
}
