import type { History } from "./history";

/**
 * The history once undone: the last past state the present, the present the first to come;
 * nothing past, the history given (ADR-0095, decision 1; `REF-REDUX-UNDO`).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param history - the history before the undo
 * @returns a new history, or the one given when nothing is past
 * @see RULE-interaction-7-navigation-and-history
 */
export function undoneHistory<T>(history: History<T>): History<T> {
  const last = history.past.at(-1);

  return last === undefined
    ? history
    : {
        future: [history.present, ...history.future],
        past: history.past.slice(0, -1),
        present: last,
      };
}
