import { cyclicItem } from "../contour";
import { cornerTriples, filletArc, turningAngles } from "../corner-radius";
import { straightCubic } from "../curve";
import { curveFilletOf } from "../curve-fillet";

import { cornerSides } from "./corner-sides";
import { limitedSide } from "./limited-side";
import { requestedSetbacks } from "./requested-setbacks";
import { sideFactors } from "./side-factors";

import type { CornerAsk } from "./corner-ask";
import type { CornerPoints } from "../contour";
import type { CornerFillet, CornerSide } from "../curve-fillet";
import type { CurveCorner } from "../handles";

/** Points read when there are none: a contour has 3 corners or more. */
const NO_TRIPLE: CornerPoints = {
  next: { x: 0, y: 0 },
  previous: { x: 0, y: 0 },
  vertex: { x: 0, y: 0 },
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
 * The fillet of one corner once its sides are shared: between two straight sides, the arc of F01
 * at the radius ADR-0007 leaves it (`DERIV-fillet-arc`, `DERIV-local-radius-clamp`); beside a
 * curve, its fillet on whole sides, or, when a straight side is shared down, the fillet reaching
 * no further than its share (ADR-0047).
 *
 * @kind geometry
 * @param ask - the corner's points, angle, radius, sides and fillet on whole sides
 * @param factors - the factors of its side before, then of its side after (`sideFactors`)
 * @returns its fillet
 * @see DERIV-fillet-beside-curve
 */
function sharedFillet(ask: CornerAsk, factors: readonly [number, number]): CornerFillet {
  const [factorBefore, factorAfter] = factors;

  if (ask.fillet === undefined) {
    const radius = ask.radius * Math.min(factorBefore, factorAfter);

    return { after: 0, arc: filletArc(ask.triple, ask.angle, radius), before: 0 };
  }

  const [setbackBefore, setbackAfter] = requestedSetbacks(ask);

  return Math.min(factorBefore, factorAfter) >= 1
    ? ask.fillet
    : curveFilletOf(
        [
          limitedSide(ask.sides[0], factorBefore, setbackBefore),
          limitedSide(ask.sides[1], factorAfter, setbackAfter),
        ],
        ask.radius,
      );
}

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
 * The fillet of every corner of a shape with handles: a point in handles mode sharp, its radius
 * kept; a corner between straight sides as in F01; a corner beside a curve tangent to it, its
 * radius reduced to the largest that fits, never sharp (ADR-0047, `DERIV-fillet-beside-curve`).
 *
 * @kind geometry
 * @param corners - corners of the shape, in contour order
 * @returns one fillet per corner, in the same order
 * @see ADR-0047
 */
export function curveFillets(corners: readonly CurveCorner[]): readonly CornerFillet[] {
  const triples = cornerTriples(corners.map((corner) => corner.point));
  const angles = turningAngles(triples);
  const sides = cornerSides(corners);
  const asks = corners.map((corner, index) =>
    askOf(corner, {
      angle: cyclicItem(angles, index, 0),
      sides: cyclicItem(sides, index, NO_SIDES),
      triple: cyclicItem(triples, index, NO_TRIPLE),
    }),
  );
  const factors = sideFactors(
    sides,
    asks.map((ask) => requestedSetbacks(ask)),
  );

  return asks.map((ask, index) =>
    sharedFillet(ask, [cyclicItem(factors, index - 1, 1), cyclicItem(factors, index, 1)]),
  );
}
