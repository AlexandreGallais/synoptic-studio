import type { Anchor } from "../geometry";

/** Regular polygon of the model: integers, an anchor, an angle and a position (ADR-0035); its vertices are derived (ADR-0003). */
export type RegularPolygon = {
  /** Angle in degrees, an integer from 0 to 359, clockwise on screen, about the center of its box (ADR-0035). */
  readonly angle: number;
  /** Place in its box: one alignment per axis; `mid` on both is F02's placement, the one editors start with (F07). */
  readonly anchor: Anchor;
  /** Number of corners `n`, integer from 3 to 12 (Q19). */
  readonly corners: number;
  /** Height of the box it fits in, integer ≥ 0. */
  readonly height: number;

  /**
   * Requested corner radius of each vertex, `n` of them in contour order (Q11); integers ≥ 0,
   * the effective ones derived (Q8, ADR-0029).
   */
  readonly radii: readonly number[];
  /** Width of the box it fits in, integer ≥ 0. */
  readonly width: number;
  /** Abscissa of the top-left of its box before it turns, in the SVG (ADR-0035). */
  readonly x: number;
  /** Ordinate of the top-left of its box before it turns, in the SVG (ADR-0035). */
  readonly y: number;
};
