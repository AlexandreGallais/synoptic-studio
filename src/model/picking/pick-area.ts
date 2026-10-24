import type { ContourPiece } from "../../geometry";
import type { Point } from "../../math";

/** What a shape of a drawing offers the pointer (EN-143): its contour as drawn, and the outline its corners join when its box has no area. */
export type PickArea = {
  /** Its corners as drawn, turned, in contour order: the outline measured when it has no area. */
  readonly corners: readonly Point[];
  /** Whether its box has a width or a height of 0, so that it paints nothing (`isFlatBox`). */
  readonly isFlat: boolean;
  /** Its contour as drawn, turned, its corners rounded and its sides curved. */
  readonly pieces: readonly ContourPiece[];
};
