import type { HistoryState } from "./history-state";

/**
 * The history of the drawing (F40): the states undo goes back to, the present, and the states
 * redo goes forward to, as Redux's `{ past, present, future }` (ADR-0095, `REF-REDUX-UNDO`).
 *
 * @template T - type of the shapes, or of what is kept of each
 */
export type History<T> = {
  /** The states before the present, the oldest first. */
  readonly past: readonly HistoryState<T>[];
  /** The state the page shows. */
  readonly present: HistoryState<T>;
  /** The states undone, the next to redo first. */
  readonly future: readonly HistoryState<T>[];
};
