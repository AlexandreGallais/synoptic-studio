import type { FittingLine } from "./fitting-line";

/** The lines that fit a box and end right before a given token, and what the tokens after need. */
export type LinesEndingAhead = {
  /** The lines, from the earliest start to the latest. */
  readonly lines: readonly FittingLine[];
  /** The fewest lines that fit and hold the tokens from there to the end; infinite for none. */
  readonly fewestAfter: number;
};
