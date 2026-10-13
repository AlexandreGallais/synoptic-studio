import { perpDot } from "../../math";
import { cubicStartDirection } from "../curve";

import type { CornerSide } from "./corner-side";
import type { FilletSide } from "./fillet-side";

/**
 * A corner's two sides oriented towards their fillet: σ = sign(a⊥ · b) from their directions at
 * the corner; the side before turned by σ, the side after by −σ (`DERIV-fillet-beside-curve`
 * step 2, `REF-MATHWORLD-PERP-DOT`); σ = 0 when they leave along one line, or one has length 0.
 *
 * Formula: n_A = σ a⊥, n_B = −σ b⊥.
 *
 * @kind geometry
 * @param before - the side before, written from the corner
 * @param after - the side after, written from the corner
 * @returns the side before, then the side after, oriented
 * @see DERIV-fillet-beside-curve
 */
export function orientedSides(
  before: CornerSide,
  after: CornerSide,
): readonly [FilletSide, FilletSide] {
  const a = cubicStartDirection(before.cubic);
  const b = cubicStartDirection(after.cubic);
  // Adding zero: no −0 for sides along one line. A side of length 0 has no direction, only the
  // rounding noise of its control points: no corner there.
  const hasCorner = before.length > 0 && after.length > 0;
  const sign = hasCorner ? Math.sign(perpDot(a, b)) + 0 : 0;

  return [
    { ...before, sign },
    { ...after, sign: -sign },
  ];
}
