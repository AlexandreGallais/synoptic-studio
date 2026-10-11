import { selectionAfterClick } from "./selection-after-click";
import { selectionAfterRange } from "./selection-after-range";
import { selectionAfterToggle } from "./selection-after-toggle";

import type { ClickKeys } from "./click-keys";
import type { VertexSelection } from "./vertex-selection";

/**
 * Selection after a click on a row of the list of points, as in any list: Shift selects a
 * range, Ctrl (Cmd) adds or removes the vertex, a plain click selects it alone (F03.AC5,
 * `REF-APPLE-SELECT`).
 *
 * @kind domain
 * @param selection - selection before the click
 * @param vertex - number of the vertex of the row, from 0
 * @param keys - modifier keys held
 * @returns the new selection
 * @see REF-APPLE-SELECT
 */
export function selectionAfterRowClick(
  selection: VertexSelection,
  vertex: number,
  keys: ClickKeys,
): VertexSelection {
  if (keys.shift) {
    return selectionAfterRange(selection, vertex);
  }

  return keys.toggle ? selectionAfterToggle(selection, vertex) : selectionAfterClick(vertex);
}
