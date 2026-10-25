import { heldCount, shapeKey, withHeldCount } from "../held-shapes";
import { stateSlotBytes } from "../history";
import { shapeBytes } from "../shape-memo";

import type { HistoryMeasure } from "./history-measure";
import type { StateChange } from "./state-change";

/**
 * How a shape's count changes, and the shapes of the history's present.
 *
 * @template T - type of the shapes, or of what is kept of each
 */
type ShapeChange<T> = {
  /** 1 when a state holding the shape joins, −1 when it leaves. */
  readonly by: 1 | -1;
  /** The shapes of the history's present. */
  readonly present: ReadonlySet<T>;
};

/**
 * The measure once a shape is held by one state more or less: its count changed; going from no
 * state to one, or from one to none, while the present does not hold it, its bytes added or taken
 * away (`shapeBytes`).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param before - the measure so far, the state's shapes before this one counted
 * @param shape - the shape a state joining or leaving holds
 * @param change - 1 or −1, and the shapes of the present
 * @returns a new measure
 * @see RULE-interaction-7-navigation-and-history
 */
function measureByShape<T extends WeakKey>(
  before: HistoryMeasure,
  shape: T,
  change: ShapeChange<T>,
): HistoryMeasure {
  const key = shapeKey(shape);
  const count = heldCount(before.held, key);
  const after = count + change.by;
  const isCrossing = (count === 0 || after === 0) && !change.present.has(shape);
  const bytes = isCrossing ? change.by * shapeBytes(shape) : 0;

  return { held: withHeldCount(before.held, key, after), size: before.size + bytes };
}

/**
 * The measure once a state joins or leaves the past or the states to come (EN-147): each shape it
 * holds held by one state more as it joins, one less as it leaves, counted once; a shape going
 * from no state to one, or from one to none, that the present does not hold adds or takes away
 * its text's length times 2 (`measureByShape`); and 8 bytes for each of its references and
 * indices (`stateSlotBytes`), added or taken away, as `historySize` counts them (ADR-0097).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param measure - the measure before
 * @param change - the state, and 1 as it joins or −1 as it leaves
 * @param present - the shapes of the history's present
 * @returns a new measure
 * @see RULE-interaction-7-navigation-and-history
 */
export function measureByState<T extends WeakKey>(
  measure: HistoryMeasure,
  change: StateChange<T>,
  present: ReadonlySet<T>,
): HistoryMeasure {
  const slots = change.by * stateSlotBytes(change.state);
  const shapes = new Set(change.state.drawing);

  return shapes
    .values()
    .reduce((before, shape) => measureByShape(before, shape, { by: change.by, present }), {
      held: measure.held,
      size: measure.size + slots,
    });
}
