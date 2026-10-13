import type { CornerPoints } from "../contour";
import type { CornerFillet, CornerSide } from "../curve-fillet";

/** What a corner asks before ADR-0007 shares its straight sides. */
export type CornerAsk = {
  /** The corner's previous point, its point and its next point. */
  readonly triple: CornerPoints;
  /** Its turning angle between its points. */
  readonly angle: number;
  /** Its radius asked, 0 in handles mode. */
  readonly radius: number;
  /** Its side before, then its side after, each free to use its whole length. */
  readonly sides: readonly [CornerSide, CornerSide];
  /** Beside a curve, its fillet on those whole sides; none between two straight sides. */
  readonly fillet: CornerFillet | undefined;
};
