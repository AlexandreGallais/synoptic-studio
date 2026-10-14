import type { Vector } from "../../math";

/** A point of the unit circle where an ellipse touches its box, and the direction of its handle after. */
export type EllipseQuarter = {
  /** The point, from the centre of the unit circle, y down. */
  readonly point: Vector;
  /** Unit tangent at the point, clockwise on screen. */
  readonly tangent: Vector;
};

/**
 * The top, right, bottom and left points of the unit circle, clockwise on screen from the top
 * (Q11), each with its tangent (`DERIV-round-shape-curve` step 3).
 */
export const ELLIPSE_QUARTERS: readonly EllipseQuarter[] = [
  { point: { x: 0, y: -1 }, tangent: { x: 1, y: 0 } },
  { point: { x: 1, y: 0 }, tangent: { x: 0, y: 1 } },
  { point: { x: 0, y: 1 }, tangent: { x: -1, y: 0 } },
  { point: { x: -1, y: 0 }, tangent: { x: 0, y: -1 } },
];
