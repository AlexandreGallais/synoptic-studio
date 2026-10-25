import type { Point } from "../../src";

/** A shape to place by a click: the tool armed and the point clicked. */
export type ShapePlacement = {
  /** The point clicked, in the coordinates of the SVG, possibly fractional. */
  readonly point: Point;
  /** The tool armed: `rectangle`, `polygon`, `round` or `text`. */
  readonly tool: string;
};
