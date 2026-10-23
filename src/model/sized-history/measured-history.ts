import type { SizedHistory } from "./sized-history";
import type { History } from "../history";

/**
 * A history with its size counted once, from its states (EN-147): each past and future state
 * added to an empty measure (`measureWithState`), so that the size is `historySize`'s; the page's
 * history starts from it, nothing past nor to come (ADR-0097).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param history - the states, past, present and to come
 * @returns a new history, its states those given
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function measuredHistory<T extends WeakKey>(history: History<T>): SizedHistory<T> {
  return { ...history, held: { depth: 0, nodes: [] }, size: -1 };
}
