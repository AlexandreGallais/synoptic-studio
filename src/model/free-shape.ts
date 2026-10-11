import type { Corner } from "../geometry";

/**
 * Free shape of the model (ADR-0032, ADR-0033): its corners — each a point in the shape's own box
 * and its requested radius — stretched to its width and height when drawn
 * (`DERIV-free-shape-stretch`).
 */
export type FreeShape = {
  /** Corners as stored, in contour order (Q11): finite points spanning a box of positive size, radii integers ≥ 0. */
  readonly corners: readonly Corner[];
  /** Height drawn, a finite number ≥ 0 (an integer when typed, Q15, Q27). */
  readonly height: number;
  /** Width drawn, a finite number ≥ 0 (an integer when typed, Q15, Q27). */
  readonly width: number;
};
