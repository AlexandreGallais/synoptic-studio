import { sizedRedoneHistory } from "./sized-redone-history";
import { sizedUndoneHistory } from "./sized-undone-history";

import type { SizedHistory } from "./sized-history";
import type { HistoryState } from "../history";

/**
 * The history whose present is a state it holds, its size kept: a past state reached by
 * as many undos as states after it are past (`sizedUndoneHistory`), a state to come by as many
 * redos as states come before it and itself (`sizedRedoneHistory`), so that the steps after it
 * are kept to redo; the present, or a state the history no longer holds, the history given.
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param history - the history before the move
 * @param state - the state to show, the object the history holds
 * @returns the history moved, or the one given
 * @see RULE-interaction-7-navigation-and-history
 */
export function sizedHistoryAt<T extends WeakKey>(
  history: SizedHistory<T>,
  state: HistoryState<T>,
): SizedHistory<T> {
  const pastIndex = history.past.lastIndexOf(state);
  const isPast = pastIndex !== -1;
  const moves = isPast ? history.past.length - pastIndex : history.future.indexOf(state) + 1;
  const move = isPast ? sizedUndoneHistory : sizedRedoneHistory;

  return Array.from({ length: moves }).reduce<SizedHistory<T>>((moved) => move(moved), history);
}
