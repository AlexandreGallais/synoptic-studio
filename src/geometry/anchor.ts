import type { Alignment } from "./alignment";

/** Place of a shape in its box: one alignment per axis, 9 places in all (glossary: anchor). */
export type Anchor = {
  /** Left (`min`), center (`mid`) or right (`max`). */
  readonly horizontal: Alignment;
  /** Top (`min`), center (`mid`) or bottom (`max`), y pointing down. */
  readonly vertical: Alignment;
};
