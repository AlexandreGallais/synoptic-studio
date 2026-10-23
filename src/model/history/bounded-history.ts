import { integerQuotient } from "../../math";

import { historySize } from "./history-size";

import type { History } from "./history";
import type { HistoryBound } from "./history-bound";

/** The counts of oldest past states still possible to drop, both ends included. */
type DroppedRange = {
  /** The fewest, its size possibly over the bound. */
  readonly fewest: number;
  /** The most, within the steps kept. */
  readonly most: number;
};

/**
 * The fewest oldest past states to drop for the history's size to be within the size set, or the
 * most of the range when none fits; found by halving the range, since each state dropped lowers
 * the size or keeps it (ADR-0095, decision 5).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param history - the history bounded
 * @param size - the most bytes it holds
 * @param range - the counts still possible
 * @returns how many of the oldest past states to drop
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function fewestDropped<T>(history: History<T>, size: number, range: DroppedRange): number {
  if (range.fewest >= range.most) {
    return range.fewest;
  }

  const middle = integerQuotient(range.fewest + range.most, 2);
  const isWithin = historySize({ ...history, past: history.past.slice(middle) }) <= size;

  return fewestDropped(
    history,
    size,
    isWithin ? { fewest: range.fewest, most: middle } : { fewest: middle + 1, most: range.most },
  );
}

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
  const most = Math.max(history.past.length - bound.steps, 0);
  const dropped = fewestDropped(history, bound.size, { fewest: 0, most });

  return dropped === 0 ? history : { ...history, past: history.past.slice(dropped) };
}
