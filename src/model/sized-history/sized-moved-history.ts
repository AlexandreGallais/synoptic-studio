import { measureByState } from "./measure-by-state";
import { measureWithPresent } from "./measure-with-present";

import type { SizedHistory } from "./sized-history";
import type { History } from "../history";

/**
 * The history once undone or redone, its size kept (EN-147): the state that becomes the present
 * removed from its side (`measureByState`, −1), the present added to the other (1), then the
 * present changed to it (`measureWithPresent`); the move leaving the history as it is, the history
 * given (ADR-0097).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param history - the history before the move
 * @param moved - its states once undone (`undoneHistory`) or redone (`redoneHistory`)
 * @returns a new history, or the one given when the move changed nothing
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function sizedMovedHistory<T extends WeakKey>(
  history: SizedHistory<T>,
  moved: History<T>,
): SizedHistory<T> {
  if (moved === history) {
    return history;
  }

  const shown = new Set(history.present.drawing);
  const removed = measureByState(history, { by: -1, state: moved.present }, shown);
  const added = measureByState(removed, { by: 1, state: history.present }, shown);
  const measure = measureWithPresent(added, history.present.drawing, moved.present.drawing);

  return { future: moved.future, past: moved.past, present: moved.present, ...measure };
}
