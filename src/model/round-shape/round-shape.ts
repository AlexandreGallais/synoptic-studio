/** Round shape of the model (ADR-0048): it fills its box, a circle in a square box and an ellipse otherwise; its points and handles are derived (ADR-0003). */
export type RoundShape = {
  /** Angle in degrees, an integer from 0 to 359, clockwise on screen, about the center of its box (ADR-0035). */
  readonly angle: number;
  /** Height of its box, integer ≥ 0. */
  readonly height: number;
  /** Tells it from the other shapes, which have corners. */
  readonly kind: "round";
  /** Width of its box, integer ≥ 0. */
  readonly width: number;
  /** Abscissa of the top-left of its box before it turns, in the SVG (ADR-0035). */
  readonly x: number;
  /** Ordinate of the top-left of its box before it turns, in the SVG (ADR-0035). */
  readonly y: number;
};
