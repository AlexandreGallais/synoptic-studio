import type { Arc } from "../contour";

/** A corner's fillet beside straight or curved sides (`DERIV-fillet-beside-curve`). */
export type CornerFillet = {
  /** The arc, from the tangent point on the side before to the one on the side after. */
  readonly arc: Arc;
  /** Parameter of the tangent point on the side before, written from the corner; 0 without arc. */
  readonly before: number;
  /** Parameter of the tangent point on the side after, written from the corner; 0 without arc. */
  readonly after: number;
};
