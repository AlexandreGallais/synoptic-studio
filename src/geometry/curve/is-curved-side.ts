import type { Vector } from "../../math";
import type { CurveCorner } from "../handles";

/**
 * Whether one handle bends its side: its point in handles mode, the vector of some length.
 *
 * @kind geometry
 * @param isOn - whether the point is in handles mode
 * @param handle - vector from the point, towards the side
 * @returns `true` when the handle pulls the side away from the straight line
 * @see REF-FIGMA-VECTOR-NETWORK
 */
function isPulling(isOn: boolean, handle: Vector): boolean {
  return isOn && (handle.x !== 0 || handle.y !== 0);
}

/**
 * Whether the side between two corners is a cubic: a handle on, of some length, at either end —
 * the handle after of its start or the handle before of its end (ADR-0046; a zero handle leaves
 * the side straight, `REF-FIGMA-VECTOR-NETWORK`).
 *
 * @kind geometry
 * @param start - corner where the side starts
 * @param end - corner where it ends, the next one
 * @returns `true` when the side is drawn as a cubic
 * @see ADR-0046
 */
export function isCurvedSide(start: CurveCorner, end: CurveCorner): boolean {
  return (
    isPulling(start.handles.isOn, start.handles.after) ||
    isPulling(end.handles.isOn, end.handles.before)
  );
}
