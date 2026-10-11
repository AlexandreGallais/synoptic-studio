import { selectionAfterClick } from "./selection-after-click";
import { selectionAfterToggle } from "./selection-after-toggle";

import type { ClickKeys } from "./click-keys";
import type { VertexSelection } from "./vertex-selection";

/**
 * Selection after a click on the handle of a vertex on the drawing: Shift or Ctrl (Cmd) adds or
 * removes the vertex; a drawing has no range (`docs/domain/interaction.md` §2–3). A plain click selects
 * it alone (Product Owner, F03.AC5). Only a click without Shift moves the start of the next
 * range (note 0006).
 *
 * @kind domain
 * @param selection - selection before the click
 * @param vertex - number of the vertex of the handle, from 0
 * @param keys - modifier keys held
 * @returns the new selection
 * @see docs/domain/interaction.md#_3-node-editing
 */
export function selectionAfterHandleClick(
  selection: VertexSelection,
  vertex: number,
  keys: ClickKeys,
): VertexSelection {
  if (keys.shift) {
    // Shift adds or removes the vertex, but a range still starts where it did.
    return { ...selectionAfterToggle(selection, vertex), start: selection.start };
  }

  return keys.toggle ? selectionAfterToggle(selection, vertex) : selectionAfterClick(vertex);
}
