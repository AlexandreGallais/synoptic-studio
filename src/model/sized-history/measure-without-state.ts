import { CODE_UNIT_BYTES, SLOT_BYTES, heldCount, shapeKey, withHeldCount } from "../held-shapes";
import { shapeTextLength } from "../history";

import type { HistoryMeasure } from "./history-measure";
import type { HistoryState } from "../history";

/**
 * The measure once a state leaves the past or the states to come (EN-147), the reverse of
 * `measureWithState`: each shape it holds held by one state less; one it alone held that the
 * present does not hold takes its text's length times 2 away; and 8 bytes for each of its
 * references and indices (ADR-0097).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param measure - the measure before, counting the state
 * @param state - the state removed
 * @param present - the shapes of the history's present
 * @returns a new measure
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function measureWithoutState<T extends WeakKey>(
  measure: HistoryMeasure,
  state: HistoryState<T>,
  present: ReadonlySet<T>,
): HistoryMeasure {
  const slots = (state.drawing.length + state.selection.length) * SLOT_BYTES;

  const shapes = new Set(state.drawing);

  return shapes.values().reduce(
    (before, shape) => {
      const key = shapeKey(shape);
      const count = heldCount(before.held, key);
      const isLast = count === 1 && !present.has(shape);
      const bytes = isLast ? shapeTextLength(shape) * CODE_UNIT_BYTES : 0;

      return { held: withHeldCount(before.held, key, count - 1), size: before.size - bytes };
    },
    { held: measure.held, size: measure.size - slots },
  );
}
