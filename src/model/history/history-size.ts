import { shapeBytes } from "../shape-memo";

import { stateSlotBytes } from "./state-slot-bytes";

import type { History } from "./history";

/**
 * The bytes a history holds beyond its present (F40): each shape a past or future state holds and
 * the present does not, counted once, its JSON text's length times 2 (`shapeBytes`), a string
 * being a sequence of 16-bit code units (`REF-ECMA-262-STRING-TYPE`); plus 8 bytes for each
 * reference to a shape and each index of a selection those states hold (`stateSlotBytes`), the
 * slot of a tagged value without pointer compression, an upper bound, V8 holding it in 4 bytes
 * with compression (`REF-V8-POINTER-COMPRESSION`). A count of the data, not the engine's bytes
 * (ADR-0095, decision 4).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each, objects never changed once made
 * @param history - the history measured
 * @returns its size in bytes, 0 with nothing past nor to come
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function historySize<T extends WeakKey>(history: History<T>): number {
  const kept = [...history.past, ...history.future];
  const held = new Set(kept.flatMap((state) => state.drawing));
  const shapes = held.difference(new Set(history.present.drawing));
  const texts = shapes.values().reduce((sum, shape) => sum + shapeBytes(shape), 0);

  return kept.reduce((sum, state) => sum + stateSlotBytes(state), texts);
}
