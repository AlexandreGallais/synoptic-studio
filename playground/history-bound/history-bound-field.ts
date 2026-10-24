/** A field of the history's bound, and what it accepts. */
export type HistoryBoundField = {
  /** The id of its input. */
  readonly id: string;
  /** The maximum it accepts, in its unit; the minimum is 0. */
  readonly maximum: number;
  /** What one of its unit counts: 1 000 000 bytes a MB, 1 step. */
  readonly unit: number;
};
