import type { Point } from "../math";

/**
 * Free shape of the model (ADR-0032): its points in their own box, stretched to its width and
 * height when drawn (`DERIV-free-shape-stretch`), and one requested radius per point (ADR-0029).
 */
export type FreeShape = {
  /** Height drawn, integer ≥ 0 (Q15). */
  readonly height: number;
  /** Points as stored, in contour order (Q11), integers, spanning a box of positive size. */
  readonly points: readonly Point[];
  /** Requested corner radius of each point, integers ≥ 0, the effective ones derived (Q8). */
  readonly radii: readonly number[];
  /** Width drawn, integer ≥ 0 (Q15). */
  readonly width: number;
};
