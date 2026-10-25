import type { History } from "./history";
import type { HistoryBound } from "./history-bound";

/**
 * The most oldest past states a bound may drop (F40): the past states beyond the steps kept, none
 * when no more are past (ADR-0095, decision 5).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param history - the history bounded
 * @param bound - the memory size and the steps kept the designer sets
 * @returns the past states less the steps kept, 0 or more
 * @see RULE-interaction-7-navigation-and-history
 */
export function mostDroppedStates<T>(history: History<T>, bound: HistoryBound): number {
  return Math.max(history.past.length - bound.steps, 0);
}
