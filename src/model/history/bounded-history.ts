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

/** How far the count of oldest past states dropped has reached, doubled at each try. */
type DroppedReach = {
  /** The count tried next, a power of 2. */
  readonly reach: number;
  /** The most, within the steps kept. */
  readonly most: number;
};

/**
 * Whether the history is within the size set once its oldest past states are dropped.
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param history - the history bounded
 * @param size - the most bytes it holds
 * @param dropped - how many of the oldest past states are dropped
 * @returns true when its size (`historySize`) is at most the size set
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function isWithinSize<T extends WeakKey>(
  history: History<T>,
  size: number,
  dropped: number,
): boolean {
  return historySize({ ...history, past: history.past.slice(dropped) }) <= size;
}

/**
 * The range the fewest count to drop lies in, found by doubling the count tried from 1 until the
 * history fits, or the most is reached: one state dropped, as after a step, is found by one
 * measure; the range then runs from past the count tried before to the one that fits.
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param history - the history bounded, over the size set as it is
 * @param size - the most bytes it holds
 * @param reach - the count tried, and the most
 * @returns the range to halve (`fewestDropped`)
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function doubledRange<T extends WeakKey>(
  history: History<T>,
  size: number,
  reach: DroppedReach,
): DroppedRange {
  const tried = Math.min(reach.reach, reach.most);

  if (tried === reach.most || isWithinSize(history, size, tried)) {
    return { fewest: integerQuotient(reach.reach, 2) + 1, most: tried };
  }

  return doubledRange(history, size, { most: reach.most, reach: reach.reach * 2 });
}

/**
 * The fewest oldest past states to drop for the history's size to be within the size set, or the
 * most of the range when none fits; found by halving the range, since each state dropped lowers
 * the size or keeps it (ADR-0095, decision 5).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param history - the history bounded
 * @param size - the most bytes it holds
 * @param range - the counts still possible
 * @returns how many of the oldest past states to drop
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function fewestDropped<T extends WeakKey>(
  history: History<T>,
  size: number,
  range: DroppedRange,
): number {
  if (range.fewest >= range.most) {
    return range.fewest;
  }

  const middle = integerQuotient(range.fewest + range.most, 2);
  const isWithin = isWithinSize(history, size, middle);

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
 * `REF-GIMP-PREFS-RESOURCES`). A history within its size, as after most steps, is measured once,
 * one over it by one state, as a step adds, twice (`doubledRange`), each measure reading every
 * shape the past and the states to come hold alone; more states to drop, a size just set
 * smaller, are found by doubling, then halving (`fewestDropped`).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param history - the history after a step, or when the bound changes
 * @param bound - the memory size and the steps kept the designer sets
 * @returns the history given when nothing is dropped, otherwise a new history
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function boundedHistory<T extends WeakKey>(
  history: History<T>,
  bound: HistoryBound,
): History<T> {
  const most = Math.max(history.past.length - bound.steps, 0);

  if (most === 0 || isWithinSize(history, bound.size, 0)) {
    return history;
  }

  const range = doubledRange(history, bound.size, { most, reach: 1 });
  const dropped = fewestDropped(history, bound.size, range);

  return { ...history, past: history.past.slice(dropped) };
}
