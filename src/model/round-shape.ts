/** Round shape of the model (ADR-0036): a circle or a capsule filling its box; its corners are derived. */
export type RoundShape = {
  /** Angle in degrees, an integer from 0 to 359, clockwise on screen, about the center of its box (ADR-0035). */
  readonly angle: number;
  /** Height of its box, integer ≥ 0. */
  readonly height: number;
  /** Discriminant: a round shape, told apart from the free shape and the regular polygon (ADR-0036). */
  readonly kind: "round";
  /** Width of its box, integer ≥ 0. */
  readonly width: number;
  /** Abscissa of the top-left of its box before it turns, in the SVG (ADR-0035). */
  readonly x: number;
  /** Ordinate of the top-left of its box before it turns, in the SVG (ADR-0035). */
  readonly y: number;
};
