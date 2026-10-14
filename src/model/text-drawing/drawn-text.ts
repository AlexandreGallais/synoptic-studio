import type { LineDecoration } from "./line-decoration";
import type { PositionedLine } from "./positioned-line";

/** What the SVG of a text draws (ADR-0052): its positioned lines and the strokes along them. */
export type DrawnText = {
  /** Its lines, in order, one `<tspan>` each. */
  readonly lines: readonly PositionedLine[];
  /** Its underlines and strike-throughs, line by line, one `<path>` each. */
  readonly decorations: readonly LineDecoration[];
};
