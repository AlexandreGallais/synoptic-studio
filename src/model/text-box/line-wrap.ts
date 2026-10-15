import type { LineMeasure } from "../text-block";

/** A paragraph being wrapped: the lines already ended by a wrap, and the line being filled. */
export type LineWrap = {
  /** The lines ended by a wrap, without their trailing spaces. */
  readonly lines: readonly string[];
  /** The line being filled with its measure, empty before the first token. */
  readonly line: LineMeasure;
};
