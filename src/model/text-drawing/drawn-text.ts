import type { LineDecoration } from "./line-decoration";
import type { PositionedLine } from "./positioned-line";
import type { Size } from "../../geometry";

/** What the SVG of a text draws (ADR-0052): its measured block, its positioned lines and the strokes along them. */
export type DrawnText = {
  /** Its block, measured once (`textBlock`): as wide as its widest line, at least 0, and its lines tall. */
  readonly block: Size;
  /** Its lines, in order, one `<tspan>` each. */
  readonly lines: readonly PositionedLine[];
  /** Its underlines and strike-throughs, line by line, one `<path>` each. */
  readonly decorations: readonly LineDecoration[];
};
