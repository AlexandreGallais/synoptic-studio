import type { DrawnStroke } from "./drawn-stroke";
import type { LineDecoration } from "./line-decoration";
import type { Size } from "../../geometry";
import type { Point } from "../../math";
import type { PositionedLine } from "../text-layout";

/** What the SVG of a text draws (ADR-0052): its measured block and its middle, its positioned lines, the backgrounds behind them and the strokes along them. */
export type DrawnText = {
  /** The backgrounds behind its lines, as a highlighter, line by line, all in one `<path>` under its letters (`DERIV-text-box`, step 10). */
  readonly backgrounds: readonly LineDecoration[];
  /** Its block, measured once (`textBlock`): as wide as its widest line, at least 0, and its lines tall. */
  readonly block: Size;
  /** The middle of its block in the SVG, or of its box when it sits in one, its position plus half of it: the point it turns about (`DERIV-turned-shape`). */
  readonly center: Point;
  /** Its lines, in order, one `<tspan>` each. */
  readonly lines: readonly PositionedLine[];
  /** Its underlines and strike-throughs, line by line, one `<path>` each in its color. */
  readonly decorations: readonly DrawnStroke[];
};
