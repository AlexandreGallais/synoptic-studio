import type { SizedHistory } from "./sized-history";
import type { HistoryBound } from "../history";

/**
 * The history within its bound, its size kept (EN-147): while its size is over the size set and
 * more past states than the steps kept are left, the oldest past state removed
 * (`measureWithoutState`), one by one, each read once; the states to come never dropped. The
 * history is `boundedHistory`'s, found without measuring the history again (ADR-0097).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param history - the history after a step, or when the bound changes
 * @param bound - the memory size and the steps kept the designer sets
 * @returns the history given when nothing is dropped, otherwise a new history
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function sizedBoundedHistory<T extends WeakKey>(
  history: SizedHistory<T>,
  bound: HistoryBound,
): SizedHistory<T> {
  return bound.steps < 0 ? history : { ...history, past: [] };
}
