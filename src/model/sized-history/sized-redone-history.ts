import { redoneHistory } from "../history";

import { measureWithPresent } from "./measure-with-present";
import { measureWithState } from "./measure-with-state";
import { measureWithoutState } from "./measure-without-state";

import type { SizedHistory } from "./sized-history";

/**
 * The history once redone, its size kept (EN-147): the redo of `redoneHistory`; the first state to
 * come removed from them (`measureWithoutState`), the present added to the past
 * (`measureWithState`), then the present changed to it (`measureWithPresent`); nothing to come,
 * the history given (ADR-0097).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param history - the history before the redo
 * @returns a new history, or the one given when nothing is to come
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function sizedRedoneHistory<T extends WeakKey>(history: SizedHistory<T>): SizedHistory<T> {
  const redone = redoneHistory(history);

  if (redone === history) {
    return history;
  }

  const shown = new Set(history.present.drawing);
  const removed = measureWithoutState(history, redone.present, shown);
  const added = measureWithState(removed, history.present, shown);
  const measure = measureWithPresent(added, history.present.drawing, redone.present.drawing);

  return { ...redone, ...measure };
}
