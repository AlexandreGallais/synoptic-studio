import { integerQuotient } from "../../math";

import type { HistoryWindow } from "./history-window";

/**
 * How many states the list of the history's steps shows on each side of the present (US-077): the
 * present always, then the rows left, at most half of them after it, rounded down
 * (`integerQuotient`), the rest before it, the side that holds fewer giving its rows to the other.
 *
 * @kind domain
 * @param past - how many states are past
 * @param future - how many states are to come
 * @param rows - the most rows shown, the present's included
 * @returns the past states shown, and those to come
 * @see RULE-interaction-7-navigation-and-history
 */
export function shownHistoryWindow(past: number, future: number, rows: number): HistoryWindow {
  const beside = Math.max(rows - 1, 0);
  const before = Math.min(past, beside - Math.min(future, integerQuotient(beside, 2)));

  return { after: Math.min(future, beside - before), before };
}
