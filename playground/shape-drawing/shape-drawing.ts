import type { Point } from "../../src";

/** A drag with a tool on the drawing (F36), from the press to the pointer. */
export type ShapeDrawing = {
  /** The point pressed, in the coordinates of the SVG, possibly fractional. */
  readonly press: Point;
  /** The point the pointer is on, possibly fractional. */
  readonly pointer: Point;
  /** The tool armed at the press: `rectangle`, `polygon`, `round` or `text`. */
  readonly tool: string;
  /** Whether Shift is held. */
  readonly isShift: boolean;
  /** Whether the new shape is drawn yet: once the pointer has gone further than 3 pixels from the press. */
  readonly isDrawn: boolean;
};
