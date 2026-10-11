import type { Corner } from "../geometry";

/**
 * Free shape of the model (ADR-0032, ADR-0033): its corners — each a point in the shape's own box
 * and its requested radius — stretched to its width and height when drawn
 * (`DERIV-free-shape-stretch`).
 */
export type FreeShape = {
  /** Angle in degrees, an integer from 0 to 359, clockwise on screen, about the center of its box (ADR-0035). */
  readonly angle: number;
  /** Corners as stored, in contour order (Q11): finite points spanning a box of positive size, radii integers ≥ 0. */
  readonly corners: readonly Corner[];
  /** Height drawn, a finite number ≥ 0 (an integer when typed, Q15, Q27). */
  readonly height: number;
  /** Width drawn, a finite number ≥ 0 (an integer when typed, Q15, Q27). */
  readonly width: number;
  /** Abscissa of the top-left of its box before it turns, in the SVG; the stored points' top-left is only the reference of the stretch (ADR-0035). */
  readonly x: number;
  /** Ordinate of the top-left of its box before it turns, in the SVG (ADR-0035). */
  readonly y: number;
};
