import type { SizedText } from "../../src";

/** The text last drawn, kept on the Text tool's options as JSON: its exact position, its block, and what X and Y showed then. */
export type DrawnTextRecord = SizedText & {
  /** The X and Y fields as they were written after it was drawn, its position at 5 decimals. */
  readonly fields: readonly string[];
};
