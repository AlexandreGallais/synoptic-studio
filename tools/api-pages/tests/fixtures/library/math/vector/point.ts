/**
 * Point of the plane in SVG user units (x to the right, y downwards).
 *
 * Model points are integers (ADR-0003); derived geometry may hold floats.
 */
export type Point = {
  /** Abscissa, growing to the right. */
  readonly x: number;
  /** Ordinate, growing downwards (SVG frame). */
  readonly y: number;
};
