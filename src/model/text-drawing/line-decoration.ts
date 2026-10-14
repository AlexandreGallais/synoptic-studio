/** A stroke drawn along a line of text, an underline or a strike-through, as a rectangle in user units (ADR-0052). */
export type LineDecoration = {
  /** Its thickness. */
  readonly height: number;
  /** Its length, the line's width. */
  readonly width: number;
  /** Abscissa of its left side. */
  readonly x: number;
  /** Ordinate of its top side. */
  readonly y: number;
};
