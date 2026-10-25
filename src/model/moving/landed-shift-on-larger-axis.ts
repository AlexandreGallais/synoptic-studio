import { landedShift } from "./landed-shift";
import { offsetOnLargerAxis } from "./offset-on-larger-axis";

import type { Point } from "../../math";

/**
 * How far the shapes selected move at the release of a drag with Shift held (F39): the pointer's
 * offset kept on the axis of the larger displacement (`offsetOnLargerAxis`), that axis landing
 * their anchor on integers (`landedShift`), the other axis kept still, never landed, as Penpot
 * leaves unrounded an axis Shift keeps (Product Owner, 2026-10-23).
 *
 * @kind domain
 * @param anchor - the top-left of the straight box of the shapes moved, their dashed box for several
 * @param offset - how far the pointer went from the press, possibly fractional
 * @returns the shift every shape moved goes by, 0 on the axis kept still
 * @see RULE-interaction-2-selector
 */
export function landedShiftOnLargerAxis(anchor: Point, offset: Point): Point {
  const kept = offsetOnLargerAxis(offset);
  const landed = landedShift(anchor, kept);

  // `offsetOnLargerAxis` sets y to 0 when it keeps the axis across, and keeps a y larger than x
  // by its size, so never 0, when it keeps the axis down.
  return kept.y === 0 ? { x: landed.x, y: 0 } : { x: 0, y: landed.y };
}
