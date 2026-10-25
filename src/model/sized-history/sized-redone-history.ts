import { redoneHistory } from "../history";

import { sizedMovedHistory } from "./sized-moved-history";

import type { SizedHistory } from "./sized-history";

/**
 * The history once redone, its size kept (EN-147): the redo of `redoneHistory`, its measure
 * updated around the move (`sizedMovedHistory`): the first state to come removed from them, the
 * present added to the past, then the present changed to it; nothing to come, the history given
 * (ADR-0097).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param history - the history before the redo
 * @returns a new history, or the one given when nothing is to come
 * @see RULE-interaction-7-navigation-and-history
 */
export function sizedRedoneHistory<T extends WeakKey>(history: SizedHistory<T>): SizedHistory<T> {
  return sizedMovedHistory(history, redoneHistory(history));
}
