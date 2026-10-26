import type { HistoryBound } from "editor";

/**
 * The history's bound of each page, by its document: the memory size and the steps kept
 * the designer set, read from the browser's storage when the page is mounted; none before.
 */
export const PAGE_HISTORY_BOUNDS = new WeakMap<Document, HistoryBound>();
