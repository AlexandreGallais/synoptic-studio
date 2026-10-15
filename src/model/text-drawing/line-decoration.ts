/** A rectangle drawn along a line of text, an underline, a strike-through or a background, in user units (ADR-0052). */
export type LineDecoration = {
  /** Its thickness, or the background's height. */
  readonly height: number;
  /** Its length, the line's width. */
  readonly width: number;
  /** Abscissa of its left side. */
  readonly x: number;
  /** Ordinate of its top side. */
  readonly y: number;
};
