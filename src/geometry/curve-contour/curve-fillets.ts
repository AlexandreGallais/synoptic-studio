import { cyclicItem } from "../contour";
import { filletArc } from "../corner-radius";
import { curveFilletOf } from "../curve-fillet";

import { cornerAsks } from "./corner-asks";
import { limitedSide } from "./limited-side";
import { requestedSetbacks } from "./requested-setbacks";
import { sideFactors } from "./side-factors";

import type { CornerAsk } from "./corner-ask";
import type { CornerFillet } from "../curve-fillet";
import type { CurveCorner } from "../handles";

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
  const asks = cornerAsks(corners);
  const factors = sideFactors(
    asks.map((ask) => ask.sides),
    asks.map((ask) => requestedSetbacks(ask)),
  );

  return asks.map((ask, index) =>
    sharedFillet(ask, [cyclicItem(factors, index - 1, 1), cyclicItem(factors, index, 1)]),
  );
}
