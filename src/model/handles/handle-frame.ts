import type { Point, Vector } from "../../math";

/** What a handle's two values are read against: its point, its side, which end it is. */
export type HandleFrame = {
  /** The point of the handle, as drawn. */
  readonly point: Point;
  /** From the point to the other end of the handle's side: the next point after, the previous before. */
  readonly side: Vector;
  /** Whether it is the handle before, whose angle from the side is mirrored (`DERIV-handle-angle` step 3). */
  readonly isBefore: boolean;
};
