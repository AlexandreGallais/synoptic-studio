import type { LineMeasure } from "../text-block";

/** A token of a paragraph measured once: with the spaces after it, and without them. */
export type TokenMeasure = {
  /** The token with the spaces after it, as it joins a line that goes on. */
  readonly whole: LineMeasure;
  /** The token without the spaces after it, as it ends a line, measured for fit. */
  readonly visible: LineMeasure;
};
