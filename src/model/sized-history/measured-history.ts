import { EMPTY_HELD_COUNTS } from "../held-shapes";

import { measureByState } from "./measure-by-state";

import type { SizedHistory } from "./sized-history";
import type { History } from "../history";

/**
 * A history with its size counted once, from its states: each past and future state
 * added to an empty measure (`measureByState`), so that the size is `historySize`'s; the page's
 * history starts from it, nothing past nor to come (ADR-0097).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param history - the states, past, present and to come
 * @returns a new history, its states those given
 * @see RULE-interaction-7-navigation-and-history
 */
export function measuredHistory<T extends WeakKey>(history: History<T>): SizedHistory<T> {
  const present = new Set(history.present.drawing);
  const measure = [...history.past, ...history.future].reduce(
    (before, state) => measureByState(before, { by: 1, state }, present),
    { held: EMPTY_HELD_COUNTS, size: 0 },
  );

  return { future: history.future, past: history.past, present: history.present, ...measure };
}
