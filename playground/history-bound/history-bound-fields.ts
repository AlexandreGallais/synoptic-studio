import type { HistoryBoundField } from "./history-bound-field";
import type { HistoryBound } from "../../src";

/**
 * The fields of the history's bound beside the Undo and Redo buttons (F40), by the part each
 * sets: Memory, the size, in whole MB from 0 to 1 000, and Steps kept, from 0 to 100 (agent's
 * choice: 100 steps of 1 000 rectangles changed at once hold about 124 MB, as much as the
 * default size).
 */
export const HISTORY_BOUND_FIELDS: Readonly<Record<keyof HistoryBound, HistoryBoundField>> = {
  size: { id: "history-memory", maximum: 1000, unit: 1_000_000 },
  steps: { id: "history-steps", maximum: 100, unit: 1 },
};
