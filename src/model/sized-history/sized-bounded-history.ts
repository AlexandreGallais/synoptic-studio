import { mostDroppedStates } from "../history";

import { measureByState } from "./measure-by-state";

import type { HistoryMeasure } from "./history-measure";
import type { SizedHistory } from "./sized-history";
import type { HistoryBound } from "../history";

/** The size the oldest past states are dropped to, and the most of them dropped. */
type DroppedBound = {
  /** The most bytes the history holds. */
  readonly size: number;
  /** The most past states dropped, the steps kept left. */
  readonly most: number;
};

/** The measure as the oldest past states are dropped, and how many are. */
type DroppedMeasure = HistoryMeasure & {
  /** How many of the oldest past states are dropped. */
  readonly dropped: number;
};

/**
 * The measure once the oldest past states are dropped one by one, while the size is over the size
 * set, among the most that may be: each state read once, those left after it not at all.
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param history - the history bounded, over the size set
 * @param bound - the size set, and the most past states dropped
 * @returns the measure left and how many states are dropped
 * @see RULE-interaction-7-navigation-and-history
 */
function droppedMeasure<T extends WeakKey>(
  history: SizedHistory<T>,
  bound: DroppedBound,
): DroppedMeasure {
  const shown = new Set(history.present.drawing);

  return history.past
    .values()
    .take(bound.most)
    .reduce<DroppedMeasure>(
      (before, state) =>
        before.size <= bound.size
          ? before
          : { ...measureByState(before, { by: -1, state }, shown), dropped: before.dropped + 1 },
      { dropped: 0, held: history.held, size: history.size },
    );
}

/**
 * The history within its bound, its size kept (EN-147): while its size is over the size set and
 * more past states than the steps kept are left (`mostDroppedStates`), the oldest past state
 * removed (`droppedMeasure`), one by one, each read once; the states to come never dropped. The
 * history is `boundedHistory`'s, found without measuring the history again (ADR-0097).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param history - the history after a step, or when the bound changes
 * @param bound - the memory size and the steps kept the designer sets
 * @returns the history given when nothing is dropped, otherwise a new history
 * @see RULE-interaction-7-navigation-and-history
 */
export function sizedBoundedHistory<T extends WeakKey>(
  history: SizedHistory<T>,
  bound: HistoryBound,
): SizedHistory<T> {
  const most = mostDroppedStates(history, bound);

  if (most === 0 || history.size <= bound.size) {
    return history;
  }

  const { dropped, held, size } = droppedMeasure(history, { most, size: bound.size });

  return { ...history, held, past: history.past.slice(dropped), size };
}
