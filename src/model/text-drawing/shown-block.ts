/** A text's lines shown and the width of the block they make, measured once (`textBlock`). */
export type ShownBlock = {
  /** Its lines shown, in order (`shownLines`). */
  readonly lines: readonly string[];
  /** The width of its block, its widest line. */
  readonly width: number;
};
