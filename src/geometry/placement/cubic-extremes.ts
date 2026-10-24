import { cubicAxisTurns } from "./cubic-axis-turns";
import { cubicPoint } from "./cubic-point";

import type { Point } from "../../math";
import type { Cubic } from "../contour";

/**
 * Points of a cubic side where its x or its y turns between its ends: with its ends, they give its
 * straight box (`DERIV-cubic-sides` step 3, `REF-POMAX-BEZIER`).
 *
 * Formula: B(t) at each root of B′ on x or on y with 0 < t < 1 (`cubicAxisTurns`).
 *
 * @kind geometry
 * @param cubic - start, control points and end
 * @returns the curve's points at those values of t, ends not included
 * @see DERIV-cubic-sides
 */
export function cubicExtremes(cubic: Cubic): readonly Point[] {
  const { end, first, second, start } = cubic;
  const turns = [
    ...cubicAxisTurns([start.x, first.x, second.x, end.x]),
    ...cubicAxisTurns([start.y, first.y, second.y, end.y]),
  ];

  return turns.map((t) => cubicPoint(cubic, t));
}
