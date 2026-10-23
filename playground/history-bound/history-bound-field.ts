/** A field of the history's bound, and what it accepts. */
export type HistoryBoundField = {
  /** The id of its input. */
  readonly id: string;
  /** The most it accepts, in its unit; the fewest is 0. */
  readonly most: number;
  /** What one of its unit counts: 1 000 000 bytes a MB, 1 step. */
  readonly unit: number;
};
