import type { HistoryBound } from "./history-bound";

/**
 * The history's bound until the designer sets another (F40): 100 MB, 100 000 000 bytes, for a view
 * of up to about 1 000 symbols, and 5 steps kept whatever their size, as GIMP's default
 * (SP-029's answers 6 and 7, `REF-GIMP-UNDO-CODE`).
 */
export const DEFAULT_HISTORY_BOUND: HistoryBound = { size: 0, steps: 0 };
