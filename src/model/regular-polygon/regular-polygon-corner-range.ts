/**
 * Allowed numbers of corners of a regular polygon: from 3 (a triangle) to 12 (Q19, Product
 * Owner, 2026-10-09). Symbols use up to 8 sides; beyond 12 a polygon is hardly told from a circle.
 */
export const REGULAR_POLYGON_CORNER_RANGE: {
  /** Largest number of corners. */
  readonly max: number;
  /** Smallest number of corners. */
  readonly min: number;
} = {
  max: 12,
  min: 3,
};
