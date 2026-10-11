import type { Anchor } from "../geometry";

/** Regular polygon of the model: integers and an anchor only; its vertices are derived (ADR-0003). */
export type RegularPolygon = {
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
};
