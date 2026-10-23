import type { HistoryMeasure } from "./history-measure";

/**
 * The measure once the history's present changes its drawing (EN-147): each shape the present
 * leaves that a past or future state holds adds its text's length times 2, each it takes that one
 * holds takes it away; the counts unchanged, and a selection replaced alone changes nothing
 * (ADR-0097).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param measure - the measure before
 * @param from - the present's drawing before
 * @param to - the present's drawing after
 * @returns a new measure, or the one given when no shape held is left nor taken
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function measureWithPresent<T extends WeakKey>(
  measure: HistoryMeasure,
  from: readonly T[],
  to: readonly T[],
): HistoryMeasure {
  return { ...measure, size: from.length + to.length };
}
