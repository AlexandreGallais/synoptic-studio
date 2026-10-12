import type { Point } from "../math";

/** Three consecutive vertices of a contour around a corner, in drawing order. */
export type CornerPoints = {
  /** Vertex before the corner. */
  readonly previous: Point;
  /** Corner vertex. */
  readonly vertex: Point;
  /** Vertex after the corner. */
  readonly next: Point;
};
