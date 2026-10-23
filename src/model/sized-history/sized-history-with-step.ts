import type { SizedHistory } from "./sized-history";
import type { HistoryState } from "../history";

/**
 * The history once a finished action is recorded, its size kept (EN-147): the step of
 * `historyWithStep`; when it takes one, the present added to the past (`measureWithState`), each
 * state to come removed (`measureWithoutState`), then the present changed
 * (`measureWithPresent`); a selection replaced alone keeps the measure (ADR-0097).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param history - the history before the action
 * @param state - the drawing and the selection once the action finished
 * @returns the history given when nothing changed, otherwise a new history
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function sizedHistoryWithStep<T extends WeakKey>(
  history: SizedHistory<T>,
  state: HistoryState<T>,
): SizedHistory<T> {
  return { ...history, present: state };
}
