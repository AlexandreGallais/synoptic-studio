import { areSameItems } from "./are-same-items";
import { sharedDrawing } from "./shared-drawing";

import type { History } from "./history";
import type { HistoryState } from "./history-state";

/**
 * The history once a finished action is recorded: its drawing shared with the present
 * (`sharedDrawing`); the same drawing makes no step, the present's selection replaced; another
 * pushes the present onto the past, the new state the present and nothing to come (ADR-0095,
 * decisions 1 and 3; `REF-REDUX-UNDO`).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param history - the history before the action
 * @param state - the drawing and the selection once the action finished
 * @returns the history given when nothing changed, otherwise a new history
 * @see RULE-interaction-7-navigation-and-history
 */
export function historyWithStep<T>(history: History<T>, state: HistoryState<T>): History<T> {
  const { drawing: present, selection: shown } = history.present;
  const drawing = sharedDrawing(state.drawing, present);

  if (!areSameItems(drawing, present)) {
    return { future: [], past: [...history.past, history.present], present: { ...state, drawing } };
  }

  return areSameItems(state.selection, shown)
    ? history
    : { ...history, present: { drawing: present, selection: state.selection } };
}
