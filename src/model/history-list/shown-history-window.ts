import type { HistoryWindow } from "./history-window";

/**
 * How many states the list of the history's steps shows on each side of the present (US-077): the
 * present always, then the rows left, at most half of them after it, the rest before it, the side
 * that holds fewer giving its rows to the other.
 *
 * @kind domain
 * @param past - how many states are past
 * @param future - how many states are to come
 * @param rows - the most rows shown, the present's included
 * @returns the past states shown, and those to come
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function shownHistoryWindow(past: number, future: number, rows: number): HistoryWindow {
  return { after: future, before: Math.min(past, rows) };
}
