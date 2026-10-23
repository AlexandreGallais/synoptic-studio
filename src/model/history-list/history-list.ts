import type { HistoryRow } from "./history-row";

/**
 * The list of the history's steps (US-077): the rows shown around the present, the oldest first,
 * and how many states are left out before and after them.
 *
 * @template T - type of the shapes, or of what is kept of each
 */
export type HistoryList<T> = {
  /** How many past states are older than the first row, not shown. */
  readonly earlier: number;
  /** How many states to come are after the last row, not shown. */
  readonly later: number;
  /** The rows shown, the oldest first, the present among them. */
  readonly rows: readonly HistoryRow<T>[];
};
