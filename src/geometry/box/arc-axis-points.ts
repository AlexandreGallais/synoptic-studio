import { perpDot } from "../../math";

import type { Point, Vector } from "../../math";
import type { Arc } from "../arc";

/** The four axis directions: right, down, left, up (y down). */
const AXES: readonly Vector[] = [
  { x: 1, y: 0 },
  { x: 0, y: 1 },
  { x: -1, y: 0 },
  { x: 0, y: -1 },
];

/**
 * Points where a fillet arc reaches an extreme x or y between its ends: `C + r·d` for each axis
 * direction `d` it sweeps through (`DERIV-arc-bounding-box` steps 2–3).
 *
 * Test: `σ (s × d) ≥ 0` and `σ (d × e) ≥ 0`, `s` and `e` from the center to the ends, `σ` the
 * sign of the arc's direction; valid for a sweep strictly between 0 and half a turn, as a fillet
 * of a rounded contour (`hasLength`), not for an arc without length.
 *
 * @kind geometry
 * @param arc - fillet arc of a rounded contour, with a length
 * @returns its axis points, in the order right, down, left, up
 * @see DERIV-arc-bounding-box
 */
export function arcAxisPoints(arc: Arc): readonly Point[] {
  const sign = arc.positive ? 1 : -1;
  const start = { x: arc.start.x - arc.center.x, y: arc.start.y - arc.center.y };
  const end = { x: arc.end.x - arc.center.x, y: arc.end.y - arc.center.y };

  return AXES.filter(
    (axis) => sign * perpDot(start, axis) >= 0 && sign * perpDot(axis, end) >= 0,
  ).map((axis) => ({
    x: arc.center.x + arc.radius * axis.x,
    y: arc.center.y + arc.radius * axis.y,
  }));
}
