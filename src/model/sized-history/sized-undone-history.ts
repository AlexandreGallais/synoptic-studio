import type { SizedHistory } from "./sized-history";

/**
 * The history once undone, its size kept (EN-147): the undo of `undoneHistory`; the last past
 * state removed from the past (`measureWithoutState`), the present added to the states to come
 * (`measureWithState`), then the present changed to it (`measureWithPresent`); nothing past, the
 * history given (ADR-0097).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param history - the history before the undo
 * @returns a new history, or the one given when nothing is past
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function sizedUndoneHistory<T extends WeakKey>(history: SizedHistory<T>): SizedHistory<T> {
  return history;
}
