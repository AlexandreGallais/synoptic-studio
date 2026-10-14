import { ZERO_VECTOR } from "../../math";
import { cyclicItem } from "../contour";
import { cornerTriples, turningAngles } from "../corner-radius";
import { straightCubic } from "../curve";
import { curveFilletOf } from "../curve-fillet";

import { cornerSides } from "./corner-sides";

import type { CornerAsk } from "./corner-ask";
import type { CornerPoints } from "../contour";
import type { CornerSide } from "../fillet-side";
import type { CurveCorner } from "../handles";

/** Points read when there are none: a contour has 3 corners or more. */
const NO_TRIPLE: CornerPoints = {
  next: ZERO_VECTOR,
  previous: ZERO_VECTOR,
  vertex: ZERO_VECTOR,
};

/** Sides read when there are none: a contour has 3 corners or more. */
const NO_SIDES: readonly [CornerSide, CornerSide] = [
  {
    cubic: straightCubic(NO_TRIPLE.vertex, NO_TRIPLE.vertex),
    isStraight: true,
    length: 0,
    limit: 1,
  },
  {
    cubic: straightCubic(NO_TRIPLE.vertex, NO_TRIPLE.vertex),
    isStraight: true,
    length: 0,
    limit: 1,
  },
];

/**
 * What a corner asks: its radius, 0 in handles mode, and beside a curve its fillet on whole sides
 * (`curveFilletOf`).
 *
 * @kind geometry
 * @param corner - its point, radius and handles
 * @param place - its points, turning angle and two sides
 * @returns what it asks
 * @see DERIV-fillet-beside-curve
 */
function askOf(corner: CurveCorner, place: Omit<CornerAsk, "fillet" | "radius">): CornerAsk {
  const radius = corner.handles.isOn ? 0 : corner.radius;
  const [before, after] = place.sides;
  const isStraight = before.isStraight && after.isStraight;

  return { ...place, fillet: isStraight ? undefined : curveFilletOf(place.sides, radius), radius };
}

/**
 * What each corner of a shape asks before its straight sides are shared: its points, turning
 * angle, radius and sides, and beside a curve its fillet on whole sides (`askOf`).
 *
 * @kind geometry
 * @param corners - corners of the shape, in contour order
 * @returns one ask per corner, in the same order
 * @see DERIV-fillet-beside-curve
 */
export function cornerAsks(corners: readonly CurveCorner[]): readonly CornerAsk[] {
  const triples = cornerTriples(corners.map((corner) => corner.point));
  const angles = turningAngles(triples);
  const sides = cornerSides(corners);

  return corners.map((corner, index) =>
    askOf(corner, {
      angle: cyclicItem(angles, index, 0),
      sides: cyclicItem(sides, index, NO_SIDES),
      triple: cyclicItem(triples, index, NO_TRIPLE),
    }),
  );
}
