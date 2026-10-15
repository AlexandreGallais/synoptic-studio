import type { LineDecoration } from "./line-decoration";
import type { PositionedLine } from "./positioned-line";
import type { Size } from "../../geometry";
import type { Point } from "../../math";

/** What the SVG of a text draws (ADR-0052): its measured block and its middle, its positioned lines and the strokes along them. */
export type DrawnText = {
  /** Its block, measured once (`textBlock`): as wide as its widest line, at least 0, and its lines tall. */
  readonly block: Size;
  /** The middle of its block in the SVG, its position plus half the block: the point it turns about (`DERIV-turned-shape`). */
  readonly center: Point;
  /** Its lines, in order, one `<tspan>` each. */
  readonly lines: readonly PositionedLine[];
  /** Its underlines and strike-throughs, line by line, one `<path>` each. */
  readonly decorations: readonly LineDecoration[];
};
