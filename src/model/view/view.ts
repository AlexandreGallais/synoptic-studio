import type { Point } from "../../math";

/**
 * The view of the drawing (ADR-0098): how much it is zoomed and where it is panned, kept outside
 * the drawing, its history and its export.
 */
export type View = {
  /** The point of the drawing at the canvas's top-left, in user units. */
  readonly origin: Point;
  /** CSS pixels per user unit, > 0: 1 at 100 %, 2 at 200 %. */
  readonly scale: number;
};
