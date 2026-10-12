import type { Point } from "../math";

/** Straight piece of an evaluated contour (ADR-0005, stage 3). */
export type Segment = {
  /** Discriminant of a contour piece. */
  readonly kind: "segment";
  /** First point. */
  readonly start: Point;
  /** Last point. */
  readonly end: Point;
};
