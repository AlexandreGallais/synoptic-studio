import { undoneHistory } from "../history";

import { sizedMovedHistory } from "./sized-moved-history";

import type { SizedHistory } from "./sized-history";

/**
 * The history once undone, its size kept (EN-147): the undo of `undoneHistory`, its measure
 * updated around the move (`sizedMovedHistory`): the last past state removed from the past, the
 * present added to the states to come, then the present changed to it; nothing past, the history
 * given (ADR-0097).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param history - the history before the undo
 * @returns a new history, or the one given when nothing is past
 * @see RULE-interaction-7-navigation-and-history
 */
export function sizedUndoneHistory<T extends WeakKey>(history: SizedHistory<T>): SizedHistory<T> {
  return sizedMovedHistory(history, undoneHistory(history));
}
