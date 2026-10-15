import type { FittingLine } from "./fitting-line";

/** The lines that fit a box and end right before a given token, by their first token in order. */
export type LinesEnding = {
  /** The lines, from the earliest start to the latest. */
  readonly lines: readonly FittingLine[];
};
