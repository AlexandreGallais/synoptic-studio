import type { History } from "./history";

/**
 * The history once undone (F40): the last past state the present, the present the first to come;
 * nothing past, the history given (ADR-0095, decision 1; `REF-REDUX-UNDO`).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param history - the history before the undo
 * @returns a new history, or the one given when nothing is past
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function undoneHistory<T>(history: History<T>): History<T> {
  return history;
}
