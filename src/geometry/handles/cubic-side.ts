import { add } from "../../math";

import type { CurveCorner } from "./curve-corner";
import type { Cubic } from "../contour";

/**
 * The cubic between two corners: from the start, through the start plus its handle after and the
 * end plus its handle before, to the end (ADR-0046, `REF-SVG2-PATHS` `C`).
 *
 * Formula: P₀ = A, P₁ = A + A.after, P₂ = B + B.before, P₃ = B.
 *
 * @kind geometry
 * @param start - corner where the side starts
 * @param end - corner where it ends
 * @returns the cubic piece
 * @see DERIV-cubic-sides
 */
export function cubicSide(start: CurveCorner, end: CurveCorner): Cubic {
  return {
    end: end.point,
    first: add(start.point, start.handles.after),
    kind: "cubic",
    second: add(end.point, end.handles.before),
    start: start.point,
  };
}
