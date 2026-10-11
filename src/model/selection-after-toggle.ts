import type { VertexSelection } from "./vertex-selection";

/**
 * Selection after Ctrl + click (Cmd on macOS), or Shift + click on the drawing: the vertex is
 * added when it was not selected, removed otherwise, and becomes the start of the next range
 * (F03.AC5, `REF-APPLE-SELECT`).
 *
 * @kind domain
 * @param selection - selection before the click
 * @param vertex - number of the vertex clicked, from 0
 * @returns the new selection, ascending
 * @see REF-APPLE-SELECT
 */
export function selectionAfterToggle(selection: VertexSelection, vertex: number): VertexSelection {
  const selected = selection.selected.includes(vertex)
    ? selection.selected.filter((other) => other !== vertex)
    : [...selection.selected, vertex].toSorted((left, right) => left - right);

  return { selected, start: vertex };
}
