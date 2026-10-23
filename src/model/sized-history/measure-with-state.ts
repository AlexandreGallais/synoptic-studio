import { CODE_UNIT_BYTES, SLOT_BYTES, heldCount, shapeKey, withHeldCount } from "../held-shapes";
import { shapeTextLength } from "../history";

import type { HistoryMeasure } from "./history-measure";
import type { HistoryState } from "../history";

/**
 * The measure once a state joins the past or the states to come (EN-147): each shape it holds
 * held by one state more, counted once; one no other state held and the present does not hold
 * adds its text's length times 2 (`shapeTextLength`, `CODE_UNIT_BYTES`); and 8 bytes for each of
 * its references and indices (`SLOT_BYTES`), as `historySize` counts them (ADR-0097).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param measure - the measure before
 * @param state - the state added
 * @param present - the shapes of the history's present
 * @returns a new measure
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function measureWithState<T extends WeakKey>(
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
      const isNew = count === 0 && !present.has(shape);
      const bytes = isNew ? shapeTextLength(shape) * CODE_UNIT_BYTES : 0;

      return { held: withHeldCount(before.held, key, count + 1), size: before.size + bytes };
    },
    { held: measure.held, size: measure.size + slots },
  );
}
