import { heldCount, shapeKey } from "../held-shapes";
import { shapeBytes } from "../shape-memo";

import type { HistoryMeasure } from "./history-measure";

/**
 * The bytes of the shapes a past or future state holds among some, each counted once.
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param measure - the measure whose counts are read
 * @param shapes - the shapes looked at
 * @returns their bytes (`shapeBytes`), summed over those held
 * @see RULE-interaction-7-navigation-and-history
 */
function heldBytes<T extends WeakKey>(measure: HistoryMeasure, shapes: ReadonlySet<T>): number {
  return shapes
    .values()
    .filter((shape) => heldCount(measure.held, shapeKey(shape)) > 0)
    .reduce((sum, shape) => sum + shapeBytes(shape), 0);
}

/**
 * The measure once the history's present changes its drawing: each shape the present
 * leaves that a past or future state holds adds its text's length times 2, each it takes that one
 * holds takes it away (`heldBytes`); the counts unchanged, and a selection replaced alone changes
 * nothing (ADR-0097).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param measure - the measure before
 * @param from - the present's drawing before
 * @param to - the present's drawing after
 * @returns a new measure, or the one given when its size is unchanged
 * @see RULE-interaction-7-navigation-and-history
 */
export function measureWithPresent<T extends WeakKey>(
  measure: HistoryMeasure,
  from: readonly T[],
  to: readonly T[],
): HistoryMeasure {
  const before = new Set(from);
  const after = new Set(to);
  const left = heldBytes(measure, before.difference(after));
  const taken = heldBytes(measure, after.difference(before));

  return left === taken ? measure : { held: measure.held, size: measure.size + left - taken };
}
