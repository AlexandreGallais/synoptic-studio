/** Parameters of the rectangle tool, typed at creation: integer width, height and corner radii, ≥ 0 (ADR-0003, Q15), angle and position; it makes a free shape, which is what is edited afterwards (ADR-0032, ADR-0035). */
export type Rectangle = {
  /** Angle in degrees, an integer from 0 to 359, clockwise on screen, about the center of its box (ADR-0035). */
  readonly angle: number;
  /** Vertical size, integer ≥ 0. */
  readonly height: number;

  /**
   * Requested corner radius of each vertex, in contour order: top left, top right, bottom right,
   * bottom left (Q11); integers ≥ 0, the effective ones derived (Q8, ADR-0029).
   */
  readonly radii: readonly number[];
  /** Horizontal size, integer ≥ 0. */
  readonly width: number;
  /** Abscissa of the top-left of its box before it turns, in the SVG, an integer (ADR-0035). */
  readonly x: number;
  /** Ordinate of the top-left of its box before it turns, in the SVG, an integer (ADR-0035). */
  readonly y: number;
};
