import type { Size } from "../../geometry";

/** What a text measures, before it is placed: its lines shown and its block (ADR-0052). */
export type MeasuredText = {
  /** The lines shown, wrapped in its box and cut at its maximum lines (`shownLines`). */
  readonly lines: readonly string[];
  /** The block those lines take (`textBlock`). */
  readonly block: Size;
};
