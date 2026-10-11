import type { Anchor } from "../geometry";

/** Regular polygon of the model: integers and an anchor only; its vertices are derived (ADR-0003). */
export type RegularPolygon = {
  /** Place in its box: one alignment per axis, `mid` on both by default (F07). */
  readonly anchor: Anchor;
  /** Number of corners `n`, integer from 3 to 12 (Q19). */
  readonly corners: number;
  /** Height of the box it fits in, integer ≥ 0. */
  readonly height: number;
  /** Requested corner radius, the same at every corner, integer ≥ 0; the effective one is derived (Q8). */
  readonly radius: number;
  /** Width of the box it fits in, integer ≥ 0. */
  readonly width: number;
};
