import type { HistoryBoundField } from "./history-bound-field";

/**
 * The fields of the history's bound beside the Undo and Redo buttons (F40): Memory, in whole MB,
 * from 0 to 1 000, and Steps kept, from 0 to 100 (agent's choice: 100 steps of 1 000 rectangles
 * changed at once hold about 124 MB, as much as the default size).
 */
export const HISTORY_BOUND_FIELDS: readonly HistoryBoundField[] = [
  { id: "history-memory", most: 1000, part: "size", unit: 1_000_000 },
  { id: "history-steps", most: 100, part: "steps", unit: 1 },
];
