import type { ShapesMove } from "./shapes-move";
import type { Point } from "../../src";

/** A press and drag with the Selector on the drawing (F37), from the press to the release. */
export type SelectionDrag = {
  /** The point pressed, in the coordinates of the drawing's SVG, possibly fractional. */
  readonly press: Point;
  /** The point the pointer is on, possibly fractional. */
  readonly pointer: Point;
  /** Whether Shift was held at the pointer's last move, keeping the larger axis of a move (F39). */
  readonly isShift: boolean;
  /** Whether it draws a selection rectangle: pressed on an empty place of the drawing, in Shape mode. */
  readonly isRectangle: boolean;
  /** The shapes it moves, pressed on a shape in Shape mode (F39); undefined for another press. */
  readonly move: ShapesMove | undefined;
  /** Whether the pointer has gone further than 3 pixels from the press: its release is then no click. */
  readonly isMoved: boolean;
};
