import type { History } from "./history";

/**
 * The history once redone: the first state to come the present, the present the last past;
 * nothing to come, the history given (ADR-0095, decision 1; `REF-REDUX-UNDO`).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param history - the history before the redo
 * @returns a new history, or the one given when nothing is to come
 * @see RULE-interaction-7-navigation-and-history
 */
export function redoneHistory<T>(history: History<T>): History<T> {
  const [next, ...rest] = history.future;

  return next === undefined
    ? history
    : { future: rest, past: [...history.past, history.present], present: next };
}
