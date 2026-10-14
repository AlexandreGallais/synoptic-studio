import type { Point } from "../../math";

/** Cubic Bézier side of an evaluated contour, from a point with handles (ADR-0046, `DERIV-cubic-sides`). */
export type Cubic = {
  /** Discriminant of a contour piece. */
  readonly kind: "cubic";
  /** First point: the side's start, a point of the shape. */
  readonly start: Point;
  /** First control point: the start plus its handle after. */
  readonly first: Point;
  /** Second control point: the end plus its handle before. */
  readonly second: Point;
  /** Last point: the side's end, the next point of the shape. */
  readonly end: Point;
};
