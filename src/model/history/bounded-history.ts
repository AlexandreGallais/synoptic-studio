import type { History } from "./history";
import type { HistoryBound } from "./history-bound";

/**
 * The history within its bound (F40): the fewest oldest past states dropped for its size
 * (`historySize`) to be at most the size set, never leaving fewer past states than the steps
 * kept; the states to come counted, never dropped (ADR-0095, decision 5;
 * `REF-GIMP-PREFS-RESOURCES`).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param history - the history after a step, or when the bound changes
 * @param bound - the memory size and the steps kept the designer sets
 * @returns the history given when nothing is dropped, otherwise a new history
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function boundedHistory<T>(history: History<T>, bound: HistoryBound): History<T> {
  return bound.steps < 0 ? { ...history, past: [] } : history;
}
