import type { LineMeasure } from "../text-block";

/** A step of a paragraph's wrap: the lines it ends, and the line being filled after it. */
export type LineWrap = {
  /** The lines this step ends by a wrap, in order, without their trailing spaces. */
  readonly ended: readonly string[];
  /** The line being filled with its measure, empty before the first token. */
  readonly line: LineMeasure;
};
