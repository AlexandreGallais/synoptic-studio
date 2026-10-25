import { historyWithStep } from "../history";

import { measureByState } from "./measure-by-state";
import { measureWithPresent } from "./measure-with-present";

import type { SizedHistory } from "./sized-history";
import type { HistoryState } from "../history";

/**
 * The history once a finished action is recorded, its size kept (EN-147): the step of
 * `historyWithStep`; when it takes one, the present added to the past and each state to come removed
 * (`measureByState`), then the present changed (`measureWithPresent`); a selection replaced alone keeps the measure (ADR-0097).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param history - the history before the action
 * @param state - the drawing and the selection once the action finished
 * @returns the history given when nothing changed, otherwise a new history
 * @see RULE-interaction-7-navigation-and-history
 */
export function sizedHistoryWithStep<T extends WeakKey>(
  history: SizedHistory<T>,
  state: HistoryState<T>,
): SizedHistory<T> {
  const stepped = historyWithStep(history, state);

  if (stepped.past === history.past) {
    return stepped === history ? history : { ...history, present: stepped.present };
  }

  const shown = new Set(history.present.drawing);
  const added = measureByState(history, { by: 1, state: history.present }, shown);
  const cleared = history.future.reduce(
    (before, future) => measureByState(before, { by: -1, state: future }, shown),
    added,
  );
  const measure = measureWithPresent(cleared, history.present.drawing, stepped.present.drawing);

  return { future: stepped.future, past: stepped.past, present: stepped.present, ...measure };
}
