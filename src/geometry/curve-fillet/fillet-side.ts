import type { CornerSide } from "./corner-side";

/** A side of a corner oriented towards its fillet (`DERIV-fillet-beside-curve` step 2). */
export type FilletSide = CornerSide & {
  /** +1 or −1: which quarter turn of the side's tangent points towards the fillet; 0 without corner. */
  readonly sign: number;
};
