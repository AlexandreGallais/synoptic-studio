import { perpDot, scale } from "../../math";
import { cubicStartDirection } from "../curve";
import { orientedSides } from "../fillet-side";
import { cubicPoint } from "../placement";

import { filletTangency } from "./fillet-tangency";
import { fittedFilletRadius } from "./fitted-fillet-radius";

import type { CornerFillet } from "./corner-fillet";
import type { Arc } from "../contour";
import type { CornerSide } from "../fillet-side";

/**
 * Whether a corner turns positively, as `turningAngle`: from the direction coming into it to the
 * one leaving it (`REF-MATHWORLD-PERP-DOT`).
 *
 * @kind geometry
 * @param sides - the side before, then the side after, written from the corner
 * @returns `true` for a positive turn
 * @see DERIV-fillet-arc
 */
function isTurningPositively(sides: readonly [CornerSide, CornerSide]): boolean {
  const coming = scale(cubicStartDirection(sides[0].cubic), -1);

  return perpDot(coming, cubicStartDirection(sides[1].cubic)) > 0;
}

/**
 * The fillet of a corner beside a curved side: the arc of the radius drawn tangent to both sides,
 * the parameters where it touches them (ADR-0047, `DERIV-fillet-beside-curve` step 8); an arc
 * without length at the corner when no radius fits.
 *
 * @kind geometry
 * @param sides - the side before, then the side after, written from the corner
 * @param radius - the radius asked, ≥ 0
 * @returns the arc, from the side before to the side after, and its two parameters
 * @see DERIV-fillet-beside-curve
 */
export function curveFilletOf(
  sides: readonly [CornerSide, CornerSide],
  radius: number,
): CornerFillet {
  const oriented = orientedSides(sides[0], sides[1]);
  const drawn = fittedFilletRadius(oriented, radius);
  const found = drawn === 0 ? undefined : filletTangency(oriented, drawn);
  const { after, before, centre } = found ?? { after: 0, before: 0, centre: sides[0].cubic.start };
  const arc: Arc = {
    center: centre,
    end: cubicPoint(sides[1].cubic, after),
    kind: "arc",
    positive: isTurningPositively(sides),
    radius: found === undefined ? 0 : drawn,
    start: cubicPoint(sides[0].cubic, before),
  };

  return { after, arc, before };
}
