import { add } from "../../math";

import type { CurveCorner } from "./curve-corner";
import type { Cubic } from "../contour";

/** The zero vector: the handle of a point that is off. */
const ORIGIN = { x: 0, y: 0 };

/**
 * The cubic between two corners: from the start, through the start plus its handle after and the
 * end plus its handle before, to the end (ADR-0046, `REF-SVG2-PATHS` `C`). A handle whose point
 * is off is kept in the model but not drawn: it counts as (0, 0).
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
    first: add(start.point, start.handles.isOn ? start.handles.after : ORIGIN),
    kind: "cubic",
    second: add(end.point, end.handles.isOn ? end.handles.before : ORIGIN),
    start: start.point,
  };
}
