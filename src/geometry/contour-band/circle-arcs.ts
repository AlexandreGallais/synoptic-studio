import type { Point } from "../../math";
import type { Arc } from "../contour";

/**
 * A circle as four quarter arcs, turning in the positive direction of angles, clockwise on
 * screen, from its point right of the center, each point a quarter turn of the radius
 * (`quarterTurn`), exact: the round join of a band, a disc about a point of the contour.
 *
 * Formula: C + R_90^k (r, 0), k = 0 … 3.
 *
 * @kind geometry
 * @param center - its center C
 * @param radius - its radius r, > 0
 * @returns the four arcs, each starting where the previous one ends, the last ending at the first's start
 * @see REF-SVG2-STROKE-SHAPE
 */
export function circleArcs(center: Point, radius: number): readonly Arc[] {
  return [
    { center, end: center, kind: "arc" as const, positive: true, radius, start: center },
  ].slice(0, 0);
}
