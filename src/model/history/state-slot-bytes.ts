import { SLOT_BYTES } from "../held-shapes";

import type { HistoryState } from "./history-state";

/**
 * The bytes of a state's slots: 8 for each reference to a shape and each index of its
 * selection, the slot of a tagged value without pointer compression, an upper bound (`SLOT_BYTES`,
 * `REF-V8-POINTER-COMPRESSION`).
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param state - the state counted
 * @returns 8 times its shapes and its indices
 * @see RULE-interaction-7-navigation-and-history
 */
export function stateSlotBytes<T>(state: HistoryState<T>): number {
  return (state.drawing.length + state.selection.length) * SLOT_BYTES;
}
