/** One line of a text placed in the SVG (ADR-0052, `DERIV-text-width` step 5), in user units. */
export type PositionedLine = {
  /** The line's characters, in the text's case, written as they are measured. */
  readonly content: string;
  /** Abscissa of the line's left edge as drawn. */
  readonly left: number;
  /** Its measured width; 0 for an empty line, negative when a spacing tightens it past nothing. */
  readonly width: number;
  /** The `x` written on its `<tspan>`: its left edge left to right, its right edge right to left. */
  readonly x: number;
  /** The `y` written on its `<tspan>`: its baseline. */
  readonly y: number;
};
