import type { Size } from "../../geometry";

/** Where a box is fitted (SP-030's answer 6): the canvas, less a padding on each side. */
export type FitFrame = {
  /** The canvas's size, in CSS pixels. */
  readonly canvas: Size;
  /** The space left on each side of the box fitted, in CSS pixels, ≥ 0: 40. */
  readonly padding: number;
};
