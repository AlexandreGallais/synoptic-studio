import type { Point } from "../math";

/**
 * Circular arc of an evaluated contour, the fillet of one corner (ADR-0001, `DERIV-fillet-arc`).
 *
 * `filletArc` returns a degenerate arc (start = end) for a corner without fillet; such arcs never
 * appear in a rounded contour.
 */
export type Arc = {
  /** Discriminant of a contour piece. */
  readonly kind: "arc";
  /** Tangent point on the incoming edge, where the arc starts. */
  readonly start: Point;
  /** Tangent point on the outgoing edge, where the arc ends. */
  readonly end: Point;
  /** Center of the circle, kept for hit-testing and snapping. */
  readonly center: Point;
  /** Radius, >= 0; an arc of a rounded contour always has a length (`hasLength`). */
  readonly radius: number;
  /** Whether the arc turns in the positive direction of angles: clockwise on screen (y down). */
  readonly positive: boolean;
};
