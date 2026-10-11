import { selectionAfterClick } from "./selection-after-click";

import type { VertexSelection } from "./vertex-selection";

/**
 * Selection after Shift + click on a row of the list: every vertex from the start of the range
 * to the one clicked, both included, replacing the selection; the start stays. Without a start
 * yet, a plain click (F03.AC5, `REF-APPLE-SELECT`, `REF-WAI-APG-LISTBOX`).
 *
 * @kind domain
 * @param selection - selection before the click
 * @param vertex - number of the vertex clicked, from 0
 * @returns the new selection, ascending
 * @see REF-APPLE-SELECT
 */
export function selectionAfterRange(selection: VertexSelection, vertex: number): VertexSelection {
  const { start } = selection;

  if (start === undefined) {
    return selectionAfterClick(vertex);
  }

  const first = Math.min(start, vertex);
  const length = Math.abs(vertex - start) + 1;

  return { selected: Array.from({ length }, (unused, index) => first + index), start };
}
