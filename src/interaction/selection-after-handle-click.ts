import { selectionAfterClick } from "./selection-after-click";
import { selectionAfterToggle } from "./selection-after-toggle";

import type { ClickKeys } from "./click-keys";
import type { VertexSelection } from "./vertex-selection";

/**
 * Selection after a click on the handle of a vertex on the drawing: Shift or Ctrl (Cmd) adds or
 * removes the vertex, as in Figma and Inkscape; a drawing has no range. A plain click selects
 * it alone (Product Owner, F03.AC5).
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
  return keys.shift || keys.toggle
    ? selectionAfterToggle(selection, vertex)
    : selectionAfterClick(vertex);
}
