import type { Cubic } from "../contour";

/** One side of a corner, written from the corner (`DERIV-fillet-beside-curve` step 0). */
export type CornerSide = {
  /** The side as a cubic starting at the corner; a straight side at its thirds (`straightCubic`). */
  readonly cubic: Cubic;
  /** The largest parameter a fillet may reach on it: 1 for a curve, a share for a straight side (ADR-0007). */
  readonly limit: number;
  /** Whether the side is straight: no handle bends it (`isCurvedSide`). */
  readonly isStraight: boolean;
  /** Distance from the corner to the side's other point: a straight side's length. */
  readonly length: number;
};
