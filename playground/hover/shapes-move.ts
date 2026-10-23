import type { Point } from "../../src";
import type { KeptShape } from "../kept-shapes";

/** A move of the shapes selected by a press on one of them with the Selector (F39). */
export type ShapesMove = {
  /** The drawing at the press, every shape where it was, in the order drawn. */
  readonly drawing: readonly KeptShape[];
  /** The shapes selected at the press, by their indices, given back to a drag cancelled. */
  readonly before: readonly number[];
  /** The shapes selected once it moves, by their indices (`shapeSelectionAfterPress`). */
  readonly selected: readonly number[];
  /** The shapes it moves, those selected less a text never measured, which is not drawn. */
  readonly moved: readonly number[];

  /**
   * The top-left of the straight box of the shapes moved, their dashed box, at the press: the
   * point a release lands on integers.
   */
  readonly anchor: Point;
};
