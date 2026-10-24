/**
 * What the designer sets to bound the history (F40): its memory size, and the steps kept whatever
 * their size, as GIMP's "Maximum undo memory" and "Minimal number of undo levels" (ADR-0095,
 * `REF-GIMP-PREFS-RESOURCES`).
 */
export type HistoryBound = {
  /** The most bytes the history holds, as `historySize` counts them. */
  readonly size: number;
  /** The fewest past states kept, however many bytes they hold. */
  readonly steps: number;
};
