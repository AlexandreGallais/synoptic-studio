/** Parameters of the rectangle tool: integer width, height and corner radii, ≥ 0 (ADR-0003, Q15); it makes a free shape (ADR-0032). */
export type Rectangle = {
  /** Vertical size, integer ≥ 0. */
  readonly height: number;

  /**
   * Requested corner radius of each vertex, in contour order: top left, top right, bottom right,
   * bottom left (Q11); integers ≥ 0, the effective ones derived (Q8, ADR-0029).
   */
  readonly radii: readonly number[];
  /** Horizontal size, integer ≥ 0. */
  readonly width: number;
};
