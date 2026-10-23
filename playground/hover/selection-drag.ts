import type { Point } from "../../src";

/** A press and drag with the Selector on the drawing (F37), from the press to the release. */
export type SelectionDrag = {
  /** The point pressed, in the coordinates of the drawing's SVG, possibly fractional. */
  readonly press: Point;
  /** The point the pointer is on, possibly fractional. */
  readonly pointer: Point;
  /** Whether it draws a selection rectangle: pressed on an empty place of the drawing, in Shape mode. */
  readonly isRectangle: boolean;
  /** Whether the pointer has left the press's integer point: its release is then no click. */
  readonly isMoved: boolean;
};
