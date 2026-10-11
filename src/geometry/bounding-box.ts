import type { Point } from "../math";

/** Smallest axis-aligned rectangle containing a set of points (glossary: bounding box). */
export type BoundingBox = {
  /** Largest abscissa. */
  readonly maxX: number;
  /** Largest ordinate (lowest point on screen). */
  readonly maxY: number;
  /** Smallest abscissa. */
  readonly minX: number;
  /** Smallest ordinate (highest point on screen). */
  readonly minY: number;
};

/**
 * Bounding box of a set of points: the smallest and largest coordinates on each axis
 * (`DERIV-regular-polygon-fit` step 3).
 *
 * An empty set has no box: its minima are `+∞` and its maxima `−∞`.
 *
 * @kind geometry
 * @param points - points, in any order
 * @returns the extreme coordinates
 * @see DERIV-regular-polygon-fit
 */
export function boundingBox(points: readonly Point[]): BoundingBox {
  // A fold rather than Math.max(...xs): engines cap the number of arguments of a call.
  return points.reduce<BoundingBox>(
    (box, point) => ({
      maxX: Math.max(box.maxX, point.x),
      maxY: Math.max(box.maxY, point.y),
      minX: Math.min(box.minX, point.x),
      minY: Math.min(box.minY, point.y),
    }),
    { maxX: -Infinity, maxY: -Infinity, minX: Infinity, minY: Infinity },
  );
}
